// Import Bevy game engine essentials
use bevy::{prelude::*, math::Vec3Swizzles, render::view::RenderLayers};
// Import components, resources, and events
use crate::components::*;

// Plugin for handling the main physics logic 
// and molecule spawning
pub struct MoleculesPlugin;

impl Plugin for MoleculesPlugin {
    fn build(&self, app: &mut App) {
        app
			.add_systems(OnEnter(GameState::Reactor), (
				reset_choices,
			))
			.add_systems(Update, (
				track_molecule,
				highlight_tracked_molecule.after(molecule_movement),
			))
			.add_systems(Update, (
				decay_velocity,
				update_molecule_count,
				update_molecule_lifetime.after(update_molecule_count),
				molecule_spawner.after(update_molecule_count),
				molecule_movement.after(update_molecule_count),
				clamp_inside_reactor.after(molecule_movement),
				move_launch_tube,
			).run_if(in_state(GameState::Reactor))
			.run_if(not(in_state(PauseState::Paused)))
		)
		;
	}
}

// Resets the selected molecule type upon re-entering the reactor
fn reset_choices(
	level: Res<SelectedLevel>,
	mut selected_molecule_type: ResMut<SelectedMoleculeType>,
) {
	for i in 0..TOTAL_MOLECULE_TYPES {
		if get_available_molecules(level.0)[i] {
			selected_molecule_type.0 = i;
			break;
		}
	}
}

// Counts the current number of molecules, this is used
// to cap molecule spawning in other systems
fn update_molecule_count(
	molecule_query: Query<With<Molecule>>,
	mut molecule_count: ResMut<MoleculeCount>,
) {
	molecule_count.total = 0;
	for _ in molecule_query.iter() {
		molecule_count.total += 1;
	}
	//println!("Current Molecule Count: {}", molecule_count.total);
}

// If the temperature and pressure conditions are correct then increment 
// each molecule's lifetimer timer, triggering a reaction when it expires
fn update_molecule_lifetime(
	mut commands: Commands,
	reactor_condition_query: Query<(&ReactorCondition, &ReactorInfo)>,
	mut molecule_query: Query<(Entity, &mut Molecule, &Transform, &ReactorInfo)>,
	mut texture_atlases: ResMut<Assets<TextureAtlas>>,
	selected_palette: Res<SelectedPalette>,
	asset_server: Res<AssetServer>,
	molecule_count: Res<MoleculeCount>,
	time: Res<Time>,
) {
	for (entity, mut molecule, transform, r_info) in molecule_query.iter_mut() {
		let (mut current_temperature, mut current_pressure) = (0.0, 0.0);
		for (condition, info) in reactor_condition_query.iter() {
			if info.reactor_id == r_info.reactor_id {
				(current_temperature, current_pressure) = (condition.temperature, condition.pressure);
			}
		}
		match &mut molecule.0 {
			Lifetime::Unstable(ref mut lifetime, reaction) => {
				match reaction {
					ReactionInfo::Reaction(products, temperature_limits, pressure_limits) => {
						if current_temperature >= temperature_limits.0 
						&& current_temperature <= temperature_limits.1
						&& current_pressure >= pressure_limits.0
						&& current_pressure <= pressure_limits.1 {
							lifetime.tick(time.delta());
							if lifetime.finished() {
								commands.entity(entity).despawn_recursive();
								for product in products {
									if molecule_count.total <= molecule_count.cap {
										let velocity = get_molecule_initial_velocity(*product);
										let direction = Vec2::new(rand::random::<f32>() - 0.5, rand::random::<f32>() - 0.5).normalize();
										commands
											.spawn((SpriteSheetBundle {
												transform: Transform::from_xyz(
													transform.translation.x + rand::random::<f32>(),
													transform.translation.y + rand::random::<f32>(),
													transform.translation.z,
												),
												texture_atlas: texture_atlases.add(TextureAtlas::from_grid(asset_server.load(get_molecule_path(*product)), Vec2::new(32.0, 32.0), 4, 2, None, None)).clone(),
												sprite: TextureAtlasSprite{
													color: get_molecule_color(*product, selected_palette.0),
													index: 0,
													custom_size: Some(Vec2::new(get_molecule_radius(*product) * 2.0, get_molecule_radius(*product) * 2.0)),
													..Default::default()
												},
												..Default::default()
											},
											*r_info,
											Molecule(get_molecule_lifetime(*product)),
											MoleculeInfo {
												index: *product,
												reacted: false,
												radius: get_molecule_radius(*product),
												mass: get_molecule_mass(*product),
											},
											ParticleTrail{
												spawn_timer: Timer::from_seconds(PARTICLE_SPAWN_DELAY, TimerMode::Repeating),
												duration: PARTICLE_DURATION,
											},
											Velocity(Vec2::new((rand::random::<f32>()-0.5)*velocity, (rand::random::<f32>()-0.5)*velocity) * direction),
											AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
											AnimationIndices{ 
												first: 0, 
												total: 8,
											},
											RenderLayers::layer(1),
											DespawnOnExitGameState,
											Name::new("Molecule")
										));
									}
								}
							}
						}
					},
					ReactionInfo::None => {
						lifetime.tick(time.delta());
						if lifetime.finished() {
							commands.entity(entity).despawn_recursive()
						}
					}
				}
			},
			Lifetime::Stable => (),
		}
	}
}

// Decreases velocity by a percentage of its value every frame to 
// simulate friction and eventually bring all molecules to a stop
fn decay_velocity(
	mut molecule_query: Query<(&mut Velocity, With<Molecule>)>,
	time: Res<Time>,
) {
	for (mut velocity, _) in molecule_query.iter_mut() {
		let prev_velocity = velocity.0;
		velocity.0 -= prev_velocity * 0.1 * time.delta_seconds();
	}
}

// Handles all molecule movement and collision logic, including
// spawning new molecules from reactions if the current temperature
// and pressure are correct
fn molecule_movement(
	mut commands: Commands,
	mut molecule_query: Query<(Entity, &mut MoleculeInfo, &mut ReactorInfo, &mut Transform, &mut Velocity)>,
	reactor_condition_query: Query<(&ReactorCondition, &ReactorInfo, Without<MoleculeInfo>)>,
	mut texture_atlases: ResMut<Assets<TextureAtlas>>,
	mut ev_w_sound_effect: EventWriter<SoundEffectEvent>,
	selected_palette: Res<SelectedPalette>,
	molecule_count: Res<MoleculeCount>,
	asset_server: Res<AssetServer>,
	time: Res<Time>,
) {
	let mut iter = molecule_query.iter_combinations_mut();
	while let Some([
		(entity_a, mut m_info_a, r_info_a, mut transform_a, mut velocity_a),
		(entity_b, mut m_info_b, r_info_b, mut transform_b, mut velocity_b),
	]) = iter.fetch_next() {
		// Skip over molecule pairs which are not in the same reactor or that have already reacted
		if r_info_a.reactor_id != r_info_b.reactor_id || m_info_a.reacted || m_info_b.reacted {
			continue;
		};
		let mut baby = false;
		let mut bounce = false;
		let offset = transform_a.translation.xy() - transform_b.translation.xy();
		// Molecule collision check takes place here
		if offset.length() <= m_info_a.radius + m_info_b.radius {
			ev_w_sound_effect.send(SoundEffectEvent{note: m_info_a.index, location: transform_a.translation.xy()});
			ev_w_sound_effect.send(SoundEffectEvent{note: m_info_b.index, location: transform_b.translation.xy()});

			let info = valid_molecule_combination(m_info_a.index, m_info_b.index);
			let (mut current_temperature, mut current_pressure) = (0.0, 0.0);
			for (condition, info, _) in reactor_condition_query.iter() {
				if info.reactor_id == r_info_a.reactor_id {
					(current_temperature, current_pressure) = (condition.temperature, condition.pressure);
				}
			}
			match info {
				ReactionInfo::Reaction(products, temperature_limits, pressure_limits) => {
					// Reaction takes place here
					if current_temperature >= temperature_limits.0 
					&& current_temperature <= temperature_limits.1
					&& current_pressure >= pressure_limits.0
					&& current_pressure <= pressure_limits.1 {
						m_info_a.reacted = true;
						m_info_b.reacted = true;
						let mut input_a_accounted_for = false;
						let mut input_b_accounted_for = false;
						let product_contains_a = products.contains(&m_info_a.index);
						let product_contains_b = products.contains(&m_info_b.index);
						if !product_contains_a {commands.entity(entity_a).despawn_recursive();};
						if !product_contains_b {commands.entity(entity_b).despawn_recursive();};
						if product_contains_a && product_contains_b {
							baby = true;
						};

						let mass_a_in = m_info_a.mass;
						let mass_b_in = m_info_b.mass;
						let velocity_a_in = velocity_a.0;
						let velocity_b_in = velocity_b.0;
						let momentum_a = mass_a_in * velocity_a_in;
						let momentum_b = mass_b_in * velocity_b_in;
						let velocity_out = (momentum_a + momentum_b)/(mass_a_in + mass_b_in);

						let total_products = products.len();

						for product in products {
							if product == m_info_a.index && !input_a_accounted_for {
								match total_products {
									1 => {
										//m_info_a.mass += m_info_b.mass;
										velocity_a.0 = velocity_out;
									}
									i => {
										//m_info_a.mass = (mass_a_in + mass_b_in)/i as f32;
										velocity_b.0 = velocity_out * Vec2::new(rand::random::<f32>() - 0.5, rand::random::<f32>() - 0.5).normalize();
									}
								}
								input_a_accounted_for = true;
							}
							else if product == m_info_b.index && !input_b_accounted_for {
								match total_products {
									1 => {
										//m_info_b.mass += m_info_a.mass;
										velocity_b.0 = velocity_out;
									}
									i => {
										//m_info_b.mass = (mass_a_in + mass_b_in)/i as f32;
										velocity_b.0 = velocity_out * Vec2::new(rand::random::<f32>() - 0.5, rand::random::<f32>() - 0.5).normalize();
									}
								}
								input_b_accounted_for = true;
							}
							else if molecule_count.total <= molecule_count.cap {
								let direction = if total_products == 1 {velocity_out.normalize()}
									else {Vec2::new(rand::random::<f32>() - 0.5, rand::random::<f32>() - 0.5).normalize()};
								commands
									.spawn((SpriteSheetBundle {
										transform: Transform::from_xyz(
											transform_b.translation.x + offset.x/2.0 + rand::random::<f32>(), 
											transform_b.translation.y + offset.y/2.0 + rand::random::<f32>(), 
											500.0),
										texture_atlas: texture_atlases.add(TextureAtlas::from_grid(asset_server.load(get_molecule_path(product)), Vec2::new(32.0, 32.0), 4, 2, None, None)).clone(),
										sprite: TextureAtlasSprite{
											color: get_molecule_color(product, selected_palette.0),
											index: 0,
											custom_size: Some(Vec2::new(get_molecule_radius(product) * 2.0, get_molecule_radius(product) * 2.0)),
											..Default::default()
										},
										..Default::default()
									},
									*r_info_a,
									Molecule(get_molecule_lifetime(product)),
									MoleculeInfo {
										index: product,
										reacted: false,
										radius: get_molecule_radius(product),
										mass: get_molecule_mass(product)
										/*match total_products {
											1 => {mass_a_in + mass_b_in},
											i => {(mass_a_in + mass_b_in)/i as f32}
										}*/
									},
									ParticleTrail{
										spawn_timer: Timer::from_seconds(PARTICLE_SPAWN_DELAY, TimerMode::Repeating),
										duration: PARTICLE_DURATION,
									},
									Velocity(Vec2::new(
										get_molecule_initial_velocity(product),
										get_molecule_initial_velocity(product),
									) * direction),
									/*match total_products {
										1 => {Velocity((momentum_a + momentum_b)/(mass_a_in + mass_b_in))},
										i => {Velocity(velocity_out/(i as f32 * direction))}
									},*/
									AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
									AnimationIndices{ 
										first: 0, 
										total: 8,
									},
									RenderLayers::layer(1),
									DespawnOnExitGameState,
									Name::new("Molecule")
								));
							}
						}
					} else {
						bounce = true;
					}
				},
				ReactionInfo::None => bounce = true,
			};

			// Molecule collision repel takes place here
			if bounce || baby {
				let relative_velocity = velocity_a.0 - velocity_b.0;
				let dp = offset * relative_velocity.dot(offset) / ((offset.length_squared()) * (m_info_a.mass + m_info_b.mass));

				velocity_a.0 -= 2.0 * m_info_b.mass * dp;
				velocity_b.0 += 2.0 * m_info_a.mass * dp;

				let push = (offset.normalize() * 1.01 * (m_info_a.radius + m_info_b.radius) - offset).extend(0.0);
				transform_a.translation += push;
				transform_b.translation -= push;
			}
		}
	}

	// Edge collision takes place here
	for (_, mut m_info, r_info, mut transform, mut velocity) in molecule_query.iter_mut() {
		m_info.reacted = false;
		let target = Vec2::new(
			transform.translation.x + velocity.0.x * time.delta_seconds(), 
			transform.translation.y + velocity.0.y * time.delta_seconds()
		);
		match r_info.reactor_type {
			ReactorType::Rectangle{origin, dimensions } => {
				let offset = (target - origin).abs();
				if offset.x > dimensions.width / 2.0 - m_info.radius {
					velocity.0.x = -velocity.0.x;
					ev_w_sound_effect.send(SoundEffectEvent{note: m_info.index, location: transform.translation.xy()});
				}
				if offset.y > dimensions.height / 2.0 - m_info.radius {
					velocity.0.y = -velocity.0.y;
					ev_w_sound_effect.send(SoundEffectEvent{note: m_info.index, location: transform.translation.xy()});
				}
			},
			ReactorType::Circle{origin, radius } => {
				let offset = (target - origin).length();
				if offset > radius - m_info.radius {
					let prev_velocity = velocity.0;
					let normal = (origin - transform.translation.xy()).normalize();
					let new_velocity = prev_velocity - (2.0*prev_velocity.dot(normal)*normal);
					velocity.0.x = new_velocity.x;
					velocity.0.y = new_velocity.y;
					ev_w_sound_effect.send(SoundEffectEvent{note: m_info.index, location: transform.translation.xy()});
				}
			},
		}

		transform.translation.x = transform.translation.x + velocity.0.x * time.delta_seconds();
		transform.translation.y = transform.translation.y + velocity.0.y * time.delta_seconds();
	}
}

fn clamp_inside_reactor(
	mut molecule_query: Query<(&MoleculeInfo, &ReactorInfo, &mut Transform, With<Molecule>)>,
) {
	for (m_info, r_info, mut transform, _) in molecule_query.iter_mut() {
		match r_info.reactor_type {
			ReactorType::Rectangle{origin, dimensions } => {
				let offset = (transform.translation.xy() - origin).abs();
				if offset.x > dimensions.width / 2.0 - m_info.radius {
					transform.translation.x = origin.x + (transform.translation.x - origin.x).signum() * (dimensions.width / 2.0 - m_info.radius);
				}
				if offset.y > dimensions.height / 2.0 - m_info.radius{
					transform.translation.y = origin.y + (transform.translation.y - origin.y).signum() * (dimensions.height / 2.0 - m_info.radius);
				}
			},
			ReactorType::Circle{origin, radius } => {
				let offset = (transform.translation.xy() - origin).length();
				if offset > radius - m_info.radius {
					transform.translation = (origin + (transform.translation.xy() - origin).normalize() * (radius - m_info.radius)).extend(transform.translation.z);
				}
			},
		}
	}
}

// Checks if the player has clicked on a molecule and adds the SelectedMolecule
// component which causes the reactor camera to follow that molecule until another
// molecule is selected or the player pans away
fn track_molecule(
	window_query: Query<&Window>,
	ortho_size: Res<OrthoSize>,
	mouse: Res<Input<MouseButton>>,
	reactor_camera_query: Query<(&Transform, &OrthographicProjection, With<ReactorCamera>)>,
	molecule_query: Query<(Entity, &MoleculeInfo, &Transform, Without<ReactorCamera>)>,
	selected_molecule_query: Query<(Entity, With<SelectedMolecule>)>,
	reactor_query: Query<(Entity, &ReactorInfo, &ReactorCondition, &Transform, (Without<SelectedReactor>, Without<ReactorCamera>, Without<MoleculeInfo>))>,
	selected_reactor_query: Query<(Entity, With<SelectedReactor>)>,
	mut lever_query: Query<(&mut Transform, &LeverInfo, (Without<ReactorCamera>, Without<MoleculeInfo>, Without<ReactorInfo>))>,
	mut commands: Commands,
) {
	// Get the current window, and the cursor position scaled 
	// to the window size
	let w = window_query.single();
	if let Some(p) = w.cursor_position() {
		let mut p = Vec2::new(
			ortho_size.width * (p.x / w.width() - 0.5), 
			-ortho_size.height * (p.y / w.height() - 0.5)
		);
		if mouse.just_pressed(MouseButton::Left) || mouse.just_pressed(MouseButton::Middle) {
			if 	(p.x - REACTOR_VIEWPORT_CENTER.x).abs() <= REACTOR_VIEWPORT_WIDTH / 2.0 && 
			(p.y - REACTOR_VIEWPORT_CENTER.y).abs() <= REACTOR_VIEWPORT_HEIGHT / 2.0 {
				// Scale the cursor position from ortho coords to viewport coords to reactor coords
				p = (p - REACTOR_VIEWPORT_CENTER) / Vec2::new(REACTOR_VIEWPORT_WIDTH, REACTOR_VIEWPORT_HEIGHT) * Vec2::new(ortho_size.width, ortho_size.height);
				// Scale reactor coords according to reactor camera's current position and scale
				let (cam_transform, ortho_proj, _) = reactor_camera_query.single();
				p = p * ortho_proj.scale + cam_transform.translation.xy();
				if mouse.just_pressed(MouseButton::Left) {
					let mut new_target = false;
					for (entity, info, transform, _) in molecule_query.iter() {
						let offset = (p - transform.translation.xy()).length();
						if offset < info.radius * 2.0 {
							commands.entity(entity).insert(SelectedMolecule);
							new_target = true;
							println!("Radius: {}\nMass: {}", info.radius, info.mass);
							break;
						} 
					}
					if new_target {
						for (tracked_entity, _) in selected_molecule_query.iter() {
							commands.entity(tracked_entity).remove::<SelectedMolecule>();
						}
					}
				}
				if mouse.just_pressed(MouseButton::Middle) {
					let mut new_reactor = false;
					for (entity, reactor, condition, transform, _) in reactor_query.iter() {
						match reactor.reactor_type {
							ReactorType::Rectangle{dimensions, ..} => {
								let offset = (p - transform.translation.xy()).abs();
								if offset.x < dimensions.width / 2.0 && offset.y < dimensions.height / 2.0 {
									for (mut transform, info, _) in lever_query.iter_mut() {
										if info.lever_type == 0 {
											transform.translation.y = info.min_height + condition.temperature * (info.max_height - info.min_height);
										} else {
											transform.translation.y = info.min_height + condition.pressure * (info.max_height - info.min_height);
										}
									}
									commands.entity(entity).insert(SelectedReactor);
									new_reactor = true;
									break;
								}
							},
							ReactorType::Circle{radius, ..} => {
								let offset = (p - transform.translation.xy()).length();
								if offset < radius {
									for (mut transform, info, _) in lever_query.iter_mut() {
										if info.lever_type == 0 {
											transform.translation.y = info.min_height + condition.temperature * (info.max_height - info.min_height);
										} else {
											transform.translation.y = info.min_height + condition.pressure * (info.max_height - info.min_height);
										}
									}
									commands.entity(entity).insert(SelectedReactor);
									new_reactor = true;
									break;
								}
							},
						}
					}
					if new_reactor {
						for (entity, _) in selected_reactor_query.iter() {
							commands.entity(entity).remove::<SelectedReactor>();
						}
					}
				}
			}
		}
	}
}

// If a molecule has been selected then highlight it by moving
// the highlight sprite just behind it each frame
fn highlight_tracked_molecule(
	tracked_query: Query<(&Transform, &MoleculeInfo, With<SelectedMolecule>)>,
	mut highlight_query: Query<(&mut Transform, With<Highlight>, Without<SelectedMolecule>)>,
) {
	let mut tracked_active = false;
	for (tracked_transform, info, _) in tracked_query.iter() {
		tracked_active = true;
		for (mut highlight_transform, _, _) in highlight_query.iter_mut() {
			highlight_transform.translation = Vec3::new(
				tracked_transform.translation.x, 
				tracked_transform.translation.y, 
				tracked_transform.translation.z - 1.0);
			highlight_transform.scale.x = info.radius * 2.0;
			highlight_transform.scale.y = info.radius * 2.0;
		}
		break;
	}
	if !tracked_active {
		for (mut highlight_transform, _, _) in highlight_query.iter_mut() {
			highlight_transform.translation.z = -1.0;
		}
	}
}

// Allows the user to move the launch tube either along the top of a reactor
// or around the perimeter or a reactor
fn move_launch_tube(
	mut launch_tube_query: Query<(&mut Transform, &mut LaunchTube)>,
	selected_reactor_query: Query<(&ReactorInfo, With<SelectedReactor>)>,
	keyboard: Res<Input<KeyCode>>,
	time: Res<Time>,
) {
	let mut movement = 0.0;
	if keyboard.pressed(KeyCode::A) {movement -= 1.0}
	else if keyboard.pressed(KeyCode::D) {movement += 1.0};

	let mut rotation = 0.0;
	if keyboard.pressed(KeyCode::Q) {rotation -= 1.0}
	else if keyboard.pressed(KeyCode::E) {rotation += 1.0};

	if movement != 0.0 || rotation != 0.0 {
		for (info, _) in selected_reactor_query.iter() {
			for (mut transform, mut launch_tube) in launch_tube_query.iter_mut() {
				if launch_tube.id == info.reactor_id {
					match info.reactor_type {
						ReactorType::Rectangle{origin, dimensions } => {
							let target = transform.translation.x + movement * (dimensions.width/2.0) * LAUNCH_TUBE_SPEED * time.delta_seconds();
							launch_tube.current_rotation += rotation * LAUNCH_TUBE_ROTATIONAL_SPEED * time.delta_seconds();
							launch_tube.current_rotation = launch_tube.current_rotation.clamp(-45.0, 45.0);
							let angle: f32 = launch_tube.current_rotation;
							transform.rotation = Quat::from_rotation_z(angle.to_radians());
							if (target - origin.x).abs() < dimensions.width / 2.0 - LAUNCH_TUBE_WIDTH / 2.0{
								transform.translation.x = target;
							}
						},
						ReactorType::Circle{origin, radius } => {
							let direction = if movement < 0.0 {(transform.translation.xy() - origin).perp().normalize()} else if movement > 0.0 {-(transform.translation.xy() - origin).perp().normalize()} else {Vec2:: ZERO};
							launch_tube.current_rotation += rotation * LAUNCH_TUBE_ROTATIONAL_SPEED * time.delta_seconds();
							launch_tube.current_rotation = launch_tube.current_rotation.clamp(-45.0, 45.0);
							let angle: f32 = launch_tube.current_rotation;
							transform.translation.x += direction.x * LAUNCH_TUBE_SPEED * radius * time.delta_seconds();
							transform.translation.y += direction.y * LAUNCH_TUBE_SPEED * radius * time.delta_seconds();
							transform.rotation = Quat::from_rotation_arc(Vec3::Y, (transform.translation.xy() - origin).normalize().extend(0.0)).mul_quat(Quat::from_rotation_z(angle.to_radians()));
							transform.translation = ((transform.translation.xy() - origin).clamp_length_max(radius) + origin).extend(transform.translation.z);
						},
					}
				}
			}
		}
	}
}

// Allows the user to drop a molecule spawner at their current location
// which spawns molecules of the selected type at fixed intervals
fn molecule_spawner(
	mut commands: Commands,
	mut molecule_spawner_query: Query<(&Transform, &mut MoleculeSpawnerInfo, &ReactorInfo)>,
	mut texture_atlases: ResMut<Assets<TextureAtlas>>,
	selected_palette: Res<SelectedPalette>,
	asset_server: Res<AssetServer>,
	molecule_count: Res<MoleculeCount>,
	time: Res<Time>,
) {
	for (transform, mut s_info, r_info) in molecule_spawner_query.iter_mut() {
		s_info.spawner_timer.tick(time.delta());
		if s_info.spawner_timer.just_finished() {
			if molecule_count.total <= molecule_count.cap {
				let molecule_index = s_info.spawner_index;
				let (target, distance) = match r_info.reactor_type {
					ReactorType::Rectangle{dimensions, ..} => (Vec2::new(transform.translation.x, transform.translation.y - dimensions.height / 2.0), dimensions.height / 2.0), 
					ReactorType::Circle{origin, radius} => (origin, radius),
				};
				let direction = (target - transform.translation.xy()).normalize();
				let velocity = get_molecule_initial_velocity(molecule_index);
				commands
					.spawn((SpriteSheetBundle {
						transform: Transform::from_translation(((Vec2::new(transform.translation.x, transform.translation.y) - target)
									.clamp_length_max(distance - get_molecule_radius(molecule_index)) + target).extend(500.0)),
						texture_atlas: texture_atlases.add(TextureAtlas::from_grid(asset_server.load(get_molecule_path(molecule_index)), Vec2::new(32.0, 32.0), 4, 2, None, None)).clone(),
						sprite: TextureAtlasSprite{
							color: get_molecule_color(molecule_index, selected_palette.0),
							index: 0,
							custom_size: Some(Vec2::new(get_molecule_radius(molecule_index) * 2.0, get_molecule_radius(molecule_index) * 2.0)),
							..Default::default()
						},
						..Default::default()
					},
					*r_info,
					Molecule(get_molecule_lifetime(molecule_index)),
					MoleculeInfo {
						index: molecule_index,
						reacted: false,
						radius: get_molecule_radius(molecule_index),
						mass: get_molecule_mass(molecule_index),
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
					Name::new("Molecule")
				));
			}
		}
	}
}

