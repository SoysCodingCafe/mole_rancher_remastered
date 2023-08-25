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
			transform.scale.x = 1440.0 * timer.0.percent();
			transform.scale.y = 810.0 * timer.0.percent();
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
	mut commands: Commands,
	mut ev_r_popup_complete: EventReader<PopupCompleteEvent>,
) {
	for _ in ev_r_popup_complete.iter() {
		let info = popup_query.single();
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
					location: Vec3::new(-520.0, -310.0, 810.0),
					dimensions: Dimensions {
						width: 200.0,
						height: 100.0,
					},
					enabled: true,
				}, ButtonEffect::PopupButton(PopupButton::ExitPopup)));
				for i in 0..=10 {
					buttons.push((StandardButton {
						location: Vec3::new(25.0 + 60.0 * i as f32, 190.0, 810.0),
						dimensions: Dimensions {
							width: 50.0,
							height: 100.0,
						},
						enabled: true,
					}, ButtonEffect::PopupButton(PopupButton::BgmVolume(i))));
					buttons.push((StandardButton {
						location: Vec3::new(25.0 + 60.0 * i as f32, 65.0, 810.0),
						dimensions: Dimensions {
							width: 50.0,
							height: 100.0,
						},
						enabled: true,
					}, ButtonEffect::PopupButton(PopupButton::SfxVolume(i))));
				}
				buttons.push((StandardButton {
					location: Vec3::new(50.0, -55.0, 810.0),
					dimensions: Dimensions {
						width: 100.0,
						height: 100.0,
					},
					enabled: true,
				}, ButtonEffect::PopupButton(PopupButton::PaletteToggle)));
				commands.spawn((Text2dBundle{
					transform: Transform::from_xyz(100.0, -180.0, 820.0),
					text: Text::from_section(format!("On"), get_settings_text_style(&asset_server))
						.with_alignment(TextAlignment::Center),
					text_anchor: bevy::sprite::Anchor::Center,
					..Default::default()
					},
					DespawnOnExitPauseState,
					Name::new("Particle Trail Enable Text")
				));
				buttons.push((StandardButton {
					location: Vec3::new(100.0, -175.0, 810.0),
					dimensions: Dimensions {
						width: 200.0,
						height: 100.0,
					},
					enabled: true,
				}, ButtonEffect::PopupButton(PopupButton::ParticleTrails(true))));
				commands.spawn((Text2dBundle{
					transform: Transform::from_xyz(350.0, -180.0, 820.0),
					text: Text::from_section(format!("Off"), get_settings_text_style(&asset_server))
						.with_alignment(TextAlignment::Center),
					text_anchor: bevy::sprite::Anchor::Center,
					..Default::default()
					},
					DespawnOnExitPauseState,
					Name::new("Particle Trail Enable Text")
				));
				buttons.push((StandardButton {
					location: Vec3::new(350.0, -175.0, 810.0),
					dimensions: Dimensions {
						width: 200.0,
						height: 100.0,
					},
					enabled: true,
				}, ButtonEffect::PopupButton(PopupButton::ParticleTrails(false))));
				for (button, effect) in buttons {
					commands
						.spawn((SpriteBundle {
							transform: Transform::from_translation(button.location),
							sprite: Sprite {
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
							transform: Transform::from_xyz(133.0 + 36.0 * i as f32, -55.0, 810.0),
							sprite: Sprite {
								color: get_molecule_color(i, selected_palette.0),
								custom_size: Some(Vec2::new(25.0, 100.0)), 
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
				let mut tabs = Vec::new();
				for i in 0..15 {
					tabs.push((StandardButton {
						location: Vec3::new(-700.0 + 100.0 * i as f32, 400.0, 810.0),
						dimensions: Dimensions {
							width: 40.0,
							height: 60.0,
						},
						enabled: true,
					}, ButtonEffect::PopupButton(PopupButton::LogbookPage(i))));
				};
				for i in 0..7 {
					tabs.push((StandardButton {
						location: Vec3::new(700.0, 300.0 - 100.0 * i as f32, 810.0),
						dimensions: Dimensions {
							width: 60.0,
							height: 40.0,
						},
						enabled: true,
					}, ButtonEffect::PopupButton(PopupButton::LogbookPage(i+15))));
				}
				for (button, effect) in tabs {
					commands
						.spawn((SpriteBundle {
							transform: Transform::from_translation(button.location),
							sprite: Sprite {
								custom_size: Some(Vec2::new(button.dimensions.width, button.dimensions.height)), 
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
					location: Vec3::new(-650.0, -350.0, 810.0),
					dimensions: Dimensions {
						width: 100.0,
						height: 50.0,
					},
					enabled: true,
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
					ButtonEffect::PopupButton(PopupButton::ExitPopup),
					button,
					DespawnOnExitPauseState,
					Name::new("Logbook Button")
				));
			},
			PopupType::LevelSelect => {
				if let Ok(save_data) = pkv.get::<SaveData>("save_data") {
					// Spawn level select buttons
					for j in 0..5 {
						for i in 0..5 {
							let button = StandardButton {
								location: Vec3::new(-200.0 + 100.0 * i as f32, 100.0 - 100.0 * j as f32, 810.0),
								dimensions: Dimensions {
									width: 50.0,
									height: 50.0,
								},
								enabled: save_data.levels_unlocked[i+5*j],
							};
							commands.spawn((SpriteBundle {
									transform: Transform::from_translation(button.location),
									sprite: Sprite {
										custom_size: Some(Vec2::new(button.dimensions.width, button.dimensions.height)), 
										..Default::default()
									},
									..Default::default()
								},
								ButtonEffect::PopupButton(PopupButton::LevelSelect(i + 5*j)),
								button,
								DespawnOnExitPauseState,
								Name::new(format!("Level Select Button {}", i + 5*j))
							));
						}
					}
				}
				let button = StandardButton {
					location: Vec3::new(-350.0, -350.0, 810.0),
					dimensions: Dimensions {
						width: 100.0,
						height: 50.0,
					},
					enabled: true,
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
					ButtonEffect::PopupButton(PopupButton::ExitPopup),
					button,
					DespawnOnExitPauseState,
					Name::new("Exit Level Select Button")
				));
			},
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
				let y = 100.0;
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
								location: Vec3::new(-300.0+600.0*i as f32, -200.0, 830.0),
								dimensions: Dimensions {
									width: 200.0,
									height: 150.0,
								},
								enabled: enabled[i],
							}, effects[i]
						));
					}
				}
				for button in buttons {
					commands
						.spawn((SpriteBundle {
							transform: Transform::from_translation(button.0.location),
							sprite: Sprite {
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