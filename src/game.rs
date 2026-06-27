mod game_pause;

use bevy::prelude::*;

use crate::despawn_ui;
use game_pause::IsPaused;

pub fn plugin(app: &mut App) {
    app.add_plugins(game_pause::plugin)
        .add_systems(OnEnter(IsPaused::Running), game_ui.spawn())
        .add_systems(OnExit(IsPaused::Running), despawn_ui)
        .add_systems(Update, handle_input.run_if(in_state(IsPaused::Running)));
}

fn handle_input(input: Res<ButtonInput<KeyCode>>, mut next_state: ResMut<NextState<IsPaused>>) {
    if input.just_pressed(KeyCode::Escape) {
        next_state.set(IsPaused::Paused);
    }
}

fn game_ui() -> impl Scene {
    bsn! {
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
        }
        Children [
            (
                Text::new("In construction\nPress escape to quit to main menu.")
                // TextColor(Color::BLACK)
                TextLayout::justify(Justify::Center)
            )
        ]
    }
}
