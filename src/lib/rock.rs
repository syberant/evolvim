extern crate rand;

use super::*;
// use constants::*;
use std::f64::consts::PI;

const ACCELERATION_ENERGY: f64 = 0.18;
const ACCELERATION_BACK_ENERGY: f64 = 0.24;
const TURN_ENERGY: f64 = 0.06;
const FRICTION: f64 = 0.004;
const ENERGY_DENSITY: f64 = 1.0
    / (super::creature::MINIMUM_SURVIVABLE_SIZE * super::creature::MINIMUM_SURVIVABLE_SIZE * PI);
const FIGHT_RANGE: f64 = 2.0;

pub struct Rock {
    // Position
    px: f64,
    py: f64,
    rotation: f64,
    // Velocity
    vx: f64,
    vy: f64,
    vr: f64,
    // Energy
    energy: f64,
    density: f64,
    // Soft Bodies In Positions
    sbip_min_x: usize,
    sbip_min_y: usize,
    sbip_max_x: usize,
    sbip_max_y: usize,
    prev_sbip_min_x: usize,
    prev_sbip_min_y: usize,
    prev_sbip_max_x: usize,
    prev_sbip_max_y: usize,
}

impl Rock {
    pub fn new_random(board_size: BoardSize, density: f64, energy: f64) -> Self {
        let (board_width, board_height) = board_size;
        Self {
            px: rand::random::<f64>() * (board_width - 1) as f64,
            py: rand::random::<f64>() * (board_height - 1) as f64,
            rotation: rand::random::<f64>() * 2.0 * PI,

            vx: 0.0,
            vy: 0.0,
            vr: 0.0,

            energy,
            density,

            sbip_min_x: 0,
            sbip_min_y: 0,
            sbip_max_x: 0,
            sbip_max_y: 0,
            prev_sbip_min_x: 0,
            prev_sbip_min_y: 0,
            prev_sbip_max_x: 0,
            prev_sbip_max_y: 0,
        }
    }

    /// Takes a new coordinate and changes itself to it without any checking.
    ///
    /// This function is unsafe because we are not checking for moving outside of the board or not,
    /// please use `set_body_x()` which has some checks included.
    pub unsafe fn set_px(&mut self, new_x: f64) {
        self.px = new_x;
    }

    /// Takes a new coordinate and changes itself to it without any checking.
    ///
    /// This function is unsafe because we are not checking for moving outside of the board or not,
    /// please use `set_body_y()` which has some checks included.
    pub unsafe fn set_py(&mut self, new_y: f64) {
        self.py = new_y;
    }

    /// Return this body's radius, used for collissions, etc.
    pub fn get_radius(&self) -> f64 {
        return (self.energy / ENERGY_DENSITY / PI).sqrt().max(0.0);
    }

    /// Return this body's mass, used for physics, etc.
    pub fn get_mass(&self) -> f64 {
        return self.energy / ENERGY_DENSITY * self.density;
    }

    /// Returns the total velocity.
    ///
    /// Does sqrt(vx^2 + vy^2).
    pub fn get_total_velocity(&self) -> f64 {
        return (self.vx.powi(2) + self.vy.powi(2)).sqrt();
    }

    /// Accelerate
    ///
    /// Costs energy.
    pub fn accelerate(&mut self, amount: f64, time_step: f64) {
        let multiplier = amount * time_step / self.get_mass();
        self.vx += self.rotation.cos() * multiplier;
        self.vy += self.rotation.sin() * multiplier;

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
        self.vr += 0.04 * amount * time_step / self.get_mass();

        // Call `abs()` because we can turn both ways.
        let energy_to_lose = (amount * self.energy * time_step * TURN_ENERGY).abs();
        self.lose_energy(energy_to_lose);
    }

    /// Updates positions and velocities based on `time_step` and some physics formulae.
    ///
    /// NOTE: Includes rotation unlike the Processing code.
    /// NOTE: Does not call `set_sbip`.
    pub fn apply_motions(&mut self, time_step: f64, board_size: BoardSize) {
        let new_px = self.px + self.vx * time_step;
        let new_py = self.py + self.vy * time_step;
        self.set_body_x(new_px, board_size.0);
        self.set_body_y(new_py, board_size.1);
        self.rotation += self.vr * time_step;

        self.vx *= 0f64.max(1.0 - FRICTION / self.get_mass());
        self.vy *= 0f64.max(1.0 - FRICTION / self.get_mass());
        self.vr *= 0f64.max(1.0 - FRICTION / self.get_mass());
    }

    pub fn moved_between_tiles(&self) -> bool {
        return self.prev_sbip_max_x != self.sbip_max_x
            || self.prev_sbip_max_y != self.sbip_max_y
            || self.prev_sbip_min_x != self.sbip_min_x
            || self.prev_sbip_min_y != self.sbip_min_y;
    }

    pub fn is_in_tile(&self, x: usize, y: usize) -> bool {
        x > self.sbip_min_x && x < self.sbip_max_x && y > self.sbip_min_y && y < self.sbip_max_y
    }

    pub fn was_in_tile(&self, x: usize, y: usize) -> bool {
        x > self.prev_sbip_min_x
            && x < self.prev_sbip_max_x
            && y > self.prev_sbip_min_y
            && y < self.prev_sbip_max_y
    }

    pub fn get_random_covered_tile(&self, board_size: BoardSize) -> (usize, usize) {
        let radius = self.get_radius();
        let mut choice_x = 0.0;
        let mut choice_y = 0.0;
        while SoftBody::distance(self.px, self.py, choice_x, choice_y) > radius {
            choice_x = rand::random::<f64>() * 2.0 * radius - radius + self.px;
            choice_y = rand::random::<f64>() * 2.0 * radius - radius + self.py;
        }

        let choice_x = SoftBody::check_center_x(choice_x.floor() as usize, board_size.0);
        let choice_y = SoftBody::check_center_y(choice_y.floor() as usize, board_size.1);

        return (choice_x, choice_y);
    }

    pub fn get_random_covered_tile_mut<'a>(
        &self,
        board_size: BoardSize,
        tiles: &'a mut Vec<Vec<Tile>>,
    ) -> &'a mut Tile {
        let pos = self.get_random_covered_tile(board_size);
        return &mut tiles[pos.0][pos.1];
    }

    /// Returns true if this body is currently on water.
    pub fn is_on_water(&self, board: &Board) -> bool {
        // TODO: determine whether this is desirable and maybe come up with a better system.
        let pos = self.get_random_covered_tile(board.get_board_size());
        let tile = &board.tiles[pos.0][pos.1];
        return tile.is_water();
    }
}

// All functions related to `SoftBodiesInPositions`
impl Rock {
    pub fn update_sbip_variables(&mut self, board_size: BoardSize) {
        let radius = self.get_radius() * FIGHT_RANGE;

        self.prev_sbip_min_x = self.sbip_min_x;
        self.prev_sbip_min_y = self.sbip_min_y;
        self.prev_sbip_max_x = self.sbip_max_x;
        self.prev_sbip_max_y = self.sbip_max_y;

        let board_width = board_size.0;
        let board_height = board_size.1;
        // use this to overcome the borrow checker
        let px = self.px;
        let py = self.py;
        self.sbip_min_x = SoftBody::check_center_x((px - radius).floor() as usize, board_width);
        self.sbip_min_y = SoftBody::check_center_y((py - radius).floor() as usize, board_height);
        self.sbip_max_x = SoftBody::check_center_x((px + radius).floor() as usize, board_width);
        self.sbip_max_y = SoftBody::check_center_y((py + radius).floor() as usize, board_height);
    }

    pub fn previous_x_range(&self) -> std::ops::RangeInclusive<usize> {
        self.prev_sbip_min_x..=self.prev_sbip_max_x
    }

    pub fn previous_y_range(&self) -> std::ops::RangeInclusive<usize> {
        self.prev_sbip_min_y..=self.prev_sbip_max_y
    }

    pub fn current_x_range(&self) -> std::ops::RangeInclusive<usize> {
        self.sbip_min_x..=self.sbip_max_x
    }

    pub fn current_y_range(&self) -> std::ops::RangeInclusive<usize> {
        self.sbip_min_y..=self.sbip_max_y
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

    /// Sets the center of this `SoftBody` and makes sure the entire body stays inside of the world.
    ///
    /// I.e. it also takes the radius of this body into account.
    pub fn set_body_x(&mut self, new_x: f64, board_width: usize) {
        let radius = self.get_radius();
        self.px = new_x.max(radius).min((board_width - 1) as f64 - radius);
    }

    /// Sets the center of this `SoftBody` and makes sure the entire body stays inside of the world.
    ///
    /// I.e. it also takes the radius of this body into account.
    pub fn set_body_y(&mut self, new_y: f64, board_height: usize) {
        let radius = self.get_radius();
        self.px = new_y.max(radius).min((board_height - 1) as f64 - radius);
    }

    pub fn add_vx(&mut self, value_to_add: f64) {
        self.vx += value_to_add;
    }

    pub fn add_vy(&mut self, value_to_add: f64) {
        self.vy += value_to_add;
    }
}

// Here are all the functions to simply get a property.
impl Rock {
    pub fn get_energy(&self) -> f64 {
        return self.energy;
    }

    pub fn get_px(&self) -> f64 {
        return self.px;
    }

    pub fn get_py(&self) -> f64 {
        return self.py;
    }
}
