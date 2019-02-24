extern crate rand;

use super::*;

pub const MINIMUM_SURVIVABLE_SIZE: f64 = 0.06;

#[derive(Serialize, Deserialize)]
pub struct Creature {
    pub base: Rock,
    birth_time: f64,
    pub brain: Brain,
}

impl std::ops::Deref for Creature {
    type Target = Rock;

    fn deref(&self) -> &Rock {
        &self.base
    }
}

impl std::ops::DerefMut for Creature {
    fn deref_mut(&mut self) -> &mut Rock {
        &mut self.base
    }
}

impl Creature {
    pub fn new_random(board_size: BoardSize, time: f64) -> Self {
        let energy = CREATURE_MIN_ENERGY
            + rand::random::<f64>() * (CREATURE_MAX_ENERGY - CREATURE_MIN_ENERGY);
        let base = Rock::new_random(board_size, CREATURE_DENSITY, energy);
        let brain = Brain::new_random();
        let birth_time = time;
        // TODO: add id

        Creature {
            base,
            birth_time,
            brain,
        }
    }

    // The `Creature` version of `apply_motions`, this is different to the `Rock` version.
    pub fn apply_motions(&mut self, time_step: f64, terrain: &Terrain, board_size: BoardSize) {
        if self.is_on_water(terrain, board_size) {
            let energy_to_lose = time_step * SWIM_ENERGY * self.get_energy();
            self.lose_energy(energy_to_lose);
        }

        self.base.apply_motions(time_step, board_size);
    }

    pub fn should_die(&self) -> bool {
        return self.get_energy() < SAFE_SIZE;
    }

    // Create a new baby, it isn't in `SoftBodiesInPositions` so please fix that.
    // While you're at it, also add it to `Board.creatures`.
    pub fn new_baby(parents: Vec<HLSoftBody>, energy: f64, time: f64) -> Creature {
        let brain = Brain::evolve(&parents);
        let base = Rock::new_from_parents(&parents, energy);

        // The current time
        let birth_time = time;

        Creature {
            base,
            birth_time,
            brain,
        }
    }

    pub fn get_baby_energy(&self) -> f64 {
        self.base.get_energy() - SAFE_SIZE
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
