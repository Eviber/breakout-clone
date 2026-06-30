use bevy::{
    feathers::{
        controls::*,
        theme::{ThemeBackgroundColor, ThemedText},
        tokens,
    },
    input_focus::tab_navigation::TabGroup,
    prelude::*,
    ui_widgets::Activate,
};

use super::GameState;
use crate::{AppState, despawn_ui};

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::GameOver), game_over_ui.spawn())
        .add_systems(OnExit(GameState::GameOver), despawn_ui)
        .add_systems(Update, handle_input.run_if(in_state(GameState::GameOver)));
}

fn handle_input(input: Res<ButtonInput<KeyCode>>, mut next_state: ResMut<NextState<AppState>>) {
    if input.just_pressed(KeyCode::Escape) {
        next_state.set(AppState::MainMenu);
    }
}

fn game_over_ui() -> impl Scene {
    bsn! {
        Node {
            width: percent(80),
            height: percent(80),
            align_items: AlignItems::Stretch,
            justify_content: JustifyContent::Center,
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            row_gap: percent(15),
            margin: auto(),
            padding: UiRect {
                left: percent(20),
                right: percent(20),
                top: percent(15),
                bottom: percent(15),
            },
        }
        TabGroup
        ThemeBackgroundColor(tokens::WINDOW_BG)
        Children [
            (
                button("Quit to main menu")
                on(|_activate: On<Activate>, mut next_state: ResMut<NextState<AppState>>| {
                    next_state.set(AppState::MainMenu);
                })
            ),
        ]
    }
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
