use super::SoftBody;
use crate::climate::Climate;
use crate::constants::*;
use crate::ecs_board::{BoardCoordinate, BoardSize};
use crate::terrain::Terrain;
use nphysics2d::object::{Body, RigidBody};
use rand::Rng;
use std::f64::consts::PI;

const ENERGY_DENSITY: f64 = 1.0
    / (super::creature::MINIMUM_SURVIVABLE_SIZE * super::creature::MINIMUM_SURVIVABLE_SIZE * PI);
pub const FIGHT_RANGE: f64 = 2.0;

#[derive(Clone, Serialize, Deserialize)]
pub struct Rock {
    // Energy
    energy: f64,
    density: f64,
    // Stats or info
    prev_energy: f64,
    birth_time: f64,
    // Miscellanious
    mouth_hue: f64,
}

impl Rock {
    pub fn new_random(board_size: BoardSize, density: f64, energy: f64, time: f64) -> Self {
        let (board_width, board_height) = board_size;

        let mut thread_rng = rand::thread_rng();
        let mouth_hue = thread_rng.gen::<f64>();

        Self {
            energy,
            density,

            prev_energy: energy,
            birth_time: time,

            mouth_hue,
        }
    }

    pub fn new_from_parents<B>(parents: &[&SoftBody<B>], energy: f64, time: f64) -> Rock {
        let parent_amount = parents.len();

        // The hue is the mean of all parent hues
        let mouth_hue = parents.iter().fold(0.0, |acc, parent| {
            acc + parent.mouth_hue / parent_amount as f64
        });

        let density = parents[0].density;

        Rock {
            energy,
            density,

            prev_energy: energy,
            birth_time: time,

            mouth_hue,
        }
    }

    /// Return this body's radius, used for collissions, etc.
    pub fn get_radius(&self) -> f64 {
        return (self.energy / ENERGY_DENSITY / PI).sqrt().max(0.0);
    }

    /// Return this body's mass, used for physics, etc.
    pub fn get_mass(&self) -> f64 {
        return self.energy / ENERGY_DENSITY * self.density;
    }

    /// Eat
    pub fn eat(
        &mut self,
        attempted_amount: f64,
        time_step: f64,
        time: f64,
        climate: &Climate,
        tile: &mut crate::terrain::tile::Tile,
        body: &RigidBody<f64>,
    ) {
        let velocity = {
            let v = body.velocity().as_slice();

            (v[0].powi(2) + v[1].powi(2)).sqrt()
        };
        let amount = attempted_amount / (1.0 + velocity * EAT_WHILE_MOVING_INEFFICIENCY_MULTIPLIER);
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

            let multiplier = tile
                .get_food_multiplier(self.get_mouth_hue())
                .unwrap_or(0.0);
            if multiplier < 0.0 {
                // Poison
                self.lose_energy(food_to_eat * -multiplier);
            } else {
                // Healthy food
                self.add_energy(food_to_eat * multiplier);
            }

            self.lose_energy(attempted_amount * EAT_ENERGY * time_step);
        }
    }

    pub fn fight<B: 'static>(
        &mut self,
        amount: f64,
        time: f64,
        time_step: f64,
        world: &mut nphysics2d::world::World<f64>,
    ) {
        // use super::MATURE_AGE;

        // if amount > 0.0 && self.get_age(time) >= MATURE_AGE {
        //     self.lose_energy(amount * time_step * FIGHT_ENERGY);

        //     let self_x = self.get_px();
        //     let self_y = self.get_py();

        //     let mut colliders: Vec<&mut SoftBody<B>> = unimplemented!();

        //     for col in colliders {
        //         let distance = distance(self_x, self_y, col.get_px(), col.get_py());
        //         let combined_radius = self.get_radius() * FIGHT_RANGE + col.get_radius();

        //         if distance < combined_radius {
        //             // collider was hit, remove energy
        //             col.lose_energy(amount * INJURED_ENERGY * time_step);
        //         }
        //     }
        // }
    }

    /// Accelerate
    ///
    /// Costs energy.
    pub fn accelerate(&mut self, amount: f64, time_step: f64, body: &mut RigidBody<f64>) {
        use nphysics2d::algebra::ForceType;
        use nphysics2d::math::Force;

        let force_type = ForceType::Force;
        let force = Force::from_slice(&[0.0, amount, 0.0]);
        body.apply_local_force(0, &force, force_type, true);

        if amount >= 0.0 {
            // Moving forward
            self.lose_energy(amount * time_step * ACCELERATION_ENERGY);
        } else {
            // Moving backward
            self.lose_energy(amount * time_step * ACCELERATION_BACK_ENERGY);
        }
    }

    /// Increase turning velocity.
    ///
    /// Costs energy.
    pub fn turn(&mut self, amount: f64, time_step: f64, body: &mut RigidBody<f64>) {
        use nphysics2d::algebra::ForceType;
        use nphysics2d::math::Force;

        let force_type = ForceType::Force;
        let force = Force::from_slice(&[0.0, 0.0, amount]);
        body.apply_local_force(0, &force, force_type, true);

        // Call `abs()` because we can turn both ways.
        let energy_to_lose = (amount * self.energy * time_step * TURN_ENERGY).abs();
        self.lose_energy(energy_to_lose);
    }

    /// Returns true if this body is currently on water.
    pub fn is_on_water(&self, terrain: &Terrain, pos: BoardCoordinate) -> bool {
        let tile = terrain.get_tile_at(pos);
        return tile.is_water();
    }
}

// Here are all the functions that safely change a property.
impl Rock {
    pub fn lose_energy(&mut self, energy_to_lose: f64) {
        self.energy -= energy_to_lose.max(0.0);
    }

    pub fn add_energy(&mut self, energy_to_add: f64) {
        self.energy += energy_to_add.max(0.0);
    }

    pub fn record_energy(&mut self) {
        self.prev_energy = self.energy;
    }

    pub fn get_energy_change(&self, time_step: f64) -> f64 {
        (self.energy - self.prev_energy) / time_step
    }

    pub fn set_mouth_hue(&mut self, value: f64) {
        self.mouth_hue = value.min(1.0).max(0.0);
    }
}

// Here are all the functions to simply get a property.
impl Rock {
    pub fn get_energy(&self) -> f64 {
        return self.energy;
    }

    pub fn get_density(&self) -> f64 {
        self.density
    }

    pub fn get_mouth_hue(&self) -> f64 {
        return self.mouth_hue;
    }

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

/// Checks if the center is inside of the world, possibly corrects it and returns it.
pub fn check_center_x(x: usize, board_width: usize) -> usize {
    return x.max(0).min(board_width - 1);
}

/// Checks if the center is inside of the world, possibly corrects it and returns it.
pub fn check_center_y(y: usize, board_height: usize) -> usize {
    return y.max(0).min(board_height - 1);
}

/// Returns the distance between two points.
///
/// Uses the Pythagorean theorem: A^2 + B^2 = C^2.
pub fn distance(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    ((x1 - x2).powi(2) + (y1 - y2).powi(2)).sqrt()
}
