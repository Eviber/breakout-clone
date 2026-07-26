use std::time::Duration;

use bevy::prelude::*;

use super::GameState;

const FRAME_FREEZE: f32 = 0.0;

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Frozen), init_timer)
        .add_systems(
            Update,
            countdown
                .run_if(in_state(GameState::Frozen))
                .run_if(resource_exists::<Timer>)
                // Ambiguity checker is unaware of run conditions (including States)
                // https://github.com/bevyengine/bevy/issues/1693
                .ambiguous_with_all(),
        );
}

#[derive(Resource, Clone, Copy)]
struct Timer(Duration);

fn init_timer(mut commands: Commands) {
    commands.insert_resource(Timer(Duration::from_secs_f32(FRAME_FREEZE)));
}

fn countdown(
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
    time: Res<Time>,
    mut timer: ResMut<Timer>,
) {
    timer.0 = timer.0.saturating_sub(time.delta());
    if timer.0.is_zero() {
        commands.remove_resource::<Timer>();
        next_state.set(GameState::Running);
    }
}
