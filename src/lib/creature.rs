use super::*;

const _MIN_CREATURE_ENERGY: f64 = 1.2;
const _MAX_CREATURE_ENERGY: f64 = 2.0;
const SWIM_ENERGY: f64 = 0.008;
pub const MINIMUM_SURVIVABLE_SIZE: f64 = 0.06;
const SAFE_SIZE: f64 = 1.25;
// Used for drawing the creature.
// const CREATURE_STROKE_WEIGHT: f64 = 0.6;

pub struct Creature {
    pub base: Rock,
    birth_time: f64,
}

impl Creature {
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
}

// Functions returning properties.
impl Creature {
    /// Returns the time when this creature was born.
    pub fn get_birth_time(&self) -> f64 {
        return self.birth_time;
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
