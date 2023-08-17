// Import Bevy game engine essentials
use bevy::prelude::*;
// Import components, resources, and events
use crate::components::*;

// Plugin for handling state transitions
pub struct StatesPlugin;

impl Plugin for StatesPlugin {
    fn build(&self, app: &mut App) {
        app
		.add_systems( Startup,(
			spawn_fade_screen,
		))
		.add_systems(Update, (
			fade_transition,
		))
		;
	}
}

// Spawn fullscreen black sprite to be used for transitions
// and screen darkening for tutorials
fn spawn_fade_screen(
	mut commands: Commands,
	ortho_size: Res<OrthoSize>,
) {
	commands
		.spawn((SpriteBundle {
			transform: Transform::from_xyz(0.0, 0.0, 999.0),
			sprite: Sprite {
				custom_size: Some(Vec2::new(ortho_size.width, ortho_size.height)), 
				color: Color::rgba(0.0, 0.0, 0.0, 0.0),
				..Default::default()},
			..Default::default()
		},
		FadeScreen,
		Name::new("Fade Screen")
	));
}

// Detects a StateTransition effect, fades in the fade screen,
// performs the StateTransition, and then fades out the fade screen
fn fade_transition(
	mut ev_r_fade_transition: EventReader<FadeTransitionEvent>,
	mut target_state: Local<GameState>,
	mut fade_screen_state: Local<FadeScreenState>,
	mut fade_timer: ResMut<FadeTransitionTimer>,
	mut fade_screen_query: Query<(&mut Sprite, With<FadeScreen>)>,
	mut next_state: ResMut<NextState<GameState>>,
	time: Res<Time>,
) {
	// Check if a fade state transition effect has been requested
	for ev in ev_r_fade_transition.iter() {
		// Only transition if not already transitioning, otherwise ignore
		if *fade_screen_state == FadeScreenState::Idle {
			*target_state = ev.0;
			*fade_screen_state = FadeScreenState::Closing;
		}
	}

	// Fade in the fade screen sprite, change state, and then fade back out again
	let mut fade_screen_sprite = fade_screen_query.single_mut();
	match *fade_screen_state {
		FadeScreenState::Idle => (),
		FadeScreenState::Closing => {
			fade_timer.0.tick(time.delta());
			fade_screen_sprite.0.color.set_a(fade_timer.0.percent());
			if fade_timer.0.just_finished() {
				fade_timer.0.reset();
				next_state.set(*target_state);
				*fade_screen_state = FadeScreenState::Opening;
			};
		},
		FadeScreenState::Opening => {
			fade_timer.0.tick(time.delta());
			fade_screen_sprite.0.color.set_a(1.0 - fade_timer.0.percent());
			if fade_timer.0.just_finished() {
				fade_timer.0.reset();
				*fade_screen_state = FadeScreenState::Idle;
			};
		},
	}
}