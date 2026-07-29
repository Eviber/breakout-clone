use bevy::audio::Volume;
use bevy::prelude::*;

use super::ball::BallCollision;
use super::blocks::BrickDestroyed;

// const BEEP_LENGTH: f32 = 0.2;
// const BEEP_VOLUME: Volume = Volume::Linear(0.2);
// const ROOT_FREQ: f32 = 144. * 2.;
// const SCALE: [i32; 7] = [0, 2, 4, 5, 7, 9, 11];
// const PENTATONIC_SCALE: [i32; 6] = [0, 2, 4, 7, 9, 12];

// TODO: Music

pub fn plugin(app: &mut App) {
    app.add_observer(play_hit_sound);
    app.add_observer(play_brick_destroyed_sound);
    app.add_systems(OnEnter(super::GameState::GameOver), play_game_over_sound);
}

fn play_brick_destroyed_sound(
    _event: On<BrickDestroyed>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    speed: Single<&super::physics::Velocity, With<super::ball::Ball>>,
) {
    let audio = asset_server.load("brickbreak.wav");
    let speed = speed.0.length() - super::ball::BALL_BASE_VELOCITY.length();

    let note = 1.0595_f32.powf(speed);
    commands.spawn((
        AudioPlayer::new(audio),
        PlaybackSettings::DESPAWN.with_speed(note),
    ));
}

/// Eheheheh 😈
fn rand64() -> u64 {
    use std::hash::{BuildHasher, Hasher};
    std::hash::RandomState::new().build_hasher().finish()
}

fn play_hit_sound(
    _event: On<BallCollision>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let audio = asset_server.load("pop.wav");
    let note = rand64() % 11;
    let note = note as i64 - 5;
    let note = note as f32 * 0.1 + 1.0;

    commands.spawn((
        AudioPlayer::new(audio),
        PlaybackSettings::DESPAWN
            .with_volume(Volume::Linear(0.2))
            .with_speed(note),
    ));
}

fn play_game_over_sound(
    mut commands: Commands,
    lives: Res<super::Lives>,
    asset_server: Res<AssetServer>,
) {
    let file = if lives.0 > 0 { "win.wav" } else { "lose.wav" };
    commands.spawn((
        AudioPlayer::new(asset_server.load(file)),
        PlaybackSettings::DESPAWN,
    ));
}
