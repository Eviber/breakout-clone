use bevy::math::bounding::Aabb2d;
use bevy::prelude::*;

use super::GameState;
use super::GameSystemSet;
use super::Lives;
use super::collision;
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
            launch_ball.in_set(GameSystemSet::Input),
            move_locked_ball.in_set(GameSystemSet::PostCollision),
            handle_collisions.in_set(GameSystemSet::Collision),
            handle_lost_ball.in_set(GameSystemSet::Collision),
        )
            .run_if(in_state(GameState::Running)),
    );
}

#[derive(EntityEvent)]
pub struct BallCollision {
    pub entity: Entity,
    pub side: collision::Collision,
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

fn launch_ball(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    ball: Single<Entity, (With<Ball>, Without<Velocity>)>,
) {
    if keyboard_input.pressed(KeyCode::Space) || keyboard_input.pressed(KeyCode::ArrowUp) {
        commands.entity(*ball).insert(Velocity(BALL_BASE_VELOCITY));
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

// TODO: Count multiple collisions in the same tick, except for bricks
fn handle_collisions(
    mut commands: Commands,
    ball: Single<(&Position, &Collider), With<Ball>>,
    other_things: Query<(&Position, &Collider, Entity), Without<Ball>>,
) {
    let (ball_position, ball_collider) = ball.into_inner();

    for (other_position, other_collider, entity) in &other_things {
        let Some(collision) = collision::collide_with_side(
            Aabb2d::new(ball_position.0, ball_collider.half_size()),
            Aabb2d::new(other_position.0, other_collider.half_size()),
        ) else {
            continue;
        };
        commands.trigger(BallCollision {
            entity,
            side: collision,
        });
        break;
    }
}

fn move_locked_ball(
    mut ball: Single<&mut Position, (With<Ball>, Without<Velocity>)>,
    paddle: Single<&Position, (With<Paddle>, Without<Ball>)>,
) {
    ball.0 = paddle.0 + vec2(0., 20.);
}
