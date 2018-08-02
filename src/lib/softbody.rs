use super::*;
use constants::*;

const COLLISION_FORCE: f64 = 0.01;
const PIECES: usize = 20;
const AGE_FACTOR: f64 = 1.0;
const MATURE_AGE: f64 = 0.01;
const METABOLISM_ENERGY: f64 = 0.004;

pub enum SoftBody {
    Rock(Rock),
    Creature(Creature),
}

impl SoftBody {
    /// Returns true if this `SoftBody` is a creature and false otherwise.
    pub fn is_creature(&self) -> bool {
        match self {
            SoftBody::Rock(_) => false,
            SoftBody::Creature(_) => true,
        }
    }

    /// Returns true if this `SoftBody` is a rock and false otherwise.
    pub fn is_rock(&self) -> bool {
        match self {
            SoftBody::Rock(_) => true,
            SoftBody::Creature(_) => false,
        }
    }

    pub fn new_random_creature(board_size: BoardSize, time: f64) -> SoftBody {
        SoftBody::Creature(Creature::new_random(board_size, time))
    }

    /// Checks if the center is inside of the world, possibly corrects it and returns it.
    pub fn check_center_x(x: usize, board_width: usize) -> usize {
        return x.max(0).min(board_width - 1);
    }

    /// Checks if the center is inside of the world, possibly corrects it and returns it.
    pub fn check_center_y(y: usize, board_height: usize) -> usize {
        return y.max(0).min(board_height - 1);
    }

    /// Updates `SoftBodiesInPositions` and updates itself by calling `update_sbip_variables()`.
    pub fn set_sbip(&mut self, sbip: &mut SoftBodiesInPositions, board_size: BoardSize) {
        // TODO: Look for optimizations here by cleaning and filling sbip more intelligently.

        self.update_sbip_variables(board_size);

        if self.moved_between_tiles() {
            for x in self.previous_x_range() {
                for y in self.previous_y_range() {
                    // Prevents deleting tiles we are currently in.
                    if !self.is_in_tile(x, y) {
                        sbip.remove_soft_body_at(x, y, &self);
                    }
                }
            }

            for x in self.current_x_range() {
                for y in self.current_y_range() {
                    // Prevents duplicate entries.
                    if !self.was_in_tile(x, y) {
                        sbip.add_soft_body_at(x, y, &self);
                    }

                    // // Just tries to add it, even when it has already been added.
                    // sbip.add_soft_body_at(x, y, &self);
                }
            }
        }
    }

    /// Completely removes this `SoftBody` from `sbip`.
    ///
    /// NOTE: `SoftBody` is added again when `set_sbip` is called.
    pub fn remove_from_sbip(&mut self, sbip: &mut SoftBodiesInPositions) {
        for x in self.current_x_range() {
            for y in self.current_y_range() {
                sbip.remove_soft_body_at(x, y, self);
            }
        }
    }

    /// Returns the distance between two points.
    ///
    /// Uses the Pythagorean theorem: A^2 + B^2 = C^2.
    pub fn distance(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
        ((x1 - x2).powi(2) + (y1 - y2).powi(2)).sqrt()
    }

    pub fn collide(&mut self, sbip: &SoftBodiesInPositions) {
        let mut colliders: SoftBodiesAt = std::collections::HashSet::new();

        // Copy all possible colliders into `colliders`.
        // NOTE: possibly tries to add one collider multiple times but this doesn't matter since `HashSet<T>` can only contain unique entries.
        for x in self.current_x_range() {
            for y in self.current_y_range() {
                for i in sbip.get_soft_bodies_at(x, y) {
                    colliders.insert(*i);
                }
            }
        }

        // Remove self
        colliders.remove(&(self as *const SoftBody));

        for collider in colliders {
            // Unsafe here is fine (and needed) as long as the references in `sbip` are okay.
            unsafe {
                let (collider_px, collider_py) = ((*collider).get_px(), (*collider).get_py());
                let distance =
                    SoftBody::distance(self.get_px(), self.get_py(), collider_px, collider_py);

                let combined_radius = self.get_radius() + (*collider).get_radius();

                if distance < combined_radius {
                    let force = combined_radius * COLLISION_FORCE;

                    let add_vx =
                        ((self.get_px() - collider_px) / distance) * force * self.get_mass();
                    let add_vy =
                        ((self.get_py() - collider_py) / distance) * force * self.get_mass();

                    self.add_vx(add_vx);
                    self.add_vy(add_vy);
                }
            }
        }

        // TODO: translate this from Processing to Rust
        // fight_level = 0;
    }
}

// Here are all the functions only applicable to `Creature`s.
impl SoftBody {
    pub fn get_creature(&self) -> &Creature {
        match self {
            SoftBody::Creature(c) => c,
            _ => panic!("This `SoftBody` is not a `Creature`! It looks like you accidentally called `get_creature`!"),
        }
    }

    pub fn get_creature_mut(&mut self) -> &mut Creature {
        match self {
            SoftBody::Creature(c) => c,
            _ => panic!("This `SoftBody` is not a `Creature`! It looks like you accidentally called `get_creature_mut`!"),
        }
    }

    pub fn get_birth_time(&self) -> f64 {
        return self.get_creature().get_birth_time();
    }

    pub unsafe fn return_to_earth(&mut self, board: *mut Board, board_size: BoardSize) {
        for _i in 0..PIECES {
            let tile_pos = self.get_random_covered_tile(board_size);
            let tile = &mut (*board).tiles[tile_pos.0][tile_pos.1];
            tile.add_food_or_nothing(self.get_energy() / PIECES as f64);

            // TODO: check if this is neccessary and fix this mess!
            tile.update((*board).get_time(), &(*board).climate);
        }

        self.remove_from_sbip(&mut (*board).soft_bodies_in_positions);

        // Unselect this creature if it was selected.
        (*board).unselect_if_dead(self.get_creature_mut());
    }

    pub fn use_brain(&mut self, time_step: f64, use_output: bool, board: &mut Board) {
        let input = self.get_input();
        let unsafe_creature = self.get_creature_mut() as *mut Creature;
        let creature = self.get_creature_mut();
        let output = creature.brain.run(input);

        let time = board.get_time();

        if use_output {
            creature.base.accelerate(output[1], time_step);
            creature.base.turn(output[2], time_step);

            // TODO: clean this mess.
            unsafe {
                let board_size = board.get_board_size();
                let mut tile = creature
                    .base
                    .get_random_covered_tile_mut(board_size, &mut board.tiles);
                (*unsafe_creature).eat(output[3], time_step, time, &board.climate, &mut tile);
            }

            // Fight
            // unimplemented!();

            unsafe {
                // Reproduce
                if output[5] > 0.0
                    && (*unsafe_creature).get_age(time) >= MATURE_AGE
                    && creature.base.get_energy() > SAFE_SIZE
                {
                    // unimplemented!();
                }
            }

            unsafe {
                (*unsafe_creature).set_mouth_hue(output[6]);
            }
        }
    }

    fn get_input(&self) -> BrainInput {
        let mut input = [0.0; 9];

        let creature = self.get_creature();
        input[0] = creature.get_energy();
        input[1] = creature.get_mouth_hue();

        return input;
    }

    /// Performs the energy requirement to keep living.
    pub fn metabolize(&mut self, time_step: f64, board: &Board) {
        // TODO: fix ugly code.
        let age = AGE_FACTOR * (board.get_time() - self.get_birth_time());
        let creature = self.get_creature_mut();
        let energy_to_lose = creature.get_energy() * METABOLISM_ENERGY * age * time_step;
        creature.lose_energy(energy_to_lose);

        // Creature should die if it doesn't have enough energy, this is done by `Board`.
    }

    pub fn should_die(&self) -> bool {
        return self.get_creature().should_die();
    }
}

// Here are all the functions which merely call the same function on the underlying types.
impl SoftBody {
    /// Calls the same function on all types and updates `SoftBodiesInPositions` by calling `set_sbip`.
    pub fn apply_motions(&mut self, time_step: f64, board: &mut Board) {
        match self {
            SoftBody::Rock(b) => b.apply_motions(time_step, board.get_board_size()),
            SoftBody::Creature(c) => c.apply_motions(time_step, board),
        };
        let board_size = board.get_board_size();
        self.set_sbip(&mut board.soft_bodies_in_positions, board_size);
    }

    fn get_random_covered_tile(&self, board_size: BoardSize) -> (usize, usize) {
        match self {
            SoftBody::Rock(b) => b.get_random_covered_tile(board_size),
            SoftBody::Creature(c) => c.base.get_random_covered_tile(board_size),
        }
    }

    /// Returns `true` if this `SoftBody` has moved between tiles since the last update.
    ///
    /// Used to determine if `SoftBodiesInPosisitions` should be updated and `set_sbip` should be called.
    fn moved_between_tiles(&self) -> bool {
        match self {
            SoftBody::Rock(b) => b.moved_between_tiles(),
            SoftBody::Creature(c) => c.base.moved_between_tiles(),
        }
    }

    fn is_in_tile(&self, x: usize, y: usize) -> bool {
        match self {
            SoftBody::Rock(b) => b.is_in_tile(x, y),
            SoftBody::Creature(c) => c.base.is_in_tile(x, y),
        }
    }

    fn was_in_tile(&self, x: usize, y: usize) -> bool {
        match self {
            SoftBody::Rock(b) => b.was_in_tile(x, y),
            SoftBody::Creature(c) => c.base.was_in_tile(x, y),
        }
    }

    fn previous_x_range(&self) -> std::ops::RangeInclusive<usize> {
        match self {
            SoftBody::Rock(b) => b.previous_x_range(),
            SoftBody::Creature(c) => c.base.previous_x_range(),
        }
    }

    fn previous_y_range(&self) -> std::ops::RangeInclusive<usize> {
        match self {
            SoftBody::Rock(b) => b.previous_y_range(),
            SoftBody::Creature(c) => c.base.previous_y_range(),
        }
    }

    fn current_x_range(&self) -> std::ops::RangeInclusive<usize> {
        match self {
            SoftBody::Rock(b) => b.current_x_range(),
            SoftBody::Creature(c) => c.base.current_x_range(),
        }
    }

    fn current_y_range(&self) -> std::ops::RangeInclusive<usize> {
        match self {
            SoftBody::Rock(b) => b.current_y_range(),
            SoftBody::Creature(c) => c.base.current_y_range(),
        }
    }

    fn update_sbip_variables(&mut self, board_size: BoardSize) {
        match self {
            SoftBody::Rock(b) => b.update_sbip_variables(board_size),
            SoftBody::Creature(c) => c.base.update_sbip_variables(board_size),
        };
    }

    fn get_px(&self) -> f64 {
        match self {
            SoftBody::Rock(b) => b.get_px(),
            SoftBody::Creature(c) => c.base.get_px(),
        }
    }

    fn get_py(&self) -> f64 {
        match self {
            SoftBody::Rock(b) => b.get_py(),
            SoftBody::Creature(c) => c.base.get_py(),
        }
    }

    fn get_radius(&self) -> f64 {
        match self {
            SoftBody::Rock(b) => b.get_radius(),
            SoftBody::Creature(c) => c.base.get_radius(),
        }
    }

    fn get_mass(&self) -> f64 {
        match self {
            SoftBody::Rock(b) => b.get_mass(),
            SoftBody::Creature(c) => c.base.get_mass(),
        }
    }

    fn get_energy(&self) -> f64 {
        match self {
            SoftBody::Rock(b) => b.get_energy(),
            SoftBody::Creature(c) => c.get_energy(),
        }
    }

    fn add_vx(&mut self, value_to_add: f64) {
        match self {
            SoftBody::Rock(b) => b.add_vx(value_to_add),
            SoftBody::Creature(c) => c.base.add_vx(value_to_add),
        }
    }

    fn add_vy(&mut self, value_to_add: f64) {
        match self {
            SoftBody::Rock(b) => b.add_vy(value_to_add),
            SoftBody::Creature(c) => c.base.add_vy(value_to_add),
        }
    }
}
