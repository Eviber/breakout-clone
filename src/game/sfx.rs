use std::time::Duration;

use bevy::{
    audio::{PlaybackMode, Volume},
    prelude::*,
};

use super::ball::BallCollision;

const BEEP_LENGTH: f32 = 0.1;
const BEEP_VOLUME: Volume = Volume::Linear(0.2);
const ROOT_FREQ: f32 = 144. * 2.;
// const SCALE: [i32; 7] = [0, 2, 4, 5, 7, 9, 11];
const PENTATONIC_SCALE: [i32; 6] = [0, 2, 4, 7, 9, 12];

pub fn plugin(app: &mut App) {
    app.add_observer(play_hit_sound);
    app.add_systems(Update, fade_sounds);
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
            Duration::from_secs_f32(BEEP_LENGTH),
        )))
        PlaybackSettings {
            mode: PlaybackMode::Despawn,
            volume: Volume::SILENT,
        }
        Fade
    });
}

#[derive(Component, Clone, Copy, Default)]
struct Fade(f32);

fn fade_sounds(
    mut commands: Commands,
    mut audio_sink: Query<(&mut AudioSink, &mut Fade, Entity)>,
    time: Res<Time>,
) {
    for (mut audio, mut fade, entity) in audio_sink.iter_mut() {
        fade.0 += time.delta_secs_f64() as f32;
        // f(x) = -4/(k^(2)) x (x-k)
        let lerper = -4. / (BEEP_LENGTH * BEEP_LENGTH) * fade.0 * (fade.0 - BEEP_LENGTH);
        audio.set_volume(Volume::SILENT.fade_towards(BEEP_VOLUME, lerper));
        if time.delta().as_secs_f32() >= BEEP_LENGTH {
            audio.set_volume(Volume::Linear(0.0));
            commands.entity(entity).despawn();
        }
    }
}
