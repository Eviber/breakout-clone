use bevy::input::mouse::AccumulatedMouseMotion;
use bevy::math::bounding::{Aabb2d, AabbCast2d};
use bevy::prelude::*;

use super::GameState;
use super::GameSystemSet;
use super::ball::{Ball, BallCollision, LaunchRequested};
use super::blocks::Gutter;
use super::physics::{Collider, Position, Velocity};
use crate::AppState;

pub fn plugin(app: &mut App) {
    app.add_systems(
        FixedUpdate,
        (
            handle_player_input.in_set(GameSystemSet::Input),
            constrain_paddle_position.in_set(GameSystemSet::PreCollision),
        )
            .run_if(in_state(GameState::Running)),
    );
}

#[derive(SceneComponent, Clone, Default)]
#[require(Velocity)]
pub struct Paddle;

pub const PADDLE_SHAPE: Rectangle = Rectangle::new(150., 20.);
pub const PADDLE_COLOR: Color = Color::srgb(0., 1., 0.);
pub const PADDLE_Y: f32 = -300.;

impl Paddle {
    pub fn scene() -> impl Scene {
        let x = 0.;
        let y = PADDLE_Y;
        bsn! {
            Name("Paddle")
            Position(vec2(x,y))
            Collider(PADDLE_SHAPE)
            Mesh2d(asset_value(PADDLE_SHAPE))
            MeshMaterial2d<ColorMaterial>(asset_value(PADDLE_COLOR))
            DespawnOnExit<AppState>(AppState::InGame)
            on(collide_paddle)
        }
    }
}

// TODO: Add vertical paddle movement
// TODO: Add paddle inclination based on movement?
fn handle_player_input(
    mut commands: Commands,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mouse_motion: Res<AccumulatedMouseMotion>,
    mut paddle_velocity: Single<&mut Velocity, With<Paddle>>,
) {
    paddle_velocity.0.x = mouse_motion.delta.x;
    if mouse_button.just_pressed(MouseButton::Left) {
        info!("Launching ball with x speed: {}", paddle_velocity.0.x);
        commands.trigger(LaunchRequested {
            x_speed: paddle_velocity.0.x,
        });
    }
}

fn constrain_paddle_position(
    paddles: Single<
        (&mut Position, &Velocity, &Collider),
        (With<Paddle>, Without<Gutter>, Without<Ball>),
    >,
    gutters: Query<(&Position, &Collider), (With<Gutter>, Without<Paddle>, Without<Ball>)>,
) {
    let (mut paddle_position, paddle_velocity, paddle_collider) = paddles.into_inner();
    let previous_pos = paddle_position.0 - paddle_velocity.0;
    let Ok(dir) = Dir2::new(paddle_position.0 - previous_pos) else {
        return;
    };
    let speed = (paddle_position.0 - previous_pos).length();
    let paddle_aabb = Aabb2d::new(Vec2::ZERO, paddle_collider.half_size());
    let paddle_cast = AabbCast2d::new(paddle_aabb, previous_pos, dir, speed);
    let epsilon_pos = previous_pos + dir * 0.001;
    let epsilon_speed = speed - 0.001;
    let epsilon_paddle_cast = AabbCast2d::new(paddle_aabb, epsilon_pos, dir, epsilon_speed);

    for (gutter_position, gutter_collider) in &gutters {
        let gutter_aabb = Aabb2d::new(gutter_position.0, gutter_collider.half_size());

        let Some(dist) = paddle_cast.aabb_collision_at(gutter_aabb) else {
            continue;
        };
        let epsilon_collision = epsilon_paddle_cast.aabb_collision_at(gutter_aabb);
        if dist <= 0. && epsilon_collision.is_none_or(|d| d > 0.) {
            continue;
        }

        paddle_position.0.x = previous_pos.x + (dist * paddle_velocity.0.x.signum());
    }
}

// TODO: Transfer paddle velocity to ball
fn collide_paddle(
    event: On<BallCollision>,
    mut commands: Commands,
    ball: Single<(&mut Velocity, &mut Position), With<Ball>>,
    paddle: Single<(&Position, &Collider, &Velocity), (With<Paddle>, Without<Ball>)>,
) {
    let (mut ball_velocity, mut ball_position) = ball.into_inner();
    let (paddle_position, paddle_collider, paddle_velocity) = *paddle;
    let x1 = paddle_position.0.x - (paddle_collider.0.half_size.x * 3. / 4.);
    let x2 = paddle_position.0.x + (paddle_collider.0.half_size.x * 3. / 4.);
    if x1 <= ball_position.0.x && ball_position.0.x <= x2 {
        ball_velocity.0.y = -ball_velocity.0.y;
        if paddle_velocity.0.x < 0. {
            let angle = 5f32.to_radians();
            ball_velocity.0 = Vec2::from_angle(angle).rotate(ball_velocity.0);
        } else if paddle_velocity.0.x > 0. {
            let angle = -5f32.to_radians();
            ball_velocity.0 = Vec2::from_angle(angle).rotate(ball_velocity.0);
        }
    } else {
        let paddle_pos = Vec2 {
            x: paddle_position.0.x,
            y: paddle_position.0.y + paddle_collider.half_size().y - paddle_collider.half_size().x,
        };
        let dir = (ball_position.0 - paddle_pos).normalize();
        let speed = ball_velocity.0.length();
        ball_velocity.0 = dir * speed;
    }
    ball_position.0 = event.pos + ball_velocity.0.normalize() * event.remaining_distance;
    commands.trigger(super::ball::BallMoved {
        from: event.pos,
        rebound_from: Some(event.entity),
    });
}
