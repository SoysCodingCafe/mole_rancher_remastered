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
	pub particles_enabled: bool,
	pub levels_unlocked: Vec<bool>,
	pub best_times: Vec<f32>,
	pub best_costs: Vec<usize>,
	pub cutscenes_unlocked: Vec<bool>,
}


// CONSTANTS
// Window Resolution
pub const ASPECT_RATIO: f32 = 16.0 / 9.0;
pub const ORTHO_HEIGHT: f32 = 900.0;
pub const ORTHO_WIDTH: f32 = ORTHO_HEIGHT * ASPECT_RATIO;

// Boot
pub const BOOT_DURATION: f32 = 10.0;

// Cutscene
pub const TEXT_BOX_WIDTH: f32 = 1200.0;
pub const TEXT_BOX_HEIGHT: f32 = 300.0;
pub const TEXT_BOX_MARGINS: f32 = 25.0;

pub const PORTRAIT_WIDTH: f32 = 300.0;
pub const PORTRAIT_HEIGHT: f32 = 300.0;

pub const ACTOR_WIDTH: f32 = 450.0;
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
pub const ZOOM_SPEED: f32 = 0.15;
pub const ZOOM_TRANSLATION_SPEED: f32 = 80.0;
pub const ZOOM_DEAD_ZONE_RADIUS: f32 = 180.0;

pub const TOOLTIP_MARGINS: f32 = 32.0;
pub const TOOLTIP_WIDTH: f32 = 320.0*1.5;
pub const TOOLTIP_HEIGHT: f32 = 265.0*1.5;

pub const REACTION_UI_SPACING: f32 = 12.0;

pub const STOPWATCH_BOX_Y: f32 = 390.0;
pub const STOPWATCH_BOX_WIDTH: f32 = 250.0;
pub const STOPWATCH_BOX_HEIGHT: f32 = 100.0;
pub const STOPWATCH_BOX_MARGINS: f32 = 8.0;

pub const COST_BOX_Y: f32 = 390.0;
pub const COST_BOX_WIDTH: f32 = 270.0;
pub const COST_BOX_HEIGHT: f32 = 100.0;
pub const COST_BOX_MARGINS: f32 = 8.0;

pub const GOAL_BOX_Y: f32 = 390.0;
pub const GOAL_BOX_WIDTH: f32 = 480.0;
pub const GOAL_BOX_HEIGHT: f32 = 100.0;
pub const GOAL_BOX_MARGINS: f32 = 8.0;

pub const PARTICLE_SPAWN_DELAY: f32 = 0.01;
pub const PARTICLE_DURATION: f32 = 0.6;

// Reactor Interactions
pub const LEVER_WIDTH: f32 = 160.0;
pub const LEVER_HEIGHT: f32 = 40.0;

pub const CONNECTION_WIDTH: f32 = 128.0;
pub const CONNECTION_HEIGHT: f32 = 256.0;

pub const LAUNCH_TUBE_WIDTH: f32 = 128.0;
pub const LAUNCH_TUBE_HEIGHT: f32 = 256.0;
pub const LAUNCH_TUBE_SPEED: f32 = 1.0;
pub const LAUNCH_TUBE_ROTATIONAL_SPEED: f32 = 150.0;


// General Parameters
pub const TOTAL_MOLECULE_TYPES: usize = 18;
pub const LAUNCH_COOLDOWN: f32 = 0.2;
pub const MOLECULE_CAP: usize = 800;

pub const FRICTION: f32 = 0.05;

pub const POPUP_EXPAND_TIME: f32 = 0.5;
pub const POPUP_WIDTH: f32 = 1440.0;
pub const POPUP_HEIGHT: f32 = 810.0;
pub const LOGBOOK_MARGINS: f32 = 80.0;

pub const FADE_TRANSITION_DURATION: f32 = 0.2;
pub const WIN_COUNTDOWN_LENGTH: f32 = 3.0;

pub const NUMBER_OF_LEVELS: usize = 31;
pub const NUMBER_OF_CUTSCENES: usize = 33;


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
	LevelIntro(usize),
	WinScreen(f32, f32, usize, usize),
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
	ParticleTrails(bool),
	LogbookPage(usize),
	LevelSelect(usize),
	ReturnToLab,
	ReplayLevel,
	CompleteLevel,
	ExitPopup,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum ReactorButton {
	SelectMolecule(usize),
	RestartLevel,
	PauseLevel,
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
	pub idle_color: Color,
	pub hovered_color: Color,
	pub disabled_color: Color,
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
pub struct WinCountdownText;

#[derive(Component)]
pub struct LogbookText(pub usize);

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
pub struct CostText;

#[derive(Component)]
pub struct TooltipText;

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
pub struct BgmHandle(pub Handle<AudioInstance>);

#[derive(Resource)]
pub struct SfxHandles(pub Vec<(Handle<AudioInstance>, f64)>);

#[derive(Resource)]
pub struct MoleculeCount {
	pub total: usize,
	pub cap: usize,
}

#[derive(Resource)]
pub struct CurrentCost(pub usize);

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
		5 => {
			available_molecules[0] = true;
			available_molecules[1] = true;
			available_molecules[3] = true;
			available_molecules[6] = true;
			available_molecules
		}
		_ => {
			for i in 0..7 {
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
		1 => "moles/spikes_sparse.png".to_string(),
		2 => "moles/spikes_dense.png".to_string(),
		3 => "moles/cage_triangle.png".to_string(),
		4 => "moles/smooth_triangle.png".to_string(),
		5 => "moles/cage_square.png".to_string(),
		6 => "moles/smooth_triangle.png".to_string(),
		7 => "moles/spikes_sparse.png".to_string(),
		8 => "moles/spikes_dense.png".to_string(),
		9 => "moles/cage_triangle.png".to_string(),
		10 => "moles/smooth_triangle.png".to_string(),
		11 => "moles/cage_square.png".to_string(),
		12 => "moles/spikes_dense.png".to_string(),
		13 => "moles/cage_triangle.png".to_string(),
		14 => "moles/smooth_triangle.png".to_string(),
		15 => "moles/spikes_dense.png".to_string(),
		16 => "moles/smooth_triangle.png".to_string(),
		17 => "moles/spikes_dense.png".to_string(),
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
			2 => Color::GREEN,
			3 => Color::WHITE,
			4 => Color::DARK_GRAY,
			5 => Color::ORANGE,
			6 => Color::YELLOW,
			7 => Color::RED,
			8 => Color::BLUE,
			9 => Color::GREEN,
			10 => Color::WHITE,
			11 => Color::DARK_GRAY,
			12 => Color::ORANGE,
			13 => Color::YELLOW,
			14 => Color::RED,
			15 => Color::BLUE,
			16 => Color::GREEN,
			17 => Color::WHITE,
			_ => Color::RED,
		}
		1 => match index {
			0 => Color::RED,
			1 => Color::ORANGE,
			2 => Color::YELLOW,
			3 => Color::GREEN,
			4 => Color::BLUE,
			5 => Color::INDIGO,
			6 => Color::VIOLET,
			7 => Color::RED,
			8 => Color::ORANGE,
			9 => Color::YELLOW,
			10 => Color::GREEN,
			11 => Color::BLUE,
			12 => Color::INDIGO,
			13 => Color::VIOLET,
			14 => Color::RED,
			15 => Color::ORANGE,
			16 => Color::YELLOW,
			17 => Color::GREEN,
			_ => Color::BLUE,
		}
		2 => match index {
			_ => Color::VIOLET,
		}
		_ => match index {
			0 => Color::rgb(0.8, 0.8, 0.8),
			1 => Color::rgb(0.3, 0.3, 0.3),
			2 => Color::rgb(1.0, 1.0, 1.0),
			3 => Color::rgb(0.5, 0.5, 0.5),
			4 => Color::rgb(0.0, 0.0, 0.0),
			5 => Color::rgb(0.7, 0.7, 0.7),
			6 => Color::rgb(0.2, 0.2, 0.2),
			7 => Color::rgb(0.6, 0.6, 0.6),
			8 => Color::rgb(0.1, 0.1, 0.1),
			9 => Color::rgb(0.9, 0.9, 0.9),
			10 => Color::rgb(0.4, 0.4, 0.4),
			11 => Color::rgb(0.6, 0.6, 0.6),
			12 => Color::rgb(0.8, 0.8, 0.8),
			13 => Color::rgb(0.3, 0.3, 0.3),
			14 => Color::rgb(1.0, 1.0, 1.0),
			15 => Color::rgb(0.5, 0.5, 0.5),
			16 => Color::rgb(0.0, 0.0, 0.0),
			17 => Color::rgb(0.7, 0.7, 0.7),
			_ => Color::rgb(0.2, 0.2, 0.2),
		}
	}
}

pub fn get_molecule_radius(
	index: usize,
) -> f32 {
	match index {
		0 => 48.0,
		1 => 64.0,
		2 => 80.0,
		3 => 16.0,
		4 => 32.0,
		5 => 56.0,
		6 => 72.0,
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

pub fn get_molecule_cost(
	index: usize,
) -> usize {
	match index {
		0 => 10,
		1 => 20,
		2 => 40,
		3 => 50,
		4 => 80,
		5 => 2,
		6 => 0,
		_ => 1,
	}
}

pub fn get_molecule_initial_velocity(
	index: usize,
) -> f32 {
	match index {
		0 => 1000.0,
		1 => 2000.0,
		2 => 1500.0,
		3 => 2500.0,
		4 => 50.0,
		5 => 3000.0,
		6 => 3000.0,
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
			reactors.push(ReactorInfo{reactor_type: ReactorType::Rectangle{origin: Vec2::new(0.0, 0.0), dimensions: Dimensions {width: 2000.0, height: 1600.0}}, reactor_id: 0, input_chamber: true, product_chamber: true});
		}
		3 => {
			reactors.push(ReactorInfo{reactor_type: ReactorType::Rectangle{origin: Vec2::new(0.0, 1000.0), dimensions: Dimensions{width: 4000.0, height: 2000.0}}, reactor_id: 0, input_chamber: true, product_chamber: false});
			reactors.push(ReactorInfo{reactor_type: ReactorType::Circle{origin: Vec2::new(0.0, -1000.0), radius: 800.0}, reactor_id: 1, input_chamber: false, product_chamber: true});
		}
		4 => {
			reactors.push(ReactorInfo{reactor_type: ReactorType::Circle{origin: Vec2::new(-4500.0, 0.0), radius: 2000.0}, reactor_id: 0, input_chamber: true, product_chamber: false});
			reactors.push(ReactorInfo{reactor_type: ReactorType::Rectangle{origin: Vec2::new(0.0, 0.0), dimensions: Dimensions{width: 4000.0, height: 600.0}}, reactor_id: 1, input_chamber: false, product_chamber: true});
			reactors.push(ReactorInfo{reactor_type: ReactorType::Circle{origin: Vec2::new(4500.0, 0.0), radius: 2000.0}, reactor_id: 2, input_chamber: true, product_chamber: false});
		}
		5 => {
			reactors.push(ReactorInfo{reactor_type: ReactorType::Circle{origin: Vec2::new(0.0, 0.0), radius: 1500.0}, reactor_id: 0, input_chamber: true, product_chamber: true});
		}
		6 => {
			{reactors.push(ReactorInfo{reactor_type: ReactorType::Circle{origin: Vec2::new(0.0, 0.0), radius: 4000.0}, reactor_id: 0, input_chamber: true, product_chamber: true});}
		}
		_ => {
			reactors.push(ReactorInfo{reactor_type: ReactorType::Rectangle{origin: Vec2::new(-3000.0, 100.0), dimensions: Dimensions{width: 3000.0, height: 3000.0}}, reactor_id: 0, input_chamber: true, product_chamber: false});
			reactors.push(ReactorInfo{reactor_type: ReactorType::Rectangle{origin: Vec2::new(3000.0, -2500.0), dimensions: Dimensions{width: 3000.0, height: 2000.0}}, reactor_id: 1, input_chamber: false, product_chamber: true});
			reactors.push(ReactorInfo{reactor_type: ReactorType::Circle{origin: Vec2::new(4000.0, 2000.0), radius: 2200.0}, reactor_id: 2, input_chamber: true, product_chamber: false});
		},
		_ => {reactors.push(ReactorInfo{reactor_type: ReactorType::Circle{origin: Vec2::new(0.0, 0.0), radius: 4500.0}, reactor_id: 0, input_chamber: true, product_chamber: true});}
		_ => {
			for j in 0..4 {
				for i in 0..8 {
					reactors.push(ReactorInfo{reactor_type: ReactorType::Circle{origin: Vec2::new(-6844.0 + 1955.5*i as f32, 3180.0 - 2120.0*j as f32), radius: 800.0}, reactor_id: i + 8*j, 
					input_chamber: if i == 0 && j == 0 {true} else {false}, product_chamber: if i == 7 && j == 3 {true} else {false}});
				}
			}
		}
		_ => {
			reactors.push(ReactorInfo{reactor_type: ReactorType::Circle{origin: Vec2::new(-2000.0, 0.0), radius: 800.0}, reactor_id: 0, input_chamber: true, product_chamber: false});
			reactors.push(ReactorInfo{reactor_type: ReactorType::Circle{origin: Vec2::new(0.0, 0.0), radius: 800.0}, reactor_id: 1, input_chamber: true, product_chamber: false});
			reactors.push(ReactorInfo{reactor_type: ReactorType::Circle{origin: Vec2::new(2000.0, 0.0), radius: 800.0}, reactor_id: 2, input_chamber: true, product_chamber: true});
			reactors.push(ReactorInfo{reactor_type: ReactorType::Rectangle{origin: Vec2::new(-2000.0, -3000.0), dimensions: Dimensions{width: 800.0, height: 800.0}}, reactor_id: 3, input_chamber: true, product_chamber: false});
			reactors.push(ReactorInfo{reactor_type: ReactorType::Rectangle{origin: Vec2::new(0.0, -3000.0), dimensions: Dimensions{width: 800.0, height: 2000.0}}, reactor_id: 4, input_chamber: true, product_chamber: false});
			reactors.push(ReactorInfo{reactor_type: ReactorType::Rectangle{origin: Vec2::new(2000.0, -3000.0), dimensions: Dimensions{width: 800.0, height: 3000.0}}, reactor_id: 5, input_chamber: true, product_chamber: false});
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
		_ => (),
		_ => match reactor_id {
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
		_ => (),
		_ => match reactor_id {
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
		_ => match reactor_id {
			i => {
				connections.push((Vec2::new(0.0, -1.0), Connection{reactor_id: reactor_id, connection_id: (i+1)%32, intake: true, filter: filter}));
				connections.push((Vec2::new(0.0, 1.0), Connection{reactor_id: reactor_id, connection_id: i, intake: false, filter: filter}));
			}
		}
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
				for j in 0..9 {
					for i in 0..9 {
						molecules.push((4, Vec2::new(-600.0 + 150.0 * i as f32, -600.0 + 150.0 * j as f32), Vec2::ZERO));
					}
				}
				molecules
			},
			_ => molecules,
		}
		2 => match reactor_id {
			0 => {
				for i in 0..18 {
					molecules.push((0, Vec2::new(-800.0 + 100.0 * i as f32, 0.0), 
					Vec2::new(0.0, if i % 2 == 0 {300.0} else {-300.0})));
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
		5 => WinCondition::GreaterThan(15, 4),
		6 => WinCondition::GreaterThan(1, 10),
		_ => WinCondition::GreaterThan(1, 0),
	}
}

pub fn get_level_goal_text(
	level: usize,
) -> String {
	match level {
		0 => format!("Have at least 5 Comba molecules in the output chamber"),
		1 => format!("Have at least 5 Comba molecules in the output chamber"),
		2 => format!("Remove all Funda molecules from the output chamber"),
		3 => format!("Have at least 5 Comba molecules in the output chamber"),
		4 => format!("Have at least 5 Densa molecules in the output chamber"),
		5 => format!("Have at least 15 Densa molecules in the output chamber"),
		_ => format!("Have fun!"),
	}
}

pub fn get_tooltip_text(
	molecule_index: usize,
	molecule_unlocked: bool,
) -> String {
	if molecule_unlocked {
		match molecule_index {
			0 => format!("Funda is the simplest molecule, and the cheapest to produce. Reacts with Supla to produce Comba."),
			1 => format!("Supla is a common reagent used in many reactions. Can be combined with Funda to produce Comba."),
			2 => format!("Comba was the first compound discovered by ranchers, and is the first step in a long journey."),
			3 => format!("Volla reacts strongly with Funda and Supla to produce destructive Morta, however, it can be stabilized by reacting with Comba."),
			4 => format!("Densa is extremely heavy and is not easily moved by other molecules. It is also very stable, but will still be destroyed by Morta."),
			5 => format!("Morta is a fast and dangerous molecule which eradicates most other molecules. Thankfully it decays quickly, and can be useful for clearing out a reactor."),
			6 => format!("Inera is a short lived molecule, and is not known to react with any other molecules. Skilled ranchers use these to push other molecules around."),
			_ => format!("This molecule is unknown!"),
		}
	} else {
		match molecule_index {
			0 => format!("Funda, the simplest molecule. Unavailable for this level."),
			1 => format!("Supla, the most common reagent. Unavailable for this level."),
			2 => format!("Comba, the original compound. Unavailable for this level."),
			3 => format!("Volla, the unstable molecule. Unavailable for this level."),
			4 => format!("Densa, the compact compound. Unavailable for this level."),
			5 => format!("Morta, the destructive molecule. Unavailable for this level."),
			6 => format!("Inera, the harmless molecule. Unavailable for this level."),
			_ => format!("This molecule is unknown!"),
		}
	}
}

pub fn get_reactor_color(
	reactor_type: usize,
) -> Color {
	match reactor_type {
		// Input
		0 => Color::YELLOW_GREEN,
		// Product
		1 => Color::MAROON,
		// Selected
		2 => Color::rgb(0.7, 0.7, 0.7),
		// Neither
		_ => Color::GRAY,
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

pub fn get_initial_zoom(
	level: usize,
) -> f32 {
	match level {
		0 => 3.0,
		1 => 5.0,
		2 => 2.2,
		3 => 9.0,
		4 => 9.0,
		5 => 4.0,
		6 => 10.0,
		_ => 10.0,
	}
}

pub fn get_intro_text(
	level: usize,
) -> String {
	match level {
		0 => format!("Welcome to the reactor view! Select a chamber with Left Click and press Spacebar to launch molecules. You can move the launcher left and right with A and D. Try hit the molecules in the center to cause a reaction!"),
		1 => format!("You are getting the hang of this! Use Left Click to select different molecules from the menu on the left. You can hover over them for more details! Also, you can hold W to continuously fire molecules. Remember to select a chamber with Left Click!"),
		2 => format!("This chamber is filled with unwanted molecules! Use that new molecule in the menu to the left to clear them out. You can rotate the launcher using Q and E, and you move faster while holding down Shift."),
		3 => format!("This level has two chambers. You can only launch molecules in the top chamber, and the pipes connecting the chambers only accept specific molecules. You can use the Mouse Wheel to zoom, and hold Right Click to pan around for a better view."),
		4 => format!("TGIF! There are three chambers this time, but it should be no problem for you! Make sure you select each chamber with Left Click to control the launcher within it. Be aware that your movement is restricted due to the connections on the side of the chamber!"),
		5 => format!("You thought Saturdays would be a holiday? No way! This will be your hardest challenge yet! Nothing new but this reaction requires two steps, though watch out for any unwanted reactions!"),
		_ => format!("I hope you are enjoying Mole Rancher Remastered! If you made it this far, leave me a comment letting me know what you think! Any feedback is appreciated! More levels will be added in future updates! This is currently a sandbox level. Use Middle Mouse Button on a mole to track it!"),
	}
}

pub fn get_logbook_text(
	page: usize,
	side: usize,
) -> String {
	match side {
		0 => match page {
			0 => format!("Welcome to the logbook! Click the tabs to view other pages!"),
			1 => format!("The Mole Ranch was first founded after the Molecular Shortages of 67, and is still in operation to this day."),
			2 => format!("According to many ranchers, the spikier a molecule is the tastier it is."),
			_ => format!("Maybe I will fill these pages myself once I have uncovered the secrets of the mole!"),
		}
		_ => match page {
			2 => format!("However, they also require the most careful cooking techniques. A strong particle trail is indicative of a potent scent when cooking."),
			_ => format!("Noone ever writes on the right page of notebooks... The ink would leak through!"),
		}
	}
}


// TEXT STYLES
pub fn get_title_text_style(
	asset_server: &Res<AssetServer>
) -> TextStyle {
	TextStyle {
		font: asset_server.load("fonts/Ronda.ttf"),
		font_size: 50.0,
		color: Color::hex("EDD6AD").unwrap(),
	}
}

pub fn get_subtitle_text_style(
	asset_server: &Res<AssetServer>
) -> TextStyle {
	TextStyle {
		font: asset_server.load("fonts/Ronda.ttf"),
		font_size: 40.0,
		color: Color::hex("EDD6AD").unwrap(),
	}
}

pub fn get_settings_text_style(
	asset_server: &Res<AssetServer>
) -> TextStyle {
	TextStyle {
		font: asset_server.load("fonts/Ronda.ttf"),
		font_size: 30.0,
		color: Color::hex("EDD6AD").unwrap(),
		..Default::default()
	}
}

pub fn get_intro_text_style(
	asset_server: &Res<AssetServer>
) -> TextStyle {
	TextStyle {
		font: asset_server.load("fonts/Ronda.ttf"),
		font_size: 30.0,
		color: Color::hex("2B2B29").unwrap(),
		..Default::default()
	}
}

pub fn get_win_countdown_text_style(
	asset_server: &Res<AssetServer>
) -> TextStyle {
	TextStyle {
		font: asset_server.load("fonts/Ronda.ttf"),
		font_size: 50.0,
		color: *Color::hex("2B2B29").unwrap().set_a(0.95),
	}
}

pub fn get_win_title_text_style(
	asset_server: &Res<AssetServer>
) -> TextStyle {
	TextStyle {
		font: asset_server.load("fonts/Ronda.ttf"),
		font_size: 50.0,
		color: Color::hex("EDD6AD").unwrap(),
	}
}

pub fn get_win_text_style(
	asset_server: &Res<AssetServer>
) -> TextStyle {
	TextStyle {
		font: asset_server.load("fonts/Ronda.ttf"),
		font_size: 25.0,
		color: Color::hex("EDD6AD").unwrap(),
	}
}

pub fn get_win_values_text_style(
	asset_server: &Res<AssetServer>
) -> TextStyle {
	TextStyle {
		font: asset_server.load("fonts/Ronda.ttf"),
		font_size: 25.0,
		color: Color::hex("CDB68D").unwrap(),
	}
}

pub fn get_tooltip_text_style(
	asset_server: &Res<AssetServer>
) -> TextStyle {
	TextStyle {
		font: asset_server.load("fonts/PixelSplitter-Bold.ttf"),
		font_size: 16.0,
		color: Color::rgba(0.1, 0.1, 0.1, 1.0),
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
		font_size: 48.0,
		color: Color::rgba(0.1, 0.3, 0.1, 1.0),
		..Default::default()
	}
}

pub fn get_cost_text_style(
	asset_server: &Res<AssetServer>
) -> TextStyle {
	TextStyle {
		font: asset_server.load("fonts/PixelSplitter-Bold.ttf"),
		font_size: 48.0,
		color: Color::rgba(0.1, 0.3, 0.1, 1.0),
		..Default::default()
	}
}

pub fn get_goal_text_style(
	asset_server: &Res<AssetServer>
) -> TextStyle {
	TextStyle {
		font: asset_server.load("fonts/PixelSplitter-Bold.ttf"),
		font_size: 28.0,
		color: Color::rgba(0.1, 0.3, 0.1, 1.0),
		..Default::default()
	}
}

pub fn get_button_text_style(
	asset_server: &Res<AssetServer>
) -> TextStyle {
	TextStyle {
		font: asset_server.load("fonts/Ronda.ttf"),
		font_size: 25.0,
		color: Color::hex("2B2B29").unwrap(),
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
			("August 1st - The Mole Ranch".to_string(),
			ActorInfo{actor: Actor::Nobody}),
			1 =>
			("So you are the intern I have heard so much about?".to_string(),
			ActorInfo{actor: Actor::Guard}),
			2 =>
			("Welcome! I will show you to your workstation.".to_string(),
			ActorInfo{actor: Actor::Guard}),
			3 =>
			("There is much to learn but I am sure you will pick it up quickly!".to_string(),
			ActorInfo{actor: Actor::Guard}),
			4 =>
			("I will pass you over to Isa, she will be able to explain better than I. But I am sure we will catch up again at some point!".to_string(),
			ActorInfo{actor: Actor::Guard}),
			5 =>
			("Ah, you have arrived. You will find a logbook on your desk to fill with your findings, and the computer is already logged in.".to_string(),
			ActorInfo{actor: Actor::Scientist}),
			6 =>
			("Progress through the training exercises I have laid out for you, they should only take a week to complete. I will check in occasionally to see how you are progressing.".to_string(),
			ActorInfo{actor: Actor::Scientist}),
			7 =>
			("Make sure you read the notes I have left for you, they will be essential when performing reactions.".to_string(),
			ActorInfo{actor: Actor::Scientist}),
			_ =>
			("I look forward to seeing you for your review on Sunday.".to_string(),
			ActorInfo{actor: Actor::Scientist}),
		},
		1 => match current_line {
			0 =>
			("August 2nd - The Mole Ranch".to_string(),
			ActorInfo{actor: Actor::Nobody}),
			1 =>
			("Ah, so we did not scare you off! Good.".to_string(),
			ActorInfo{actor: Actor::Guard}),
			2 =>
			("Isa is out today, I believe she is ranching a new mole type for you to use tomorrow!".to_string(),
			ActorInfo{actor: Actor::Guard}),
			3 =>
			("She likes efficient workers, so try to use as few moles as possible to keep reaction costs down! Though getting your work done quickly can be its own reward too.".to_string(),
			ActorInfo{actor: Actor::Guard}),
			4 =>
			("Do not worry about those moles in the center of the reactor today, I have never seen them do much of anything, and they are not easily budged.".to_string(),
			ActorInfo{actor: Actor::Guard}),
			_ =>
			("Your desk should be the way you left it, good luck!".to_string(),
			ActorInfo{actor: Actor::Guard}),
		},
		2 => match current_line {
			0 =>
			("August 3rd - The Mole Ranch".to_string(),
			ActorInfo{actor: Actor::Nobody}),
			1 =>
			("Welcome back.".to_string(),
			ActorInfo{actor: Actor::Scientist}),
			2 =>
			("The mole you will be working with today is very dangerous, and is capable of destroying most other moles.".to_string(),
			ActorInfo{actor: Actor::Scientist}),
			3 =>
			("As such you will be using it to clean out an old reactor. With correct launcher positioning it should be pretty simple.".to_string(),
			ActorInfo{actor: Actor::Scientist}),
			_ =>
			("Keep up the good work.".to_string(),
			ActorInfo{actor: Actor::Scientist}),
		},
		3 => match current_line {
			0 =>
			("August 4th - The Mole Ranch".to_string(),
			ActorInfo{actor: Actor::Nobody}),
			1 =>
			("Ah, intern. You are picking things up quickly.".to_string(),
			ActorInfo{actor: Actor::Scientist}),
			2 =>
			("You will soon be moving onto reactors with multiple chambers, so do not be afraid to revisit previous problems to get a strong grasp on the basics.".to_string(),
			ActorInfo{actor: Actor::Scientist}),
			3 =>
			("The reactions you learn here will be vital to more complex problems you will face in the future.".to_string(),
			ActorInfo{actor: Actor::Scientist}),
			_ =>
			("Keep this up and you will be a rancher in no time.".to_string(),
			ActorInfo{actor: Actor::Scientist}),
		},
		4 => match current_line {
			0 =>
			("August 5th - The Mole Ranch".to_string(),
			ActorInfo{actor: Actor::Nobody}),
			1 =>
			("Friday already! This week has flown past. Almost as fast as you have flown through your training!".to_string(),
			ActorInfo{actor: Actor::Guard}),
			2 =>
			("Even Isa seems to be impressed with the rate you are progressing!".to_string(),
			ActorInfo{actor: Actor::Guard}),
			_ =>
			("Make sure you remember me once you are up in the big leagues, eh?".to_string(),
			ActorInfo{actor: Actor::Guard}),
		},
		5 => match current_line {
			0 =>
			("August 6th - The Mole Ranch".to_string(),
			ActorInfo{actor: Actor::Nobody}),
			1 =>
			("Today will be tricky, make sure you are careful about which reactions you trigger.".to_string(),
			ActorInfo{actor: Actor::Scientist}),
			2 =>
			("Do not get frustrated if you feel like you have lost progress, as even mistakes can be valuable experiences!".to_string(),
			ActorInfo{actor: Actor::Scientist}),
			_ =>
			("I will see you tomorrow for your review.".to_string(),
			ActorInfo{actor: Actor::Scientist}),
		},
		6 => match current_line {
			0 =>
			("August 7th - Outside The Main Entrance".to_string(),
			ActorInfo{actor: Actor::Nobody}),
			1 =>
			("Well, this is it. Time to see if you have impressed Isa enough to stay.".to_string(),
			ActorInfo{actor: Actor::Guard}),
			2 =>
			("Hopefully you decide to stick around if all things go well. But regardless, it has been nice seeing you.".to_string(),
			ActorInfo{actor: Actor::Guard}),
			3 =>
			("Isa is waiting for you in her office, best not to keep her waiting too long.".to_string(),
			ActorInfo{actor: Actor::Guard}),
			4 =>
			("My name is Arnie by the way. It has been a pleasure getting to see you uncover the wonders of mole ranching. So thanks.".to_string(),
			ActorInfo{actor: Actor::Guard}),
			5 =>
			("Until next time.".to_string(),
			ActorInfo{actor: Actor::Guard}),
			6 =>
			("August 7th - Inside the Office".to_string(),
			ActorInfo{actor: Actor::Nobody}),
			7 =>
			("One week down. You have made great strides. It has been an honor seeing you grow and thrive.".to_string(),
			ActorInfo{actor: Actor::Scientist}),
			8 =>
			("If you would be willing, it would be great if you would stay on as a full time rancher here.".to_string(),
			ActorInfo{actor: Actor::Scientist}),
			_ =>
			("I will give you some time to think it over. Feel free to experiment on the computer, you should have access to all the moles you have seen so far.".to_string(),
			ActorInfo{actor: Actor::Scientist}),
		},
		_ => match current_line {
			0 =>
			("August ??? - Please Report This".to_string(),
			ActorInfo{actor: Actor::Nobody}),
			_ =>
			("You seem to have stumbled upon an unreachable cutscene, interesting. This should not have happened! Please let me know how you got here so I can fix it!".to_string(),
			ActorInfo{actor: Actor::You}),
		}
	}
}

pub fn lines_per_scene(
	current_scene: usize,
) -> usize {
	match current_scene {
		0 => 8,
		1 => 5,
		2 => 4,
		3 => 4,
		4 => 3,
		5 => 3,
		6 => 9,
		_ => 1,
	}
}

pub fn actors_in_scene(
	current_scene: usize,
) -> Vec<Actor> {
	match current_scene {
		0 => vec![Actor::Scientist, Actor::Guard],
		1 => vec![Actor::Guard],
		2 => vec![Actor::Scientist],
		3 => vec![Actor::Scientist],
		4 => vec![Actor::Guard],
		5 => vec![Actor::Scientist],
		6 => vec![Actor::Scientist, Actor::Guard],
		_ => vec![Actor::You],
	}
}

pub fn get_actor_path(
	actor: Actor,
) -> String {
	match actor {
		Actor::Nobody => "".to_string(),
		Actor::You => "".to_string(),
		Actor::Guard => "sprites/characters/arnie_puppet.png".to_string(),
		Actor::Scientist => "sprites/characters/isa_puppet.png".to_string(),
	}
}

pub fn get_portrait_path(
	actor: Actor,
) -> String {
	match actor {
		Actor::Nobody => "".to_string(),
		Actor::You => "".to_string(),
		Actor::Guard => "sprites/characters/arnie_portrait.png".to_string(),
		Actor::Scientist => "sprites/characters/isa_portrait.png".to_string(),
	}
}

pub fn get_actor_name(
	actor: Actor,
) -> String {
	match actor {
		Actor::Nobody => "Nobody".to_string(),
		Actor::You => "You".to_string(),
		Actor::Guard => "Arnie".to_string(),
		Actor::Scientist => "Isa".to_string(),
	}
}
