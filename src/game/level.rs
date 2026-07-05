use bevy::prelude::*;

use super::ball::Ball;
use super::blocks::{GUTTER_WIDTH, brick, gutter};
use super::paddle::Paddle;
use super::{GameSystemSet, Lives, Score};
use crate::AppState;

pub fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(AppState::MainMenu),
        (
            spawn_entities.in_set(GameSystemSet::Preload),
            spawn_bricks.in_set(GameSystemSet::Preload),
            init_resources,
        ),
    );
}

fn init_resources(mut commands: Commands) {
    commands.insert_resource(Lives(3));
    commands.insert_resource(Score(0));
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
