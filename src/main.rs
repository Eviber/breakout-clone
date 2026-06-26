//! This example shows off the various Bevy Feathers widgets.

#![allow(clippy::too_many_arguments)]

use bevy::{
    feathers::{
        FeathersPlugins,
        controls::*,
        dark_theme::create_dark_theme,
        theme::{ThemeBackgroundColor, ThemedText, UiTheme},
        tokens,
    },
    input_focus::tab_navigation::TabGroup,
    // input_focus::AutoFocus,
    prelude::*,
    ui::InteractionDisabled,
    ui_widgets::Activate,
};

#[derive(States, Default, Debug, Hash, Eq, PartialEq, Clone)]
enum GameState {
    #[default]
    MainMenu,
    InGame,
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, FeathersPlugins))
        .insert_resource(UiTheme(create_dark_theme()))
        .init_state::<GameState>()
        .add_systems(Startup, camera_scene.spawn())
        .add_systems(OnEnter(GameState::MainMenu), main_menu.spawn())
        .run();
}

fn camera_scene() -> impl SceneList {
    bsn! { Camera2d }
}

fn button(name: &'static str) -> impl Scene {
    bsn! {
        @FeathersButton {
            @caption: bsn! { Text(name) ThemedText },
        }
        Node { flex_grow: 1.0, }
        AccessibleLabel(name)
        on(move |_activate: On<Activate>| {
            info!("{name} button clicked!");
        })
    }
}

fn main_menu() -> impl Scene {
    bsn! {
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Stretch,
            justify_content: JustifyContent::Center,
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            row_gap: percent(10),
            padding: UiRect {
                left: percent(30),
                right: percent(30),
                top: percent(20),
                bottom: percent(20),
            },
        }
        TabGroup
        ThemeBackgroundColor(tokens::WINDOW_BG)
        Children [
            (
                button("Start")
                // AutoFocus // not using autofocus so that this is the first selected button when tabbing
                on(|_activate: On<Activate>, mut next_state: ResMut<NextState<GameState>>| {
                    next_state.set(GameState::InGame);
                })
            ),
            (
                button("Settings")
                InteractionDisabled
            ),
            (
                button("Quit")
                on(|_activate: On<Activate>, mut commands: Commands| {
                    commands.write_message(AppExit::Success);
                })
            ),
        ]
    }
}
