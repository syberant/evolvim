extern crate rand;

use super::*;
use constants::*;

const _MIN_CREATURE_ENERGY: f64 = 1.2;
const _MAX_CREATURE_ENERGY: f64 = 2.0;
const SWIM_ENERGY: f64 = 0.008;
const EAT_ENERGY: f64 = 0.05;
pub const MINIMUM_SURVIVABLE_SIZE: f64 = 0.06;
const EAT_WHILE_MOVING_INEFFICIENCY_MULTIPLIER: f64 = 2.0;
const EAT_SPEED: f64 = 0.5;
// Used for drawing the creature.
// const CREATURE_STROKE_WEIGHT: f64 = 0.6;

pub struct Creature {
    pub base: Rock,
    birth_time: f64,
    pub brain: Brain,
    mouth_hue: f64,
}

impl Creature {
    pub fn new_random(time: f64) -> Self {
        let base = Rock::new_random();
        let brain = Brain::new_random();
        let birth_time = time;
        let mouth_hue = rand::random();

        Creature {
            base,
            birth_time,
            brain,
            mouth_hue,
        }
    }

    /// The `Creature` version of `apply_motions`, this is different to the `Rock` version.
    pub fn apply_motions(&mut self, time_step: f64, board: &mut Board) {
        if self.is_on_water(board) {
            let energy_to_lose = time_step * SWIM_ENERGY * self.get_energy();
            self.lose_energy(energy_to_lose);
        }

        self.base.apply_motions(time_step, board.get_board_size());
    }

    pub fn should_die(&self) -> bool {
        return self.get_energy() > SAFE_SIZE;
    }

    pub fn eat(
        &mut self,
        attempted_amount: f64,
        time_step: f64,
        time: f64,
        climate: &Climate,
        tile: &mut Tile,
    ) {
        let amount = attempted_amount
            / (1.0 + self.base.get_total_velocity() * EAT_WHILE_MOVING_INEFFICIENCY_MULTIPLIER);
        if amount < 0.0 {
            // Vomit
            // TODO: implement vomiting.
        } else {
            // Eat
            let food_level = tile.get_food_level();

            let mut food_to_eat = food_level * (1.0 - (1.0 - EAT_SPEED).powf(amount * time_step));
            food_to_eat = food_to_eat.min(food_level);
            // Remove eaten food from tile.
            tile.remove_food(food_to_eat);
            tile.update(time, climate);

            let multiplier = tile.get_food_multiplier(self.mouth_hue);
            if multiplier < 0.0 {
                // Poison
                self.base.lose_energy(food_to_eat * -multiplier);
            } else {
                // Healthy food
                self.base.add_energy(food_to_eat * multiplier);
            }

            self.base
                .lose_energy(attempted_amount * EAT_ENERGY * time_step);
        }
    }
}

// Functions returning properties.
impl Creature {
    /// Returns the time when this creature was born.
    pub fn get_birth_time(&self) -> f64 {
        return self.birth_time;
    }

    /// Returns the age of this creature.
    ///
    /// More concretely: this function is equivalent to `time - self.get_birth_time()`.
    pub fn get_age(&self, time: f64) -> f64 {
        return time - self.birth_time;
    }
}

// Functions calling themselves for `self.base`
impl Creature {
    pub fn is_on_water(&self, board: &Board) -> bool {
        return self.base.is_on_water(board);
    }

    pub fn get_energy(&self) -> f64 {
        return self.base.get_energy();
    }

    pub fn lose_energy(&mut self, energy_to_lose: f64) {
        self.base.lose_energy(energy_to_lose);
    }
}
