// Import Bevy game engine essentials
use bevy::prelude::*;
// Import components, resources, and events
use crate::components::*;

// Plugin for generating the main menu
pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
			.add_systems( OnEnter(GameState::Menu), (
				spawn_menu,
			))
		;
	}
}

// Spawns the background, title text, and buttons
// for the main menu
fn spawn_menu(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	ortho_size: Res<OrthoSize>,
) {
	// Menu Background
	commands.spawn((SpriteBundle {
			texture: asset_server.load("sprites/background/menu.png"),
			transform: Transform::from_xyz(0.0, 0.0, 10.0),
			sprite: Sprite {
				custom_size: Some(Vec2::new(ortho_size.width, ortho_size.height)),
				..Default::default()
			},
			..Default::default()
		},
		DespawnOnExitGameState,
		Name::new("Menu Background")
	));

	// Title Text
	/*commands.spawn((Text2dBundle {
			transform: Transform::from_translation(Vec3::new(0.0, 250.0, 100.0)),
			text: Text::from_section(format!("Mole Rancher"), get_title_text_style(&asset_server))
					.with_alignment(TextAlignment::Center),
			..Default::default()
		},
		DespawnOnExitGameState,
		Name::new("Title Text"),
	)).with_children(|parent| {
		parent.spawn((Text2dBundle {
				transform: Transform::from_translation(Vec3::new(0.0, -65.0, 0.0)),
				text: Text::from_section(format!("Remastered Edition"), get_subtitle_text_style(&asset_server))
						.with_alignment(TextAlignment::Center),
				..Default::default()
			},
			DespawnOnExitGameState,
			Name::new("Subtitle Text"),
		));
	});*/

	//Title Logo
	commands.spawn((SpriteBundle {
			texture: asset_server.load("splash/title.png"),
			transform: Transform::from_xyz(0.0, 200.0, 100.0),
			sprite: Sprite {
				..Default::default()
			},
			..Default::default()
		},
		DespawnOnExitGameState,
		Name::new("Title Logo")
	));

	// Buttons
	let effect = [
		ButtonEffect::MenuButton(MenuButton::StartGame),
		ButtonEffect::MenuButton(MenuButton::Settings),
		ButtonEffect::MenuButton(MenuButton::ExitGame),
	];
	// Button text
	let text = [
		"Play".to_string(), 
		"Settings".to_string(), 
		"Quit".to_string() 
	];

	for i in 0..3 {
		let button = StandardButton {
			location: Vec3::new(0.0, -70.0 * i as f32, 100.0),
			dimensions: Dimensions {
				width: 452.0,
				height: 60.0,
			},
			enabled: true,
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
			effect[i],
			button,
			DespawnOnExitGameState,
			Name::new("Menu Button")
		)).with_children(|parent| {
			parent
				.spawn((Text2dBundle {
					transform: Transform::from_xyz(0.0, -5.0, 10.0,),
					text: Text::from_section(format!("{}", text[i]), get_button_text_style(&asset_server))
						.with_alignment(TextAlignment::Center),
					..Default::default()
				},
				Name::new("Menu Button Text")
			));
		});
	}

}