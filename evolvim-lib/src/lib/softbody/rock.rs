use super::{HLSoftBody, SoftBody};
use crate::board::{BoardCoordinate, BoardPreciseCoordinate, BoardSize};
use crate::climate::Climate;
use crate::constants::*;
use crate::sbip::{SoftBodiesAt, SoftBodiesInPositions};
use crate::terrain::Terrain;
use rand::Rng;
use std::f64::consts::PI;
use std::ops::Range;

const FRICTION: f64 = 0.004;
const ENERGY_DENSITY: f64 = 1.0
    / (super::creature::MINIMUM_SURVIVABLE_SIZE * super::creature::MINIMUM_SURVIVABLE_SIZE * PI);
pub const FIGHT_RANGE: f64 = 2.0;

#[derive(Clone, Serialize, Deserialize)]
pub struct Rock {
    // Position
    px: f64,
    py: f64,
    rotation: f64,
    // Energy
    energy: f64,
    density: f64,
    // Soft Bodies In Positions
    sbip_min_x: usize,
    sbip_min_y: usize,
    sbip_max_x: usize,
    sbip_max_y: usize,
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
        let px = thread_rng.gen::<f64>() * (board_width - 1) as f64;
        let py = thread_rng.gen::<f64>() * (board_height - 1) as f64;
        let mouth_hue = thread_rng.gen::<f64>();

        Self {
            px,
            py,
            rotation: rand::random::<f64>() * 2.0 * PI,

            energy,
            density,

            sbip_min_x: 0,
            sbip_min_y: 0,
            sbip_max_x: 0,
            sbip_max_y: 0,

            prev_energy: energy,
            birth_time: time,

            mouth_hue,
        }
    }

    /// TODO: prevent px and py from being directly on top of the parent.
    pub fn new_from_parents<B>(parents: &[&SoftBody<B>], energy: f64, time: f64) -> Rock {
        let parent_amount = parents.len();

        let px = parents
            .iter()
            .fold(0.0, |acc, parent| acc + parent.px / parent_amount as f64);
        let py = parents
            .iter()
            .fold(0.0, |acc, parent| acc + parent.py / parent_amount as f64);
        let rotation = parents.iter().fold(0.0, |acc, parent| {
            acc + parent.rotation / parent_amount as f64
        });

        // The hue is the mean of all parent hues
        let mouth_hue = parents.iter().fold(0.0, |acc, parent| {
            acc + parent.mouth_hue / parent_amount as f64
        });

        let density = parents[0].density;

        Rock {
            px,
            py,
            rotation,

            energy,
            density,

            sbip_min_x: 0,
            sbip_min_y: 0,
            sbip_max_x: 0,
            sbip_max_y: 0,

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
    ) {
        // let amount = attempted_amount
        //     / (1.0 + self.get_total_velocity() * EAT_WHILE_MOVING_INEFFICIENCY_MULTIPLIER);
        let amount: f64 = unimplemented!();
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
        sbip: &SoftBodiesInPositions<B>,
        world: &mut nphysics2d::world::World<f64>,
        self_pointer: HLSoftBody<B>,
    ) {
        use super::MATURE_AGE;
        use crate::sbip::SoftBodyBucket;

        if amount > 0.0 && self.get_age(time) >= MATURE_AGE {
            self.lose_energy(amount * time_step * FIGHT_ENERGY);

            let self_x = self.get_px();
            let self_y = self.get_py();

            let mut colliders = self.get_colliders(sbip);

            // Remove self
            colliders.remove_softbody(self_pointer);

            for collider in colliders {
                let mut col = collider.borrow_mut(world);
                let distance = distance(self_x, self_y, col.get_px(), col.get_py());
                let combined_radius = self.get_radius() * FIGHT_RANGE + col.get_radius();

                if distance < combined_radius {
                    // collider was hit, remove energy
                    col.lose_energy(amount * INJURED_ENERGY * time_step);
                }
            }
        }
    }

    /// Accelerate
    ///
    /// Costs energy.
    pub fn accelerate(&mut self, amount: f64, time_step: f64) {
        let multiplier = amount * time_step / self.get_mass();
        // self.vx += self.rotation.cos() * multiplier;
        // self.vy += self.rotation.sin() * multiplier;
        unimplemented!();

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
    pub fn turn(&mut self, amount: f64, time_step: f64) {
        // self.vr += 0.04 * amount * time_step / self.get_mass();
        unimplemented!();

        // Call `abs()` because we can turn both ways.
        let energy_to_lose = (amount * self.energy * time_step * TURN_ENERGY).abs();
        self.lose_energy(energy_to_lose);
    }

    pub fn is_in_tile(&self, x: usize, y: usize) -> bool {
        x > self.sbip_min_x && x < self.sbip_max_x && y > self.sbip_min_y && y < self.sbip_max_y
    }

    pub fn get_random_covered_tile(&self, board_size: BoardSize) -> BoardCoordinate {
        let radius = self.get_radius();
        let mut choice_x = 0.0;
        let mut choice_y = 0.0;
        while distance(self.px, self.py, choice_x, choice_y) > radius {
            choice_x = rand::random::<f64>() * 2.0 * radius - radius + self.px;
            choice_y = rand::random::<f64>() * 2.0 * radius - radius + self.py;
        }

        let choice_x = check_center_x(choice_x.floor() as usize, board_size.0);
        let choice_y = check_center_y(choice_y.floor() as usize, board_size.1);

        return (choice_x, choice_y);
    }

    /// Returns true if this body is currently on water.
    pub fn is_on_water(&self, terrain: &Terrain, board_size: BoardSize) -> bool {
        // TODO: determine whether this is desirable and maybe come up with a better system.
        let pos = self.get_random_covered_tile(board_size);
        let tile = terrain.get_tile_at(pos);
        return tile.is_water();
    }
}

// All functions related to `SoftBodiesInPositions`
impl Rock {
    pub fn get_colliders<B>(&self, sbip: &SoftBodiesInPositions<B>) -> SoftBodiesAt<B> {
        sbip.get_soft_bodies_in(self.current_x_range(), self.current_y_range())
    }

    pub fn update_sbip_variables(&mut self, board_size: BoardSize) {
        let radius = self.get_radius() * FIGHT_RANGE;

        let board_width = board_size.0;
        let board_height = board_size.1;
        // use this to overcome the borrow checker
        let px = self.px;
        let py = self.py;
        self.sbip_min_x = check_center_x((px - radius).floor() as usize, board_width);
        self.sbip_min_y = check_center_y((py - radius).floor() as usize, board_height);
        self.sbip_max_x = check_center_x((px + radius).floor() as usize, board_width);
        self.sbip_max_y = check_center_y((py + radius).floor() as usize, board_height);
    }

    pub fn current_x_range(&self) -> Range<usize> {
        self.sbip_min_x..self.sbip_max_x + 1
    }

    pub fn current_y_range(&self) -> Range<usize> {
        self.sbip_min_y..self.sbip_max_y + 1
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

    /// Sets the center of this `SoftBody` and makes sure the entire body stays inside of the world.
    ///
    /// I.e. it also takes the radius of this body into account.
    pub fn set_body_x(&mut self, new_x: f64, board_width: usize) {
        let radius = self.get_radius();
        self.px = new_x.max(radius).min(board_width as f64 - radius);
    }

    /// Sets the center of this `SoftBody` and makes sure the entire body stays inside of the world.
    ///
    /// I.e. it also takes the radius of this body into account.
    pub fn set_body_y(&mut self, new_y: f64, board_height: usize) {
        let radius = self.get_radius();
        self.py = new_y.max(radius).min(board_height as f64 - radius);
    }

    /// Sets the rotation of this `Rock` to `new_rot`.
    pub fn set_body_rotation(&mut self, new_rot: f64) {
        self.rotation = new_rot;
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

    pub fn get_px(&self) -> f64 {
        return self.px;
    }

    pub fn get_py(&self) -> f64 {
        return self.py;
    }

    pub fn get_rotation(&self) -> f64 {
        return self.rotation;
    }

    pub fn get_position(&self) -> BoardPreciseCoordinate {
        BoardPreciseCoordinate(self.get_px(), self.get_py())
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
