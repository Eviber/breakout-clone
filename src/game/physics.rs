use bevy::prelude::*;

use super::{GameState, GameSystemSet};
use crate::AppState;

pub fn plugin(app: &mut App) {
    app.add_systems(FixedUpdate, move_entities.in_set(GameSystemSet::Movement));
    app.add_systems(
        OnEnter(AppState::MainMenu),
        project_positions.in_set(GameSystemSet::Display),
    )
    .add_systems(
        FixedUpdate,
        project_positions
            .in_set(GameSystemSet::Display)
            .run_if(in_state(GameState::Running)),
    );
}

#[derive(Component, Clone, Default)]
#[require(Transform)]
pub struct Position(pub Vec2);

#[derive(Component, Clone, Default)]
pub struct Velocity(pub Vec2);

#[derive(Component, Clone, Default)]
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
