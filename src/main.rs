//! This example shows off the various Bevy Feathers widgets.

#![allow(clippy::too_many_arguments)]

mod main_menu;

use bevy::{feathers::FeathersPlugins, prelude::*};

#[derive(States, Default, Debug, Hash, Eq, PartialEq, Clone)]
enum GameState {
    #[default]
    MainMenu,
    InGame,
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, FeathersPlugins))
        .add_plugins(main_menu::plugin)
        .init_state::<GameState>()
        .add_systems(Update, log_state_change)
        .add_systems(Startup, camera_scene.spawn())
        .run();
}

fn log_state_change(state: Res<State<GameState>>) {
    if state.is_changed() {
        info!("Switched to state {:?}", **state);
    }
}

fn despawn_ui(mut commands: Commands, root_node: Single<Entity, (With<Node>, Without<ChildOf>)>) {
    commands.entity(*root_node).despawn();
}

fn camera_scene() -> impl SceneList {
    bsn! { Camera2d }
}
