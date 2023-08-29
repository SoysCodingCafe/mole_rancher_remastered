// Import Bevy game engine essentials
use bevy::prelude::*;
// Import components, resources, and events
use crate::components::*;

// Plugin for spawning lab sprites
pub struct LabPlugin;

impl Plugin for LabPlugin {
    fn build(&self, app: &mut App) {
        app
			.add_systems( OnEnter(GameState::Lab), (
				spawn_lab,
			))
			.add_systems(OnExit(PauseState::Paused), (
				hide_bright_lab,
			))
		;
	}
}

// Spawn all the sprites required for the lab screen which
// acts as a menu to switch between the level select, the 
// logbook, and exiting back to the main menu
fn spawn_lab(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	ortho_size: Res<OrthoSize>,
) {
	// Spawn background for lab when dark
	commands
		.spawn((SpriteBundle {
			texture: asset_server.load("sprites/background/lab_bright.png"),
			transform: Transform::from_xyz(0.0, 0.0, 0.0),
			sprite: Sprite {
				custom_size: Some(Vec2::new(ortho_size.width, ortho_size.height)), 
				..Default::default()
			},
			visibility: Visibility::Visible,
			..Default::default()
		},
		DespawnOnExitGameState,
		Name::new("Lab Dark")
	));

	// Spawn background for bright lab, but initially set to hidden
	commands
		.spawn((SpriteBundle {
			texture: asset_server.load("sprites/background/lab_dark.png"),
			transform: Transform::from_xyz(0.0, 0.0, 10.0),
			sprite: Sprite {
				custom_size: Some(Vec2::new(ortho_size.width, ortho_size.height)), 
				..Default::default()
			},
			visibility: Visibility::Hidden,
			..Default::default()
		},
		BrightLab,
		DespawnOnExitGameState,
		Name::new("Lab Bright")
	));

	for i in 0..3 {
		commands
			.spawn((SpriteBundle {
				transform: Transform::from_xyz(0.0, 0.0, 810.0),
				sprite: Sprite {
					color: Color::rgba(i as f32 / 2.0, 1.0, 1.0, 1.0),
					custom_size: Some(Vec2::new(400.0, 400.0)), 
					..Default::default()
				},
				visibility: Visibility::Hidden,
				..Default::default()
			},
			Logbook(i),
			DespawnOnExitGameState,
			Name::new(format!("Logbook Page {}", i))
		));
	}

	// Name, Path, Z-Level, Alpha, Visibility, Interaction
	let lab_sprite_info = [
		("Monitor", "sprites/ui/monitor.png", 11.0, 1.0, Visibility::Hidden, ButtonEffect::CustomLabButton(CustomLabButton::MonitorActivate)),
		("Logbook", "sprites/ui/logbook.png", 11.0, 1.0, Visibility::Hidden, ButtonEffect::CustomLabButton(CustomLabButton::LogbookOpen)),
		("Exit", "sprites/ui/exit.png", 11.0, 1.0, Visibility::Hidden, ButtonEffect::CustomLabButton(CustomLabButton::ExitLab)),
		("Exit Glow", "sprites/ui/glow.png", 11.0, 0.05, Visibility::Hidden, ButtonEffect::CustomLabButton(CustomLabButton::ExitLab)),
	];

	for (name, path, z, a, visibility, interaction) in lab_sprite_info {
		commands
			.spawn((SpriteBundle {
				texture: asset_server.load(path),
				transform: Transform::from_xyz(0.0, 0.0, z),
				sprite: Sprite {
					color: Color::rgba(1.0, 1.0, 1.0, a),
					custom_size: Some(Vec2::new(ortho_size.width, ortho_size.height)),
					..Default::default()
				},
				visibility: visibility,
				..Default::default()
			},
			DespawnOnExitGameState,
			interaction,
			Name::new(name)
		));
	}
}

// Toggle back to dark lab sprite when exiting level select popup
fn hide_bright_lab(
	mut bright_lab_query: Query<(&mut Visibility, With<BrightLab>)>,
) {
	for (mut visibility, _) in bright_lab_query.iter_mut() {
		*visibility = Visibility::Hidden;
	}
}