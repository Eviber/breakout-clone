use bevy::math::bounding::{Aabb2d, BoundingVolume, IntersectsVolume};
use bevy::prelude::*;

use super::IsPaused;

use crate::GameState;

#[derive(Component, Clone, Default)]
#[require(Transform)]
struct Position(Vec2);

#[derive(Component, Clone, Default)]
struct Velocity(Vec2);

#[derive(Component, Clone, Default)]
struct Collider(Rectangle);

impl Collider {
    fn half_size(&self) -> Vec2 {
        self.0.half_size
    }
}

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
            Collider(Rectangle::new(BALL_SIZE, BALL_SIZE))
            Mesh2d(asset_value(BALL_SHAPE))
            MeshMaterial2d<ColorMaterial>(asset_value(BALL_COLOR))
            DespawnOnExit<GameState>(GameState::InGame)
        }
    }
}

#[derive(SceneComponent, Clone, Default)]
#[require(Velocity)]
struct Paddle;

const PADDLE_SHAPE: Rectangle = Rectangle::new(50., 10.);
const PADDLE_COLOR: Color = Color::srgb(0., 1., 0.);
const PADDLE_SPEED: f32 = 5.;

impl Paddle {
    fn scene() -> impl Scene {
        let x = 0.;
        let y = -350.;
        bsn! {
            Position(vec2(x,y))
            Collider(PADDLE_SHAPE)
            Mesh2d(asset_value(PADDLE_SHAPE))
            MeshMaterial2d<ColorMaterial>(asset_value(PADDLE_COLOR))
            DespawnOnExit<GameState>(GameState::InGame)
        }
    }
}

#[derive(Component, Clone, Default)]
struct Gutter;

const GUTTER_COLOR: Color = Color::srgb(0., 0., 1.);
const GUTTER_WIDTH: f32 = 20.;

fn gutter(x: f32, y: f32, shape: Rectangle) -> impl Scene {
    bsn! {
        Gutter
        Position(vec2(x, y))
        Collider(shape)
        Mesh2d(asset_value(shape))
        MeshMaterial2d<ColorMaterial>(asset_value(GUTTER_COLOR))
        DespawnOnExit<GameState>(GameState::InGame)
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Collision {
    Left,
    Right,
    Top,
    Bottom,
}

pub fn plugin(app: &mut App) {
    // HACK: Project position on entering state, to make them visible sooner
    app.add_systems(
        OnEnter(GameState::InGame),
        (spawn_entities, project_positions).chain(),
    )
    .add_systems(
        FixedUpdate,
        (
            project_positions,
            move_ball.before(project_positions),
            handle_collisions.after(move_ball),
            handle_player_input.before(move_paddle),
            move_paddle.before(project_positions),
        )
            .run_if(in_state(IsPaused::Running)),
    );
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

fn move_paddle(
    paddle: Single<(&mut Position, &Velocity), With<Paddle>>
) {
    let (mut position, velocity) = paddle.into_inner();
    position.0 += velocity.0;
}

// Returns `Some` if `ball` collides with `wall`. The returned `Collision` is the
// side of `wall` that `ball` hit.
fn collide_with_side(ball: Aabb2d, wall: Aabb2d) -> Option<Collision> {
    if !ball.intersects(&wall) {
        return None;
    }

    let closest_point = wall.closest_point(ball.center());
    let offset = ball.center() - closest_point;

    let side = if offset.x.abs() > offset.y.abs() {
        if offset.x < 0. {
            Collision::Left
        } else {
            Collision::Right
        }
    } else if offset.y > 0. {
        Collision::Top
    } else {
        Collision::Bottom
    };

    Some(side)
}

fn handle_collisions(
    ball: Single<(&mut Velocity, &Position, &Collider), With<Ball>>,
    other_things: Query<(&Position, &Collider), Without<Ball>>,
) {
    let (mut ball_velocity, ball_position, ball_collider) = ball.into_inner();

    for (other_position, other_collider) in &other_things {
        let Some(collision) = collide_with_side(
            Aabb2d::new(ball_position.0, ball_collider.half_size()),
            Aabb2d::new(other_position.0, other_collider.half_size()),
        ) else {
            continue;
        };
        match collision {
            Collision::Left | Collision::Right => {
                ball_velocity.0.x *= -1.;
            }
            Collision::Top | Collision::Bottom => {
                ball_velocity.0.y *= -1.;
            }
        }
    }
}

fn move_ball(ball: Single<(&mut Position, &Velocity), With<Ball>>) {
    let (mut position, velocity) = ball.into_inner();
    position.0 += velocity.0 * BALL_SPEED;
}

fn spawn_entities(mut commands: Commands, window: Single<&Window>) {
    let half_width = window.resolution.width() / 2.;
    let half_height = window.resolution.height() / 2.;
    let shape_v = Rectangle::new(GUTTER_WIDTH, window.resolution.height());
    let shape_h = Rectangle::new(window.resolution.width(), GUTTER_WIDTH);

    commands.spawn_scene_list(bsn_list! [
        @Ball,
        @Paddle,
        gutter(half_width, 0., shape_v),
        gutter(-half_width, 0., shape_v),
        gutter(0., half_height, shape_h),
    ]);
}

fn project_positions(mut positionables: Query<(&mut Transform, &Position)>) {
    for (mut transform, position) in &mut positionables {
        transform.translation = position.0.extend(0.);
    }
}
