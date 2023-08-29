use std::time::Duration;

// Import Bevy game engine essentials
use bevy::{prelude::*, math::Vec3Swizzles};
// Import Kira audio for Bevy to handle loading sound files
use bevy_kira_audio::{Audio, AudioControl, AudioInstance, AudioTween};
use bevy_pkv::PkvStore;
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
			update_bgm_volume.run_if(in_state(GameState::Menu)),
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
	pkv: Res<PkvStore>,
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
	commands.insert_resource(SfxHandles(audio_handles));

	if let Ok(save_data) = pkv.get::<SaveData>("save_data") {
		let bgm_handle = audio
			.play(asset_server.load("audio/bgm.ogg"))
			.looped()
			.with_volume(save_data.bgm_volume.powf(2.5))
			.handle();

		commands.insert_resource(BgmHandle(bgm_handle));
	}
}

fn update_bgm_volume(
	pkv: Res<PkvStore>,
	bgm_handle: Res<BgmHandle>,
	mut audio_instances: ResMut<Assets<AudioInstance>>,
) {
	if let Ok(save_data) = pkv.get::<SaveData>("save_data") {
		if let Some(instance) = audio_instances.get_mut(&bgm_handle.0) {
			instance.set_volume(save_data.bgm_volume.powf(2.5), AudioTween::linear(Duration::from_millis(100)));
		}
	}
}

fn play_collision_sfx(
	reactor_camera_query: Query<(&OrthographicProjection, &Transform, With<ReactorCamera>)>,
	pkv: ResMut<PkvStore>,
	mut audio_handles: ResMut<SfxHandles>,
	mut audio_instances: ResMut<Assets<AudioInstance>>,
	mut ev_r_sound_effect: EventReader<SoundEffectEvent>,
	time: Res<Time>,
) {
	if let Ok(save_data) = pkv.get::<SaveData>("save_data") {
		for handle in audio_handles.0.iter_mut() {
			if let Some(instance) = audio_instances.get_mut(&handle.0) {
				handle.1 -= 20.0 * save_data.sfx_volume * time.delta_seconds() as f64;
				instance.set_volume((handle.1).clamp(0.0, save_data.sfx_volume), AudioTween::linear(Duration::from_millis(100)));
			}
		}

		let (ortho_proj, transform, _) = reactor_camera_query.single();
		for ev in ev_r_sound_effect.iter() {
			let offset = (ev.location - transform.translation.xy()).abs();
			if offset.y < 500.0 * ortho_proj.scale && offset.x < 500.0 * ortho_proj.scale * ASPECT_RATIO {
				if let Some(instance) = audio_instances.get_mut(&audio_handles.0[ev.note % 8].0) {
					audio_handles.0[ev.note % 8].1 = save_data.sfx_volume;
					instance.set_volume(((save_data.sfx_volume as f32 - offset.length()/(500.0 * ortho_proj.scale)).powf(2.0)).clamp(0.0, save_data.sfx_volume as f32) as f64,
					AudioTween::linear(Duration::from_millis(100)));
					instance.set_panning(((ev.location.x - transform.translation.x)/(500.0*ortho_proj.scale)*0.5 + 0.5).clamp(0.0, 1.0) as f64,
					AudioTween::linear(Duration::from_millis(100)));
				}
			}
		}
	}
}

fn silence_collision_sfx(
	audio_handles: Res<SfxHandles>,
	mut audio_instances: ResMut<Assets<AudioInstance>>,
) {
	for handle in &audio_handles.0 {
		if let Some(instance) = audio_instances.get_mut(&handle.0) {
			instance.set_volume(0.0, AudioTween::linear(Duration::from_millis(200)));
		}
	}
}