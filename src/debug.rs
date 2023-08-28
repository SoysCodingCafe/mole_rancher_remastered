// Import Bevy game engine essentials
use bevy::{prelude::*, render::view::RenderLayers, math::Vec3Swizzles};
// Import random number generation for adding variation
use rand::Rng;
// Import components, resources, and events
use crate::components::*;

// Plugin for devtools only available in the
// debug version of the game
pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app
			.add_systems( Update, (
				change_game_state,
				change_pause_state,
				//debug_popup,
				//debug_molecule,
				vent_reactor,
			))
		;
	}
}

// Allows the user to freely change the current game state
fn change_game_state(
	keyboard: Res<Input<KeyCode>>,
	mut ev_w_fade_transition: EventWriter<FadeTransitionEvent>,
) {
	if keyboard.just_pressed(KeyCode::Z) {
		ev_w_fade_transition.send(FadeTransitionEvent(GameState::Boot));
	}

	if keyboard.just_pressed(KeyCode::X) {
		ev_w_fade_transition.send(FadeTransitionEvent(GameState::Menu));
	}

	if keyboard.just_pressed(KeyCode::C) {
		ev_w_fade_transition.send(FadeTransitionEvent(GameState::Cutscene));
	}

	if keyboard.just_pressed(KeyCode::V) {
		ev_w_fade_transition.send(FadeTransitionEvent(GameState::Lab));
	}

	if keyboard.just_pressed(KeyCode::B) {
		ev_w_fade_transition.send(FadeTransitionEvent(GameState::Reactor));
	}
}

// Allows the user to freely change the pause state
fn change_pause_state(
	keyboard: Res<Input<KeyCode>>,
	current_state: Res<State<PauseState>>,
	mut next_state: ResMut<NextState<PauseState>>,
) {
	if keyboard.just_pressed(KeyCode::P) {
		if current_state.get() == &PauseState::Unpaused {
			next_state.set(PauseState::Paused);
		} else {
			next_state.set(PauseState::Unpaused);
		}
	}
}

// Allows the user to create a popup to appear at the cursor's current location
fn debug_popup(
	window_query: Query<&Window>,
	ortho_size: Res<OrthoSize>,
	mouse: Res<Input<MouseButton>>,
	asset_server: Res<AssetServer>,
	mut ev_w_popup: EventWriter<PopupEvent>,
) {
	// Get the current window, and the cursor position scaled 
	// to the window size
	let w = window_query.single();
	if let Some(p) = w.cursor_position() {
		let p = Vec2::new(
			ortho_size.width * (p.x / w.width() - 0.5), 
			-ortho_size.height * (p.y / w.height() - 0.5)
		);
		if mouse.just_pressed(MouseButton::Right) {
			ev_w_popup.send(PopupEvent{
				origin: p,
				image: asset_server.load("sprites/popup/level_select.png"),
				alpha: 1.0,
				popup_type: PopupType::LevelSelect,
			});
		}
	}
}



// Allows the user to spawn molecules and spawners
fn debug_molecule(
	mut commands: Commands,
	mut texture_atlases: ResMut<Assets<TextureAtlas>>,
	mut launch_timer: ResMut<LaunchTimer>,
	mut current_cost: ResMut<CurrentCost>,
	selected_palette: Res<SelectedPalette>,
	selected_molecule_type: Res<SelectedMoleculeType>,
	selected_reactor_query: Query<(&ReactorInfo, With<SelectedReactor>)>,
	launch_tube_query: Query<(&Transform, &LaunchTube)>,
	asset_server: Res<AssetServer>,
	keyboard: Res<Input<KeyCode>>,
	time: Res<Time>,
) {
	launch_timer.0.tick(time.delta());
	// Space for selected, R for random, T for true random, S for spawner
	if keyboard.just_pressed(KeyCode::S) || keyboard.just_pressed(KeyCode::Space) || keyboard.pressed(KeyCode::R) || keyboard.pressed(KeyCode::T) || keyboard.pressed(KeyCode::W) {
		let mut rng = rand::thread_rng();
		let molecule_index = if keyboard.just_pressed(KeyCode::Space) || keyboard.pressed(KeyCode::W) {selected_molecule_type.0} else {rng.gen_range(0..5)};
		let radius = if keyboard.pressed(KeyCode::T) {rand::random::<f32>() * 128.0 + 16.0} else {get_molecule_radius(molecule_index)};
		let mass = if keyboard.pressed(KeyCode::T) {rand::random::<f32>() * 3000.0 + 10.0} else {get_molecule_mass(molecule_index)};

		let texture_handle = asset_server.load(get_molecule_path(molecule_index));
		let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 4, 2, None, None);
		let texture_atlas_handle = texture_atlases.add(texture_atlas);

		for (info, _) in selected_reactor_query.iter() {
			for (transform, launch_tube) in launch_tube_query.iter() {
				if launch_tube.id == info.reactor_id {
					if keyboard.just_pressed(KeyCode::Space) || keyboard.pressed(KeyCode::R) || keyboard.pressed(KeyCode::T) || keyboard.pressed(KeyCode::W) {
						if launch_timer.0.finished() {
							launch_timer.0.reset();
							current_cost.0 += get_molecule_cost(molecule_index);
							let (target, distance) = match info.reactor_type {
								ReactorType::Rectangle{dimensions, ..} => (Vec2::new(transform.translation.x, transform.translation.y - dimensions.height / 2.0), dimensions.height / 2.0), 
								ReactorType::Circle{origin, radius} => (origin, radius),
							};
							let direction = -transform.local_y().xy();
							let velocity = get_molecule_initial_velocity(molecule_index);
							commands
								.spawn((SpriteSheetBundle {
									transform: Transform::from_translation(((Vec2::new(transform.translation.x, transform.translation.y) - target)
										.clamp_length_max(distance - get_molecule_radius(molecule_index)) + target).extend(500.0)),
									texture_atlas: texture_atlas_handle.clone(),
									sprite: TextureAtlasSprite{
										color: get_molecule_color(molecule_index, selected_palette.0),
										index: 0,
										custom_size: Some(Vec2::new(radius * 2.0, radius * 2.0)),
										..Default::default()
									},
									..Default::default()
								},
								*info,
								Molecule(get_molecule_lifetime(molecule_index)),
								MoleculeInfo {
									index: molecule_index,
									reacted: false,
									radius: radius,
									mass: mass,
								},
								ParticleTrail{
									spawn_timer: Timer::from_seconds(PARTICLE_SPAWN_DELAY, TimerMode::Repeating),
									duration: PARTICLE_DURATION,
								},
								Velocity(Vec2::new(velocity, velocity) * direction),
								AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
								AnimationIndices{ 
									first: 0, 
									total: 8,
								},
								RenderLayers::layer(1),
								DespawnOnExitGameState,
								Name::new("Debug Molecule")
							));
						}
					}
					// S for Spawner
					if keyboard.just_pressed(KeyCode::S) {
						let (target, distance) = match info.reactor_type {
							ReactorType::Rectangle{dimensions, ..} => (Vec2::new(transform.translation.x, transform.translation.y - dimensions.height / 2.0), dimensions.height / 2.0), 
							ReactorType::Circle{origin, radius} => (origin, radius),
						};
						commands
							.spawn((SpriteBundle {
								transform: Transform::from_translation(((Vec2::new(transform.translation.x, transform.translation.y) - target)
									.clamp_length_max(distance - 64.0) + target).extend(400.0))
									.with_rotation(Quat::from_rotation_arc(Vec3::Y, (transform.translation.xy() - target).normalize().extend(0.0))),
								sprite: Sprite{
									color: Color::BLACK,
									custom_size: Some(Vec2::new(64.0, 128.0)),
									..Default::default()
								},
								..Default::default()
							},
							*info,
							MoleculeSpawnerInfo{
								spawner_index: selected_molecule_type.0,
								spawner_timer: Timer::from_seconds(3.0, TimerMode::Repeating),
							},
							RenderLayers::layer(1),
							DespawnOnExitGameState,
							Name::new("Debug Molecule Spawner"),
						));
					};
				}
			}
		}
	}
}

// Allows the user to remove all molecules from the selected reactor
fn vent_reactor(
	mut commands: Commands,
	keyboard: Res<Input<KeyCode>>,
	selected_reactor_query: Query<(&ReactorInfo, With<SelectedReactor>)>,
	molecule_query: Query<(Entity, &ReactorInfo, With<Molecule>)>,
) {
	if keyboard.just_pressed(KeyCode::L) {
		for (r_info, _) in selected_reactor_query.iter() {
			for (entity, m_r_info, _) in molecule_query.iter() {
				if m_r_info.reactor_id == r_info.reactor_id {
					commands.entity(entity).despawn_recursive();
				}
			}
		}
	}
}