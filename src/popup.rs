// Import Bevy game engine essentials
use bevy::prelude::*;
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
	mut commands: Commands,
	mut ev_r_popup_complete: EventReader<PopupCompleteEvent>,
) {
	for _ in ev_r_popup_complete.iter() {
		let info = popup_query.single();
		match info.popup_type {
			PopupType::Settings => {
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
					Name::new("Exit Settings Button")
				));
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
				// Spawn level select buttons
				for j in 0..5 {
					for i in 0..5 {
						let button = StandardButton {
							location: Vec3::new(-200.0 + 100.0 * i as f32, 100.0 - 100.0 * j as f32, 810.0),
							dimensions: Dimensions {
								width: 50.0,
								height: 50.0,
							},
							enabled: true,
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
			PopupType::WinScreen => {
				let button = StandardButton {
					location: Vec3::new(0.0, -200.0, 810.0),
					dimensions: Dimensions {
						width: 300.0,
						height: 300.0,
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
					ButtonEffect::PopupButton(PopupButton::CompleteLevel),
					button,
					DespawnOnExitPauseState,
					Name::new("Exit Win Screen Button")
				));
			}
		}
	}
}