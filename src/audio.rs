use std::time::Duration;

// Import Bevy game engine essentials
use bevy::{prelude::*, math::Vec3Swizzles};
// Import Kira audio for Bevy to handle loading sound files
use bevy_kira_audio::{Audio, AudioControl, AudioInstance, AudioTween};
// Import components, resources, and events
use crate::components::*;

// Plugin for background music and sound effects
pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app
		.add_systems(Startup, (
			initialize_audio_instances,
		))
		.add_systems(Update, (
			play_collision_sfx.run_if(in_state(GameState::Reactor)),
		))
		.add_systems(OnExit(GameState::Reactor), (
			silence_collision_sfx,
		))
		;
	}
}

fn initialize_audio_instances(
	mut commands: Commands,
	audio: Res<Audio>,
	asset_server: Res<AssetServer>,
) {
	let mut audio_handles: Vec<(Handle<AudioInstance>, f64)> = Vec::new();
	for i in 0..8 {
		audio_handles.push((
			audio
				.play(asset_server.load(get_audio_path(i)))
				.looped()
				.with_volume(0.0)
				.handle(),
				0.0
		));
	}
	commands.insert_resource(AudioHandles(audio_handles));
}

fn play_collision_sfx(
	reactor_camera_query: Query<(&OrthographicProjection, &Transform, With<ReactorCamera>)>,
	mut audio_handles: ResMut<AudioHandles>,
	mut audio_instances: ResMut<Assets<AudioInstance>>,
	mut ev_r_sound_effect: EventReader<SoundEffectEvent>,
	time: Res<Time>,
) {
	for handle in audio_handles.0.iter_mut() {
		if let Some(instance) = audio_instances.get_mut(&handle.0) {
			handle.1 -= 20.0 * time.delta_seconds() as f64;
			instance.set_volume((handle.1).clamp(0.0, 1.0), AudioTween::linear(Duration::from_millis(100)));
		}
	}

	let (ortho_proj, transform, _) = reactor_camera_query.single();
	for ev in ev_r_sound_effect.iter() {
		let offset = (ev.location - transform.translation.xy()).abs();
		if offset.y < 500.0 * ortho_proj.scale && offset.x < 500.0 * ortho_proj.scale * ASPECT_RATIO {
			if let Some(instance) = audio_instances.get_mut(&audio_handles.0[ev.note % 8].0) {
				audio_handles.0[ev.note % 8].1 = 1.0;
				instance.set_volume(((1.0 - offset.length()/(500.0 * ortho_proj.scale)).powf(2.0)).clamp(0.0, 1.0) as f64,
				 AudioTween::linear(Duration::from_millis(100)));
				instance.set_panning(((ev.location.x - transform.translation.x)/(500.0*ortho_proj.scale)*0.5 + 0.5).clamp(0.0, 1.0) as f64,
				 AudioTween::linear(Duration::from_millis(100)));
			}
		}
	}
}

fn silence_collision_sfx(
	audio_handles: Res<AudioHandles>,
	mut audio_instances: ResMut<Assets<AudioInstance>>,
) {
	for handle in &audio_handles.0 {
		if let Some(instance) = audio_instances.get_mut(&handle.0) {
			instance.set_volume(0.0, AudioTween::linear(Duration::from_millis(200)));
		}
	}
}