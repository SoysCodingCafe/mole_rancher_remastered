// Import Bevy game engine essentials
use bevy::{prelude::*, app::AppExit};
// Import Pkv Store for saving and loading game data
use bevy_pkv::PkvStore;
// Import components, resources, and events
use crate::components::*;

// Plugin for handling button interactions and
// state changes
pub struct ButtonsPlugin;

impl Plugin for ButtonsPlugin {
    fn build(&self, app: &mut App) {
        app
			.add_systems(Update, (
				handle_button_calls,
				standard_buttons,
			))
			.add_systems(Update, (
				custom_lab_buttons,
			).run_if(in_state(GameState::Lab))
			.run_if(in_state(PauseState::Unpaused)))
		;
	}
}

// Checks if the cursor is hovering over a button, and 
// emits an event if clicked
fn standard_buttons(
	time: Res<Time>,
	window_query: Query<&Window>,
	ortho_size: Res<OrthoSize>,
	mouse: Res<Input<MouseButton>>,
	current_state: Res<State<PauseState>>,
	audio_volume: Res<AudioVolume>,
	mut button_query: Query<(&mut Sprite, &StandardButton, &ButtonEffect)>,
	mut animation_query: Query<(&mut TextureAtlasSprite, &mut AnimationTimer, &AnimationIndices, &MoleculeButton)>,
	mut tooltip_query: Query<(&mut Transform, With<Tooltip>)>,
	mut ev_w_button_call: EventWriter<ButtonCall>,
) {
	// Get the current window, and the cursor position scaled 
	// to the window size
	let w = window_query.single();
	if let Some(p) = w.cursor_position() {
		let p = Vec2::new(
			ortho_size.width * (p.x / w.width() - 0.5), 
			-ortho_size.height * (p.y / w.height() - 0.5)
		);
		let mut hovering_any = false;
		for (mut sprite, button, effect) in button_query.iter_mut() {
			if button.enabled {
				if *current_state == PauseState::Paused {
					match effect {
						ButtonEffect::PopupButton(_) => (),
						_ => continue,
					}
				}
				if (button.location.x - p.x).abs() < button.dimensions.width / 2.0 && (button.location.y - p.y).abs() < button.dimensions.height / 2.0 {
					match effect {
						ButtonEffect::ReactorButton(ReactorButton::SelectMolecule(index)) => {
							for (mut transform, _) in tooltip_query.iter_mut() {
								let offset = if index < &9 {
									Vec2::new(TOOLTIP_WIDTH/2.0, -TOOLTIP_HEIGHT/2.0)
								} else {
									Vec2::new(TOOLTIP_WIDTH/2.0, TOOLTIP_HEIGHT/2.0)
								};
								transform.translation = Vec3::new(
									p.x + offset.x,
									p.y + offset.y,
									900.0,
								);
								for (mut spritesheet, mut timer, indices, molecule) in animation_query.iter_mut() {
									if molecule.0 == *index {
										timer.0.tick(time.delta());
										if timer.0.just_finished() {
											spritesheet.index = (spritesheet.index + 1) % indices.total + indices.first;
										}
									}
								};
							}
						},
						_ => (),
					}
					sprite.color = Color::BLUE;
					hovering_any = true;
					if mouse.just_pressed(MouseButton::Left) {
						ev_w_button_call.send(ButtonCall(*effect));
					}
				}
			} else {
				sprite.color = Color::GRAY;
				if (button.location.x - p.x).abs() < button.dimensions.width / 2.0 && (button.location.y - p.y).abs() < button.dimensions.height / 2.0 {
					for (mut spritesheet, mut timer, indices, molecule) in animation_query.iter_mut() {
						match effect {
							ButtonEffect::ReactorButton(ReactorButton::SelectMolecule(i)) => {
								if molecule.0 == *i {
									timer.0.tick(time.delta());
									if timer.0.just_finished() {
										spritesheet.index = (spritesheet.index + 1) % indices.total + indices.first;
									}
								}
							},
							_ => (),
						}
					};
				}
			}
		}
		// If not hovering over any buttons then hide all effects
		if !hovering_any {
			for (mut sprite, button, _) in button_query.iter_mut() {
				if button.enabled {sprite.color = Color::WHITE} else {sprite.color = Color::GRAY};
			}
			for (mut transform, _) in tooltip_query.iter_mut() {
				transform.translation.z = -1.0;
			}
		}
		for (mut sprite, _, effect) in button_query.iter_mut() {
			match effect {
				ButtonEffect::PopupButton(PopupButton::BgmVolume(i)) => {
					if (audio_volume.bgm * 10.0) as usize == *i {
						if sprite.color != Color::BLUE {sprite.color = Color::RED};
					} else {
						if sprite.color != Color::BLUE {sprite.color = Color::WHITE};
					}
				},
				ButtonEffect::PopupButton(PopupButton::SfxVolume(i)) => {
					if (audio_volume.sfx * 10.0) as usize == *i {
						if sprite.color != Color::BLUE {sprite.color = Color::RED};
					} else {
						if sprite.color != Color::BLUE {sprite.color = Color::WHITE};
					}
				}
				_ => (),
			}
		}
	}
}

// Checks if the cursor is hovering over interactable elements in the lab, 
// due to their unusual shape
fn custom_lab_buttons(
	window_query: Query<&Window>,
	ortho_size: Res<OrthoSize>,
	mouse: Res<Input<MouseButton>>,
	mut interaction_query: Query<(&mut Visibility, &ButtonEffect)>,
	mut ev_w_button_call: EventWriter<ButtonCall>,
) {
	// Top Left, Top Right, Bottom Right, Bottom Left, Lab Interaction
	let buttons = [
		([Vec2::new(88.0, 78.0), Vec2::new(404.0, 54.0), Vec2::new(336.0, -226.0), Vec2::new(68.0, -162.0)], 
		ButtonEffect::CustomLabButton(CustomLabButton::MonitorActivate)), // Monitor
		([Vec2::new(-72.0, -158.0), Vec2::new(204.0, -190.0), Vec2::new(220.0, -290.0), Vec2::new(-104.0, -238.0)], 
		ButtonEffect::CustomLabButton(CustomLabButton::MonitorActivate)), // Keyboard
		([Vec2::new(-408.0, -134.0), Vec2::new(-148.0, -86.0), Vec2::new(-128.0, -190.0), Vec2::new(-464.0, -274.0)], 
		ButtonEffect::CustomLabButton(CustomLabButton::LogbookOpen)), // Logbook
		([Vec2::new(-800.0, 382.0), Vec2::new(-728.0, 374.0), Vec2::new(-488.0, -178.0), Vec2::new(-800.0, -346.0)], 
		ButtonEffect::CustomLabButton(CustomLabButton::ExitLab)), // Door
		([Vec2::new(-672.0, 318.0), Vec2::new(-440.0, 302.0), Vec2::new(-428.0, 242.0), Vec2::new(-656.0, 258.0)], 
		ButtonEffect::CustomLabButton(CustomLabButton::ExitLab)), // Exit Sign
		([Vec2::new(492.0, 322.0), Vec2::new(796.0, 350.0), Vec2::new(652.0, -18.0), Vec2::new(404.0, 34.0)], 
		ButtonEffect::CustomLabButton(CustomLabButton::Poster)), // Poster
	];

	// Get the current window, and the cursor position scaled 
	// to the window size
	let w = window_query.single();
	if let Some(p) = w.cursor_position() {
		let p = Vec2::new(
			ortho_size.width * (p.x / w.width() - 0.5), 
			-ortho_size.height * (p.y / w.height() - 0.5)
		);
		// Flag to track if no buttons are being hovered
		let mut hovering_any = false;
		// Iterate through each of the buttons on screen and check if 
		// the cursor is inside any of them
		for (v, action) in buttons {
			let mut hovering = true;
			for i in 0..v.len() {
				let j = (i+1) % v.len();
				let ab = v[j] - v[i];
				let ac = p - v[i];
				let ab_cross_ac = ab.perp_dot(ac);
				if ab_cross_ac.is_sign_positive() {
					hovering = false;
					break;
				}
			}
			// If not hovering over the button then skip to 
			// next button in the loop
			if !hovering {
				continue;
			} else {
				hovering_any = true;
			}
			// If hovering over a button then query sprite and interaction type
			for (mut visibility, interaction) in interaction_query.iter_mut() {
				// Display current hovered sprite and hide the rest
				*visibility = if *interaction == action {Visibility::Visible} else {Visibility::Hidden};
				// If mouse pressed then trigger the action of the button
				if mouse.just_pressed(MouseButton::Left) && *interaction == action {
					ev_w_button_call.send(ButtonCall(action));
				}
			}
		}
		// If not hovering over any buttons then hide all effects
		if !hovering_any {
			for (mut visibility, _) in interaction_query.iter_mut() {
				*visibility = Visibility::Hidden;
			}
		}
	}
}

// Handle all the buttons calls by calling the respective transitions
// or toggling visibility on sprites
fn handle_button_calls(
	asset_server: Res<AssetServer>,
	mut pkv: ResMut<PkvStore>,
	mut cutscene_tracker: ResMut<CutsceneTracker>,
	mut audio_volume: ResMut<AudioVolume>,
	mut selected_level: ResMut<SelectedLevel>,
	mut selected_palette: ResMut<SelectedPalette>,
	mut selected_logbook_page: ResMut<SelectedLogbookPage>,
	mut selected_molecule_type: ResMut<SelectedMoleculeType>,
	mut ev_r_button_call: EventReader<ButtonCall>,
	mut ev_w_exit: EventWriter<AppExit>,
	mut ev_w_fade_transition: EventWriter<FadeTransitionEvent>,
	mut ev_w_popup: EventWriter<PopupEvent>,
	mut bright_lab_query: Query<(&mut Visibility, With<BrightLab>)>,
	mut palette_query: Query<(&mut Sprite, &Palette)>,
	mut next_state: ResMut<NextState<PauseState>>,
) {
	for ev in ev_r_button_call.iter() {
		match ev.0 {
			ButtonEffect::MenuButton(ref effect) => {
				match effect {
					MenuButton::StartGame => {
						if let Ok(mut save_data) = pkv.get::<SaveData>("save_data") {
							if save_data.levels_unlocked[0] == false {
								cutscene_tracker.cutscene_state = CutsceneState::Initialize;
								ev_w_fade_transition.send(FadeTransitionEvent(GameState::Cutscene));
								save_data.levels_unlocked[0] = true;
								pkv.set("save_data", &save_data)
									.expect("Unable to save data");
							} else {
								ev_w_fade_transition.send(FadeTransitionEvent(GameState::Lab));
							}
						}
					},
					MenuButton::Settings => {
						next_state.set(PauseState::Paused);
						ev_w_popup.send(PopupEvent{ 
							origin: Vec2::new(0.0, -140.0), 
							image: asset_server.load("sprites/popup/settings.png"),
							alpha: 1.0,
							popup_type: PopupType::Settings,
						});
					},
					MenuButton::ExitGame => ev_w_exit.send(AppExit),
				}
			}
			ButtonEffect::CustomLabButton(ref effect) => {
				match effect {
					// Show bright lab, play reactor sfx, display level select popup
					CustomLabButton::MonitorActivate => {
						let (mut vis, _) = bright_lab_query.single_mut();
						*vis = Visibility::Visible;
						next_state.set(PauseState::Paused);
						ev_w_popup.send(PopupEvent{ 
							origin: Vec2::new(228.0, -10.0), 
							image: asset_server.load("sprites/popup/level_select.png"),
							alpha: 0.9,
							popup_type: PopupType::LevelSelect,
						});
					},
					CustomLabButton::LogbookOpen => {
						next_state.set(PauseState::Paused);
						ev_w_popup.send(PopupEvent{ 
							origin: Vec2::new(-276.0, -162.0), 
							image: asset_server.load("sprites/popup/logbook_base.png"),
							alpha: 1.0,
							popup_type: PopupType::Logbook,
						});
					},
					CustomLabButton::ExitLab => {
						// Fade transition back to menu, play door sfx
						ev_w_fade_transition.send(FadeTransitionEvent(GameState::Menu));
					},
					CustomLabButton::Poster => {
						// Play cat sfx
					},
				};
			},
			ButtonEffect::PopupButton(ref effect) => {
				match effect {
					PopupButton::BgmVolume(volume) => {
						audio_volume.bgm = *volume as f32/10.0;
					},
					PopupButton::SfxVolume(volume) => {
						audio_volume.sfx = *volume as f32/10.0;
					},
					PopupButton::PaletteToggle => {
						selected_palette.0 = (selected_palette.0 + 1) % 4;
						for (mut sprite, palette) in palette_query.iter_mut() {
							sprite.color = get_molecule_color(palette.0, selected_palette.0);
						}
					},
					PopupButton::LogbookPage(page) => {
						selected_logbook_page.0 = *page;
					},
					PopupButton::LevelSelect(level) => {
						if let Ok(save_data) = pkv.get::<SaveData>("save_data") {
							if save_data.levels_unlocked[*level] {
								selected_level.0 = *level;
								next_state.set(PauseState::Unpaused);
								ev_w_fade_transition.send(FadeTransitionEvent(GameState::Reactor));
							}
						}
					},
					PopupButton::CompleteLevel => {
						if let Ok(mut save_data) = pkv.get::<SaveData>("save_data") {
							save_data.levels_unlocked[selected_level.0 + 1] = true;
							next_state.set(PauseState::Unpaused);
							cutscene_tracker.cutscene_state = CutsceneState::Initialize;
							cutscene_tracker.current_scene = selected_level.0 + 1;
							ev_w_fade_transition.send(FadeTransitionEvent(GameState::Cutscene));
							pkv.set("save_data", &save_data)
								.expect("Unable to save data");
						}
					},
					PopupButton::ExitPopup => {
						next_state.set(PauseState::Unpaused);
					},
				}
			},
			ButtonEffect::ReactorButton(ref effect) => {
				match effect {
					ReactorButton::SelectMolecule(molecule_index) => {
						selected_molecule_type.0 = *molecule_index;
					},
					ReactorButton::ExitReactor => {
						ev_w_fade_transition.send(FadeTransitionEvent(GameState::Lab));
					},
				}
			},
		}
	}
}
