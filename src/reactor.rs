// Import Bevy game engine essentials
use bevy::{prelude::*, render::view::RenderLayers, math::Vec3Swizzles, time::Stopwatch};
use bevy_pkv::PkvStore;
// Import components, resources, and events
use crate::components::*;

// Plugin for handling reactor sprites and logic
pub struct ReactorPlugin;

impl Plugin for ReactorPlugin {
    fn build(&self, app: &mut App) {
        app
			.add_systems(OnEnter(GameState::Reactor), (
				spawn_reactor_intro,
				spawn_reactor_visuals,
				spawn_reactor_buttons,
				spawn_reactor_levers,
				spawn_reactors,
			))
			.add_systems(Update, (
				recolor_selected_reactor,
				update_cost,
				update_stopwatch,
				handle_levers,
				intake_connections,
				outlet_connections.after(intake_connections),
				check_product_reactor,
			).run_if(in_state(GameState::Reactor))
			.run_if(not(in_state(PauseState::Paused))))
		;
	}
}

fn recolor_selected_reactor (
	mut reactor_query: Query<(&mut Sprite, &mut ReactorInfo, (With<ReactorCondition>, Without<SelectedReactor>))>,
	mut selected_reactor_query: Query<(&mut Sprite, With<SelectedReactor>)>,
) {
	for (mut sprite, r_info, _) in reactor_query.iter_mut() {
		if r_info.product_chamber {
			sprite.color = get_reactor_color(0);
		} else {
			sprite.color = get_reactor_color(1);
		}
	}
	for (mut sprite, _) in selected_reactor_query.iter_mut() {
		sprite.color = get_reactor_color(2);
	}
}

fn spawn_reactor_intro(
	asset_server: Res<AssetServer>,
	level: Res<SelectedLevel>,
	mut next_state: ResMut<NextState<PauseState>>,
	mut ev_w_popup: EventWriter<PopupEvent>,
) {
	next_state.set(PauseState::Paused);
	ev_w_popup.send(PopupEvent{ 
		origin: Vec2::new(0.0, 0.0), 
		image: asset_server.load("sprites/popup/popup.png"),
		alpha: 0.95,
		popup_type: PopupType::LevelIntro(level.0),
	});
}

// Spawn all the visual elements of the reactor such
// as the backgrounds and UI text, as well as the highlight
// and tooltip sprites which are initially hidden
fn spawn_reactor_visuals(
	mut commands: Commands,
	selected_level: Res<SelectedLevel>,
	asset_server: Res<AssetServer>,
	ortho_size: Res<OrthoSize>,
) {
	commands
		.spawn((SpriteBundle {
			texture: asset_server.load("sprites/background/reactor_controls.png"),
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
			texture: asset_server.load("sprites/background/reactor_background.png"),
			transform: Transform::from_xyz(0.0, 0.0, 0.0),
			sprite: Sprite {
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
			texture: asset_server.load("sprites/ui/circle.png"),
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
			texture: asset_server.load("sprites/popup/note_small.png"),
			transform: Transform::from_xyz(0.0, 0.0, -1.0),
			sprite: Sprite {
				custom_size: Some(Vec2::new(TOOLTIP_WIDTH, TOOLTIP_HEIGHT)),
				..Default::default()
			},
			..Default::default()
		},
		Tooltip,
		DespawnOnExitGameState,
		Name::new("Tooltip"),
	)).with_children(|parent| {
		parent
			.spawn((Text2dBundle {
				text_2d_bounds: bevy::text::Text2dBounds{ size: Vec2::new(
					TOOLTIP_WIDTH - TOOLTIP_MARGINS * 2.0,
					TOOLTIP_HEIGHT - TOOLTIP_MARGINS * 2.0,
				)},
				transform: Transform::from_xyz(-TOOLTIP_WIDTH/2.0 + TOOLTIP_MARGINS, TOOLTIP_HEIGHT/2.0 - TOOLTIP_MARGINS, 0.1),
				text_anchor: bevy::sprite::Anchor::TopLeft,
				text: Text::from_section(format!(""), get_tooltip_text_style(&asset_server))
				.with_alignment(TextAlignment::Left),
				..Default::default()
			},
			TooltipText,
			Name::new("Tooltip Text")
		));
	});

	commands
		.spawn((Text2dBundle {
			transform: Transform::from_xyz(REACTOR_VIEWPORT_CENTER.x, REACTOR_VIEWPORT_CENTER.y, 710.0),
			text_anchor: bevy::sprite::Anchor::Center,
			text: Text::from_section(format!("3.00"), get_win_countdown_text_style(&asset_server))
				.with_alignment(TextAlignment::Center),
				visibility: Visibility::Hidden,
			..Default::default()
		},
		DespawnOnExitGameState,
		WinCountdownText,
		Name::new("Win Countdown Text")
	));

	commands
		.spawn((SpriteBundle {
			transform: Transform::from_xyz(REACTOR_VIEWPORT_CENTER.x + REACTOR_VIEWPORT_WIDTH/2.0 - STOPWATCH_BOX_WIDTH/2.0, STOPWATCH_BOX_Y, 730.0),
			sprite: Sprite {
				color: Color::hex("F2F2F2").unwrap(),
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
				transform: Transform::from_xyz(-STOPWATCH_BOX_WIDTH / 2.0 + STOPWATCH_BOX_MARGINS, 0.0, 10.0),
				text_anchor: bevy::sprite::Anchor::CenterLeft,
				text: Text::from_section(format!("0.00 s"), get_stopwatch_text_style(&asset_server))
				.with_alignment(TextAlignment::Right),
				..Default::default()
			},
			StopwatchText(Stopwatch::default()),
			Name::new("Stopwatch Text")
		));
	});

	commands
		.spawn((SpriteBundle {
			transform: Transform::from_xyz(REACTOR_VIEWPORT_CENTER.x - REACTOR_VIEWPORT_WIDTH/2.0 + GOAL_BOX_WIDTH + COST_BOX_WIDTH/2.0 + REACTION_UI_SPACING, COST_BOX_Y, 730.0),
			sprite: Sprite {
				color: Color::hex("F2F2F2").unwrap(),
				custom_size: Some(Vec2::new(COST_BOX_WIDTH, COST_BOX_HEIGHT)),
				..Default::default()
			},
			..Default::default()
		},
		DespawnOnExitGameState,
		Name::new("Cost Box Sprite")
	)).with_children(|parent| {
		parent
			.spawn((Text2dBundle {
				text_2d_bounds: bevy::text::Text2dBounds{ size: Vec2::new(
					COST_BOX_WIDTH - COST_BOX_MARGINS * 2.0,
					COST_BOX_HEIGHT - COST_BOX_MARGINS,
				)},
				transform: Transform::from_xyz(-COST_BOX_WIDTH / 2.0 + COST_BOX_MARGINS, 0.0, 10.0),
				text_anchor: bevy::sprite::Anchor::CenterLeft,
				text: Text::from_section(format!("0 c"), get_cost_text_style(&asset_server))
				.with_alignment(TextAlignment::Right),
				..Default::default()
			},
			CostText,
			Name::new("Cost Text")
		));
	});

	commands
		.spawn((SpriteBundle {
			transform: Transform::from_xyz(REACTOR_VIEWPORT_CENTER.x - REACTOR_VIEWPORT_WIDTH/2.0 + GOAL_BOX_WIDTH/2.0, GOAL_BOX_Y, 730.0),
			sprite: Sprite {
				color: Color::hex("F2F2F2").unwrap(),
				custom_size: Some(Vec2::new(GOAL_BOX_WIDTH, GOAL_BOX_HEIGHT)),
				..Default::default()
			},
			..Default::default()
		},
		DespawnOnExitGameState,
		Name::new("Goal Box Sprite")
	)).with_children(|parent| {
		parent
			.spawn((Text2dBundle {
				text_2d_bounds: bevy::text::Text2dBounds{ size: Vec2::new(
					GOAL_BOX_WIDTH - GOAL_BOX_MARGINS * 2.0,
					GOAL_BOX_HEIGHT - GOAL_BOX_MARGINS * 2.0,
				)},
				transform: Transform::from_xyz(0.0, 0.0, 10.0),
				text_anchor: bevy::sprite::Anchor::Center,
				text: Text::from_section(get_level_goal_text(selected_level.0), get_goal_text_style(&asset_server))
				.with_alignment(TextAlignment::Center),
				..Default::default()
			},
			Name::new("Goal Text")
		));
	});
}

// Update the stopwatch to track time spent on a level
fn update_stopwatch(
	mut stopwatch_text_query: Query<(&mut Text, &mut StopwatchText)>,
	time: Res<Time>,
) {
	for (mut text, mut stopwatch) in stopwatch_text_query.iter_mut() {
		stopwatch.0.tick(time.delta());
		text.sections[0].value =
			if stopwatch.0.elapsed_secs() < 60.0 {format!("{:.2} s", stopwatch.0.elapsed_secs())}
			else if stopwatch.0.elapsed_secs() < 6000.0 {format!("{:.0} m {:.0} s", (stopwatch.0.elapsed_secs() / 60.0).floor(), stopwatch.0.elapsed_secs() % 60.0)}
			else if stopwatch.0.elapsed_secs() < 600000.0 {format!("{:.0} m", (stopwatch.0.elapsed_secs() / 60.0).floor())}
			else {format!("You win!")};
	}
}

// Update the cost to track cost spent on a level
fn update_cost(
	current_cost: Res<CurrentCost>,
	mut cost_text_query: Query<(&mut Text, With<CostText>)>,
) {
	for (mut text, _) in cost_text_query.iter_mut() {
		text.sections[0].value = 
		if current_cost.0 < 1000 {format!("{} c", current_cost.0)}
		else if current_cost.0 < 10000 {format!("{:.3} kc", current_cost.0 as f32/1000.0)}
		else if current_cost.0 < 100000 {format!("{:.2} kc", current_cost.0 as f32/1000.0)}
		else if current_cost.0 < 1000000 {format!("{:.1} kc", current_cost.0 as f32/1000.0)}
		else if current_cost.0 < 10000000 {format!("{:.3} Mc", current_cost.0 as f32/1000000.0)}
		else if current_cost.0 < 100000000 {format!("{:.2} Mc", current_cost.0 as f32/1000000.0)}
		else if current_cost.0 < 1000000000 {format!("{:.1} Mc", current_cost.0 as f32/1000000.0)}
		else {format!("Expensive")};
	}
}

// Spawn the reactor buttons for selecting molecules
// or for exiting the level
fn spawn_reactor_buttons(
	mut commands: Commands,
	mut texture_atlases: ResMut<Assets<TextureAtlas>>,
	level: Res<SelectedLevel>,
	asset_server: Res<AssetServer>,
	selected_palette: Res<SelectedPalette>,
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
				enabled: get_available_molecules(level.0)[i + j*3],
				idle_color: Color::hex("EDD6AD").unwrap(),
				hovered_color: Color::hex("CDB68D").unwrap(),
				disabled_color: Color::hex("9D865D").unwrap(),
			};

			let loc = button.location;
			let dim = button.dimensions;
			let en = button.enabled;

			let texture_handle = asset_server.load(get_molecule_path(i + 3*j));
			let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 4, 2, None, None);
			let texture_atlas_handle = texture_atlases.add(texture_atlas);

			commands.spawn((SpriteBundle {
				transform: Transform::from_translation(loc),
				sprite: Sprite {
					color: if en{Color::hex("EDD6AD").unwrap()} else {Color::hex("9D865D").unwrap()},
					custom_size: Some(Vec2::new(dim.width, dim.height)), 
					..Default::default()
				},
				..Default::default()
				},
				ButtonEffect::ReactorButton(ReactorButton::SelectMolecule(i + j*3)),
				button,
				DespawnOnExitGameState,
				Name::new(format!("Molecule Select Button {}", i + j*3))
			));
			commands
				.spawn((SpriteSheetBundle {
					texture_atlas: texture_atlas_handle.clone(),
					transform: Transform::from_xyz(loc.x, loc.y, loc.z + 1.0),
					sprite: TextureAtlasSprite {
						color: get_molecule_color(i + j*3, selected_palette.0),
						index: 1,
						custom_size: Some(Vec2::new(dim.width, dim.height)), 
						..Default::default()
					},
					..Default::default()
				},
				MoleculeButton(i+j*3),
				AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
				AnimationIndices{ 
					first: 0, 
					total: 8,
				},
				DespawnOnExitGameState,
				Name::new(format!("Molecule Select Button Sprite {}", i + j*3))
			));
			if !en {
				let texture_handle = asset_server.load("moles/lock.png".to_string());
				let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 4, 2, None, None);
				let texture_atlas_handle = texture_atlases.add(texture_atlas);
				commands
					.spawn((SpriteSheetBundle {
						texture_atlas: texture_atlas_handle.clone(),
						transform: Transform::from_xyz(loc.x, loc.y, loc.z + 2.0),
						sprite: TextureAtlasSprite {
							color: Color::WHITE,
							index: 1,
							custom_size: Some(Vec2::new(dim.width, dim.height)), 
							..Default::default()
						},
						..Default::default()
					},
					MoleculeButton(i+j*3),
					AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
					AnimationIndices{ 
						first: 0, 
						total: 8,
					},
					DespawnOnExitGameState,
					Name::new(format!("Molecule Select Lock Button Sprite {}", i + j*3))
				));	
			}
			
		}
	}

	let button = StandardButton {
		location: Vec3::new(-400.0, -375.0, 710.0),
		dimensions: Dimensions {
			width: 400.0,
			height: 40.0,
		},
		enabled: true,
		idle_color: Color::hex("EDD6AD").unwrap(),
		hovered_color: Color::hex("CDB68D").unwrap(),
		disabled_color: Color::hex("9D865D").unwrap(),
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
	)).with_children(|parent| {
		parent
			.spawn((Text2dBundle {
				transform: Transform::from_xyz(0.0, -2.5, 10.0,),
				text: Text::from_section(format!("Exit"), get_button_text_style(&asset_server))
					.with_alignment(TextAlignment::Center),
				..Default::default()
			},
			Name::new("Exit Reactor Button")
		));
	});

	let button = StandardButton {
		location: Vec3::new(0.0, -375.0, 710.0),
		dimensions: Dimensions {
			width: 400.0,
			height: 40.0,
		},
		enabled: true,
		idle_color: Color::hex("EDD6AD").unwrap(),
		hovered_color: Color::hex("CDB68D").unwrap(),
		disabled_color: Color::hex("9D865D").unwrap(),
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
		ButtonEffect::ReactorButton(ReactorButton::PauseLevel),
		button,
		DespawnOnExitGameState,
	)).with_children(|parent| {
		parent
			.spawn((Text2dBundle {
				transform: Transform::from_xyz(0.0, -2.5, 10.0,),
				text: Text::from_section(format!("Pause"), get_button_text_style(&asset_server))
					.with_alignment(TextAlignment::Center),
				..Default::default()
			},
			Name::new("Pause Button")
		));
	});

	let button = StandardButton {
		location: Vec3::new(400.0, -375.0, 710.0),
		dimensions: Dimensions {
			width: 400.0,
			height: 40.0,
		},
		enabled: true,
		idle_color: Color::hex("EDD6AD").unwrap(),
		hovered_color: Color::hex("CDB68D").unwrap(),
		disabled_color: Color::hex("9D865D").unwrap(),
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
		ButtonEffect::ReactorButton(ReactorButton::RestartLevel),
		button,
		DespawnOnExitGameState,
	)).with_children(|parent| {
		parent
			.spawn((Text2dBundle {
				transform: Transform::from_xyz(0.0, -2.5, 10.0,),
				text: Text::from_section(format!("Replay"), get_button_text_style(&asset_server))
					.with_alignment(TextAlignment::Center),
				..Default::default()
			},
			Name::new("Replay Level Button")
		));
	});
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
			max_height: -30.0,
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
	lever_query: Query<(Entity, &Transform, (With<LeverInfo>, Without<SelectedLever>))>,
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
				if (c_transform.translation.xy() - m_transform.translation.xy()).length() < CONNECTION_WIDTH 
				&& connection.intake 
				&& connection.filter[m_info.index] {
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
		let z = 910.0;
		match reactor.reactor_type {
			ReactorType::Rectangle{origin, dimensions} => {
				let texture: Handle<Image> = asset_server.load("sprites/ui/rectangle.png");
				r.insert(texture);
				r.insert(Transform::from_xyz(origin.x, origin.y, 10.0));
				r.insert(Sprite{
					color: if reactor.product_chamber {Color::BISQUE} else {Color::GREEN}, 
					custom_size: Some(Vec2::new(dimensions.width, dimensions.height)), 
					..Default::default()
				});
				if reactor.input_chamber {
					commands
						.spawn((SpriteBundle {
							transform: Transform::from_xyz(origin.x, origin.y + dimensions.height / 2.0, z),
							texture: asset_server.load("sprites/ui/launcher.png"),
							sprite: Sprite {
								//color: Color::DARK_GRAY,
								custom_size: Some(Vec2::new(LAUNCH_TUBE_WIDTH, LAUNCH_TUBE_HEIGHT)),
								..Default::default()
							},
							..Default::default()
						},
						LaunchTube{
							id: i,
							current_rotation: 0.0,
							limits: get_launch_tube_limits(level.0, reactor.reactor_id),
						},
						RenderLayers::layer(1),
						DespawnOnExitGameState,
						Name::new("Launch Tube"),
					));
				}
				for (index, location, velocity) in get_reactor_initialization(level.0, reactor.reactor_id) {
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
						Velocity(velocity),
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
					let translation = Vec3::new(
						origin.x + direction.x * dimensions.width / 2.0, 
						origin.y + direction.y * dimensions.height/2.0, 
						z + connection.connection_id as f32
					);
					commands.spawn((SpriteBundle {
						texture: asset_server.load("sprites/ui/connection.png"),
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
				let texture: Handle<Image> = asset_server.load("sprites/ui/circle.png");
				r.insert(texture);
				r.insert(Transform::from_xyz(origin.x, origin.y, 10.0));
				r.insert(Sprite{
					color: if reactor.product_chamber {Color::BISQUE} else {Color::GREEN}, 
					custom_size: Some(Vec2::new(radius*2.0, radius*2.0)), 
					..Default::default()
				});
				if reactor.input_chamber {
					commands
						.spawn((SpriteBundle {
							transform: Transform::from_xyz(origin.x, origin.y + radius, z),
							texture: asset_server.load("sprites/ui/launcher.png"),
							sprite: Sprite {
								//color: Color::DARK_GRAY,
								custom_size: Some(Vec2::new(LAUNCH_TUBE_WIDTH, LAUNCH_TUBE_HEIGHT)),
								..Default::default()
							},
							..Default::default()
						},
						LaunchTube{
							id: i,
							current_rotation: 0.0,
							limits: get_launch_tube_limits(level.0, reactor.reactor_id),
						},
						RenderLayers::layer(1),
						DespawnOnExitGameState,
						Name::new("Launch Tube"),
					));
				}
				for (index, location, velocity) in get_reactor_initialization(level.0, reactor.reactor_id) {
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
						Velocity(velocity),
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
					let translation = Vec3::new(
						origin.x + direction.x * radius, 
						origin.y + direction.y * radius, 
						z + connection.connection_id as f32
					);
					commands.spawn((SpriteBundle {
						texture: asset_server.load("sprites/ui/connection.png"),
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
	mut pkv: ResMut<PkvStore>,
	mut ev_w_popup: EventWriter<PopupEvent>,
	mut next_state: ResMut<NextState<PauseState>>,
	mut win_countdown: ResMut<WinCountdown>,
	mut win_countdown_text_query: Query<(&mut Text, &mut Visibility, With<WinCountdownText>)>,
	asset_server: Res<AssetServer>,
	current_cost: Res<CurrentCost>,
	reactor_query: Query<(&ReactorInfo, With<ReactorCondition>)>,
	molecule_query: Query<(&MoleculeInfo, &ReactorInfo, With<Molecule>)>,
	stopwatch_query: Query<&StopwatchText>,
	selected_level: Res<SelectedLevel>,
	time: Res<Time>,
) {
	for (r_info, _) in reactor_query.iter() {
		if r_info.product_chamber {
			let win_condition = get_level_goal(selected_level.0);
			let mut desired_count = 0;
			let mut condition_passed = false;
			match win_condition {
				WinCondition::GreaterThan(desired_quantity, desired_molecule) => {
					for (m_info, m_r_info, _) in molecule_query.iter() {
						if r_info.reactor_id == m_r_info.reactor_id && m_info.index == desired_molecule {
							desired_count += 1;
							if desired_count >= desired_quantity {
								condition_passed = true;
								break;
							}
						}
					}
				},
				WinCondition::LessThan(desired_quantity, desired_molecule) => {
					condition_passed = true;
					for (m_info, m_r_info, _) in molecule_query.iter() {
						if r_info.reactor_id == m_r_info.reactor_id && m_info.index == desired_molecule {
							desired_count += 1;
							if desired_count >= desired_quantity {
								condition_passed = false;
								break;
							}
						}
					}
				},
			}

			if condition_passed {
				win_countdown.0.tick(time.delta());
				for (mut text, mut visibility, _) in win_countdown_text_query.iter_mut() {
					*visibility = Visibility::Visible;
					let time_left = 3.0 - 3.0 * win_countdown.0.percent();
					text.sections[0].value = format!("Reaction\nComplete in:\n{:.2}", time_left);
				}
				if win_countdown.0.just_finished() {
					for (_, mut visibility, _) in win_countdown_text_query.iter_mut() {
						*visibility = Visibility::Hidden;
					}
					let mut prev_best_cost = 999999;
					let mut prev_best_time = 999999.0;
					let mut current_time = 999999.0;
					if let Ok(mut save_data) = pkv.get::<SaveData>("save_data") {
						prev_best_cost = save_data.best_costs[selected_level.0];
						if current_cost.0 < prev_best_cost {
							save_data.best_costs[selected_level.0] = current_cost.0;
						}
						for stopwatch in stopwatch_query.iter() {
							prev_best_time = save_data.best_times[selected_level.0];
							current_time = stopwatch.0.elapsed_secs();
							if current_time < prev_best_time {
								save_data.best_times[selected_level.0] = current_time;
							}
						}
						save_data.levels_unlocked[selected_level.0 + 1] = true;
						pkv.set("save_data", &save_data)
							.expect("Unable to save data");
					}
					next_state.set(PauseState::Paused);
					ev_w_popup.send(PopupEvent{ 
						origin: Vec2::new(0.0, 0.0), 
						image: asset_server.load("sprites/popup/logbook_base.png"),
						alpha: 1.0,
						popup_type: PopupType::WinScreen(prev_best_time, current_time, prev_best_cost, current_cost.0),
					});
				}
			} else {
				win_countdown.0.reset();
				for (_, mut visibility, _) in win_countdown_text_query.iter_mut() {
					*visibility = Visibility::Hidden;
				}
			}
		}
	}
}