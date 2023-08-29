// Import Bevy game engine essentials
use bevy::{prelude::*, asset::ChangeWatcher};
// Import duration for the asset hot reloading change watcher
use std::time::Duration;

// MODULES
mod audio;
mod buttons;
mod camera;
mod components;
mod cutscene;
mod lab;
mod menu;
mod molecules;
mod particles;
mod popup;
mod reactor;
mod setup;
mod states;

// Only include in debug builds
#[cfg(debug_assertions)]
mod debug;

// Can't forget main!
fn main() {
	// Create app to hold all our plugins, resources, events, and systems
	let mut app = App::new();
	app
		// Default plugins provided by Bevy handles all essentials for a game
		// such as the game window, asset management, input handling, and time
		.add_plugins(DefaultPlugins
			.set(WindowPlugin {
				primary_window: Some(Window {
					// Stops the game from stopping keyboard shortcuts e.g. F12
					prevent_default_event_handling: false,
					// Default to Borderless Fullscreen
					//mode: bevy::window::WindowMode::BorderlessFullscreen,
					// Set custom window title
					title: "Mole Rancher Remastered".to_string(),
					..default()
				}),
				..default()
			})
			.set(AssetPlugin {
				// Enables asset hot reloading
				//watch_for_changes: ChangeWatcher::with_delay(Duration::from_millis(200)),
				..Default::default()
			})
			// Prevents pixel art sprites from becoming blurry
			.set(ImagePlugin::default_nearest())
		)

		// Plugins
		.add_plugins((
			// Kira audio plugin for Bevy for playing sound files
			bevy_kira_audio::AudioPlugin,
			// For playing background music and sound effects
			audio::AudioPlugin,
			// Button logic and interactions
			buttons::ButtonsPlugin,
			// Camera panning and zooming
			camera::CameraPlugin,
			// Text and sprites for cutscenes
			cutscene::CutscenePlugin,
			// Spawns sprites for lab which acts as a hub menu
			lab::LabPlugin,
			// Spawns title and menu buttons
			menu::MenuPlugin,
			// Molecule spawning and collision logic
			molecules::MoleculesPlugin,
			// Spawn and fade particle trails
			particles::ParticlesPlugin,
			// Spawns popup menus and buttons such as level select
			popup::PopupPlugin,
			// Reactor sprite spawning and logic
			reactor::ReactorPlugin,
			// Camera spawn, save file loading, and resource initialization
			setup::SetupPlugin,
			// Handles screen transistion events
			states::StatesPlugin,
		))
		;

	{
		// Only include in debug builds
		#[cfg(debug_assertions)]
		app
			// Debug module for dev tools
			.add_plugins(debug::DebugPlugin)
		;
	}

	app.run();
}