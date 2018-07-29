use super::*;
use std::f64::consts::PI;

// The amount of creatures to display in a list in the UI.
// const LIST_SLOTS: usize = 6;
/// The amount of times a year an object is updated.
const _OBJECT_TIMESTEPS_PER_YEAR: f64 = 100.0;
const _POPULATION_HISTORY_LENGTH: usize = 200;
const _THERMOMETER_MIN: f64 = -2.0;
const _THERMOMETER_MAX: f64 = 2.0;

pub struct Board {
    // Fields relevant for the board itself.
    board_width: usize,
    board_height: usize,
    _tiles: Vec<Vec<Tile>>,

    // Fields relevant for the creatures.
    _creature_minimum: usize,
    _soft_bodies_in_positions: Vec<Vec<Vec<SoftBody>>>,
    _creatures: Vec<Creature>,
    _selected_creature: Option<Creature>,
    _creature_id_up_to: usize,
    _creature_rank_metric: usize,

    // Fields relevant for time or history
    year: f64,
    // _time_step: f64,
    _population_history: Vec<usize>,
    _playspeed: usize,

    // Fields relevant for temperature
    _temperature: f64,
    min_temperature: f64,
    max_temperature: f64,

    // Fields relevant for rocks
    _rocks: Vec<Rock>,

    // Miscelanious
    _user_control: bool,
}

impl std::default::Default for Board {
    fn default() -> Board {
        unimplemented!();

        // Board {
        // creature_id_up_to = 0,
        // creature_rank_metric = 0,
        // }
    }
}

impl Board {
    pub fn update(&mut self, _time_step: f64) {
        unimplemented!();
    }

    pub fn get_growth_over_time_range(&self, _last_updated: f64) -> f64 {
        unimplemented!();
    }

    /// Returns the current growth rate (temperature) based on the season.
    pub fn get_growth_rate(&self) -> f64 {
        let temp_range = self.max_temperature - self.min_temperature;
        return self.min_temperature + temp_range * 0.5
            - temp_range * 0.5 * ((self.year % 1.0) * 2.0 * PI).cos();
    }

    /// Returns the current time, i.e. `self.year`.
    pub fn get_time(&self) -> f64 {
        return self.year;
    }

    /// Returns the width of the board.
    pub fn get_board_width(&self) -> usize {
        return self.board_width;
    }

    /// Returns the height of the board.
    pub fn get_board_height(&self) -> usize {
        return self.board_height;
    }

    /// Returns a `String` representing the current season.
    ///
    /// Can be either "Winter", "Spring", "Summer" or "Autumn".
    pub fn get_season(&self) -> String {
        const SEASONS: [&str; 4] = ["Winter", "Spring", "Summer", "Autumn"];
        let season: usize = ((self.year % 1.0) * 4.0).ceil() as usize;

        return SEASONS[season].to_string();
    }
}
