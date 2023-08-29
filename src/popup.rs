// Import Bevy game engine essentials
use bevy::prelude::*;
use bevy_pkv::PkvStore;
// Import components, resources, and events
use crate::components::*;

// Plugin for generating popup visuals and 
// menus which do not change the GameState
pub struct PopupPlugin;

impl Plugin for PopupPlugin {
    fn build(&self, app: &mut App) {
        app
			.add_systems(Update, (
				spawn_popup,
				expand_popup,
				spawn_popup_buttons.run_if(in_state(PauseState::Paused)),
			))
		;
	}
}

// Waits for PopupEvents and then spawns a sprite with
// the origin and image from the event
fn spawn_popup(
	mut commands: Commands,
	mut ev_r_popup: EventReader<PopupEvent>,
) {
	for ev in ev_r_popup.iter() {
		commands
			.spawn((SpriteBundle {
				texture: ev.image.clone(),
				transform: Transform::from_xyz(ev.origin.x, ev.origin.y, 800.0),
				sprite: Sprite {
					color: Color::rgba(1.0, 1.0, 1.0, ev.alpha),
					custom_size: Some(Vec2::new(1.0, 1.0)), 
					..Default::default()
				},
				..Default::default()
			},
			PopupInfo{
				origin: ev.origin,
				full_size: false,
				popup_type: ev.popup_type,
			},
			PopupTimer(Timer::from_seconds(POPUP_EXPAND_TIME, TimerMode::Once)),
			DespawnOnExitPauseState,
			Name::new("Popup"),
		));
	}
}

// If there are any popups on screen then expand them to target size
// and move to the center of the screen
fn expand_popup(
	mut popup_query: Query<(&mut Transform, &mut PopupInfo, &mut PopupTimer)>,
	mut ev_w_popup_complete: EventWriter<PopupCompleteEvent>,
	time: Res<Time>,
) {
	for (mut transform, mut info, mut timer) in popup_query.iter_mut() {
		// Stop once popup is at full size
		if !info.full_size {
			timer.0.tick(time.delta());
			transform.translation.x = info.origin.x - info.origin.x * timer.0.percent();
			transform.translation.y = info.origin.y - info.origin.y * timer.0.percent();
			transform.scale.x = POPUP_WIDTH * timer.0.percent();
			transform.scale.y = POPUP_HEIGHT * timer.0.percent();
			if timer.0.just_finished() {
				info.full_size = true;
				ev_w_popup_complete.send(PopupCompleteEvent);
			}
		}
	}
}

// Spawns all the buttons for popups once they are
// fully scaled
fn spawn_popup_buttons(
	popup_query: Query<(&PopupInfo)>,
	asset_server: Res<AssetServer>,
	selected_palette: Res<SelectedPalette>,
	selected_level: Res<SelectedLevel>,
	pkv: Res<PkvStore>,
	ortho_size: Res<OrthoSize>,
	mut commands: Commands,
	mut ev_r_popup_complete: EventReader<PopupCompleteEvent>,
) {
	for _ in ev_r_popup_complete.iter() {
		for info in popup_query.iter() {
			match info.popup_type {
				PopupType::Settings => {
					commands.spawn((Text2dBundle{
						transform: Transform::from_xyz(0.0, 300.0, 810.0),
						text: Text::from_section(format!("Settings"), get_title_text_style(&asset_server)),
						..Default::default()
						},
						DespawnOnExitPauseState,
						Name::new("Settings Text")
					));
					commands.spawn((Text2dBundle{
						transform: Transform::from_xyz(-25.0, 0.0, 810.0),
						text: Text::from_section(format!("BGM Volume:\n\nSFX Volume:\n\nToggle Palette:\n\nParticle Trails:"), get_settings_text_style(&asset_server))
							.with_alignment(TextAlignment::Right),
						text_anchor: bevy::sprite::Anchor::CenterRight,
						..Default::default()
						},
						DespawnOnExitPauseState,
						Name::new("Settings Text")
					));

					let mut buttons  = Vec::new();
					buttons.push((StandardButton {
						location: Vec3::new(0.0, -310.0, 810.0),
						dimensions: Dimensions {
							width: 400.0,
							height: 40.0,
						},
						enabled: true,
						idle_color: Color::hex("EDD6AD").unwrap(),
						hovered_color: Color::hex("CDB68D").unwrap(),
						disabled_color: Color::hex("9D865D").unwrap(),
					}, ButtonEffect::PopupButton(PopupButton::ExitPopup)));
					commands.spawn((Text2dBundle {
							transform: Transform::from_xyz(0.0, -310.0, 820.0),
							text: Text::from_section(format!("Back"), get_button_text_style(&asset_server))
								.with_alignment(TextAlignment::Center),
							..Default::default()
						},
						DespawnOnExitPauseState,
						Name::new("Settings Quit Button")
					));
					for i in 0..=10 {
						buttons.push((StandardButton {
							location: Vec3::new(25.0 + 30.0 * i as f32, 95.0, 810.0),
							dimensions: Dimensions {
								width: 25.0,
								height: 50.0,
							},
							enabled: true,
							idle_color: Color::hex("EDD6AD").unwrap(),
							hovered_color: Color::hex("CDB68D").unwrap(),
							disabled_color: Color::hex("9D865D").unwrap(),
						}, ButtonEffect::PopupButton(PopupButton::BgmVolume(i))));
						buttons.push((StandardButton {
							location: Vec3::new(25.0 + 30.0 * i as f32, 32.5, 810.0),
							dimensions: Dimensions {
								width: 25.0,
								height: 50.0,
							},
							enabled: true,
							idle_color: Color::hex("EDD6AD").unwrap(),
							hovered_color: Color::hex("CDB68D").unwrap(),
							disabled_color: Color::hex("9D865D").unwrap(),
						}, ButtonEffect::PopupButton(PopupButton::SfxVolume(i))));
					}
					buttons.push((StandardButton {
						location: Vec3::new(37.5, -27.5, 810.0),
						dimensions: Dimensions {
							width: 50.0,
							height: 50.0,
						},
						enabled: true,
						idle_color: Color::hex("EDD6AD").unwrap(),
						hovered_color: Color::hex("CDB68D").unwrap(),
						disabled_color: Color::hex("9D865D").unwrap(),
					}, ButtonEffect::PopupButton(PopupButton::PaletteToggle)));
					commands.spawn((Text2dBundle{
						transform: Transform::from_xyz(62.5, -90.0, 820.0),
						text: Text::from_section(format!("On"), get_settings_text_style(&asset_server))
							.with_alignment(TextAlignment::Center),
						text_anchor: bevy::sprite::Anchor::Center,
						..Default::default()
						},
						DespawnOnExitPauseState,
						Name::new("Particle Trail Enable Text")
					));
					buttons.push((StandardButton {
						location: Vec3::new(62.5, -87.5, 810.0),
						dimensions: Dimensions {
							width: 100.0,
							height: 50.0,
						},
						enabled: true,
						idle_color: Color::hex("EDD6AD").unwrap(),
						hovered_color: Color::hex("CDB68D").unwrap(),
						disabled_color: Color::hex("9D865D").unwrap(),
					}, ButtonEffect::PopupButton(PopupButton::ParticleTrails(true))));
					commands.spawn((Text2dBundle{
						transform: Transform::from_xyz(175.0, -90.0, 820.0),
						text: Text::from_section(format!("Off"), get_settings_text_style(&asset_server))
							.with_alignment(TextAlignment::Center),
						text_anchor: bevy::sprite::Anchor::Center,
						..Default::default()
						},
						DespawnOnExitPauseState,
						Name::new("Particle Trail Enable Text")
					));
					buttons.push((StandardButton {
						location: Vec3::new(175.0, -87.5, 810.0),
						dimensions: Dimensions {
							width: 100.0,
							height: 50.0,
						},
						enabled: true,
						idle_color: Color::hex("EDD6AD").unwrap(),
						hovered_color: Color::hex("CDB68D").unwrap(),
						disabled_color: Color::hex("9D865D").unwrap(),
					}, ButtonEffect::PopupButton(PopupButton::ParticleTrails(false))));
					for (button, effect) in buttons {
						commands
							.spawn((SpriteBundle {
								transform: Transform::from_translation(button.location),
								sprite: Sprite {
									color: Color::hex("EDD6AD").unwrap(),
									custom_size: Some(Vec2::new(button.dimensions.width, button.dimensions.height)), 
									..Default::default()
								},
								..Default::default()
							},
							effect,
							button,
							DespawnOnExitPauseState,
							Name::new("Settings Button")
						));
					}
					for i in 0..15 {
						commands
							.spawn((SpriteBundle{
								transform: Transform::from_xyz(85.0 + 17.5 * i as f32, -27.5, 810.0),
								sprite: Sprite {
									color: get_molecule_color(i, selected_palette.0),
									custom_size: Some(Vec2::new(12.5, 50.0)), 
									..Default::default()
								},
								..Default::default()
							},
							Palette(i),
							DespawnOnExitPauseState,
							Name::new("Palette")
						));
					}
				},
				PopupType::Logbook => {
					commands
						.spawn((SpriteBundle{
							transform: Transform::from_xyz(0.0, 0.0, 800.0),
							sprite: Sprite{
								color: Color::rgba(0.5, 0.5, 0.5, 0.4),
								custom_size: Some(Vec2::new(ortho_size.width, ortho_size.height)),
								..Default::default()
							},
							..Default::default()
						},
						DespawnOnExitPauseState,
						Name::new("Logbook Backdrop"),
					));
					commands
						.spawn((SpriteBundle{
							transform: Transform::from_xyz(0.0, 0.0, 805.0),
							texture: asset_server.load("sprites/popup/logbook_page.png"),
							sprite: Sprite{
								custom_size: Some(Vec2::new(POPUP_WIDTH, POPUP_HEIGHT)),
								..Default::default()
							},
							..Default::default()
						},
						DespawnOnExitPauseState,
						Name::new("Logbook Page"),
					));
					commands.spawn((Text2dBundle{
						transform: Transform::from_xyz(-POPUP_WIDTH/2.0 + LOGBOOK_MARGINS, POPUP_HEIGHT/2.0 - LOGBOOK_MARGINS, 810.0),
						text_2d_bounds: bevy::text::Text2dBounds{ size: Vec2::new(
							POPUP_WIDTH/2.0 - LOGBOOK_MARGINS * 2.0,
							POPUP_HEIGHT - LOGBOOK_MARGINS * 2.0,
						)},
						text: Text::from_section(get_logbook_text(0, 0), get_logbook_text_style(&asset_server))
							.with_alignment(TextAlignment::Left),
						text_anchor: bevy::sprite::Anchor::TopLeft,
						..Default::default()
						},
						LogbookText(0),
						DespawnOnExitPauseState,
						Name::new("Logbook Text")
					));
					commands.spawn((Text2dBundle{
						transform: Transform::from_xyz(LOGBOOK_MARGINS, POPUP_HEIGHT/2.0 - LOGBOOK_MARGINS, 810.0),
						text_2d_bounds: bevy::text::Text2dBounds{ size: Vec2::new(
							POPUP_WIDTH/2.0 - LOGBOOK_MARGINS * 2.0,
							POPUP_HEIGHT - LOGBOOK_MARGINS * 2.0,
						)},
						text: Text::from_section(get_logbook_text(0, 1), get_logbook_text_style(&asset_server))
							.with_alignment(TextAlignment::Left),
						text_anchor: bevy::sprite::Anchor::TopLeft,
						..Default::default()
						},
						LogbookText(1),
						DespawnOnExitPauseState,
						Name::new("Logbook Text")
					));
					let mut tabs = Vec::new();
					for i in 0..20 {
						let color = get_molecule_color(i, selected_palette.0);
						tabs.push((StandardButton {
							location: if i == 0 {Vec3::new(-600.0 + 60.0 * i as f32 + (i as f32 * 7.0).sin() * 8.0, 390.0 + (i as f32 * 9.0).cos() * 5.0, 810.0)}
								else if i < 10 {Vec3::new(-600.0 + 60.0 * i as f32 + (i as f32 * 7.0).sin() * 8.0, 390.0 + (i as f32 * 9.0).cos() * 5.0, 801.0)}
								else {Vec3::new(60.0 + 60.0 * (i - 10)as f32 + (i as f32 * 7.0).cos() * 8.0, 390.0 + (i as f32 * 9.0).sin() * 5.0, 801.0)},
							dimensions: Dimensions {
								width: 40.0,
								height: 100.0,
							},
							enabled: true,
							idle_color: color,
							hovered_color: Color::rgb((color.r() - 0.3).clamp(0.0, 1.0), (color.g() - 0.3).clamp(0.0, 1.0), (color.b() - 0.3).clamp(0.0, 1.0)),
							disabled_color: Color::hex("9D865D").unwrap(),
						}, ButtonEffect::PopupButton(PopupButton::LogbookPage(i)),
						0.0_f32,
						if i < 10 {true} else {false}));
					};
					/*for i in 0..7 {
						tabs.push((StandardButton {
							location: Vec3::new(700.0, 300.0 - 100.0 * i as f32, 810.0),
							dimensions: Dimensions {
								width: 100.0,
								height: 40.0,
							},
							enabled: true,
						}, ButtonEffect::PopupButton(PopupButton::LogbookPage(i+15)),
						get_molecule_color(i, selected_palette.0),
						90.0_f32));
					}*/
					for (button, effect, rotation, left) in tabs {
						commands
							.spawn((SpriteBundle {
								transform: Transform::from_translation(button.location)
									.with_rotation(Quat::from_rotation_z(rotation.to_radians()))
									.with_scale(if left {Vec3::splat(1.0)} else {Vec3::new(-1.0, 1.0, 1.0)}),
								texture: asset_server.load("sprites/ui/bookmark.png"),
								sprite: Sprite {
									color: button.hovered_color,
									custom_size: if rotation == 0.0 {Some(Vec2::new(button.dimensions.width, button.dimensions.height))}
										else {Some(Vec2::new(button.dimensions.height, button.dimensions.width))}, 
									..Default::default()
								},
								..Default::default()
							},
							effect,
							button,
							DespawnOnExitPauseState,
							Name::new("Logbook Button")
						));
					};
					let button = StandardButton {
						location: Vec3::new(-500.0, -350.0, 810.0),
						dimensions: Dimensions {
							width: 300.0,
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
								color: Color::hex("EDD6AD").unwrap(),
								custom_size: Some(Vec2::new(button.dimensions.width, button.dimensions.height)), 
								..Default::default()
							},
							..Default::default()
						},
						ButtonEffect::PopupButton(PopupButton::ExitPopup),
						button,
						DespawnOnExitPauseState,
						Name::new("Logbook Button")
					));
					commands.spawn((Text2dBundle {
						transform: Transform::from_xyz(-500.0, -350.0, 820.0),
						text: Text::from_section(format!("Back"), get_button_text_style(&asset_server))
							.with_alignment(TextAlignment::Center),
						..Default::default()
					},
					DespawnOnExitPauseState,
					Name::new("Logbook Button")
					));
				},
				PopupType::LevelSelect => {
					if let Ok(save_data) = pkv.get::<SaveData>("save_data") {
						// Spawn level select buttons
						for j in 0..=4 {
							for i in 0..7 {
								if j == 4 && i > 2 {continue};
								let button = StandardButton {
									location: Vec3::new(-270.0 + 90.0 * i as f32, 115.0 - 90.0 * j as f32, 810.0),
									dimensions: Dimensions {
										width: 85.0,
										height: 85.0,
									},
									enabled: save_data.levels_unlocked[i+7*j],
									idle_color: Color::hex("EDD6AD").unwrap(),
									hovered_color: Color::hex("CDB68D").unwrap(),
									disabled_color: Color::hex("9D865D").unwrap(),
								};
								commands.spawn((SpriteBundle {
										transform: Transform::from_translation(button.location),
										sprite: Sprite {
											color: Color::hex("EDD6AD").unwrap(),
											custom_size: Some(Vec2::new(button.dimensions.width, button.dimensions.height)), 
											..Default::default()
										},
										..Default::default()
									},
									ButtonEffect::PopupButton(PopupButton::LevelSelect(i + 7*j)),
									button,
									DespawnOnExitPauseState,
									Name::new(format!("Level Select Button {}", i + 7*j))
								)).with_children(|parent| {
									parent
										.spawn((Text2dBundle {
											transform: Transform::from_xyz(0.0, -5.0, 10.0,),
											text: Text::from_section(format!("{}", i+7*j+1), get_button_text_style(&asset_server))
												.with_alignment(TextAlignment::Center),
											..Default::default()
										},
										Name::new("Level Select Button Text")
									));
								});
							}
						}
					}
					let button = StandardButton {
						location: Vec3::new(-525.0, -337.5, 810.0),
						dimensions: Dimensions {
							width: 300.0,
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
								color: Color::hex("EDD6AD").unwrap(),
								custom_size: Some(Vec2::new(button.dimensions.width, button.dimensions.height)), 
								..Default::default()
							},
							..Default::default()
						},
						ButtonEffect::PopupButton(PopupButton::ExitPopup),
						button,
						DespawnOnExitPauseState,
						Name::new("Exit Level Select Button")
					));
					commands.spawn((Text2dBundle {
						transform: Transform::from_xyz(-525.0, -337.5, 820.0),
						text: Text::from_section(format!("Back"), get_button_text_style(&asset_server))
							.with_alignment(TextAlignment::Center),
						..Default::default()
					},
					DespawnOnExitPauseState,
					Name::new("Exit Level Select Button")
					));
				},
				PopupType::LevelIntro(level) => {
					commands.spawn((Text2dBundle{
						transform: Transform::from_xyz(0.0, 300.0, 810.0),
						text: Text::from_section(format!("Hints for day {}", level + 1), get_title_text_style(&asset_server))
							.with_alignment(TextAlignment::Center),
						text_anchor: bevy::sprite::Anchor::Center,
						..Default::default()
						},
						DespawnOnExitPauseState,
						Name::new("Level Intro Text")
					));
					commands.spawn((Text2dBundle{
						transform: Transform::from_xyz(0.0, 0.0, 810.0),
						text_2d_bounds: bevy::text::Text2dBounds{ size: Vec2::new(
							POPUP_WIDTH - TEXT_BOX_MARGINS * 2.0,
							POPUP_HEIGHT - TEXT_BOX_MARGINS,
						)},
						text: Text::from_section(get_intro_text(level), get_intro_text_style(&asset_server))
							.with_alignment(TextAlignment::Left),
						text_anchor: bevy::sprite::Anchor::Center,
						..Default::default()
						},
						DespawnOnExitPauseState,
						Name::new("Level Intro Text")
					));
					let button = StandardButton {
						location: Vec3::new(400.0, -300.0, 810.0),
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
								color: Color::hex("EDD6AD").unwrap(),
								custom_size: Some(Vec2::new(button.dimensions.width, button.dimensions.height)), 
								..Default::default()
							},
							..Default::default()
						},
						ButtonEffect::PopupButton(PopupButton::ExitPopup),
						button,
						DespawnOnExitPauseState,
						Name::new("Exit Intro Button")
					));
					commands
						.spawn((Text2dBundle {
							transform: Transform::from_xyz(400.0, -300.0, 820.0),
							text: Text::from_section(format!("Continue"), get_button_text_style(&asset_server))
								.with_alignment(TextAlignment::Center),
							..Default::default()
						},
						DespawnOnExitPauseState,
						Name::new("Exit Intro Button")
					));
					let button = StandardButton {
						location: Vec3::new(-400.0, -300.0, 810.0),
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
								color: Color::hex("EDD6AD").unwrap(),
								custom_size: Some(Vec2::new(button.dimensions.width, button.dimensions.height)), 
								..Default::default()
							},
							..Default::default()
						},
						ButtonEffect::PopupButton(PopupButton::ReturnToLab),
						button,
						DespawnOnExitPauseState,
						Name::new("Return to Lab Button")
					));
					commands.spawn((Text2dBundle {
						transform: Transform::from_xyz(-400.0, -300.0, 820.0),
						text: Text::from_section(format!("Exit"), get_button_text_style(&asset_server))
							.with_alignment(TextAlignment::Center),
						..Default::default()
					},
					DespawnOnExitPauseState,
					Name::new("Return to Lab Button")
					));
				}
				PopupType::WinScreen(prev_best_time, current_time, prev_best_cost, current_cost) => {
					commands.spawn((Text2dBundle{
						transform: Transform::from_xyz(0.0, 300.0, 810.0),
						text: Text::from_section(format!("Reaction Successful!"), get_win_title_text_style(&asset_server))
							.with_alignment(TextAlignment::Center),
						text_anchor: bevy::sprite::Anchor::Center,
						..Default::default()
						},
						DespawnOnExitPauseState,
						Name::new("Win Text")
					));
					let prev_best_time_text = if prev_best_time < 60.0 {format!("{:.2} s", prev_best_time)}
						else if prev_best_time < 6000.0 {format!("{:.0} m {:.0} s", (prev_best_time / 60.0).floor(), prev_best_time % 60.0)}
						else if prev_best_time < 999999.0 {format!("{:.0} m", (prev_best_time / 60.0).floor())}
						else {format!("None")};
					let current_time_text = if current_time < 60.0 {format!("{:.2} s", current_time)}
						else if current_time < 6000.0 {format!("{:.0} m {:.0} s", (current_time / 60.0).floor(), current_time % 60.0)}
						else if current_time < 999999.0 {format!("{:.0} m", (current_time / 60.0).floor())}
						else {format!("A While")};
					let new_best_time = if current_time < prev_best_time {format!("New Best Time: ")} else {format!("Reaction Time: ")};
					let new_best_cost = if current_cost < prev_best_cost {format!("New Best Cost: ")} else {format!("Reaction Cost: ")};
					let prev_best_cost_text = if prev_best_cost < 999999 {format!("{} c", prev_best_cost)} else {format!("None")};
					let current_cost_text = format!("{} c", current_cost);
					let win_text = [
						prev_best_time_text,
						current_time_text,
						prev_best_cost_text,
						current_cost_text,
					];

					let x = 10.0;
					let y = 0.0;
					let z = 810.0;
					commands.spawn((Text2dBundle{
						transform: Transform::from_xyz(-x, y, z),
						text: Text::from_section(format!("Previous Best Time: \n{}\nPrevious Best Cost: \n{}", new_best_time, new_best_cost), get_win_text_style(&asset_server))
							.with_alignment(TextAlignment::Right),
						text_anchor: bevy::sprite::Anchor::CenterRight,
						..Default::default()
						},
						DespawnOnExitPauseState,
						Name::new("Win Text")
					));

					commands.spawn((Text2dBundle{
						transform: Transform::from_xyz(x, y, z),
						text: Text::from_section(format!("{}\n{}\n{}\n{}", win_text[0], win_text[1], win_text[2], win_text[3]), get_win_values_text_style(&asset_server))
							.with_alignment(TextAlignment::Left),
						text_anchor: bevy::sprite::Anchor::CenterLeft,
						..Default::default()
						},
						DespawnOnExitPauseState,
						Name::new("Win Text")
					));
					commands
						.spawn((Text2dBundle {
							transform: Transform::from_xyz(-300.0, -300.0, 840.0),
							text: Text::from_section(format!("Replay"), get_button_text_style(&asset_server))
								.with_alignment(TextAlignment::Center),
							..Default::default()
						},
						DespawnOnExitPauseState,
						Name::new("Exit Win Screen Text")
					));
					commands
						.spawn((Text2dBundle {
							transform: Transform::from_xyz(300.0, -300.0, 840.0),
							text: Text::from_section(format!("Continue"), get_button_text_style(&asset_server))
								.with_alignment(TextAlignment::Center),
							..Default::default()
						},
						DespawnOnExitPauseState,
						Name::new("Continue Win Screen Text")
					));
					let mut buttons = Vec::new();
					let effects = [
						ButtonEffect::PopupButton(PopupButton::ReplayLevel),
						ButtonEffect::PopupButton(PopupButton::CompleteLevel),
					];
					if let Ok(save_data) = pkv.get::<SaveData>("save_data") {
						let enabled = [save_data.cutscenes_unlocked[selected_level.0 + 1], true];
						for i in 0..2 {
							buttons.push((
								StandardButton {
									location: Vec3::new(-300.0+600.0*i as f32, -300.0, 830.0),
									dimensions: Dimensions {
										width: 400.0,
										height: 40.0,
									},
									enabled: enabled[i],
									idle_color: Color::hex("EDD6AD").unwrap(),
									hovered_color: Color::hex("CDB68D").unwrap(),
									disabled_color: Color::hex("9D865D").unwrap(),
								}, effects[i]
							));
						}
					}
					for button in buttons {
						commands
							.spawn((SpriteBundle {
								transform: Transform::from_translation(button.0.location),
								sprite: Sprite {
									color: Color::hex("EDD6AD").unwrap(),
									custom_size: Some(Vec2::new(button.0.dimensions.width, button.0.dimensions.height)), 
									..Default::default()
								},
								..Default::default()
							},
							button.0,
							button.1,
							DespawnOnExitPauseState,
							Name::new("Win Screen Button")
						));
					}
				}
			}
		}
	}
}