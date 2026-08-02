use bevy::math::bounding::Aabb2d;
use bevy::math::bounding::BoundingCircle;
use bevy::math::bounding::RayCast2d;
use bevy::prelude::*;

use super::GameState;
use super::GameSystemSet;
use super::Lives;
use super::paddle::{PADDLE_Y, Paddle};
use super::physics::{Collider, Position, Velocity};
use crate::AppState;

pub fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(AppState::MainMenu),
        (move_locked_ball.in_set(GameSystemSet::PostCollision),),
    )
    .add_systems(
        FixedUpdate,
        (
            move_locked_ball.in_set(GameSystemSet::PostCollision),
            trigger_ball_moved.in_set(GameSystemSet::Collision),
            handle_lost_ball.in_set(GameSystemSet::Collision),
        )
            .run_if(in_state(GameState::Running)),
    )
    .add_observer(launch_ball)
    .add_observer(handle_collisions);
}

#[derive(EntityEvent)]
pub struct BallCollision {
    pub entity: Entity,
    pub pos: Vec2,
    pub remaining_distance: f32,
}

#[derive(Event)]
pub struct BallMoved {
    pub from: Vec2,
    pub rebound_from: Option<Entity>,
}

#[derive(SceneComponent, Clone, Default)]
pub struct Ball;

pub const BALL_SIZE: f32 = 10.;
pub const BALL_SHAPE: Circle = Circle::new(BALL_SIZE);
pub const BALL_COLOR: Color = Color::srgb(1., 0., 0.);
pub const BALL_SPEED: f32 = 4.;
pub const BALL_BASE_POS: Vec2 = vec2(0., -200.);
pub const BALL_BASE_VELOCITY: Vec2 = vec2(0., BALL_SPEED);

impl Ball {
    pub fn scene() -> impl Scene {
        bsn! {
            Name("Ball")
            Position(BALL_BASE_POS)
            Collider(Rectangle::new(BALL_SIZE, BALL_SIZE))
            Mesh2d(asset_value(BALL_SHAPE))
            MeshMaterial2d<ColorMaterial>(asset_value(BALL_COLOR))
            DespawnOnExit<AppState>(AppState::InGame)
        }
    }
}

#[derive(Event)]
pub struct LaunchRequested {
    pub x_speed: f32,
}

fn launch_ball(
    event: On<LaunchRequested>,
    mut commands: Commands,
    ball: Single<Entity, (With<Ball>, Without<Velocity>)>,
) {
    let mut ball_velocity = BALL_BASE_VELOCITY;
    let angle = if event.x_speed < 0. {
        45f32.to_radians()
    } else {
        -45f32.to_radians()
    };
    ball_velocity = Vec2::from_angle(angle).rotate(ball_velocity);
    commands.entity(*ball).insert(Velocity(ball_velocity));
}

fn handle_lost_ball(
    mut commands: Commands,
    ball: Single<(Entity, &Position), With<Ball>>,
    mut lives: ResMut<Lives>,
) {
    let (ball_entity, ball_position) = ball.into_inner();
    if ball_position.0.y < PADDLE_Y - 100. {
        lives.0 -= 1;
        commands.entity(ball_entity).remove::<Velocity>();
    }
}

fn trigger_ball_moved(mut commands: Commands, ball: Single<(&Position, &Velocity), With<Ball>>) {
    let (ball_position, ball_velocity) = ball.into_inner();
    let old_pos = ball_position.0 - ball_velocity.0;
    commands.trigger(BallMoved {
        from: old_pos,
        rebound_from: None,
    });
}

fn handle_collisions(
    event: On<BallMoved>,
    mut commands: Commands,
    ball: Single<&Position, With<Ball>>,
    other_things: Query<(&Position, &Collider, Entity), Without<Ball>>,
) {
    let ball_position = ball.into_inner();
    let old_pos = event.from;
    let dir = Dir2::new(ball_position.0 - old_pos).unwrap();
    let speed = (ball_position.0 - old_pos).length();
    let ray_cast = RayCast2d::new(old_pos, dir, speed);

    let mut closest_collision: Option<BallCollision> = None;

    for (other_position, other_collider, entity) in &other_things {
        if event.rebound_from.is_some_and(|e| e == entity) {
            // Do not collide with the entity we just collided with.
            continue;
        }

        let dist = ball_to_collider_collision(&ray_cast, other_position, other_collider);
        let Some(dist) = dist else {
            continue;
        };

        let collision_point = old_pos + dir * dist;
        if closest_collision
            .as_ref()
            .is_none_or(|c| c.remaining_distance < speed - dist)
        {
            closest_collision = Some(BallCollision {
                entity,
                pos: collision_point,
                remaining_distance: speed - dist,
            });
        }
    }
    if let Some(collision) = closest_collision {
        commands.trigger(collision);
    }
}

fn ball_to_collider_collision(
    ray_cast: &RayCast2d,
    position: &Position,
    collider: &Collider,
) -> Option<f32> {
    let e_ray_cast = epsilon_ray_cast(ray_cast);

    let expanded_hw = collider.half_size().x + BALL_SIZE;
    let expanded_hh = collider.half_size().y + BALL_SIZE;

    let mut collisions = Vec::new();

    let aabb = aabb_from_half(position, collider.half_size().x, expanded_hh);
    if let Some(dist) = aabb_epsilon_collision(ray_cast, &e_ray_cast, &aabb) {
        collisions.push(dist);
    }
    let aabb = aabb_from_half(position, expanded_hw, collider.half_size().y);
    if let Some(dist) = aabb_epsilon_collision(ray_cast, &e_ray_cast, &aabb) {
        collisions.push(dist);
    }

    let aabb = aabb_from_half(position, collider.half_size().x, collider.half_size().y);
    let center = aabb.min;
    let bc = BoundingCircle::new(center, BALL_SIZE);
    if let Some(dist) = bc_epsilon_collision(ray_cast, &e_ray_cast, &bc) {
        collisions.push(dist);
    }
    let center = Vec2::new(aabb.min.x, aabb.max.y);
    let bc = BoundingCircle::new(center, BALL_SIZE);
    if let Some(dist) = bc_epsilon_collision(ray_cast, &e_ray_cast, &bc) {
        collisions.push(dist);
    }
    let center = Vec2::new(aabb.max.x, aabb.min.y);
    let bc = BoundingCircle::new(center, BALL_SIZE);
    if let Some(dist) = bc_epsilon_collision(ray_cast, &e_ray_cast, &bc) {
        collisions.push(dist);
    }
    let center = aabb.max;
    let bc = BoundingCircle::new(center, BALL_SIZE);
    if let Some(dist) = bc_epsilon_collision(ray_cast, &e_ray_cast, &bc) {
        collisions.push(dist);
    }

    let dist = collisions.into_iter().min_by(|a, b| a.total_cmp(b))?;
    Some(dist)
}

fn bc_epsilon_collision(
    ray_cast: &RayCast2d,
    e_ray_cast: &RayCast2d,
    bc: &BoundingCircle,
) -> Option<f32> {
    let dist = ray_cast.circle_intersection_at(bc)?;
    if dist > 0. {
        return Some(dist);
    }
    // Only collide if there's a collision at 0.0 with epsilon too
    let e_dist = e_ray_cast.circle_intersection_at(bc)?;
    (e_dist <= 0.).then_some(dist)
}

fn aabb_epsilon_collision(
    ray_cast: &RayCast2d,
    e_ray_cast: &RayCast2d,
    collider: &Aabb2d,
) -> Option<f32> {
    let dist = ray_cast.aabb_intersection_at(collider)?;
    if dist > 0. {
        return Some(dist);
    }
    // Only collide if there's a collision at 0.0 with epsilon too
    let e_dist = e_ray_cast.aabb_intersection_at(collider)?;
    (e_dist <= 0.).then_some(dist)
}

fn aabb_from_half(position: &Position, hw: f32, hh: f32) -> Aabb2d {
    let half_size = Vec2::new(hw, hh);
    Aabb2d::new(position.0, half_size)
}

fn epsilon_ray_cast(ray_cast: &RayCast2d) -> RayCast2d {
    let e_origin = ray_cast.ray.origin + ray_cast.ray.direction * 0.001;
    let e_dir = ray_cast.ray.direction;
    let e_len = ray_cast.max - 0.001;
    RayCast2d::new(e_origin, e_dir, e_len)
}

fn move_locked_ball(
    mut ball: Single<&mut Position, (With<Ball>, Without<Velocity>)>,
    paddle: Single<&Position, (With<Paddle>, Without<Ball>)>,
) {
    ball.0 = paddle.0 + vec2(0., 25.);
}
