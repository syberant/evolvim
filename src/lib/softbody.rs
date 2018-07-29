use super::*;

const COLLISION_FORCE: f64 = 0.01;

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

    /// Checks if the center is inside of the world, possibly corrects it and returns it.
    pub fn check_center_x(x: usize, board_width: usize) -> usize {
        return x.max(0).min(board_width - 1);
    }

    /// Checks if the center is inside of the world, possibly corrects it and returns it.
    pub fn check_center_y(y: usize, board_height: usize) -> usize {
        return y.max(0).min(board_height - 1);
    }

    /// Updates `SoftBodiesInPositions` and updates itself by calling `update_sbip_variables()`.
    fn set_sbip(&mut self, board: &Board, sbip: &mut SoftBodiesInPositions) {
        // TODO: Look for optimizations here by cleaning and filling sbip more intelligently.

        self.update_sbip_variables(board);

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

    /// Returns the distance between two points.
    ///
    /// Uses the Pythagorean theorem: A^2 + B^2 = C^2.
    fn distance(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
        ((x1 - x2).powi(2) + (y1 - y2).powi(2)).sqrt()
    }

    pub fn collide(&mut self, sbip: &SoftBodiesInPositions) {
        let mut colliders: SoftBodiesAt = std::collections::HashSet::new();

        // Copy all possible colliders into `colliders`.
        for x in self.current_x_range() {
            for y in self.current_y_range() {
                for i in sbip.get_soft_bodies_at(x, y) {
                    colliders.insert(*i);
                }
            }
        }

        // Remove itself
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

// Here are all the functions which merely call the same function on the underlying types.
impl SoftBody {
    pub fn apply_motions(
        &mut self,
        time_step: f64,
        board: &Board,
        sbip: &mut SoftBodiesInPositions,
    ) {
        match self {
            SoftBody::Rock(b) => b.apply_motions(time_step, board),
            SoftBody::Creature(_b) => unimplemented!(),
        };

        self.set_sbip(board, sbip);
    }

    /// Returns `true` if this `SoftBody` has moved between tiles since the last update.
    ///
    /// Used to determine if `SoftBodiesInPosisitions` should be updated and `set_sbip` should be called.
    fn moved_between_tiles(&self) -> bool {
        match self {
            SoftBody::Rock(b) => b.moved_between_tiles(),
            SoftBody::Creature(_b) => unimplemented!(),
        }
    }

    fn is_in_tile(&self, x: usize, y: usize) -> bool {
        match self {
            SoftBody::Rock(b) => b.is_in_tile(x, y),
            SoftBody::Creature(_b) => unimplemented!(),
        }
    }

    fn was_in_tile(&self, x: usize, y: usize) -> bool {
        match self {
            SoftBody::Rock(b) => b.was_in_tile(x, y),
            SoftBody::Creature(_b) => unimplemented!(),
        }
    }

    fn previous_x_range(&self) -> std::ops::RangeInclusive<usize> {
        match self {
            SoftBody::Rock(b) => b.previous_x_range(),
            SoftBody::Creature(_b) => unimplemented!(),
        }
    }

    fn previous_y_range(&self) -> std::ops::RangeInclusive<usize> {
        match self {
            SoftBody::Rock(b) => b.previous_y_range(),
            SoftBody::Creature(_b) => unimplemented!(),
        }
    }

    fn current_x_range(&self) -> std::ops::RangeInclusive<usize> {
        match self {
            SoftBody::Rock(b) => b.current_x_range(),
            SoftBody::Creature(_b) => unimplemented!(),
        }
    }

    fn current_y_range(&self) -> std::ops::RangeInclusive<usize> {
        match self {
            SoftBody::Rock(b) => b.current_y_range(),
            SoftBody::Creature(_b) => unimplemented!(),
        }
    }

    fn update_sbip_variables(&mut self, board: &Board) {
        match self {
            SoftBody::Rock(b) => b.update_sbip_variables(board),
            SoftBody::Creature(_b) => unimplemented!(),
        };
    }

    fn get_px(&self) -> f64 {
        match self {
            SoftBody::Rock(b) => b.get_px(),
            SoftBody::Creature(_b) => unimplemented!(),
        }
    }

    fn get_py(&self) -> f64 {
        match self {
            SoftBody::Rock(b) => b.get_py(),
            SoftBody::Creature(_b) => unimplemented!(),
        }
    }

    fn get_radius(&self) -> f64 {
        match self {
            SoftBody::Rock(b) => b.get_radius(),
            SoftBody::Creature(_b) => unimplemented!(),
        }
    }

    fn get_mass(&self) -> f64 {
        match self {
            SoftBody::Rock(b) => b.get_mass(),
            SoftBody::Creature(_b) => unimplemented!(),
        }
    }

    fn add_vx(&mut self, value_to_add: f64) {
        match self {
            SoftBody::Rock(b) => b.add_vx(value_to_add),
            SoftBody::Creature(_b) => unimplemented!(),
        }
    }

    fn add_vy(&mut self, value_to_add: f64) {
        match self {
            SoftBody::Rock(b) => b.add_vy(value_to_add),
            SoftBody::Creature(_b) => unimplemented!(),
        }
    }
}
