use bevy::prelude::*;

use bevy::math::bounding::{Aabb2d, BoundingCircle, RayCast2d};

use super::ball::BALL_SIZE;
use super::{GameState, GameSystemSet};
use crate::AppState;

pub fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(AppState::MainMenu),
        project_positions.in_set(GameSystemSet::Display),
    )
    .add_systems(
        FixedUpdate,
        (
            move_entities.in_set(GameSystemSet::Movement),
            project_positions.in_set(GameSystemSet::Display),
        )
            .run_if(in_state(GameState::Running)),
    );
}

#[derive(Component, Clone, Copy, Default, Reflect)]
#[require(Transform)]
pub struct Position(pub Vec2);

#[derive(Component, Clone, Default, Reflect)]
pub struct Velocity(pub Vec2);

#[derive(Component, Clone, Default, Reflect)]
pub struct Collider(pub Rectangle);

impl Collider {
    pub fn half_size(&self) -> Vec2 {
        self.0.half_size
    }
}

fn project_positions(mut positionables: Query<(&mut Transform, &Position)>) {
    for (mut transform, position) in &mut positionables {
        transform.translation = position.0.extend(0.);
    }
}

fn move_entities(entities: Query<(&mut Position, &Velocity)>) {
    for (mut position, velocity) in entities {
        position.0 += velocity.0;
    }
}

enum BoundingShape {
    Circle(BoundingCircle),
    Rectangle(Aabb2d),
}

impl From<BoundingCircle> for BoundingShape {
    fn from(bc: BoundingCircle) -> Self {
        BoundingShape::Circle(bc)
    }
}

impl From<Aabb2d> for BoundingShape {
    fn from(aabb: Aabb2d) -> Self {
        BoundingShape::Rectangle(aabb)
    }
}

impl BoundingShape {
    fn ray_intersection_at(&self, ray_cast: &RayCast2d) -> Option<f32> {
        match self {
            BoundingShape::Circle(bc) => ray_cast.circle_intersection_at(bc),
            BoundingShape::Rectangle(aabb) => ray_cast.aabb_intersection_at(aabb),
        }
    }
}

pub fn ball_to_collider_collision(
    ray_cast: &RayCast2d,
    position: &Position,
    collider: &Collider,
) -> Option<f32> {
    let e_ray_cast = epsilon_ray_cast(ray_cast);

    let expanded_hw = collider.half_size().x + BALL_SIZE;
    let expanded_hh = collider.half_size().y + BALL_SIZE;
    let aabb = aabb_from_half(position, collider.half_size().x, collider.half_size().y);

    let shapes = [
        aabb_from_half(position, collider.half_size().x, expanded_hh).into(),
        aabb_from_half(position, expanded_hw, collider.half_size().y).into(),
        BoundingCircle::new(aabb.min, BALL_SIZE).into(),
        BoundingCircle::new(Vec2::new(aabb.min.x, aabb.max.y), BALL_SIZE).into(),
        BoundingCircle::new(Vec2::new(aabb.max.x, aabb.min.y), BALL_SIZE).into(),
        BoundingCircle::new(aabb.max, BALL_SIZE).into(),
    ];

    shapes
        .into_iter()
        .filter_map(|shape| shape_collision(ray_cast, &e_ray_cast, shape))
        .min_by(|a, b| a.total_cmp(b))
}

fn shape_collision(
    ray_cast: &RayCast2d,
    e_ray_cast: &RayCast2d,
    shape: BoundingShape,
) -> Option<f32> {
    let dist = shape.ray_intersection_at(ray_cast)?;
    if dist > 0. {
        return Some(dist);
    }
    // Only collide if there's a collision at 0.0 with epsilon too
    let e_dist = shape.ray_intersection_at(e_ray_cast)?;
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
