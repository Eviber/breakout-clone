//! This example shows off the various Bevy Feathers widgets.

#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]

mod game_wrapper;
mod main_menu;

use bevy::{feathers::FeathersPlugins, prelude::*};

#[derive(States, Default, Debug, Hash, Eq, PartialEq, Clone)]
pub enum GameState {
    #[default]
    MainMenu,
    InGame,
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, FeathersPlugins))
        .add_plugins(main_menu::plugin)
        .add_plugins(game_wrapper::plugin)
        .init_state::<GameState>()
        .add_systems(Startup, camera_scene.spawn())
        .run();
}

fn despawn_ui(mut commands: Commands, root_node: Single<Entity, (With<Node>, Without<ChildOf>)>) {
    commands.entity(*root_node).despawn();
}

fn camera_scene() -> impl SceneList {
    bsn! { Camera2d }
}
