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

const PADDLE_SHAPE: Rectangle = Rectangle::new(100., 10.);
const PADDLE_COLOR: Color = Color::srgb(0., 1., 0.);
const PADDLE_SPEED: f32 = 5.;

impl Paddle {
    fn scene() -> impl Scene {
        let x = 0.;
        let y = -300.;
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

#[derive(Component, Clone, Default)]
#[require(Position, Collider)]
struct Brick;

const BRICK_COLOR: Color = Color::srgb(1., 1., 1.);
const BRICK_SHAPE: Rectangle = Rectangle::new(60., 20.);

fn brick(x: f32, y: f32) -> impl Scene {
    bsn! {
        Brick
        Gutter
        Position(vec2(x, y))
        Collider(BRICK_SHAPE)
        Mesh2d(asset_value(BRICK_SHAPE))
        MeshMaterial2d<ColorMaterial>(asset_value(BRICK_COLOR))
        DespawnOnExit<GameState>(GameState::InGame)
    }
}

fn spawn_bricks(mut commands: Commands) {
    for line in 0..3 {
        for col in 0..10 {
            let x = (col * 100 - 500) as f32;
            let y = (line * 50 + 200) as f32;
            commands.spawn_scene(brick(x, y));
        }
    }
}

pub fn plugin(app: &mut App) {
    // HACK: Project position on entering state, to make them visible sooner
    app.add_systems(
        OnEnter(GameState::InGame),
        (
            spawn_entities.before(project_positions),
            spawn_bricks.before(project_positions),
            project_positions,
        ),
    )
    .add_systems(
        FixedUpdate,
        (
            project_positions,
            move_ball.before(project_positions),
            handle_collisions.after(move_ball),
            handle_player_input.before(move_paddle),
            move_paddle.before(project_positions),
            constrain_paddle_position.after(move_paddle),
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

fn move_paddle(paddle: Single<(&mut Position, &Velocity), With<Paddle>>) {
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

fn constrain_paddle_position(
    paddles: Single<(&mut Position, &Collider), (With<Paddle>, Without<Gutter>)>,
    gutters: Query<(&Position, &Collider), (With<Gutter>, Without<Paddle>)>,
) {
    let (mut paddle_position, paddle_collider) = paddles.into_inner();
    for (gutter_position, gutter_collider) in &gutters {
        let paddle_aabb = Aabb2d::new(paddle_position.0, paddle_collider.half_size());
        let gutter_aabb = Aabb2d::new(gutter_position.0, gutter_collider.half_size());

        if let Some(collision) = collide_with_side(paddle_aabb, gutter_aabb) {
            match collision {
                Collision::Left => {
                    paddle_position.0.x = gutter_position.0.x
                        - gutter_collider.half_size().x
                        - paddle_collider.half_size().x;
                }
                Collision::Right => {
                    paddle_position.0.x = gutter_position.0.x
                        + gutter_collider.half_size().x
                        + paddle_collider.half_size().x;
                }
                _ => {}
            }
        }
    }
}

fn handle_collisions(
    mut commands: Commands,
    ball: Single<(&mut Velocity, &Position, &Collider), With<Ball>>,
    other_things: Query<(&Position, &Collider, Has<Paddle>, Has<Brick>, Entity), Without<Ball>>,
) {
    let (mut ball_velocity, ball_position, ball_collider) = ball.into_inner();

    for (other_position, other_collider, is_paddle, is_brick, entity) in &other_things {
        let Some(collision) = collide_with_side(
            Aabb2d::new(ball_position.0, ball_collider.half_size()),
            Aabb2d::new(other_position.0, other_collider.half_size()),
        ) else {
            continue;
        };
        if is_paddle {
            let paddle_pos = Vec2 {
                x: other_position.0.x,
                y: other_position.0.y + other_collider.half_size().y - other_collider.half_size().x,
            };
            let dir = (ball_position.0 - paddle_pos).normalize();
            ball_velocity.0 = dir * BALL_SPEED;
            continue;
        }
        match collision {
            Collision::Left | Collision::Right => {
                ball_velocity.0.x *= -1.;
            }
            Collision::Top | Collision::Bottom => {
                ball_velocity.0.y *= -1.;
            }
        }
        if is_brick {
            commands.entity(entity).despawn();
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
