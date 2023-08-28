// Import Bevy game engine essentials
use bevy::{prelude::*, app::AppExit, render::view::RenderLayers};
use bevy_kira_audio::{Audio, AudioControl};
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
				replay_level.run_if(in_state(GameState::Reactor)),
				handle_button_calls.after(replay_level),
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
	pkv: Res<PkvStore>,
	audio: Res<Audio>,
	asset_server: Res<AssetServer>,
	mut button_query: Query<(&mut Sprite, &StandardButton, &ButtonEffect)>,
	mut tooltip_text_query: Query<(&mut Text, With<TooltipText>)>,
	mut animation_query: Query<(&mut TextureAtlasSprite, &mut AnimationTimer, &AnimationIndices, &MoleculeButton)>,
	mut tooltip_query: Query<(&mut Transform, With<Tooltip>)>,
	mut ev_w_button_call: EventWriter<ButtonCall>,
) {
	let mut hovering_any = false;
	let idle_color = Color::hex("EDD6AD").unwrap();
	let hovered_color = Color::hex("CDB68D").unwrap();
	let disabled_color = Color::hex("9D865D").unwrap();
	// Get the current window, and the cursor position scaled 
	// to the window size
	let w = window_query.single();
	if let Some(p) = w.cursor_position() {
		let p = Vec2::new(
			ortho_size.width * (p.x / w.width() - 0.5), 
			-ortho_size.height * (p.y / w.height() - 0.5)
		);
		for (mut sprite, button, effect) in button_query.iter_mut() {
			if button.enabled {
				if *current_state == PauseState::Paused {
					match effect {
						ButtonEffect::PopupButton(_) => (),
						_ => continue,
					}
				}
				sprite.color = idle_color;
				if (button.location.x - p.x).abs() < button.dimensions.width / 2.0 && (button.location.y - p.y).abs() < button.dimensions.height / 2.0 {
					hovering_any = true;
					sprite.color = hovered_color;
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
							for (mut text, _) in tooltip_text_query.iter_mut() {
								text.sections[0].value = get_tooltip_text(*index, true);
							}
						},
						_ => (),
					}
					if mouse.just_pressed(MouseButton::Left) {
						if let Ok(save_data) = pkv.get::<SaveData>("save_data") {
							audio
								.play(asset_server.load("audio/haptics/click.wav"))
								.with_volume(save_data.sfx_volume);
						}
						ev_w_button_call.send(ButtonCall(*effect));
					}
				}
			} else {
				sprite.color = disabled_color;
				if (button.location.x - p.x).abs() < button.dimensions.width / 2.0 && (button.location.y - p.y).abs() < button.dimensions.height / 2.0 {
					hovering_any = true;
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
							for (mut text, _) in tooltip_text_query.iter_mut() {
								text.sections[0].value = get_tooltip_text(*index, false);
							}
						},
						_ => (),
					};
				}
			}
		}
	}
	// If not hovering over any buttons then hide all effects
	if !hovering_any {
		for (mut transform, _) in tooltip_query.iter_mut() {
			transform.translation.z = -1.0;
		}
	}
	for (mut sprite, _, effect) in button_query.iter_mut() {
		if let Ok(save_data) = pkv.get::<SaveData>("save_data") {
			match effect {
				ButtonEffect::PopupButton(PopupButton::BgmVolume(i)) => {
					if (save_data.bgm_volume * 10.0) as usize == *i {
						sprite.color = disabled_color;
					}
				},
				ButtonEffect::PopupButton(PopupButton::SfxVolume(i)) => {
					if (save_data.sfx_volume * 10.0) as usize == *i {
						sprite.color = disabled_color;
					}
				}
				ButtonEffect::PopupButton(PopupButton::ParticleTrails(enable)) => {	
					if save_data.particles_enabled == *enable {
						sprite.color = disabled_color;
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
	mut selected_level: ResMut<SelectedLevel>,
	mut selected_palette: ResMut<SelectedPalette>,
	mut selected_molecule_type: ResMut<SelectedMoleculeType>,
	mut ev_r_button_call: EventReader<ButtonCall>,
	mut ev_w_exit: EventWriter<AppExit>,
	mut ev_w_fade_transition: EventWriter<FadeTransitionEvent>,
	mut ev_w_replay_level: EventWriter<ReplayLevelEvent>,
	mut ev_w_popup: EventWriter<PopupEvent>,
	mut logbook_text_query: Query<(&mut Text, &LogbookText)>,
	mut bright_lab_query: Query<(&mut Visibility, With<BrightLab>)>,
	mut palette_query: Query<(&mut Sprite, &Palette)>,
	mut next_pause_state: ResMut<NextState<PauseState>>,
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
						next_pause_state.set(PauseState::Paused);
						ev_w_popup.send(PopupEvent{ 
							origin: Vec2::new(0.0, -70.0), 
							image: asset_server.load("sprites/popup/note_small.png"),
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
						next_pause_state.set(PauseState::Paused);
						ev_w_popup.send(PopupEvent{ 
							origin: Vec2::new(228.0, -10.0), 
							image: asset_server.load("sprites/popup/level_select.png"),
							alpha: 1.0,
							popup_type: PopupType::LevelSelect,
						});
					},
					CustomLabButton::LogbookOpen => {
						next_pause_state.set(PauseState::Paused);
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
						if let Ok(mut save_data) = pkv.get::<SaveData>("save_data") {
							save_data.bgm_volume = *volume as f64/10.0;
							pkv.set("save_data", &save_data)
									.expect("Unable to save data");
						}
						
					},
					PopupButton::SfxVolume(volume) => {
						if let Ok(mut save_data) = pkv.get::<SaveData>("save_data") {
							save_data.sfx_volume = *volume as f64/10.0;
							pkv.set("save_data", &save_data)
									.expect("Unable to save data");
						}
					},
					PopupButton::PaletteToggle => {
						selected_palette.0 = (selected_palette.0 + 1) % 4;
						for (mut sprite, palette) in palette_query.iter_mut() {
							sprite.color = get_molecule_color(palette.0, selected_palette.0);
						}
						if let Ok(mut save_data) = pkv.get::<SaveData>("save_data") {
							save_data.selected_palette = selected_palette.0;
							pkv.set("save_data", &save_data)
									.expect("Unable to save data");
						}
					},
					PopupButton::ParticleTrails(enable) => {
						if let Ok(mut save_data) = pkv.get::<SaveData>("save_data") {
							save_data.particles_enabled = *enable;
							pkv.set("save_data", &save_data)
									.expect("Unable to save data");
						}
					},
					PopupButton::LogbookPage(page) => {
						for (mut text, side) in logbook_text_query.iter_mut() {
							text.sections[0].value = get_logbook_text(*page, side.0);
						}
					},
					PopupButton::LevelSelect(level) => {
						if let Ok(save_data) = pkv.get::<SaveData>("save_data") {
							if save_data.levels_unlocked[*level] {
								selected_level.0 = *level;
								next_pause_state.set(PauseState::Unpaused);
								ev_w_fade_transition.send(FadeTransitionEvent(GameState::Reactor));
							}
						}
					},
					PopupButton::ReturnToLab => {
						next_pause_state.set(PauseState::Unpaused);
						ev_w_fade_transition.send(FadeTransitionEvent(GameState::Lab));
					}
					PopupButton::ReplayLevel => {
						next_pause_state.set(PauseState::Unpaused);
						ev_w_replay_level.send(ReplayLevelEvent);
					},
					PopupButton::CompleteLevel => {
						next_pause_state.set(PauseState::Unpaused);
						if let Ok(mut save_data) = pkv.get::<SaveData>("save_data") {
							if save_data.cutscenes_unlocked[selected_level.0 + 1] {
								ev_w_fade_transition.send(FadeTransitionEvent(GameState::Lab));
							} else {
								save_data.cutscenes_unlocked[selected_level.0 + 1] = true;
								pkv.set("save_data", &save_data)
										.expect("Unable to save data");
								cutscene_tracker.cutscene_state = CutsceneState::Initialize;
								cutscene_tracker.current_scene = selected_level.0 + 1;
								ev_w_fade_transition.send(FadeTransitionEvent(GameState::Cutscene));
							}
						}
					},
					PopupButton::ExitPopup => {
						next_pause_state.set(PauseState::Unpaused);
					},
				}
			},
			ButtonEffect::ReactorButton(ref effect) => {
				match effect {
					ReactorButton::SelectMolecule(molecule_index) => {
						selected_molecule_type.0 = *molecule_index;
					},
					ReactorButton::RestartLevel => {
						ev_w_replay_level.send(ReplayLevelEvent);
					},
					ReactorButton::PauseLevel => {
						next_pause_state.set(PauseState::Paused);
						ev_w_popup.send(PopupEvent{
							origin: Vec2::ZERO, 
							image: asset_server.load("sprites/popup/note_small.png"), 
							alpha: 0.9, 
							popup_type: PopupType::LevelIntro(selected_level.0), 
						})
					}
					ReactorButton::ExitReactor => {
						ev_w_fade_transition.send(FadeTransitionEvent(GameState::Lab));
					},
				}
			},
			ButtonEffect::CutsceneButton(CutsceneButton::SkipCutscene) => {
				ev_w_fade_transition.send(FadeTransitionEvent(GameState::Lab));
			},
		}
	}
}

fn replay_level(
	molecule_query: Query<(Entity, With<Molecule>)>,
	asset_server: Res<AssetServer>,
	level: Res<SelectedLevel>,
	selected_palette: Res<SelectedPalette>,
	mut current_cost: ResMut<CurrentCost>,
	mut ev_r_replay_level: EventReader<ReplayLevelEvent>,
	mut ev_w_popup: EventWriter<PopupEvent>,
	mut commands: Commands,
	mut texture_atlases: ResMut<Assets<TextureAtlas>>,
	mut next_state: ResMut<NextState<PauseState>>,
	mut selected_molecule_type: ResMut<SelectedMoleculeType>,
	mut stopwatch_text_query: Query<(&mut Text, &mut StopwatchText)>,
	mut reactor_query: Query<(Entity, &mut ReactorCondition)>,
	mut launch_tube_query: Query<(&mut Transform, &mut LaunchTube, Without<ReactorCamera>)>,
	mut reactor_camera_query: Query<(&mut OrthographicProjection, &mut Transform, With<ReactorCamera>)>,
) {
	for _ in ev_r_replay_level.iter() {
		next_state.set(PauseState::Paused);
		ev_w_popup.send(PopupEvent{ 
			origin: Vec2::new(0.0, 0.0), 
			image: asset_server.load("sprites/popup/note_small.png"),
			alpha: 1.0,
			popup_type: PopupType::LevelIntro(level.0),
		});
		for i in 0..TOTAL_MOLECULE_TYPES {
			if get_available_molecules(level.0)[i] {
				selected_molecule_type.0 = i;
				break;
			}
		}
		current_cost.0 = 0;
		let (mut ortho_proj, mut transform, _) = reactor_camera_query.single_mut();
		ortho_proj.scale = get_initial_zoom(level.0);
		transform.translation.x = 0.0;
		transform.translation.y = 0.0;
		for (mut text, mut stopwatch) in stopwatch_text_query.iter_mut() {
			text.sections[0].value = "".to_string();
			stopwatch.0.reset();
		}
		for (entity, _) in molecule_query.iter() {
			commands.entity(entity).despawn_recursive();
		}
		for (entity, mut condition) in reactor_query.iter_mut() {
			condition.temperature = 0.0;
			condition.pressure = 0.0;
			commands.entity(entity).remove::<SelectedReactor>();
		}
		let reactors = get_reactors(level.0);
		let z = 910.0;
		for reactor in reactors.iter() {
			match reactor.reactor_type {
				ReactorType::Rectangle{origin, dimensions} => {
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
					for (mut transform, mut launch_tube, _) in launch_tube_query.iter_mut() {
						if launch_tube.id == reactor.reactor_id {
							*transform = Transform::from_translation(Vec3::new(origin.x, origin.y + dimensions.height / 2.0, z));
							launch_tube.current_rotation = 0.0;
						}
					}
				},
				ReactorType::Circle{origin, radius} => {
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
					for (mut transform, mut launch_tube, _) in launch_tube_query.iter_mut() {
						if launch_tube.id == reactor.reactor_id {
							*transform = Transform::from_translation(Vec3::new(origin.x, origin.y + radius, z));
							launch_tube.current_rotation = 0.0;
						}
					}
				},
			}
		}
	}
}