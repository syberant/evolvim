extern crate rand;

use super::*;
use constants::*;

/// The amount of times a year an object is updated.
///
/// TODO: eliminate this variable because it's not needed.
const OBJECT_TIMESTEPS_PER_YEAR: f64 = 100.0;
const _POPULATION_HISTORY_LENGTH: usize = 200;
const _THERMOMETER_MIN: f64 = -2.0;
const _THERMOMETER_MAX: f64 = 2.0;

pub type BoardSize = (usize, usize);
pub type BoardCoordinate = (usize, usize);
pub type BoardPreciseCoordinate = (f64, f64);

pub struct Board {
    // Fields relevant for the board itself.
    board_width: usize,
    board_height: usize,
    pub terrain: Terrain,

    // Fields relevant for the creatures.
    creature_minimum: usize,
    pub soft_bodies_in_positions: SoftBodiesInPositions,
    creatures: Vec<SoftBody>,
    creature_id_up_to: usize,
    // _creature_rank_metric: usize,

    // Fields relevant for time or history
    year: f64,

    // Fields relevant for temperature
    pub climate: Climate,

    // Fields relevant for rocks
    rocks: Vec<SoftBody>,

    // Miscelanious
    user_control: bool,
    selected_creature: Option<*mut Creature>,
}

impl Default for Board {
    fn default() -> Self {
        let board_size = DEFAULT_BOARD_SIZE;
        let noise_step_size = DEFAULT_NOISE_STEP_SIZE;
        let creature_minimum = DEFAULT_CREATURE_MINIMUM;
        let amount_rocks = DEFAULT_ROCK_AMOUNT;
        let min_temp = DEFAULT_MIN_TEMP;
        let max_temp = DEFAULT_MAX_TEMP;
        let user_control = START_IN_CONTROL;

        return Board::new_random(
            board_size,
            noise_step_size,
            creature_minimum,
            amount_rocks,
            min_temp,
            max_temp,
            user_control,
        );
    }
}

impl Board {
    /// Randomly generates a new `Board`.
    pub fn new_random(
        board_size: BoardSize,
        noise_step_size: f64,
        creature_minimum: usize,
        amount_rocks: usize,
        min_temp: f64,
        max_temp: f64,
        user_control: bool,
    ) -> Self {
        let creatures = Vec::with_capacity(creature_minimum);
        let rocks = Vec::with_capacity(amount_rocks);

        // Initialize climate.
        let mut climate = Climate::new(min_temp, max_temp);
        climate.update(0.0);

        let mut board = Board {
            board_width: board_size.0,
            board_height: board_size.1,
            terrain: Terrain::generate_perlin(board_size, noise_step_size),

            creature_minimum,
            soft_bodies_in_positions: SoftBodiesInPositions::new_allocated(board_size),
            creatures,
            creature_id_up_to: 0,

            year: 0.0,

            climate,

            rocks,

            user_control,
            selected_creature: None,
        };

        // Initialize creatures.
        board.maintain_creature_minimum();

        // Initialize rocks.
        // TODO

        return board;
    }

    /// Checks if the given creature was selected and if so, removes it by setting `self.selected_creature` to `None`.
    pub fn unselect_if_dead(&mut self, creature: &mut Creature) {
        let creature_pointer: *mut Creature = creature as *mut Creature;
        if Some(creature_pointer) == self.selected_creature {
            self.selected_creature = None;
        }
    }

    pub fn update(&mut self, time_step: f64) {
        // let previous_year = self.year;
        self.year += time_step;

        // Possibly record population history here.

        self.climate.update(self.year);
        let temp_change_into_frame =
            self.climate.get_temperature() - self.climate.get_growth_rate(self.year - time_step);
        let temp_change_out_of_frame =
            self.climate.get_growth_rate(self.year + time_step) - self.climate.get_temperature();

        if temp_change_into_frame * temp_change_out_of_frame < 0.0 {
            // Temperature change flipped direction
            self.terrain.update_all(self.year, &self.climate);
        }

        // Update all rocks.
        for r in &mut self.rocks {
            r.collide(&self.soft_bodies_in_positions);
        }

        // TODO: fix ugly and unidiomatic code.
        // I know I create a mutable pointer here and use an immutable pointer to `self` further on,
        // but it saves me tons of time doing it this way.
        let creatures_pointer = &mut self.creatures as *mut Vec<SoftBody>;
        unsafe {
            for c in (*creatures_pointer).iter_mut() {
                // These functions take an immutable pointer to `self`.
                c.collide(&self.soft_bodies_in_positions);
                c.metabolize(time_step, &self);

                c.use_brain(time_step, !self.user_control, self);

                if self.user_control {
                    // TODO: provide user control over creature.
                }
            }
        }

        // Kill weak creatures.
        self.remove_dead_creatures();

        // Experimental: this was moved from above to always keep the creature minimum.
        self.maintain_creature_minimum();

        // Finish the iteration.
        let rocks_pointer = &mut self.rocks as *mut Vec<SoftBody>;
        unsafe {
            for r in (*rocks_pointer).iter_mut() {
                // This function takes a mutable pointer to `self`.
                r.apply_motions(time_step * OBJECT_TIMESTEPS_PER_YEAR, self);
            }

            for c in (*creatures_pointer).iter_mut() {
                // This function takes a mutable pointer to `self`.
                c.apply_motions(time_step * OBJECT_TIMESTEPS_PER_YEAR, self);

                // TODO: implement seeing.
                // c.see();
            }
        }

        // TODO: implement filesaving.
    }

    /// Maintains the creature minimum by adding random creatures until there are at least `self.creature_minimum` creatures.
    fn maintain_creature_minimum(&mut self) {
        while self.creatures.len() < self.creature_minimum {
            let board_size = self.get_board_size();
            let mut creature = SoftBody::new_random_creature(board_size, self.year);

            // Initialize in `SoftBodiesInPositions` as well.
            creature.set_sbip(&mut self.soft_bodies_in_positions, board_size);
            // Just to set the prevSBIP variables.
            creature.set_sbip(&mut self.soft_bodies_in_positions, board_size);

            self.creatures.push(creature);
            self.creature_id_up_to += 1;
        }
    }

    /// Checks for all creatures whether they are fit enough to live and kills them off if they're not.
    ///
    /// Utilizes the `should_die` function of `SoftBody`.
    fn remove_dead_creatures(&mut self) {
        let board_size = self.get_board_size();
        let self_ptr: *mut Board = self as *mut Board;

        // TODO: possibly optimise code
        let mut i = 0;
        while i < self.creatures.len() {
            // let creature = &mut self.creatures[i];
            if self.creatures[i].should_die() {
                unsafe {
                    // Infallable
                    self.creatures[i].return_to_earth(self_ptr, board_size);
                }
                self.creatures.remove(i);
            } else {
                i += 1;
            }
        }
    }

    pub fn get_growth_since(&self, last_updated: f64) -> f64 {
        return self
            .climate
            .get_growth_over_time_range(self.year, last_updated);
    }

    /// Returns the current growth rate (temperature) based on the season.
    pub fn get_current_growth_rate(&self) -> f64 {
        self.climate.get_growth_rate(self.year)
    }

    /// Returns the current time, i.e. `self.year`.
    pub fn get_time(&self) -> f64 {
        return self.year;
    }

    /// Returns a tuple with the width and height of this `Board`.
    ///
    /// Equivalent to `(board.get_board_width(), board.get_board_height())`.
    pub fn get_board_size(&self) -> (usize, usize) {
        return (self.board_width, self.board_height);
    }

    /// Returns the width of the board.
    pub fn get_board_width(&self) -> usize {
        return self.board_width;
    }

    /// Returns the height of the board.
    pub fn get_board_height(&self) -> usize {
        return self.board_height;
    }
}

impl Board {
    /// Gets the size of the current population; i.e. how many creatures are currently alive.
    pub fn get_population_size(&self) -> usize {
        return self.creatures.len();
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
