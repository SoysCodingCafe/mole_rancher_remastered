// Import Bevy game engine essentials
use bevy::{prelude::*, render::{camera::ScalingMode, view::RenderLayers}, core_pipeline::clear_color::ClearColorConfig};
// Import Pkv Store for saving and loading game data
use bevy_pkv::PkvStore;
// Import components, resources, and events
use crate::components::*;

// Plugin for handling all initial one time setup 
// such as camera spawning, loading save data and 
// initializing resources
pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app
			// States
			.add_state::<GameState>()
			.add_state::<PauseState>()
			// Events
			.add_event::<ButtonCall>()
			.add_event::<FadeTransitionEvent>()
			.add_event::<ReplayLevelEvent>()
			.add_event::<PopupEvent>()
			.add_event::<PopupCompleteEvent>()
			.add_event::<ConnectionEvent>()
			.add_event::<SoundEffectEvent>()
			// Resources
			.insert_resource(OrthoSize{width: ORTHO_WIDTH, height: ORTHO_HEIGHT})
			.insert_resource(AudioVolume{bgm: 0.8, sfx: 0.8})
			.insert_resource(PkvStore::new(".SoysCodingCafe", "Mole Rancher Remastered"))
			.insert_resource(CutsceneTracker{
				current_scene: 0,
				current_line: 0,
				current_character: 0,
				full_line: "".to_string(),
				actor_info: ActorInfo { actor: Actor::Nobody },
				cutscene_state: CutsceneState::Initialize,
			})
			.insert_resource(SelectedPalette(0))
			.insert_resource(SelectedLevel(0))
			.insert_resource(SelectedMoleculeType(0))
			.insert_resource(CurrentCost(0))
			.insert_resource(MoleculeCount{total: 0, cap: MOLECULE_CAP})
			.insert_resource(BootTimer(Timer::from_seconds(BOOT_DURATION, TimerMode::Once)))
			.insert_resource(LaunchTimer(Timer::from_seconds(LAUNCH_COOLDOWN, TimerMode::Once)))
			.insert_resource(WinCountdown(Timer::from_seconds(WIN_COUNTDOWN_LENGTH, TimerMode::Once)))
			.insert_resource(FadeTransitionTimer(Timer::from_seconds(FADE_TRANSITION_DURATION, TimerMode::Once)))
			.insert_resource(TextSpeedTimer(Timer::from_seconds(TEXT_SPEED, TimerMode::Repeating)))
			// Systems
			.add_systems( Startup,(
				load_game,
				spawn_cameras,
				spawn_splash_screen,
			))
			.add_systems( Update,(
				animate_sprites.run_if(not(in_state(PauseState::Paused))),
				advance_splash_screen.run_if(in_state(GameState::Boot)),
			))
			.add_systems(OnExit(PauseState::Paused), (
				despawn_entities_with::<DespawnOnExitPauseState>,
			))
			.add_systems(OnExit(GameState::Boot), (
				despawn_entities_with::<DespawnOnExitGameState>,
			))
			.add_systems(OnExit(GameState::Cutscene), (
				despawn_entities_with::<DespawnOnExitGameState>,
			))
			.add_systems(OnExit(GameState::Menu), (
				despawn_entities_with::<DespawnOnExitGameState>,
			))
			.add_systems(OnExit(GameState::Lab), (
				despawn_entities_with::<DespawnOnExitGameState>,
			))
			.add_systems(OnExit(GameState::Reactor), (
				despawn_entities_with::<DespawnOnExitGameState>,
			))
		;
	}
}

// On startup loads data from save file if it exists, otherwise
// creates a new blank save file for the player
fn load_game(
	mut pkv: ResMut<PkvStore>,
	mut selected_palette: ResMut<SelectedPalette>,
	mut audio_volume: ResMut<AudioVolume>,
) {
	if let Ok(save_data) = pkv.get::<SaveData>("save_data") {
		selected_palette.0 = save_data.selected_palette;
		audio_volume.bgm = save_data.bgm_volume;
		audio_volume.sfx = save_data.sfx_volume;
	} else {
		let mut levels_unlocked = Vec::new();
		let mut best_times = Vec::new();
		let mut best_costs = Vec::new();
		let mut cutscenes_unlocked = Vec::new();
		for _ in 0..NUMBER_OF_LEVELS {
			levels_unlocked.push(false);
			best_times.push(999999.0);
			best_costs.push(999999);
		}
		for _ in 0..NUMBER_OF_CUTSCENES {
			cutscenes_unlocked.push(false);
		}
		let save_data = SaveData{
			sfx_volume: 0.8,
			bgm_volume: 0.8,
			selected_palette: 0,
			particles_enabled: true,
			levels_unlocked: levels_unlocked,
			best_times: best_times,
			best_costs: best_costs,
			cutscenes_unlocked: cutscenes_unlocked,
		};
		pkv.set("save_data", &save_data)
			.expect("Unable to save data");
	}
}

// Spawns the two cameras used throughout the game:
// Main Camera - Renders main UI and menus, with
// orthographic scaling to allow window resizing
// Reactor Camera - Renders reactor view, allows
// panning and zooming
fn spawn_cameras(
	mut commands: Commands,
	ortho_size: Res<OrthoSize>,
) {
	commands
		.spawn((
			Camera2dBundle{
				camera: Camera {
					order: 1,
					..Default::default()
				},
				camera_2d: Camera2d {clear_color: ClearColorConfig::None, ..Default::default()},
				transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1000.0)),
				projection: OrthographicProjection {
					scale: 1.0,
					scaling_mode: ScalingMode::Fixed {width: ortho_size.width, height: ortho_size.height},
					..Default::default()
				},
				..default()
			},
			MainCamera,
			Name::new("Main Camera"),
	));
	commands
		.spawn((
			Camera2dBundle{
				camera: Camera {
					order: 0,
					..Default::default()
				},
				transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1000.0)),
				projection: OrthographicProjection {
					scale: 1.0,
					scaling_mode: ScalingMode::Fixed {width: ortho_size.width, height: ortho_size.height},
					..Default::default()
				},
				..default()
			},
			RenderLayers::layer(1),
			ReactorCamera,
			Name::new("Reactor Camera"),
	));
}

// Spawn splash screen when the game starts
fn spawn_splash_screen(
	mut commands: Commands,
	ortho_size: Res<OrthoSize>,
	asset_server: Res<AssetServer>,
) {
	commands
		.spawn((SpriteBundle {
			transform: Transform::from_xyz(0.0, 0.0, 0.0),
			sprite: Sprite {
				custom_size: Some(Vec2::new(ortho_size.width, ortho_size.height)), 
				color: Color::hex("A63FAB").unwrap(),
				..Default::default()},
			..Default::default()
		},
		DespawnOnExitGameState,
		Name::new("Splash Screen")
	));
	commands
		.spawn((SpriteBundle {
			texture: asset_server.load("soycodingcafe_small.png"),
			transform: Transform:: from_xyz(0.0, 0.0, 1.0),
			..default()
		},
		DespawnOnExitGameState,
		Name::new("Logo")
	));
}

// Fade transitions into menu after a certain amount 
// of time or when the user clicks
fn advance_splash_screen(
	mouse: Res<Input<MouseButton>>,
	time: Res<Time>,
	mut boot_timer: ResMut<BootTimer>,
	mut ev_w_fade_transition: EventWriter<FadeTransitionEvent>,
) {
	boot_timer.0.tick(time.delta());
	if mouse.just_pressed(MouseButton::Left) || boot_timer.0.just_finished() {
		ev_w_fade_transition.send(FadeTransitionEvent(GameState::Menu));
	}
}


// Animate every texture atlas sprite
fn animate_sprites(
	mut animation_query: Query<(&mut TextureAtlasSprite, &mut AnimationTimer, &AnimationIndices, Without<MoleculeButton>)>,
	time: Res<Time>,
) {
	for (mut sprite, mut timer, indices, _) in animation_query.iter_mut() {
		timer.0.tick(time.delta());
		if timer.0.just_finished() {
			sprite.index = (sprite.index + 1) % indices.total + indices.first;
		}
	};
}

// Generic function used for despawning all entities with a specific component,
// mainly used for cleanup on state transitions
pub fn despawn_entities_with<T: Component>(
	mut commands: Commands,
	to_despawn: Query<Entity, With<T>>, 
) {
	for entity in &to_despawn {
		commands.entity(entity).despawn_recursive();
	}
}