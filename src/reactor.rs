// Import Bevy game engine essentials
use bevy::{prelude::*, render::view::RenderLayers, math::Vec3Swizzles, time::Stopwatch};
// Import components, resources, and events
use crate::components::*;

// Plugin for handling reactor sprites and logic
pub struct ReactorPlugin;

impl Plugin for ReactorPlugin {
    fn build(&self, app: &mut App) {
        app
			.add_systems(OnEnter(GameState::Reactor), (
				spawn_reactor_visuals,
				spawn_reactor_buttons,
				spawn_reactor_levers,
				spawn_reactors,
			))
			.add_systems(Update, (
				update_stopwatch,
				handle_levers,
				intake_connections,
				outlet_connections.after(intake_connections),
				check_product_reactor,
			).run_if(in_state(GameState::Reactor))
			.run_if(not(in_state(PauseState::Paused)))
		)
		;
	}
}

// Spawn all the visual elements of the reactor such
// as the backgrounds and UI text, as well as the highlight
// and tooltip sprites which are initially hidden
fn spawn_reactor_visuals(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	ortho_size: Res<OrthoSize>,
) {
	commands
		.spawn((SpriteBundle {
			texture: asset_server.load("sprites/reactor_controls.png"),
			transform: Transform::from_xyz(0.0, 0.0, 0.0),
			sprite: Sprite {
				custom_size: Some(Vec2::new(ortho_size.width, ortho_size.height)),
				..Default::default()
			},
			//visibility: Visibility::Hidden,
			..Default::default()
		},
		DespawnOnExitGameState,
		Name::new("Reactor Controls"),
	));

	commands
		.spawn((SpriteBundle {
			transform: Transform::from_xyz(0.0, 0.0, 0.0),
			sprite: Sprite {
				color: Color::DARK_GREEN,
				custom_size: Some(Vec2::new(ortho_size.width * 10.0, ortho_size.height * 10.0)),
				..Default::default()
			},
			..Default::default()
		},
		RenderLayers::layer(1),
		DespawnOnExitGameState,
		Name::new("Reactor Background"),
	));

	commands
		.spawn((SpriteBundle {
			texture: asset_server.load("sprites/circle.png"),
			transform: Transform::from_xyz(0.0, 0.0, -1.0),
			sprite: Sprite {
				color: Color::YELLOW,
				custom_size: Some(Vec2::new(1.0, 1.0)),
				..Default::default()
			},
			..Default::default()
		},
		Highlight,
		RenderLayers::layer(1),
		DespawnOnExitGameState,
		Name::new("Highlight"),
	));

	commands
		.spawn((SpriteBundle {
			transform: Transform::from_xyz(0.0, 0.0, -1.0),
			sprite: Sprite {
				color: Color::GRAY,
				custom_size: Some(Vec2::new(TOOLTIP_WIDTH, TOOLTIP_HEIGHT)),
				..Default::default()
			},
			..Default::default()
		},
		Tooltip,
		DespawnOnExitGameState,
		Name::new("Tooltip"),
	));

	commands
		.spawn((SpriteBundle {
			transform: Transform::from_xyz(112.0, 390.0, 730.0),
			sprite: Sprite {
				custom_size: Some(Vec2::new(STOPWATCH_BOX_WIDTH, STOPWATCH_BOX_HEIGHT)),
				..Default::default()
			},
			..Default::default()
		},
		DespawnOnExitGameState,
		Name::new("Stopwatch Box Sprite")
	)).with_children(|parent| {
		parent
			.spawn((Text2dBundle {
				text_2d_bounds: bevy::text::Text2dBounds{ size: Vec2::new(
					STOPWATCH_BOX_WIDTH - STOPWATCH_BOX_MARGINS * 2.0,
					STOPWATCH_BOX_HEIGHT - STOPWATCH_BOX_MARGINS,
				)},
				transform: Transform::from_xyz(STOPWATCH_BOX_WIDTH / 2.0 - STOPWATCH_BOX_MARGINS, 0.0, 10.0),
				text_anchor: bevy::sprite::Anchor::CenterRight,
				text: Text::from_section(format!("0.00"), get_cutscene_text_style(&asset_server))
				.with_alignment(TextAlignment::Right),
				..Default::default()
			},
			StopwatchText(Stopwatch::default()),
			Name::new("Stopwatch Text")
		));
	});
}

// Update the stopwatch to track time spent on a level
fn update_stopwatch(
	mut stopwatch_text_query: Query<(&mut Text, &mut StopwatchText)>,
	asset_server: Res<AssetServer>,
	time: Res<Time>,
) {
	for (mut text, mut stopwatch) in stopwatch_text_query.iter_mut() {
		stopwatch.0.tick(time.delta());
		text.sections = vec![
			TextSection::new(
				if stopwatch.0.elapsed_secs() < 60.0 {format!("{:.2} s", stopwatch.0.elapsed_secs())}
				else if stopwatch.0.elapsed_secs() < 6000.0 {format!("{:.0} m {:.0} s", (stopwatch.0.elapsed_secs() / 60.0).floor(), stopwatch.0.elapsed_secs() % 60.0)}
				else if stopwatch.0.elapsed_secs() < 600000.0 {format!("{:.0} m", (stopwatch.0.elapsed_secs() / 60.0).floor())}
				else {format!("You win!")},
				get_stopwatch_text_style(&asset_server),
			)
		];
	}
}

// Spawn the reactor buttons for selecting molecules
// or for exiting the level
fn spawn_reactor_buttons(
	mut commands: Commands,
) {
	// Spawn molecule select buttons
	for j in 0..6 {
		for i in 0..3 {
			let button = StandardButton {
				location: Vec3::new(-720.0 + 125.0 * i as f32, 288.0 - 100.0 * j as f32, 710.0),
				dimensions: Dimensions {
					width: 75.0,
					height: 75.0,
				},
			};
			commands.spawn((SpriteBundle {
					transform: Transform::from_translation(button.location),
					sprite: Sprite {
						custom_size: Some(Vec2::new(button.dimensions.width, button.dimensions.height)), 
						..Default::default()
					},
					..Default::default()
				},
				ButtonEffect::ReactorButton(ReactorButton::SelectMolecule(i + j*3)),
				button,
				DespawnOnExitGameState,
				Name::new(format!("Molecule Select Button {}", i + j*3))
			));
		}
	}

	let button = StandardButton {
		location: Vec3::new(675.0, -375.0, 710.0),
		dimensions: Dimensions {
			width: 150.0,
			height: 75.0,
		},
	};
	commands
		.spawn((SpriteBundle {
			transform: Transform::from_translation(button.location),
			sprite: Sprite {
				custom_size: Some(Vec2::new(button.dimensions.width, button.dimensions.height)), 
				..Default::default()
			},
			..Default::default()
		},
		ButtonEffect::ReactorButton(ReactorButton::ExitReactor),
		button,
		DespawnOnExitGameState,
		Name::new("Exit Reactor Button"),
	));
}

// Spawn reactor levers for controlling the temperature
// and pressure of the selected reactors
fn spawn_reactor_levers(
	mut commands: Commands,
) {
	commands
		.spawn((SpriteBundle {
			transform: Transform::from_xyz(712.0, 100.0, 720.0),
			sprite: Sprite {
				color: Color::BLACK,
				custom_size: Some(Vec2::new(LEVER_WIDTH, LEVER_HEIGHT)),
				..Default::default()
			},
			..Default::default()
		},
		LeverInfo{
			lever_type: 0,
			min_height: 100.0,
			max_height: 300.0,
		},
		DespawnOnExitGameState,
		Name::new("Temperature Lever"),
	));

	commands
		.spawn((SpriteBundle {
			transform: Transform::from_xyz(712.0, -230.0, 720.0),
			sprite: Sprite {
				color: Color::BLACK,
				custom_size: Some(Vec2::new(LEVER_WIDTH, LEVER_HEIGHT)),
				..Default::default()
			},
			..Default::default()
		},
		LeverInfo{
			lever_type: 1,
			min_height: -230.0,
			max_height: 30.0,
		},
		DespawnOnExitGameState,
		Name::new("Pressure Lever"),
	));
}

// Allow user to click and drag levers to adjust their position and 
// thus change the temperature and pressure of the selected reactor
fn handle_levers(
	window_query: Query<&Window>,
	ortho_size: Res<OrthoSize>,
	mouse: Res<Input<MouseButton>>,
	lever_query: Query<(Entity, &Transform, Without<SelectedLever>)>,
	mut commands: Commands,
	mut reactor_condition_query: Query<(&mut ReactorCondition, With<SelectedReactor>)>,
	mut selected_lever_query: Query<(Entity, &mut Transform, &LeverInfo, With<SelectedLever>)>,
) {
	// Get the current window, and the cursor position scaled 
	// to the window size
	let w = window_query.single();
	if let Some(p) = w.cursor_position() {
		let p = Vec2::new(
			ortho_size.width * (p.x / w.width() - 0.5), 
			-ortho_size.height * (p.y / w.height() - 0.5)
		);
		if mouse.just_pressed(MouseButton::Left) {
			for (entity, transform, _) in lever_query.iter() {
				if (p.x - transform.translation.x).abs() < LEVER_WIDTH / 2.0 
				&& (p.y - transform.translation.y).abs() < LEVER_HEIGHT / 2.0 {
					commands.entity(entity).insert(SelectedLever);
				}
			}
		}
		if !mouse.pressed(MouseButton::Left) {
			for (entity, _, _, _) in selected_lever_query.iter() {
				commands.entity(entity).remove::<SelectedLever>();
			}
		}
		for (_, mut transform, info, _) in selected_lever_query.iter_mut() {
			transform.translation.y = p.y.clamp(info.min_height, info.max_height);
			let percent = (transform.translation.y - info.min_height)/(info.max_height - info.min_height);
			for (mut condition, _) in reactor_condition_query.iter_mut() {
				if info.lever_type == 0 {
					condition.temperature = percent;
				} else {
					condition.pressure = percent;
				}
			}
		}
	}
}

// Check for collisions between intake connections and molecules
// and if so emit a connection event
fn intake_connections(
	mut commands: Commands,
	mut ev_w_connection: EventWriter<ConnectionEvent>,
	selected_molecule_query: Query<(Entity, &Transform, &MoleculeInfo, &Velocity, &ReactorInfo, With<SelectedMolecule>)>,
	molecule_query: Query<(Entity, &Transform, &MoleculeInfo, &Velocity, &ReactorInfo, (With<Molecule>, Without<SelectedMolecule>))>,
	connection_query: Query<(&Transform, &Connection)>,
) {
	for (c_transform, connection) in connection_query.iter() {
		for (entity, m_transform, m_info, velocity, r_info, _) in molecule_query.iter() {
			if connection.reactor_id == r_info.reactor_id {
				if (c_transform.translation.xy() - m_transform.translation.xy()).length() < CONNECTION_WIDTH 
				&& connection.intake
				&& connection.filter[m_info.index] {
					ev_w_connection.send(ConnectionEvent{
						connection_id: connection.connection_id,
						m_info: *m_info,
						r_info: *r_info,
						velocity: velocity.0,
						selected: false,
					});
					commands.entity(entity).despawn_recursive();
				}
			}
		}
		for (entity, m_transform, m_info, velocity, r_info, _) in selected_molecule_query.iter() {
			if connection.reactor_id == r_info.reactor_id {
				if (c_transform.translation.xy() - m_transform.translation.xy()).length() < CONNECTION_WIDTH && connection.intake {
					ev_w_connection.send(ConnectionEvent{
						connection_id: connection.connection_id,
						m_info: *m_info,
						r_info: *r_info,
						velocity: velocity.0,
						selected: true,
					});
					commands.entity(entity).despawn_recursive();
				}
			}
		}
	}
}

// Handles connection events by spawning a new molecule
// at every outlet connection with a matching ID
fn outlet_connections(
	mut commands: Commands,
	mut ev_r_connection: EventReader<ConnectionEvent>,
	mut texture_atlases: ResMut<Assets<TextureAtlas>>,
	asset_server: Res<AssetServer>,
	selected_palette: Res<SelectedPalette>,
	connection_query: Query<(&Transform, &Connection)>,
	reactor_query: Query<&ReactorInfo>,
) {
	for ev in ev_r_connection.iter() {
		for (transform, connection) in connection_query.iter() {
			if connection.connection_id == ev.connection_id && !connection.intake {
				let texture_handle = asset_server.load(get_molecule_path(ev.m_info.index));
				let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 4, 2, None, None);
				let texture_atlas_handle = texture_atlases.add(texture_atlas);

				let mut r_info = ev.r_info;
				for r_new_info in reactor_query.iter() {
					if connection.reactor_id == r_new_info.reactor_id {
						r_info = *r_new_info;
					}
				}

				let mut prev_transform = *transform;
				prev_transform.rotate_local_z((rand::random::<f32>() - 0.5) * 0.75_f32);
				let direction = -prev_transform.local_y().xy().normalize();

				let mut mole = commands
					.spawn((SpriteSheetBundle {
						transform: Transform::from_translation((transform.translation.xy() + -prev_transform.local_y().xy().normalize() * ev.m_info.radius * 1.01).extend(500.0)),
						texture_atlas: texture_atlas_handle.clone(),
						sprite: TextureAtlasSprite{
							color: get_molecule_color(ev.m_info.index, selected_palette.0),
							index: 0,
							custom_size: Some(Vec2::new(ev.m_info.radius * 2.0, ev.m_info.radius * 2.0)),
							..Default::default()
						},
						..Default::default()
					},
					r_info,
					Molecule(get_molecule_lifetime(ev.m_info.index)),
					ev.m_info,
					ParticleTrail{
						spawn_timer: Timer::from_seconds(PARTICLE_SPAWN_DELAY, TimerMode::Repeating),
						duration: PARTICLE_DURATION,
					},
					Velocity(ev.velocity.length() * direction),
					AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
					AnimationIndices{ 
						first: 0, 
						total: 8,
					},
					RenderLayers::layer(1),
					DespawnOnExitGameState,
					Name::new("Molecule")
				));
				if ev.selected {
					mole.insert(SelectedMolecule);
				}
			}
		}
	}
}

// Spawns all the reactors for the given level and assigns them
// a unique ID, as well as spawning launch tubes and connections
fn spawn_reactors(
	mut commands: Commands,
	mut texture_atlases: ResMut<Assets<TextureAtlas>>,
	asset_server: Res<AssetServer>,
	level: Res<SelectedLevel>,
	selected_palette: Res<SelectedPalette>,
) {
	let reactors = get_reactors(level.0);
	for (i, reactor) in reactors.iter().enumerate() {
		// Spawn entity with features common to both reactors
		let mut r = commands
			.spawn((SpriteBundle::default(),
			*reactor,
			ReactorCondition{
				temperature: 0.0,
				pressure: 0.0,
			},
			RenderLayers::layer(1),
			DespawnOnExitGameState,
			Name::new(format!("Reactor {}", i))
		));
		// Match reactor type and add additional features to specific reactor types
		let z = 900.0;
		match reactor.reactor_type {
			ReactorType::Rectangle{origin, dimensions} => {
				r.insert(Transform::from_xyz(origin.x, origin.y, 10.0));
				r.insert(Sprite{color: Color::GREEN, custom_size: Some(Vec2::new(dimensions.width, dimensions.height)), ..Default::default()});
				if reactor.input_chamber {
					commands
						.spawn((SpriteBundle {
							transform: Transform::from_xyz(origin.x, origin.y + dimensions.height / 2.0, z),
							sprite: Sprite {
								color: Color::DARK_GRAY,
								custom_size: Some(Vec2::new(LAUNCH_TUBE_WIDTH, LAUNCH_TUBE_HEIGHT)),
								..Default::default()
							},
							..Default::default()
						},
						LaunchTube{
							id: i,
							current_rotation: 0.0,
						},
						RenderLayers::layer(1),
						DespawnOnExitGameState,
						Name::new("Launch Tube"),
					));
				}
				for (index, location, velocity) in get_reactor_initialization(level.0, reactor.reactor_id) {
					let direction = Vec2::new(rand::random::<f32>() - 0.5, rand::random::<f32>() - 0.5).normalize();
					commands
						.spawn((SpriteSheetBundle {
							transform: Transform::from_xyz(
								origin.x + location.x + rand::random::<f32>(),
								origin.y + location.y + rand::random::<f32>(),
								500.0,
							),
							texture_atlas: texture_atlases.add(TextureAtlas::from_grid(asset_server.load(get_molecule_path(index)), Vec2::new(32.0, 32.0), 4, 2, None, None)).clone(),
							sprite: TextureAtlasSprite{
								color: get_molecule_color(index, selected_palette.0),
								index: 0,
								custom_size: Some(Vec2::new(get_molecule_radius(index) * 2.0, get_molecule_radius(index) * 2.0)),
								..Default::default()
							},
							..Default::default()
						},
						*reactor,
						Molecule(get_molecule_lifetime(index)),
						MoleculeInfo {
							index: index,
							reacted: false,
							radius: get_molecule_radius(index),
							mass: get_molecule_mass(index),
						},
						ParticleTrail{
							spawn_timer: Timer::from_seconds(PARTICLE_SPAWN_DELAY, TimerMode::Repeating),
							duration: PARTICLE_DURATION,
						},
						Velocity(Vec2::new((rand::random::<f32>()-0.5)*velocity.x, (rand::random::<f32>()-0.5)*velocity.y) * direction),
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
				for (direction, connection) in get_reactor_connections(level.0, reactor.reactor_id).0 {
					let translation = Vec3::new(origin.x + direction.x * dimensions.width / 2.0, origin.y + direction.y * dimensions.height/2.0, z);
					commands.spawn((SpriteBundle {
						transform: Transform::from_translation(translation)
						.with_rotation(Quat::from_rotation_z(if direction.y == 1.0 {0.0} else if direction.y == -1.0 {180.0_f32.to_radians()} else {if direction.x == 1.0 {-90.0_f32.to_radians()} else {90.0_f32.to_radians()}})),
						sprite: Sprite {
							color: if connection.intake {get_molecule_color(connection.connection_id, selected_palette.0)} 
								else {*get_molecule_color(connection.connection_id, selected_palette.0).set_a(0.8)},
							custom_size: Some(Vec2::new(CONNECTION_WIDTH, CONNECTION_HEIGHT)),
							..Default::default()
						},
						..Default::default()
					},
					connection,
					RenderLayers::layer(1),
					DespawnOnExitGameState,
					Name::new("Connection"),
					));
				};
			},
			ReactorType::Circle{origin, radius} => {
				let texture: Handle<Image> = asset_server.load("sprites/circle.png");
				r.insert(texture);
				r.insert(Transform::from_xyz(origin.x, origin.y, 10.0));
				r.insert(Sprite{color: Color::GREEN, custom_size: Some(Vec2::new(radius*2.0, radius*2.0)), ..Default::default()});
				if reactor.input_chamber {
					commands
						.spawn((SpriteBundle {
							transform: Transform::from_xyz(origin.x, origin.y + radius, z),
							sprite: Sprite {
								color: Color::DARK_GRAY,
								custom_size: Some(Vec2::new(LAUNCH_TUBE_WIDTH, LAUNCH_TUBE_HEIGHT)),
								..Default::default()
							},
							..Default::default()
						},
						LaunchTube{
							id: i,
							current_rotation: 0.0,
						},
						RenderLayers::layer(1),
						DespawnOnExitGameState,
						Name::new("Launch Tube"),
					));
				}
				for (index, location, velocity) in get_reactor_initialization(level.0, reactor.reactor_id) {
					let direction = Vec2::new(rand::random::<f32>() - 0.5, rand::random::<f32>() - 0.5).normalize();
					commands
						.spawn((SpriteSheetBundle {
							transform: Transform::from_xyz(
								origin.x + location.x + rand::random::<f32>(),
								origin.y + location.y + rand::random::<f32>(),
								500.0,
							),
							texture_atlas: texture_atlases.add(TextureAtlas::from_grid(asset_server.load(get_molecule_path(index)), Vec2::new(32.0, 32.0), 4, 2, None, None)).clone(),
							sprite: TextureAtlasSprite{
								color: get_molecule_color(index, selected_palette.0),
								index: 0,
								custom_size: Some(Vec2::new(get_molecule_radius(index) * 2.0, get_molecule_radius(index) * 2.0)),
								..Default::default()
							},
							..Default::default()
						},
						*reactor,
						Molecule(get_molecule_lifetime(index)),
						MoleculeInfo {
							index: index,
							reacted: false,
							radius: get_molecule_radius(index),
							mass: get_molecule_mass(index),
						},
						ParticleTrail{
							spawn_timer: Timer::from_seconds(PARTICLE_SPAWN_DELAY, TimerMode::Repeating),
							duration: PARTICLE_DURATION,
						},
						Velocity(Vec2::new((rand::random::<f32>()-0.5)*velocity.x, (rand::random::<f32>()-0.5)*velocity.y) * direction),
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
				for (mut direction, connection) in get_reactor_connections(level.0, reactor.reactor_id).0 {
					direction = direction.normalize();
					let translation = Vec3::new(origin.x + direction.x * radius, origin.y + direction.y * radius, z);
					commands.spawn((SpriteBundle {
						transform: Transform::from_translation(translation)
						.with_rotation(Quat::from_rotation_arc(Vec3::Y, (translation.xy() - origin).normalize().extend(0.0))),
						sprite: Sprite {
							color: if connection.intake {get_molecule_color(connection.connection_id, selected_palette.0)} 
								else {*get_molecule_color(connection.connection_id, selected_palette.0).set_a(0.8)},
							custom_size: Some(Vec2::new(CONNECTION_WIDTH, CONNECTION_HEIGHT)),
							..Default::default()
						},
						..Default::default()
					},
					connection,
					RenderLayers::layer(1),
					DespawnOnExitGameState,
					Name::new("Connection"),
					));
				};
			},
		}
	}
}

// Checks to see if the target amount of molecules of the
// desired type are held in the product reactor and if so
// triggers a win screen popup
fn check_product_reactor(
	mut ev_w_popup: EventWriter<PopupEvent>,
	mut next_state: ResMut<NextState<PauseState>>,
	mut win_countdown: ResMut<WinCountdown>,
	asset_server: Res<AssetServer>,
	reactor_query: Query<(&ReactorInfo, With<ReactorCondition>)>,
	molecule_query: Query<(&MoleculeInfo, &ReactorInfo, With<Molecule>)>,
	current_level: Res<SelectedLevel>,
	time: Res<Time>,
) {
	for (r_info, _) in reactor_query.iter() {
		if r_info.product_chamber {
			let (desired_quantity, desired_molecule) = get_level_goal(current_level.0);
			let mut desired_count = 0;
			for (m_info, m_r_info, _) in molecule_query.iter() {
				if r_info.reactor_id == m_r_info.reactor_id && m_info.index == desired_molecule {
					desired_count += 1;
					if desired_count >= desired_quantity {
						break;
					}
				}
			}
			if desired_count >= desired_quantity {
				win_countdown.0.tick(time.delta());
				if win_countdown.0.just_finished() {
					next_state.set(PauseState::Paused);
					ev_w_popup.send(PopupEvent{ 
						origin: Vec2::new(0.0, 0.0), 
						image: asset_server.load("sprites/logbook_base.png"),
						alpha: 1.0,
						popup_type: PopupType::WinScreen,
					});
				}
			} else {
				win_countdown.0.reset();
			}
		}
	}
}