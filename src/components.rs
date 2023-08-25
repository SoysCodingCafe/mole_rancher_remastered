// Import Bevy game engine essentials
use bevy::{prelude::*, time::Stopwatch};
// Import Kira audio for Bevy to handle loading sound files
use bevy_kira_audio::AudioInstance;
// Import serde for serializing and deserializing
// data for save files
use serde::{Serialize, Deserialize};

// CONTENTS
// - Save Data
// - Constants
// - States
// - Enums
// - Structs
// - System Sets
// - Components
// - Resources
// - Events
// - Audio Helper Functions
// - Molecule Helper Functions
// - Reactor Helper Functions
// - Text Styles
// - Cutscene Helper Functions

// SAVE DATA
#[derive(Serialize, Deserialize)]
pub struct SaveData {
	pub sfx_volume: f64,
	pub bgm_volume: f64,
	pub selected_palette: usize,
	pub levels_unlocked: Vec<bool>,
	pub best_times: Vec<f32>,
	pub cutscenes_unlocked: Vec<bool>,
}


// CONSTANTS
// Window Resolution
pub const ASPECT_RATIO: f32 = 16.0 / 9.0;
pub const ORTHO_HEIGHT: f32 = 900.0;
pub const ORTHO_WIDTH: f32 = ORTHO_HEIGHT * ASPECT_RATIO;

// Boot
pub const BOOT_DURATION: f32 = 2.0;

// Cutscene
pub const TEXT_BOX_WIDTH: f32 = 1200.0;
pub const TEXT_BOX_HEIGHT: f32 = 300.0;
pub const TEXT_BOX_MARGINS: f32 = 25.0;

pub const PORTRAIT_WIDTH: f32 = 300.0;
pub const PORTRAIT_HEIGHT: f32 = 300.0;

pub const ACTOR_WIDTH: f32 = 600.0;
pub const ACTOR_HEIGHT: f32 = 900.0;

pub const TEXT_SPEED: f32 = 0.01;
pub const FADE_ACTOR_SPEED: f32 = 6.0;

// Reactor Visuals
pub const REACTOR_VIEWPORT_HEIGHT: f32 = 576.0;
pub const REACTOR_VIEWPORT_WIDTH: f32 = REACTOR_VIEWPORT_HEIGHT * ASPECT_RATIO;
pub const REACTOR_VIEWPORT_X: f32 = 400.0;
pub const REACTOR_VIEWPORT_Y: f32 = 124.0;
pub const REACTOR_VIEWPORT_CENTER: Vec2 = Vec2::new(
	-(ORTHO_WIDTH / 2.0 - (REACTOR_VIEWPORT_X + REACTOR_VIEWPORT_WIDTH / 2.0)),
	ORTHO_HEIGHT / 2.0 - (REACTOR_VIEWPORT_Y + REACTOR_VIEWPORT_HEIGHT / 2.0), 
);

pub const MAX_ZOOM: f32 = 0.5;
pub const MIN_ZOOM: f32 = 10.0;
pub const ZOOM_SPEED: f32 = 15.0;
pub const ZOOM_TRANSLATION_SPEED: f32 = 60.0;
pub const ZOOM_DEAD_ZONE_RADIUS: f32 = 180.0;

pub const TOOLTIP_WIDTH: f32 = 320.0*1.5;
pub const TOOLTIP_HEIGHT: f32 = 265.0*1.5;

pub const STOPWATCH_BOX_WIDTH: f32 = 300.0;
pub const STOPWATCH_BOX_HEIGHT: f32 = 80.0;
pub const STOPWATCH_BOX_MARGINS: f32 = 8.0;

pub const PARTICLE_SPAWN_DELAY: f32 = 0.01;
pub const PARTICLE_DURATION: f32 = 0.6;

// Reactor Interactions
pub const LEVER_WIDTH: f32 = 160.0;
pub const LEVER_HEIGHT: f32 = 40.0;

pub const CONNECTION_WIDTH: f32 = 256.0;
pub const CONNECTION_HEIGHT: f32 = 128.0;

pub const LAUNCH_TUBE_WIDTH: f32 = 128.0;
pub const LAUNCH_TUBE_HEIGHT: f32 = 256.0;
pub const LAUNCH_TUBE_SPEED: f32 = 2.0;
pub const LAUNCH_TUBE_ROTATIONAL_SPEED: f32 = 300.0;


// General Parameters
pub const TOTAL_MOLECULE_TYPES: usize = 18;
pub const LAUNCH_COOLDOWN: f32 = 0.2;
pub const MOLECULE_CAP: usize = 800;

pub const FADE_TRANSITION_DURATION: f32 = 0.2;
pub const POPUP_EXPAND_TIME: f32 = 0.5;
pub const WIN_COUNTDOWN_LENGTH: f32 = 3.0;

pub const NUMBER_OF_LEVELS: usize = 30;
pub const NUMBER_OF_CUTSCENES: usize = 32;


// STATES
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
	#[default]
	Boot,
	Menu,
	Cutscene,
	Lab,
	Reactor,
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum PauseState {
	#[default]
	Unpaused,
	Paused,
}


// ENUMS
#[derive(Eq, PartialEq, Default)]
pub enum FadeScreenState {
	#[default]
	Idle,
	Closing,
	Opening,
}

#[derive(Eq, PartialEq, Default)]
pub enum CutsceneState {
	#[default]
	Initialize,
	Started,
	Ended,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum WinCondition {
	GreaterThan(usize, usize),
	LessThan(usize, usize),
}

#[derive(Clone, Copy, Debug)]
pub enum PopupType {
	Settings,
	Logbook,
	LevelSelect,
	WinScreen(f32, f32),
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum MenuButton {
	StartGame,
	Settings,
	ExitGame,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum CustomLabButton {
	MonitorActivate,
	LogbookOpen,
	ExitLab,
	Poster,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum PopupButton {
	BgmVolume(usize),
	SfxVolume(usize),
	PaletteToggle,
	LogbookPage(usize),
	LevelSelect(usize),
	ReplayLevel,
	CompleteLevel,
	ExitPopup,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum ReactorButton {
	SelectMolecule(usize),
	ExitReactor,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum CutsceneButton {
	SkipCutscene,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug, Default)]
pub enum Actor {
	#[default]
	Nobody,
	You,
	Guard,
	Scientist,
}

#[derive(Clone, Copy)]
pub enum ReactorType {
	Rectangle{
		origin: Vec2,
		dimensions: Dimensions,
	},
	Circle{
		origin: Vec2,
		radius: f32,
	},
}

pub enum ReactionInfo {
	Reaction(Vec<usize>, Limits, Limits),
	None,
}

pub enum Lifetime {
	Unstable(Timer, ReactionInfo),
	Stable,
}


// STRUCTS
#[derive(Clone, Copy)]
pub struct Dimensions {
	pub width: f32,
	pub height: f32,
}

pub struct Limits(pub f32, pub f32);

// SYSTEM SETS


// COMPONENTS
#[derive(Component)]
pub struct StandardButton {
	pub location: Vec3,
	pub dimensions: Dimensions,
	pub enabled: bool,
}

#[derive(Component)]
pub struct MoleculeButton(pub usize);

#[derive(Component, PartialEq, Eq, Clone, Copy, Debug)]
pub enum ButtonEffect {
	MenuButton(MenuButton),
	CustomLabButton(CustomLabButton),
	PopupButton(PopupButton),
	ReactorButton(ReactorButton),
	CutsceneButton(CutsceneButton),
}

#[derive(Component)]
pub struct ReactorConnections(pub Vec<(Vec2, Connection)>);

#[derive(Component)]
pub struct Connection {
	pub reactor_id: usize, 
	pub connection_id: usize,
	pub intake: bool,
	pub filter: [bool; TOTAL_MOLECULE_TYPES],
}

#[derive(Component, Clone, Copy)]
pub struct ReactorInfo {
	pub input_chamber: bool,
	pub product_chamber: bool,
	pub reactor_type: ReactorType,
	pub reactor_id: usize,
}

#[derive(Component)]
pub struct Molecule(pub Lifetime);

#[derive(Component, Clone, Copy)]
pub struct MoleculeInfo {
	pub index: usize,
	pub reacted: bool,
	pub radius: f32,
	pub mass: f32,
}

#[derive(Component)]
pub struct ParticleTrail {
	pub spawn_timer: Timer,
	pub duration: f32,
}

#[derive(Component)]
pub struct Particle {
	pub duration: Timer,
}

#[derive(Component, Clone, Copy)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct SelectedLever;

#[derive(Component)]
pub struct LeverInfo {
	pub lever_type: usize,
	pub min_height: f32,
	pub max_height: f32,
}

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct ReactorCamera;

#[derive(Component)]
pub struct SelectedMolecule;

#[derive(Component)]
pub struct SelectedReactor;

#[derive(Component)]
pub struct Highlight;

#[derive(Component)]
pub struct Tooltip;

#[derive(Component)]
pub struct Palette(pub usize);

#[derive(Component)]
pub struct LaunchTube{
	pub id: usize,
	pub current_rotation: f32,
	pub limits: Limits,
}

#[derive(Component, Default)]
pub struct ActorInfo {
	pub actor: Actor,
}

#[derive(Component)]
pub struct StopwatchText(pub Stopwatch);

#[derive(Component)]
pub struct CutsceneText;

#[derive(Component)]
pub struct PopupTimer(pub Timer);

#[derive(Component)]
pub struct FadeScreen;

#[derive(Component)]
pub struct BrightLab;

#[derive(Component)]
pub struct Logbook(pub usize);

#[derive(Component)]
pub struct PopupInfo{
	pub origin: Vec2,
	pub full_size: bool,
	pub popup_type: PopupType,
}

#[derive(Component)]
pub struct MoleculeSpawnerInfo{
	pub spawner_index: usize,
	pub spawner_timer: Timer,
}

#[derive(Component)]
pub struct ReactorCondition{
	pub temperature: f32,
	pub pressure: f32,
}

#[derive(Component)]
pub struct AnimationTimer(pub Timer);

#[derive(Component)]
pub struct AnimationIndices {
	pub first: usize,
	pub total: usize,
}

#[derive(Component)]
pub struct DespawnOnExitGameState;

#[derive(Component)]
pub struct DespawnOnExitPauseState;


// RESOURCES
// Defines the internal resolution used by sprites 
// before scaling to window size
#[derive(Resource)]
pub struct OrthoSize {
	pub width: f32,
	pub height: f32,
}

#[derive(Resource)]
pub struct AudioVolume {
	pub bgm: f64,
	pub sfx: f64,
}

#[derive(Resource)]
pub struct AudioHandles(pub Vec<(Handle<AudioInstance>, f64)>);

#[derive(Resource)]
pub struct MoleculeCount {
	pub total: usize,
	pub cap: usize,
}

#[derive(Resource)]
pub struct BootTimer(pub Timer);

#[derive(Resource)]
pub struct LaunchTimer(pub Timer);

#[derive(Resource)]
pub struct WinCountdown(pub Timer);

#[derive(Resource)]
pub struct FadeTransitionTimer(pub Timer);

#[derive(Resource)]
pub struct TextSpeedTimer(pub Timer);

#[derive(Resource, Deref, DerefMut)]
pub struct SelectedPalette(pub usize);

#[derive(Resource, Deref, DerefMut)]
pub struct SelectedLogbookPage(pub usize);

#[derive(Resource, Deref, DerefMut)]
pub struct SelectedLevel(pub usize);

#[derive(Resource, Deref, DerefMut)]
pub struct SelectedMoleculeType(pub usize);

#[derive(Resource)]
pub struct CutsceneTracker {
	pub current_scene: usize,
	pub current_line: usize,
	pub current_character: usize,
	pub full_line: String,
	pub actor_info: ActorInfo,
	pub cutscene_state: CutsceneState,
}


// EVENTS
#[derive(Event)]
pub struct ButtonCall(pub ButtonEffect);

#[derive(Event)]
pub struct FadeTransitionEvent(pub GameState);

#[derive(Event)]
pub struct ReplayLevelEvent;

#[derive(Event)]
pub struct PopupEvent{
	pub origin: Vec2,
	pub image: Handle<Image>,
	pub alpha: f32,
	pub popup_type: PopupType,
}

#[derive(Event)]
pub struct PopupCompleteEvent;

#[derive(Event)]
pub struct ConnectionEvent{
	pub connection_id: usize,
	pub m_info: MoleculeInfo,
	pub r_info: ReactorInfo,
	pub velocity: Vec2,
	pub selected: bool,
}

#[derive(Event)]
pub struct SoundEffectEvent{
	pub note: usize,
	pub location: Vec2,
}


// AUDIO HELPER FUNCTIONS
pub fn get_audio_path(
	note: usize,
) -> String {
	match note {
		0 => "audio/C.ogg".to_string(),
		1 => "audio/E.ogg".to_string(),
		2 => "audio/G.ogg".to_string(),
		3 => "audio/D.ogg".to_string(),
		4 => "audio/F.ogg".to_string(),
		5 => "audio/A.ogg".to_string(),
		6 => "audio/B.ogg".to_string(),
		_ => "audio/C2.ogg".to_string(),
	}
}


// MOLECULE HELPER FUNCTIONS
pub fn get_available_molecules(
	level: usize,
) -> [bool; TOTAL_MOLECULE_TYPES] {
	let mut available_molecules = [false; TOTAL_MOLECULE_TYPES];
	match level {
		0 => {
			available_molecules[0] = true;
			available_molecules
		}
		1 | 3 => {
			available_molecules[0] = true;
			available_molecules[1] = true;
			available_molecules
		}
		2 => {
			available_molecules[5] = true;
			available_molecules
		}
		4 => {
			available_molecules[2] = true;
			available_molecules[3] = true;
			available_molecules
		}
		_ => {
			for i in 0..TOTAL_MOLECULE_TYPES {
				available_molecules[i] = true;
			};
			available_molecules
		}
	}
}

pub fn get_molecule_path(
	index: usize,
) -> String {
	match index {
		0 => "moles/smooth_triangle.png".to_string(),
		1 => "moles/cage_triangle.png".to_string(),
		2 => "moles/cage_square.png".to_string(),
		3 => "moles/spikes_dense.png".to_string(),
		4 => "moles/spikes_sparse.png".to_string(),
		5 => "moles/cage_square.png".to_string(),
		_ => "moles/smooth_triangle.png".to_string(),
	}
}

pub fn get_molecule_color(
	index: usize,
	palette: usize,
) -> Color {
	match palette {
		0 => match index {
			0 => Color::RED,
			1 => Color::BLUE,
			2 => Color::PURPLE,
			3 => Color::ORANGE,
			4 => Color::DARK_GRAY,
			5 => Color::WHITE,
			6 => Color::YELLOW_GREEN,
			_ => Color::RED,
		}
		_ => match index {
			0 => Color::RED,
			1 => Color::ORANGE,
			2 => Color::YELLOW,
			3 => Color::GREEN,
			4 => Color::BLUE,
			5 => Color::INDIGO,
			_ => Color::VIOLET,
		}
	}
}

pub fn get_molecule_radius(
	index: usize,
) -> f32 {
	match index {
		0 => 32.0,
		1 => 32.0,
		2 => 64.0,
		3 => 64.0,
		4 => 8.0,
		5 => 32.0,
		6 => 64.0,
		_ => 32.0,
	}
}

pub fn get_molecule_mass(
	index: usize,
) -> f32 {
	match index {
		0 => 10.0,
		1 => 20.0,
		2 => 30.0,
		3 => 40.0,
		4 => 10000.0,
		5 => 1000.0,
		6 => 5.0,
		_ => 100.0,
	}
}

pub fn get_molecule_initial_velocity(
	index: usize,
) -> f32 {
	match index {
		0 => 1200.0,
		1 => 1200.0,
		2 => 1600.0,
		3 => 2000.0,
		4 => 5.0,
		5 => 3000.0,
		6 => 2000.0,
		_ => 600.0,
	}
}

pub fn get_molecule_lifetime(
	index: usize,
) -> Lifetime {
	match index {
		//2 => Lifetime::Unstable(Timer::from_seconds(rand::random::<f32>() * 5.0 + 5.0, TimerMode::Once), 
		//ReactionInfo::None),
		//4 => Lifetime::Unstable(Timer::from_seconds(rand::random::<f32>() * 3.0 + 0.2, TimerMode::Once), 
		//ReactionInfo::Reaction(vec![], Limits(0.0, 0.1), Limits(0.0, 1.0))),
		5 => Lifetime::Unstable(Timer::from_seconds(rand::random::<f32>() * 0.2 + 0.8, TimerMode::Once), 
			ReactionInfo::Reaction(vec![], Limits(0.0, 1.0), Limits(0.0, 1.0))),
		6 => Lifetime::Unstable(Timer::from_seconds(rand::random::<f32>() * 0.2 + 0.8, TimerMode::Once), 
			ReactionInfo::Reaction(vec![], Limits(0.0, 1.0), Limits(0.0, 1.0))),
		_ => Lifetime::Stable,
	}
}

pub fn valid_molecule_combination(
	mol_a: usize,
	mol_b: usize,
) -> ReactionInfo {
	let (mol_a, mol_b) = (mol_a.min(mol_b), mol_a.max(mol_b));
	match mol_a {
		0 => match mol_b {
			1 => ReactionInfo::Reaction(vec![2], Limits(0.0, 1.0), Limits(0.0, 1.0)),
			3 => ReactionInfo::Reaction(vec![5, 5, 5, 5, 5], Limits(0.0, 1.0), Limits(0.0, 1.0)),
			5 => ReactionInfo::Reaction(vec![5], Limits(0.0, 1.0), Limits(0.0, 1.0)),
			_ => ReactionInfo::None,
		},
		1 => match mol_b {
			3 => ReactionInfo::Reaction(vec![5, 5, 5, 5, 5], Limits(0.0, 1.0), Limits(0.0, 1.0)),
			5 => ReactionInfo::Reaction(vec![5], Limits(0.0, 1.0), Limits(0.0, 1.0)),
			_ => ReactionInfo::None,
		},
		2 => match mol_b {
			3 => ReactionInfo::Reaction(vec![4], Limits(0.0, 1.0), Limits(0.0, 1.0)),
			5 => ReactionInfo::Reaction(vec![5], Limits(0.0, 1.0), Limits(0.0, 1.0)),
			_ => ReactionInfo::None,
		},
		3 => match mol_b {
			5 => ReactionInfo::Reaction(vec![5], Limits(0.0, 1.0), Limits(0.0, 1.0)),
			_ => ReactionInfo::None,
		},
		4 => match mol_b {
			5 => ReactionInfo::Reaction(vec![5], Limits(0.0, 1.0), Limits(0.0, 1.0)),
			_ => ReactionInfo::None,
		},
		5 => match mol_b {
			_ => ReactionInfo::None,
		},
		6 => match mol_b {
			_ => ReactionInfo::None,
		},
		_ => ReactionInfo::None,
	}
}


// REACTOR HELPER FUNCTIONS
pub fn get_reactors(
	level: usize,
) -> Vec<ReactorInfo> {
	let mut reactors = Vec::new();
	match level {
		0 => {
			reactors.push(ReactorInfo{reactor_type: ReactorType::Rectangle{origin: Vec2::new(0.0, 0.0), dimensions: Dimensions{width: 3000.0, height: 2000.0}}, reactor_id: 0, input_chamber: true, product_chamber: true});
		}
		1 => {
			reactors.push(ReactorInfo{reactor_type: ReactorType::Circle{origin: Vec2::new(0.0, 0.0), radius: 2000.0}, reactor_id: 0, input_chamber: true, product_chamber: true});
		}
		2 => {
			reactors.push(ReactorInfo{reactor_type: ReactorType::Circle{origin: Vec2::new(0.0, 0.0), radius: 800.0}, reactor_id: 0, input_chamber: true, product_chamber: true});
		}
		3 => {
			reactors.push(ReactorInfo{reactor_type: ReactorType::Rectangle{origin: Vec2::new(0.0, 2000.0), dimensions: Dimensions{width: 4000.0, height: 2000.0}}, reactor_id: 0, input_chamber: true, product_chamber: false});
			reactors.push(ReactorInfo{reactor_type: ReactorType::Circle{origin: Vec2::new(0.0, -1500.0), radius: 2000.0}, reactor_id: 1, input_chamber: false, product_chamber: true});
		}
		4 => {
			reactors.push(ReactorInfo{reactor_type: ReactorType::Circle{origin: Vec2::new(-4500.0, 0.0), radius: 2000.0}, reactor_id: 0, input_chamber: true, product_chamber: false});
			reactors.push(ReactorInfo{reactor_type: ReactorType::Rectangle{origin: Vec2::new(0.0, 0.0), dimensions: Dimensions{width: 4000.0, height: 1000.0}}, reactor_id: 1, input_chamber: false, product_chamber: true});
			reactors.push(ReactorInfo{reactor_type: ReactorType::Circle{origin: Vec2::new(4500.0, 0.0), radius: 2000.0}, reactor_id: 2, input_chamber: true, product_chamber: false});
		}
		5 => {
			reactors.push(ReactorInfo{reactor_type: ReactorType::Rectangle{origin: Vec2::new(-3000.0, 100.0), dimensions: Dimensions{width: 3000.0, height: 3000.0}}, reactor_id: 0, input_chamber: true, product_chamber: false});
			reactors.push(ReactorInfo{reactor_type: ReactorType::Rectangle{origin: Vec2::new(3000.0, -2500.0), dimensions: Dimensions{width: 3000.0, height: 2000.0}}, reactor_id: 1, input_chamber: false, product_chamber: true});
			reactors.push(ReactorInfo{reactor_type: ReactorType::Circle{origin: Vec2::new(4000.0, 2000.0), radius: 2200.0}, reactor_id: 2, input_chamber: true, product_chamber: false});
		},
		6 => {reactors.push(ReactorInfo{reactor_type: ReactorType::Circle{origin: Vec2::new(0.0, 0.0), radius: 4500.0}, reactor_id: 0, input_chamber: true, product_chamber: true});}
		7 => {
			for j in 0..4 {
				for i in 0..8 {
					reactors.push(ReactorInfo{reactor_type: ReactorType::Circle{origin: Vec2::new(-6844.0 + 1955.5*i as f32, 3180.0 - 2120.0*j as f32), radius: 800.0}, reactor_id: i + 8*j, 
					input_chamber: if i == 0 && j == 0 {true} else {false}, product_chamber: if i == 7 && j == 3 {true} else {false}});
				}
			}
		}
		8 => {
			reactors.push(ReactorInfo{reactor_type: ReactorType::Circle{origin: Vec2::new(-2000.0, 0.0), radius: 800.0}, reactor_id: 0, input_chamber: true, product_chamber: false});
			reactors.push(ReactorInfo{reactor_type: ReactorType::Circle{origin: Vec2::new(0.0, 0.0), radius: 800.0}, reactor_id: 1, input_chamber: true, product_chamber: false});
			reactors.push(ReactorInfo{reactor_type: ReactorType::Circle{origin: Vec2::new(2000.0, 0.0), radius: 800.0}, reactor_id: 2, input_chamber: true, product_chamber: true});
			reactors.push(ReactorInfo{reactor_type: ReactorType::Rectangle{origin: Vec2::new(-2000.0, -3000.0), dimensions: Dimensions{width: 800.0, height: 800.0}}, reactor_id: 3, input_chamber: true, product_chamber: false});
			reactors.push(ReactorInfo{reactor_type: ReactorType::Rectangle{origin: Vec2::new(0.0, -3000.0), dimensions: Dimensions{width: 800.0, height: 2000.0}}, reactor_id: 4, input_chamber: true, product_chamber: false});
			reactors.push(ReactorInfo{reactor_type: ReactorType::Rectangle{origin: Vec2::new(2000.0, -3000.0), dimensions: Dimensions{width: 800.0, height: 3000.0}}, reactor_id: 5, input_chamber: true, product_chamber: false});
		}
		9 => {
			{reactors.push(ReactorInfo{reactor_type: ReactorType::Circle{origin: Vec2::new(0.0, 0.0), radius: 4500.0}, reactor_id: 0, input_chamber: true, product_chamber: true});}
		}
		_ => (),
	}
	reactors
}

pub fn get_reactor_connections(
	level: usize,
	reactor_id: usize,
) -> ReactorConnections {
	let mut filter = [false; TOTAL_MOLECULE_TYPES];
	let (mut filter_a, mut filter_b, mut filter_c, mut filter_d) = (filter, filter, filter, filter); 
	filter_a[0] = true;
	filter_b[1] = true;
	filter_c[2] = true;
	filter_d[3] = true;
	let mut connections = Vec::new();
	match level {
		3 => match reactor_id {
			0 => {
				connections.push((Vec2::new(-0.8, -1.0), Connection{reactor_id: reactor_id, connection_id: 0, intake: true, filter: filter_a}));
				connections.push((Vec2::new(0.8, -1.0), Connection{reactor_id: reactor_id, connection_id: 1, intake: true, filter: filter_b}));
			},
			1 => {
				connections.push((Vec2::new(0.0, 1.0), Connection{reactor_id: reactor_id, connection_id: 0, intake: false, filter: filter}));
				connections.push((Vec2::new(0.0, 1.0), Connection{reactor_id: reactor_id, connection_id: 1, intake: false, filter: filter}));
			}
			_ => (),
		}
		4 => match reactor_id {
			0 => {
				connections.push((Vec2::new(1.0, 0.0), Connection{reactor_id: reactor_id, connection_id: 2, intake: true, filter: filter_c}));
			},
			1 => {
				connections.push((Vec2::new(-1.0, 0.0), Connection{reactor_id: reactor_id, connection_id: 2, intake: false, filter: filter}));
				connections.push((Vec2::new(1.0, 0.0), Connection{reactor_id: reactor_id, connection_id: 3, intake: false, filter: filter}));
			},
			2 => {
				connections.push((Vec2::new(-1.0, 0.0), Connection{reactor_id: reactor_id, connection_id: 3, intake: true, filter: filter_d}));
			},
			_ => (),
		}
		5 => match reactor_id {
			0 => {
				connections.push((Vec2::new(-1.0, 0.0), Connection{reactor_id: reactor_id, connection_id: 1, intake: true, filter: filter}));
				connections.push((Vec2::new(1.0, 0.0), Connection{reactor_id: reactor_id, connection_id: 1, intake: false, filter: filter}));
				connections.push((Vec2::new(0.0, -1.0), Connection{reactor_id: reactor_id, connection_id: 0, intake: false, filter: filter}));
			},
			1 => {
				connections.push((Vec2::new(0.0, 1.0), Connection{reactor_id: reactor_id, connection_id: 1, intake: false, filter: filter}));
			}
			2 => {
				filter[0] = false;
				connections.push((Vec2::new(1.0, 1.0), Connection{reactor_id: reactor_id, connection_id: 0, intake: true, filter: filter}));
			}
			_ => (),
		}
		6 => match reactor_id {
			_ => {
				connections.push((Vec2::new(0.0, -1.0), Connection{reactor_id: reactor_id, connection_id: 0, intake: true, filter: filter}));
				connections.push((Vec2::new(1.0, 1.0), Connection{reactor_id: reactor_id, connection_id: 0, intake: false, filter: filter}));
				connections.push((Vec2::new(-1.0, -1.0), Connection{reactor_id: reactor_id, connection_id: 1, intake: true, filter: filter}));
				connections.push((Vec2::new(-1.0, 1.0), Connection{reactor_id: reactor_id, connection_id: 1, intake: false, filter: filter}));
				connections.push((Vec2::new(1.0, -1.0), Connection{reactor_id: reactor_id, connection_id: 2, intake: true, filter: filter}));
				connections.push((Vec2::new(-1.0, 0.0), Connection{reactor_id: reactor_id, connection_id: 2, intake: false, filter: filter}));
				connections.push((Vec2::new(1.0, 0.0), Connection{reactor_id: reactor_id, connection_id: 3, intake: true, filter: filter}));
				connections.push((Vec2::new(0.0, 1.0), Connection{reactor_id: reactor_id, connection_id: 3, intake: false, filter: filter}));
			}
		}
		7 => match reactor_id {
			i => {
				connections.push((Vec2::new(0.0, -1.0), Connection{reactor_id: reactor_id, connection_id: (i+1)%32, intake: true, filter: filter}));
				connections.push((Vec2::new(0.0, 1.0), Connection{reactor_id: reactor_id, connection_id: i, intake: false, filter: filter}));
			}
		}
		_ => (),
	}
	ReactorConnections(connections)
}

pub fn get_reactor_initialization(
	level: usize,
	reactor_id: usize,
) -> Vec<(usize, Vec2, Vec2)> {
	let mut molecules = Vec::new();
	match level {
		0 => match reactor_id {
			0 => {
				for i in 0..5 {
					molecules.push((1, Vec2::new(-1000.0 + 500.0 * i as f32, 0.0), Vec2::ZERO));
				}
				molecules
			},
			_ => molecules,
		}
		1 => match reactor_id {
			0 => {
				for _ in 0..50 {
					molecules.push((4, Vec2::new((rand::random::<f32>() - 0.5) * 3000.0, (rand::random::<f32>() - 0.5) * 3000.0), Vec2::ZERO));
				}
				molecules
			},
			_ => molecules,
		}
		2 => match reactor_id {
			0 => {
				for _ in 0..100 {
					molecules.push((0, Vec2::new((rand::random::<f32>() - 0.5) * 1500.0, (rand::random::<f32>() - 0.5) * 1500.0), 
					Vec2::new((rand::random::<f32>() - 0.5) * 3000.0, (rand::random::<f32>() - 0.5) * 3000.0)));
				}
				molecules
			},
			_ => molecules,
		}
		_ => {
			molecules
		}
	}
}

pub fn get_level_goal(
	level: usize,
) -> WinCondition {
	match level {
		0 => WinCondition::GreaterThan(5, 2),
		1 => WinCondition::GreaterThan(5, 2),
		2 => WinCondition::LessThan(1, 0),
		3 => WinCondition::GreaterThan(5, 2),
		4 => WinCondition::GreaterThan(5, 4),
		_ => WinCondition::GreaterThan(1, 0),
	}
}

// For rectangles, limits between 0.0 and 1.0 represent centre to edge
// For circles, limits between 0.0 and 1.0 represent top, going anticlockwise, back to the top
pub fn get_launch_tube_limits(
	level: usize,
	reactor_id: usize,
) -> Limits {
	match level {
		4 => match reactor_id {
			0 => Limits(0.625, 0.875),
			2 => Limits(0.125, 0.375),
			_ => Limits(1.0, 1.0),
		}
		_ => match reactor_id {
			_ => Limits(1.0, 1.0),
		},
	}
}


// TEXT STYLES
pub fn get_title_text_style(
	asset_server: &Res<AssetServer>
) -> TextStyle {
	TextStyle {
		font: asset_server.load("fonts/Ronda.ttf"),
		font_size: 120.0,
		color: Color::hex("EDD6AD").unwrap(),
	}
}

pub fn get_subtitle_text_style(
	asset_server: &Res<AssetServer>
) -> TextStyle {
	TextStyle {
		font: asset_server.load("fonts/Ronda.ttf"),
		font_size: 60.0,
		color: Color::hex("EDD6AD").unwrap(),
	}
}

pub fn get_settings_text_style(
	asset_server: &Res<AssetServer>
) -> TextStyle {
	TextStyle {
		font: asset_server.load("fonts/PixelSplitter-Bold.ttf"),
		font_size: 60.0,
		color: Color::rgba(0.9, 0.9, 0.9, 1.0),
		..Default::default()
	}
}

pub fn get_cutscene_text_style(
	asset_server: &Res<AssetServer>
) -> TextStyle {
	TextStyle {
		font: asset_server.load("fonts/PixelSplitter-Bold.ttf"),
		font_size: 32.0,
		color: Color::rgba(0.1, 0.1, 0.1, 1.0),
		..Default::default()
	}
}

pub fn get_stopwatch_text_style(
	asset_server: &Res<AssetServer>
) -> TextStyle {
	TextStyle {
		font: asset_server.load("fonts/PixelSplitter-Bold.ttf"),
		font_size: 64.0,
		color: Color::rgba(0.1, 0.3, 0.1, 1.0),
		..Default::default()
	}
}


// CUTSCENE HELPER FUNCTIONS
pub fn next_line(
	current_scene: usize,
	current_line: usize,
) -> (String, ActorInfo) {
	match current_scene {
		0 => match current_line {
			0 =>
			("DAY 1 - OUTSIDE MAIN ENTRANCE".to_string(),
			ActorInfo{actor: Actor::Nobody}),
			1 =>
			("New text! This text is very long for debug purposes! It also covers multiple lines! Wow! So cool! MMMMMMMMMMMMMMMMMMMM MMMMMMMMMMMMM MMMMMM MMMMMM M MMMMM MMMMM MMM MMMMMMMMMMMMMMM IIIIIIIIIIII IIII IIIII IIIIIIII IIIII IIIIIIIIIII IIIIIIIIIIIIIIIII IIIIIIIIIIIIII IIII Fini".to_string(),
			ActorInfo{actor: Actor::Scientist}),
			_ =>
			("I am a guard! Do do doooo!".to_string(),
			ActorInfo{actor: Actor::Guard}),
		},
		1 => match current_line {
			0 =>
			("DAY 2 - OUTSIDE MAIN ENTRANCE".to_string(),
			ActorInfo{actor: Actor::Nobody}),
			1 =>
			("So you are the scraps they have tossed me.".to_string(),
			ActorInfo{actor: Actor::Guard}),
			2 =>
			("You must get inside quickly. Your predecessor is already starting to decompose, and the morgue is full.".to_string(),
			ActorInfo{actor: Actor::Guard}),
			3 =>
			("Producing an acid to dispose of the body is so simple I could do it, but then who would guard the door?".to_string(),
			ActorInfo{actor: Actor::Guard}),
			4 =>
			("The computer is already logged in, and the logbook is now yours, assuming you can read.".to_string(),
			ActorInfo{actor: Actor::Guard}),
			_ =>
			("Now get to work.".to_string(),
			ActorInfo{actor: Actor::Guard}),
		},
		_ => match current_line {
			0 =>
			("DAY ? - PLEASE REPORT THIS".to_string(),
			ActorInfo{actor: Actor::Nobody}),
			_ =>
			("You have reached an unreachable cutscene, interesting. This should not have happened".to_string(),
			ActorInfo{actor: Actor::You}),
		}
	}
}

pub fn lines_per_scene(
	current_scene: usize,
) -> usize {
	match current_scene {
		0 => 2,
		1 => 5,
		_ => 1,
	}
}

pub fn actors_in_scene(
	current_scene: usize,
) -> Vec<Actor> {
	match current_scene {
		0 => vec![Actor::Scientist, Actor::Guard],
		1 => vec![Actor::Guard],
		_ => vec![Actor::You],
	}
}

pub fn get_actor_path(
	actor: Actor,
) -> String {
	match actor {
		Actor::Nobody => "".to_string(),
		Actor::You => "".to_string(),
		Actor::Guard => "actors/guard.png".to_string(),
		Actor::Scientist => "actors/scientist.png".to_string(),
	}
}

pub fn get_actor_name(
	actor: Actor,
) -> String {
	match actor {
		Actor::Nobody => "Nobody".to_string(),
		Actor::You => "You".to_string(),
		Actor::Guard => "Guard".to_string(),
		Actor::Scientist => "Scientist".to_string(),
	}
}
