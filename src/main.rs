//! This example shows off the various Bevy Feathers widgets.

#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]

mod game;
mod main_menu;

use bevy::{feathers::FeathersPlugins, prelude::*};
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

#[derive(States, Default, Debug, Hash, Eq, PartialEq, Clone)]
pub enum AppState {
    #[default]
    MainMenu,
    InGame,
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, FeathersPlugins))
        .add_plugins(EguiPlugin::default())
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(main_menu::plugin)
        .add_plugins(game::plugin)
        .init_state::<AppState>()
        .add_systems(Startup, camera_scene.spawn())
        .run();
}

fn camera_scene() -> impl SceneList {
    bsn! { Camera2d }
}
