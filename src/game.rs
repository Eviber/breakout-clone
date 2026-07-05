mod ball;
mod blocks;
mod collision;
mod game_over;
mod game_pause;
mod hud;
mod level;
mod paddle;
mod physics;

use bevy::ecs::schedule::{LogLevel, ScheduleBuildSettings};
use bevy::prelude::*;

use crate::AppState;
use blocks::Brick;

#[derive(SubStates, Default, Debug, Hash, Eq, PartialEq, Clone)]
#[source(AppState = AppState::InGame)]
enum GameState {
    #[default]
    Running,
    Paused,
    GameOver,
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
enum GameSystemSet {
    Preload,
    Input,
    Movement,
    PreCollision,
    Collision,
    PostCollision,
    Display,
}

#[derive(Resource)]
struct Lives(usize);

#[derive(Resource)]
struct Score(u32);

pub fn plugin(app: &mut App) {
    app.edit_schedule(FixedUpdate, |schedule| {
        schedule.set_build_settings(ScheduleBuildSettings {
            ambiguity_detection: LogLevel::Warn,
            ..default()
        });
    });
    app.configure_sets(
        OnEnter(AppState::MainMenu),
        (
            GameSystemSet::Preload,
            GameSystemSet::PostCollision,
            GameSystemSet::Display,
        )
            .chain(),
    );
    app.configure_sets(
        FixedUpdate,
        (
            GameSystemSet::Input,
            GameSystemSet::Movement,
            GameSystemSet::PreCollision,
            GameSystemSet::Collision,
            GameSystemSet::PostCollision,
            GameSystemSet::Display,
        )
            .chain(),
    );
    app.add_plugins(game_pause::plugin)
        .add_plugins(game_over::plugin)
        .add_plugins(hud::plugin)
        .add_plugins(physics::plugin)
        .add_plugins(ball::plugin)
        .add_plugins(paddle::plugin)
        .add_plugins(blocks::plugin)
        .add_plugins(level::plugin)
        .add_systems(Update, check_pause.run_if(in_state(GameState::Running)))
        .add_systems(
            FixedUpdate,
            (
                set_win_state
                    .run_if(not(any_with_component::<Brick>))
                    .in_set(GameSystemSet::PostCollision)
                    .ambiguous_with(check_out_of_lives),
                check_out_of_lives
                    .run_if(resource_changed::<Lives>)
                    .in_set(GameSystemSet::PostCollision),
            )
                .run_if(in_state(GameState::Running)),
        );
}

fn check_pause(input: Res<ButtonInput<KeyCode>>, mut next_state: ResMut<NextState<GameState>>) {
    if input.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::Paused);
    }
}

fn check_out_of_lives(mut next_state: ResMut<NextState<GameState>>, lives: Res<Lives>) {
    if lives.0 == 0 {
        next_state.set(GameState::GameOver);
    }
}

fn set_win_state(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::GameOver);
}
