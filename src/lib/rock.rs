use super::*;
use std::f64::consts::PI;

const FRICTION: f64 = 0.004;
const ENERGY_DENSITY: f64 = 1.0
    / (super::creature::MINIMUM_SURVIVABLE_SIZE * super::creature::MINIMUM_SURVIVABLE_SIZE * PI);
const FIGHT_RANGE: f64 = 2.0;

pub struct Rock {
    // Position
    px: f64,
    py: f64,
    // Velocity
    vx: f64,
    vy: f64,
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

    /// Takes a new coordinate and changes itself to it without any checking.
    ///
    /// This function is unsafe because we are not checking for moving outside of the board or not,
    /// please use `set_body_x()` which has some checks included.
    pub fn set_px(&mut self, new_x: f64) {
        self.px = new_x;
    }

    /// Takes a new coordinate and changes itself to it without any checking.
    ///
    /// This function is unsafe because we are not checking for moving outside of the board or not,
    /// please use `set_body_y()` which has some checks included.
    pub fn set_py(&mut self, new_y: f64) {
        self.py = new_y;
    }

    pub fn get_px(&self) -> f64 {
        return self.px;
    }

    pub fn get_py(&self) -> f64 {
        return self.py;
    }

    pub fn add_vx(&mut self, value_to_add: f64) {
        self.vx += value_to_add;
    }

    pub fn add_vy(&mut self, value_to_add: f64) {
        self.vy += value_to_add;
    }

    /// Return this body's radius, used for collissions, etc.
    pub fn get_radius(&self) -> f64 {
        return (self.energy / ENERGY_DENSITY / PI).sqrt().max(0.0);
    }

    /// Return this body's mass, used for physics, etc.
    pub fn get_mass(&self) -> f64 {
        return self.energy / ENERGY_DENSITY * self.density;
    }

    /// Updates positions and velocities based on `time_step` and some physics formulae.
    ///
    /// NOTE: Does not call `set_sbip`.
    pub fn apply_motions(&mut self, time_step: f64, board: &Board) {
        let new_px = self.px + self.vx * time_step;
        let new_py = self.py + self.vy * time_step;
        self.set_body_x(new_px, board.get_board_width());
        self.set_body_y(new_py, board.get_board_height());

        self.vx *= 0f64.max(1.0 - FRICTION / self.get_mass());
        self.vy *= 0f64.max(1.0 - FRICTION / self.get_mass());
    }

    pub fn update_sbip_variables(&mut self, board: &Board) {
        let radius = self.get_radius() * FIGHT_RANGE;

        self.prev_sbip_min_x = self.sbip_min_x;
        self.prev_sbip_min_y = self.sbip_min_y;
        self.prev_sbip_max_x = self.sbip_max_x;
        self.prev_sbip_max_y = self.sbip_max_y;

        let board_width = board.get_board_width();
        let board_height = board.get_board_height();
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
}
