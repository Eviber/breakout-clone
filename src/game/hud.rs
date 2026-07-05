use bevy::prelude::*;

use super::{Lives, Score};
use crate::AppState;

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            update_lives_display.run_if(resource_changed::<Lives>),
            update_score_display.run_if(resource_changed::<Score>),
        ),
    )
    .add_systems(OnEnter(AppState::InGame), game_ui.spawn());
}

#[derive(Component, Default, Clone, Copy)]
struct LivesDisplay;

#[derive(Component, Default, Clone, Copy)]
struct ScoreDisplay;

fn game_ui() -> impl Scene {
    bsn! {
        Node {
            width: percent(100),
            // height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::FlexEnd,
        }
        DespawnOnExit<AppState>(AppState::InGame)
        Children [
            (
                ScoreDisplay
                Text::new(format!("{} points", 0))
            ),
            (
                LivesDisplay
                Text::new(format!("{} lives", 3))
                // TextColor(Color::BLACK)
                // TextLayout::justify(Justify::Center)
            ),
        ]
    }
}

fn update_lives_display(mut text: Single<&mut Text, With<LivesDisplay>>, lives: Res<Lives>) {
    info!("Lives updated: {}", lives.0);
    text.0 = format!("{} lives", lives.0);
}

fn update_score_display(mut text: Single<&mut Text, With<ScoreDisplay>>, score: Res<Score>) {
    info!("Score updated: {}", score.0);
    text.0 = format!("{} points", score.0);
}
