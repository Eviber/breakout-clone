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

use crate::{GameState, despawn_ui};

#[derive(SubStates, Default, Debug, Hash, Eq, PartialEq, Clone)]
#[source(GameState = GameState::InGame)]
pub enum IsPaused {
    #[default]
    Running,
    Paused,
}

pub fn plugin(app: &mut App) {
    app.add_sub_state::<IsPaused>()
        .add_systems(OnEnter(IsPaused::Paused), pause_ui.spawn())
        .add_systems(OnExit(IsPaused::Paused), despawn_ui)
        .add_systems(Update, handle_input.run_if(in_state(IsPaused::Paused)));
}

fn handle_input(input: Res<ButtonInput<KeyCode>>, mut next_state: ResMut<NextState<IsPaused>>) {
    if input.just_pressed(KeyCode::Escape) {
        next_state.set(IsPaused::Running);
    }
}

fn pause_ui() -> impl Scene {
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
                button("Resume")
                on(|_activate: On<Activate>, mut next_state: ResMut<NextState<IsPaused>>| {
                    next_state.set(IsPaused::Running);
                })
            ),
            (
                button("Quit to main menu")
                on(|_activate: On<Activate>, mut next_state: ResMut<NextState<GameState>>| {
                    next_state.set(GameState::MainMenu);
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
