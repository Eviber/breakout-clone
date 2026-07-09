use std::time::Duration;

use bevy::{
    audio::{PlaybackMode, Volume},
    prelude::*,
};

use super::ball::BallCollision;

const BEEP_LENGTH_MS: u32 = 100;
const BEEP_VOLUME: Volume = Volume::Linear(0.2);
const ROOT_FREQ: f32 = 150.0;
// const SCALE: [i32; 7] = [0, 2, 4, 5, 7, 9, 11];
const PENTATONIC_SCALE: [i32; 6] = [0, 2, 4, 7, 9, 12];

pub fn plugin(app: &mut App) {
    app.add_observer(play_hit_sound);
}

/// Eheheheh 😈
fn rand64() -> u64 {
    use std::hash::{BuildHasher, Hasher};
    std::hash::RandomState::new().build_hasher().finish()
}

fn random_note() -> f32 {
    let total_notes = PENTATONIC_SCALE.len() as u64;

    let idx = rand64() % total_notes;

    let semitone_offset = PENTATONIC_SCALE[idx as usize];

    ROOT_FREQ * 2f32.powf(semitone_offset as f32 / 12.0)
}

fn play_hit_sound(_event: On<BallCollision>, mut commands: Commands) {
    let pitch = random_note();
    commands.spawn_scene(bsn! {
        AudioPlayer<Pitch>(asset_value(Pitch::new(
            pitch,
            Duration::new(0, BEEP_LENGTH_MS * 1_000_000),
        )))
        PlaybackSettings {
            mode: PlaybackMode::Despawn,
            volume: BEEP_VOLUME,
        }
    });
}
