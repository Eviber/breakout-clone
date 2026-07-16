use bevy::math::bounding::Aabb2d;
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
    keyboard_input: Res<ButtonInput<KeyCode>>,
    ball: Single<Entity, (With<Ball>, Without<Velocity>)>,
) {
    if keyboard_input.pressed(KeyCode::Space) || keyboard_input.pressed(KeyCode::ArrowUp) {
        let mut ball_velocity = BALL_BASE_VELOCITY;
        let angle = if event.x_speed < 0. {
            45f32.to_radians()
        } else {
            -45f32.to_radians()
        };
        ball_velocity = Vec2::from_angle(angle).rotate(ball_velocity);
        commands.entity(*ball).insert(Velocity(ball_velocity));
    }
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

// TODO: Implement real corner collision detection, instead of just inflating the collider.
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
        let w = (other_collider.0.half_size.x + BALL_SIZE) * 2.;
        let h = (other_collider.0.half_size.y + BALL_SIZE) * 2.;
        let other_collider = Rectangle::new(w, h);
        let other_collider = Aabb2d::new(other_position.0, other_collider.half_size);

        if let Some(dist) = ray_cast.aabb_intersection_at(&other_collider) {
            let collision_point = old_pos + dir * dist;

            if closest_collision
                .as_ref()
                .is_none_or(|c| c.remaining_distance > speed - dist)
            {
                closest_collision = Some(BallCollision {
                    entity,
                    pos: collision_point,
                    remaining_distance: speed - dist,
                });
            }
        }
    }
    if let Some(collision) = closest_collision {
        commands.trigger(collision);
    }
}

fn move_locked_ball(
    mut ball: Single<&mut Position, (With<Ball>, Without<Velocity>)>,
    paddle: Single<&Position, (With<Paddle>, Without<Ball>)>,
) {
    ball.0 = paddle.0 + vec2(0., 25.);
}
