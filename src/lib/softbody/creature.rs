extern crate rand;

use super::*;

pub const MINIMUM_SURVIVABLE_SIZE: f64 = 0.06;
const EAT_WHILE_MOVING_INEFFICIENCY_MULTIPLIER: f64 = 2.0;
const EAT_SPEED: f64 = 0.5;

pub struct Creature {
    pub base: Rock,
    birth_time: f64,
    pub brain: Brain,
    mouth_hue: f64,
}

impl Creature {
    pub fn new_random(board_size: BoardSize, time: f64) -> Self {
        let energy = CREATURE_MIN_ENERGY
            + rand::random::<f64>() * (CREATURE_MAX_ENERGY - CREATURE_MIN_ENERGY);
        let base = Rock::new_random(board_size, CREATURE_DENSITY, energy);
        let brain = Brain::new_random();
        let birth_time = time;
        let mouth_hue = rand::random();
        // TODO: add id

        Creature {
            base,
            birth_time,
            brain,
            mouth_hue,
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

    pub fn eat(
        &mut self,
        attempted_amount: f64,
        time_step: f64,
        time: f64,
        climate: &Climate,
        tile: &mut super::terrain::tile::Tile,
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

            let multiplier = tile.get_food_multiplier(self.mouth_hue).unwrap_or(0.0);
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

    // Create a new baby, it isn't in `SoftBodiesInPositions` so please fix that.
    // While you're at it, also add it to `Board.creatures`.
    pub fn new_baby(parents: Vec<HLSoftBody>, energy: f64, time: f64) -> Creature {
        let parent_amount = parents.len();

        let brain = Brain::evolve(&parents);
        let base = Rock::new_from_parents(&parents, energy);

        // The current time
        let birth_time = time;
        // The hue is the mean of all parent hues
        let mouth_hue = parents.iter().fold(0.0, |acc, parent| {
            acc + parent.borrow().get_creature().mouth_hue / parent_amount as f64
        });

        Creature {
            base,
            birth_time,
            brain,
            mouth_hue,
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

    pub fn get_mouth_hue(&self) -> f64 {
        return self.mouth_hue;
    }

    pub fn set_mouth_hue(&mut self, value: f64) {
        self.mouth_hue = value;
    }
}

// Functions calling themselves for `self.base`
impl Creature {
    pub fn is_on_water(&self, terrain: &Terrain, board_size: BoardSize) -> bool {
        return self.base.is_on_water(terrain, board_size);
    }

    pub fn get_energy(&self) -> f64 {
        return self.base.get_energy();
    }

    pub fn lose_energy(&mut self, energy_to_lose: f64) {
        self.base.lose_energy(energy_to_lose);
    }
}
