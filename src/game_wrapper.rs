mod game;
mod game_over;
mod game_pause;

use bevy::prelude::*;

use crate::AppState;
use crate::despawn_all_ui;

#[derive(SubStates, Default, Debug, Hash, Eq, PartialEq, Clone)]
#[source(AppState = AppState::InGame)]
pub enum GameState {
    #[default]
    Running,
    Paused,
    GameOver,
}

#[derive(Resource)]
pub struct Lives(usize);

pub fn plugin(app: &mut App) {
    app.add_plugins(game_pause::plugin)
        .add_plugins(game::plugin)
        .add_plugins(game_over::plugin)
        .add_systems(OnEnter(GameState::Running), game_ui.spawn())
        .add_systems(OnExit(GameState::Running), despawn_all_ui)
        .add_systems(Update, handle_input.run_if(in_state(GameState::Running)));
}

fn handle_input(input: Res<ButtonInput<KeyCode>>, mut next_state: ResMut<NextState<GameState>>) {
    if input.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::Paused);
    }
}

fn game_ui() -> impl Scene {
    bsn! {
        // Node {
        //     width: percent(100),
        //     height: percent(100),
        //     align_items: AlignItems::Center,
        //     justify_content: JustifyContent::Center,
        // }
        // Children [
        //     (
        //         Text::new("In construction\nPress escape to quit to main menu.")
        //         // TextColor(Color::BLACK)
        //         TextLayout::justify(Justify::Center)
        //     )
        // ]
    }
}
