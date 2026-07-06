use std::time::Duration;

use bevy::{
    audio::{PlaybackMode, Volume},
    prelude::*,
};

use super::ball::BallCollision;

const BEEP_LENGTH_MS: u32 = 100;
const BEEP_PITCH: f32 = 150.0;
const BEEP_VOLUME: Volume = Volume::Linear(0.2);

pub fn plugin(app: &mut App) {
    app.add_observer(play_hit_sound);
}

fn play_hit_sound(_event: On<BallCollision>, mut commands: Commands) {
    commands.spawn_scene(bsn! {
        AudioPlayer<Pitch>(asset_value(Pitch::new(
            BEEP_PITCH,
            Duration::new(0, BEEP_LENGTH_MS * 1_000_000),
        )))
        PlaybackSettings {
            mode: PlaybackMode::Despawn,
            volume: BEEP_VOLUME,
        }
    });
}
