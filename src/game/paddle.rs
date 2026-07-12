use bevy::math::bounding::Aabb2d;
use bevy::prelude::*;

use super::GameState;
use super::GameSystemSet;
use super::ball::{Ball, BallCollision};
use super::blocks::Gutter;
use super::collision;
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
pub const PADDLE_SPEED: f32 = 5.;
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

fn handle_player_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut paddle_velocity: Single<&mut Velocity, With<Paddle>>,
) {
    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        paddle_velocity.0.x = -PADDLE_SPEED;
    } else if keyboard_input.pressed(KeyCode::ArrowRight) {
        paddle_velocity.0.x = PADDLE_SPEED;
    } else {
        paddle_velocity.0.x = 0.;
    }
}

fn constrain_paddle_position(
    paddles: Single<(&mut Position, &Collider), (With<Paddle>, Without<Gutter>, Without<Ball>)>,
    gutters: Query<(&Position, &Collider), (With<Gutter>, Without<Paddle>, Without<Ball>)>,
) {
    let (mut paddle_position, paddle_collider) = paddles.into_inner();
    for (gutter_position, gutter_collider) in &gutters {
        let paddle_aabb = Aabb2d::new(paddle_position.0, paddle_collider.half_size());
        let gutter_aabb = Aabb2d::new(gutter_position.0, gutter_collider.half_size());

        if let Some(collision) = collision::collide_with_side(paddle_aabb, gutter_aabb) {
            match collision {
                collision::Collision::Left => {
                    paddle_position.0.x = gutter_position.0.x
                        - gutter_collider.half_size().x
                        - paddle_collider.half_size().x;
                }
                collision::Collision::Right => {
                    paddle_position.0.x = gutter_position.0.x
                        + gutter_collider.half_size().x
                        + paddle_collider.half_size().x;
                }
                _ => {}
            }
        }
    }
}

// TODO: Add ball effect
fn collide_paddle(
    _event: On<BallCollision>,
    ball: Single<(&mut Velocity, &Position), With<Ball>>,
    paddle: Single<(&Position, &Collider), (With<Paddle>, Without<Ball>)>,
) {
    let (mut ball_velocity, ball_position) = ball.into_inner();
    let (paddle_position, paddle_collider) = *paddle;
    let x1 = paddle_position.0.x - (paddle_collider.0.half_size.x * 3. / 4.);
    let x2 = paddle_position.0.x + (paddle_collider.0.half_size.x * 3. / 4.);
    if x1 <= ball_position.0.x && ball_position.0.x <= x2 {
        ball_velocity.0.y = -ball_velocity.0.y;
        return;
    }
    let paddle_pos = Vec2 {
        x: paddle_position.0.x,
        y: paddle_position.0.y + paddle_collider.half_size().y - paddle_collider.half_size().x,
    };
    let dir = (ball_position.0 - paddle_pos).normalize();
    let speed = ball_velocity.0.length();
    ball_velocity.0 = dir * speed;
}
