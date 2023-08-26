// Import Bevy game engine essentials
use bevy::prelude::*;
use bevy_pkv::PkvStore;
// Import components, resources, and events
use crate::components::*;

// Plugin for handling narrative and text
// based elements of the game
pub struct CutscenePlugin;

impl Plugin for CutscenePlugin {
    fn build(&self, app: &mut App) {
        app
			.add_systems(OnEnter(GameState::Cutscene),
				spawn_cutscene,
			)
			.add_systems(Update, (
				update_cutscene_text,
				fade_actors,
			).run_if(in_state(GameState::Cutscene)))
		;
	}
}

// Spawn the actors portraits and
// the textbox
fn spawn_cutscene(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	cutscene_tracker: Res<CutsceneTracker>,
) {
	let (initial_line, _) = next_line(cutscene_tracker.current_scene, 0);
	for char in actors_in_scene(cutscene_tracker.current_scene) {
		commands
			.spawn((SpriteBundle {
				//texture: asset_server.load(get_actor_path(char)),
				transform: Transform::from_xyz(if Actor::Guard == char {-350.0} else {350.0}, -50.0, 200.0),
				sprite: Sprite {
					color: Color::rgba(6.0, 6.0, 1.0, 0.0),
					custom_size: Some(Vec2::new(ACTOR_WIDTH, ACTOR_HEIGHT)),
					..Default::default()
				},
				..Default::default()
			},
			ActorInfo{
				actor: char,
			},
			DespawnOnExitGameState,
			Name::new(get_actor_name(char))
		));
	}

	let char_portrait_size = Vec2::new(PORTRAIT_WIDTH, PORTRAIT_HEIGHT);
	commands
		.spawn((SpriteBundle {
			transform: Transform::from_xyz(-625.0, -250.0, 600.0),
			sprite: Sprite {
				custom_size: Some(Vec2::new(char_portrait_size.x, char_portrait_size.y)),
				..Default::default()
			},
			..Default::default()
		},
		DespawnOnExitGameState,
		Name::new("Text Box Portrait")
	));

	commands
		.spawn((SpriteBundle {
			transform: Transform::from_xyz(150.0, -250.0, 600.0),
			sprite: Sprite {
				custom_size: Some(Vec2::new(TEXT_BOX_WIDTH, TEXT_BOX_HEIGHT)),
				..Default::default()
			},
			..Default::default()
		},
		DespawnOnExitGameState,
		Name::new("Text Box Sprite")
	)).with_children(|parent| {
		parent
			.spawn((Text2dBundle {
				text_2d_bounds: bevy::text::Text2dBounds{ size: Vec2::new(
					TEXT_BOX_WIDTH - TEXT_BOX_MARGINS * 2.0,
					TEXT_BOX_HEIGHT - TEXT_BOX_MARGINS * 2.0,
				)},
				transform: Transform::from_xyz(
					-TEXT_BOX_WIDTH / 2.0 + TEXT_BOX_MARGINS,
					TEXT_BOX_HEIGHT / 2.0 - TEXT_BOX_MARGINS,
					10.0,
				),
				text_anchor: bevy::sprite::Anchor::TopLeft,
				text: Text::from_section(initial_line, get_cutscene_text_style(&asset_server))
				.with_alignment(TextAlignment::Left),
				..Default::default()
			},
			CutsceneText,
			Name::new("Cutscene Text")
		));
	});

	let button = StandardButton {
		location: Vec3::new(600.0, 350.0, 810.0),
		dimensions: Dimensions {
			width: 200.0,
			height: 100.0,
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
		ButtonEffect::CutsceneButton(CutsceneButton::SkipCutscene),
		button,
		DespawnOnExitGameState,
		Name::new("Skip Cutscene Button")
	));
}

// Update cutscene text as user advances through
// the cutscene
pub fn update_cutscene_text(
	mut cutscene_text_query: Query<(&mut Text, With<CutsceneText>)>,
	mut cutscene_tracker: ResMut<CutsceneTracker>,
	mut text_speed_timer: ResMut<TextSpeedTimer>,
	mut ev_w_fade_transition: EventWriter<FadeTransitionEvent>,
	asset_server: Res<AssetServer>,
	mouse: Res<Input<MouseButton>>,
	keyboard: Res<Input<KeyCode>>,
	time: Res<Time>,
) {
	if mouse.just_pressed(MouseButton::Left) || keyboard.just_pressed(KeyCode::Space){
		match cutscene_tracker.cutscene_state {
			CutsceneState::Initialize => {
				cutscene_tracker.current_character = 0;
				cutscene_tracker.current_line = 1;
				(cutscene_tracker.full_line, cutscene_tracker.actor_info) = next_line(cutscene_tracker.current_scene, cutscene_tracker.current_line);
				cutscene_tracker.cutscene_state = CutsceneState::Started;
			},
			CutsceneState::Started => {
				if cutscene_tracker.current_character != cutscene_tracker.full_line.len() {
					cutscene_tracker.current_character = cutscene_tracker.full_line.len() - 1;
				} else {
					if cutscene_tracker.current_line != lines_per_scene(cutscene_tracker.current_scene) {
						cutscene_tracker.current_character = 0;
						cutscene_tracker.current_line += 1;
						(cutscene_tracker.full_line, cutscene_tracker.actor_info) = next_line(cutscene_tracker.current_scene, cutscene_tracker.current_line);
					} else {
						match cutscene_tracker.current_scene {
							_ => ev_w_fade_transition.send(FadeTransitionEvent(GameState::Lab)),
						}
						cutscene_tracker.current_scene = 0;
						cutscene_tracker.current_line = 0;
						cutscene_tracker.current_character = 0;
						cutscene_tracker.actor_info = ActorInfo{actor: Actor::Nobody};
						cutscene_tracker.cutscene_state = CutsceneState::Ended;
					}
				}
			},
			CutsceneState::Ended => {

			}
		}
	}
	text_speed_timer.0.tick(time.delta());
	if cutscene_tracker.cutscene_state == CutsceneState::Started {
		if text_speed_timer.0.just_finished() {
			if cutscene_tracker.current_character + 1 <= cutscene_tracker.full_line.len() {
				cutscene_tracker.current_character += 1;
				for (mut text, _) in cutscene_text_query.iter_mut() {
					text.sections = vec![
						TextSection::new(
							&cutscene_tracker.full_line[0..cutscene_tracker.current_character],
							get_cutscene_text_style(&asset_server),
						)
					];
				}
			}
		}
	}
}

// Fade actors in and out depending on if they
// are currently active and speaking
fn fade_actors(
	mut actor_query: Query<(&mut Sprite, &ActorInfo)>,
	cutscene_tracker: Res<CutsceneTracker>,
	time: Res<Time>,
) {
	for (mut sprite, info) in actor_query.iter_mut() {
		let current_a = sprite.color.a();
		if info.actor == cutscene_tracker.actor_info.actor {
			sprite.color.set_a((current_a + FADE_ACTOR_SPEED * time.delta_seconds()).clamp(0.0, 1.0));
		} else {
			sprite.color.set_a((current_a - FADE_ACTOR_SPEED * time.delta_seconds()).clamp(0.0, 1.0));
		}
	}
}