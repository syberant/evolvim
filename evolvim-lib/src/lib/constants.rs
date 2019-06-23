//! The collection of all constants used by `evolvim`.
//!
//! If you want to tinker around with the world of `evolvim` and try to get different creatures,
//! you have come to the right place!
//! Change the constants in this file to whatever you want, compile, and you're off to a world with brand new possibilities!
//! You should be able to find a description of what each constant does and estimate it's impact.
//! Have fun!
//!
//! TODO: transport all constants over to this file.

pub const SAFE_SIZE: f64 = 1.25;

/// used by creature.rs
pub const CREATURE_DENSITY: f64 = 1.0;

pub const ROCK_DENSITY: f64 = 5.0;

/// Used by creature.rs
pub const CREATURE_MIN_ENERGY: f64 = 1.2;

/// Used by creature.rs
pub const CREATURE_MAX_ENERGY: f64 = 2.0;

/// The default width when generating a new `Board`.
pub const DEFAULT_BOARD_WIDTH: usize = 100;

/// The default height when generating a new `Board`.
pub const DEFAULT_BOARD_HEIGHT: usize = 100;

/// The default size when generating a new `Board`.
///
/// NOTE: Don't change the value of this constant, change `DEFAULT_BOARD_WIDTH` and/or `DEFAULT_BOARD_HEIGHT` instead.
pub const DEFAULT_BOARD_SIZE: crate::ecs_board::BoardSize = (DEFAULT_BOARD_WIDTH, DEFAULT_BOARD_HEIGHT);

/// The default minimum amount of creatures.
///
/// New random creatures will be generated if the population drops under this amount.
pub const DEFAULT_CREATURE_MINIMUM: usize = 60;

/// The coldest it is going to get.
pub const DEFAULT_MIN_TEMP: f64 = -0.5;

/// The hottest it is going to get.
pub const DEFAULT_MAX_TEMP: f64 = 0.7;

/// Used for terrain generation.
pub const DEFAULT_NOISE_STEP_SIZE: f64 = 0.1;

// ************************* //
// ******** DRAWING ******** //
// ************************* //

/// [Hue, Saturation, Brightness, Alpha]
pub const COLOR_WATER: [f32; 4] = [0., 0., 0., 1.];

/// [Hue, Saturation, Brightness]
pub const COLOR_BARREN: [f32; 3] = [0., 0., 1.];

/// [Hue, Saturation, Brightness]
pub const COLOR_FERTILE: [f32; 3] = [0., 0., 0.2];

pub const COLOR_BLACK: [f32; 3] = [0., 1., 0.];

// ******************** //
// ******** UI ******** //
// ******************** //

/// Determines how fast dragging works.
///
/// The bigger, the slower.
pub const MOUSE_SPEED: f64 = 10.0;

// ********************** //
// ******* ENERGY ******* //
// ********************** //

pub const ACCELERATION_ENERGY: f64 = 0.18;
pub const ACCELERATION_BACK_ENERGY: f64 = 0.24;
pub const TURN_ENERGY: f64 = 0.06;

pub const METABOLISM_ENERGY: f64 = 0.004;

pub const SWIM_ENERGY: f64 = 0.008;
pub const EAT_ENERGY: f64 = 0.05;
pub const FIGHT_ENERGY: f64 = 0.06;
pub const INJURED_ENERGY: f64 = 0.25;

// ********************* //
// ******* FOOD ******** //
// ********************* //

pub const FOOD_GROWTH_RATE: f64 = 1.0;
pub const MAX_GROWTH_LEVEL: f64 = 3.0;
pub const FOOD_SENSITIVITY: f64 = 0.3;

pub const EAT_WHILE_MOVING_INEFFICIENCY_MULTIPLIER: f64 = 2.0;
pub const EAT_SPEED: f64 = 0.5;

// ********************** //
// **** REPRODUCTION **** //
// ********************** //
pub const BABY_SIZE: f64 = SAFE_SIZE + 0.1;
