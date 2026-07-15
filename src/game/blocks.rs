use bevy::prelude::*;

use super::Score;
use super::ball::{BALL_SPEED, Ball, BallCollision};
use super::collision;
use super::physics::{Collider, Position, Velocity};
use crate::AppState;

pub fn plugin(app: &mut App) {
    app.add_observer(destroy_brick);
}

#[derive(Component, Clone, Default)]
pub struct Gutter;

pub const GUTTER_COLOR: Color = Color::srgb(0., 0., 1.);
pub const GUTTER_WIDTH: f32 = 20.;

pub fn gutter(x: f32, y: f32, shape: Rectangle) -> impl Scene {
    bsn! {
        Name("Gutter")
        Gutter
        Position(vec2(x, y))
        Collider(shape)
        Mesh2d(asset_value(shape))
        MeshMaterial2d<ColorMaterial>(asset_value(GUTTER_COLOR))
        DespawnOnExit<AppState>(AppState::InGame)
        on(collide_gutter)
    }
}

fn collide_gutter(
    event: On<BallCollision>,
    mut commands: Commands,
    ball: Single<(&mut Position, &mut Velocity), With<Ball>>,
) {
    let (mut ball_position, mut ball_velocity) = ball.into_inner();
    match event.side {
        collision::Collision::Left | collision::Collision::Right => {
            ball_velocity.0.x *= -1.;
        }
        collision::Collision::Top | collision::Collision::Bottom => {
            ball_velocity.0.y *= -1.;
        }
    }
    ball_position.0 = event.pos + ball_velocity.0.normalize() * event.remaining_distance;
    commands.trigger(super::ball::BallMoved {
        from: event.pos,
        rebound_from: Some(event.entity),
    });
}

#[derive(EntityEvent)]
pub struct BrickDestroyed {
    entity: Entity,
}

#[derive(Component, Clone, Default)]
#[require(Position, Collider)]
pub struct Brick;

const BRICK_COLOR: Color = Color::srgb(1., 1., 1.);
const BRICK_SHAPE: Rectangle = Rectangle::new(80., 20.);

pub fn brick(x: f32, y: f32) -> impl Scene {
    bsn! {
        Name("Brick")
        Brick
        Position(vec2(x, y))
        Collider(BRICK_SHAPE)
        Mesh2d(asset_value(BRICK_SHAPE))
        MeshMaterial2d<ColorMaterial>(asset_value(BRICK_COLOR))
        DespawnOnExit<AppState>(AppState::InGame)
        on(collide_brick)
    }
}

// TODO: Add combo mechanic? Do the paddle resets the combo, or only losing a life?
fn destroy_brick(event: On<BrickDestroyed>, mut commands: Commands, mut score: ResMut<Score>) {
    commands.entity(event.entity).despawn();
    score.0 += 10;
}

// TODO: Add angle collision
fn collide_brick(
    event: On<BallCollision>,
    mut commands: Commands,
    ball: Single<(&mut Position, &mut Velocity), With<Ball>>,
) {
    let (mut ball_position, mut ball_velocity) = ball.into_inner();
    match event.side {
        collision::Collision::Left | collision::Collision::Right => {
            ball_velocity.0.x *= -1.;
        }
        collision::Collision::Top | collision::Collision::Bottom => {
            ball_velocity.0.y *= -1.;
        }
    }
    ball_position.0 = event.pos + ball_velocity.0.normalize() * event.remaining_distance;
    commands.trigger(super::ball::BallMoved {
        from: event.pos,
        rebound_from: Some(event.entity),
    });
    let len = ball_velocity.0.length() + BALL_SPEED * 0.1;
    ball_velocity.0 = ball_velocity.0.clamp_length_min(len);
    info!("Speed: {len}");
    commands.trigger(BrickDestroyed {
        entity: event.entity,
    });
}
