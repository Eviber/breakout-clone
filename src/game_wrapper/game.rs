use bevy::prelude::*;

use super::IsPaused;

use crate::GameState;

#[derive(Component, Clone, Default)]
#[require(Transform)]
struct Position(Vec2);

#[derive(Component, Clone, Default)]
struct Velocity(Vec2);

#[derive(SceneComponent, Clone, Default)]
struct Ball;

const BALL_SIZE: f32 = 10.;
const BALL_SHAPE: Circle = Circle::new(BALL_SIZE);
const BALL_COLOR: Color = Color::srgb(1., 0., 0.);
const BALL_SPEED: f32 = 2.;

impl Ball {
    fn scene() -> impl Scene {
        let x = 0.;
        let y = -200.;
        bsn! {
            Position(vec2(x, y))
            Velocity(vec2(0., -BALL_SPEED))
            Mesh2d(asset_value(BALL_SHAPE))
            MeshMaterial2d<ColorMaterial>(asset_value(BALL_COLOR))
            DespawnOnExit<GameState>(GameState::InGame)
        }
    }
}

#[derive(SceneComponent, Clone, Default)]
struct Paddle;

const PADDLE_SHAPE: Rectangle = Rectangle::new(50., 10.);
const PADDLE_COLOR: Color = Color::srgb(0., 1., 0.);

impl Paddle {
    fn scene() -> impl Scene {
        let x = 0.;
        let y = -350.;
        bsn! {
            Position(vec2(x,y))
            Mesh2d(asset_value(PADDLE_SHAPE))
            MeshMaterial2d<ColorMaterial>(asset_value(PADDLE_COLOR))
            DespawnOnExit<GameState>(GameState::InGame)
        }
    }
}

pub fn plugin(app: &mut App) {
    // HACK: Project position on entering state, to make them visible sooner
    app.add_systems(OnEnter(GameState::InGame), (entities.spawn(), project_positions).chain())
        .add_systems(
            FixedUpdate,
            (move_ball, project_positions)
                .chain()
                .run_if(in_state(IsPaused::Running)),
        );
}

fn move_ball(ball: Single<(&mut Position, &Velocity), With<Ball>>) {
    let (mut position, velocity) = ball.into_inner();
    position.0 += velocity.0 * BALL_SPEED;
}

fn entities() -> impl SceneList {
    bsn_list! [
        @Ball,
        @Paddle,
    ]
}

fn project_positions(mut positionables: Query<(&mut Transform, &Position)>) {
    for (mut transform, position) in &mut positionables {
        transform.translation = position.0.extend(0.);
    }
}
