use self::constants::*;
use super::*;

mod creature;
mod rock;

pub use self::creature::*;
pub use self::rock::*;
use std::cell::{Ref, RefMut};

#[cfg(multithreading)]
type ReferenceCounter = std::sync::Arc;
#[cfg(not(multithreading))]
type ReferenceCounter<A> = std::rc::Rc<A>;

#[cfg(multithreading)]
type MutPoint = std::sync::RwLock;
#[cfg(not(multithreading))]
type MutPoint<A> = std::cell::RefCell<A>;

const COLLISION_FORCE: f64 = 0.01;
const PIECES: usize = 20;
const AGE_FACTOR: f64 = 1.0;
const MATURE_AGE: f64 = 0.01;

/// Higher-Level SoftBody
///
/// This is a wrapper struct providing some useful functions.
///
/// TODO: come up with a better name.
pub struct HLSoftBody(ReferenceCounter<MutPoint<SoftBody>>);

impl From<SoftBody> for HLSoftBody {
    fn from(sb: SoftBody) -> HLSoftBody {
        HLSoftBody(ReferenceCounter::new(MutPoint::new(sb)))
    }
}

impl Clone for HLSoftBody {
    fn clone(&self) -> Self {
        HLSoftBody(ReferenceCounter::clone(&self.0))
    }
}

impl PartialEq<HLSoftBody> for HLSoftBody {
    fn eq(&self, rhs: &HLSoftBody) -> bool {
        ReferenceCounter::ptr_eq(&self.0, &rhs.0)
    }
}

impl HLSoftBody {
    /// Wrapper function
    #[cfg(multithreading)]
    pub fn borrow(&self) -> RwLockReadGuard<SoftBody> {
        return self.0.read().unwrap();
    }
    #[cfg(not(multithreading))]
    pub fn borrow(&self) -> Ref<SoftBody> {
        return self.0.borrow();
    }

    /// Wrapper function
    #[cfg(multithreading)]
    pub fn borrow_mut(&self) -> RwLockWriteGuard<SoftBody> {
        return self.0.write().unwrap();
    }
    #[cfg(not(multithreading))]
    pub fn borrow_mut(&self) -> RefMut<SoftBody> {
        return self.0.borrow_mut();
    }

    /// Returns a boolean indicating whether this `HLSoftBody` is currently borrowed, useful for debugging.
    #[cfg(multithreading)]
    pub fn can_borrow_mut(&self) -> bool {
        return self.0.try_write().is_ok();
    }
    #[cfg(not(multithreading))]
    pub fn can_borrow_mut(&self) -> bool {
        return self.0.try_borrow_mut().is_ok();
    }

    /// Calls the same function on all types and updates `SoftBodiesInPositions` by calling `set_sbip`.
    pub fn apply_motions(
        &self,
        time_step: f64,
        board_size: BoardSize,
        terrain: &Terrain,
        sbip: &mut SoftBodiesInPositions,
    ) {
        use std::ops::DerefMut;

        self.borrow_mut()
            .deref_mut()
            .apply_motions(time_step, terrain, board_size);

        self.set_sbip(sbip, board_size);
    }

    /// Updates `SoftBodiesInPositions` and updates itself by calling `update_sbip_variables()`.
    pub fn set_sbip(&self, sbip: &mut SoftBodiesInPositions, board_size: BoardSize) {
        // TODO: Look for optimizations here by cleaning and filling sbip more intelligently.

        let mut self_borrow = self.borrow_mut();

        self_borrow.update_sbip_variables(board_size);

        if self_borrow.moved_between_tiles() {
            for x in self_borrow.previous_x_range() {
                for y in self_borrow.previous_y_range() {
                    // Prevents deleting tiles we are currently in.
                    if !self_borrow.is_in_tile(x, y) {
                        sbip.remove_soft_body_at(x, y, self.clone());
                    }
                }
            }

            for x in self_borrow.current_x_range() {
                for y in self_borrow.current_y_range() {
                    // Prevents duplicate entries.
                    if !self_borrow.was_in_tile(x, y) {
                        sbip.add_soft_body_at(x, y, self.clone());
                    }
                }
            }
        }
    }

    /// Completely removes this `HLSoftBody` from `sbip`.
    ///
    /// NOTE: `HLSoftBody` is added again when `set_sbip` is called.
    pub fn remove_from_sbip(&mut self, sbip: &mut SoftBodiesInPositions) {
        for x in self.borrow().current_x_range() {
            for y in self.borrow().current_y_range() {
                sbip.remove_soft_body_at(x, y, self.clone());
            }
        }
    }

    pub fn fight(&mut self, amount: f64, time: f64, time_step: f64, sbip: &SoftBodiesInPositions) {
        let mut creature = self.borrow_mut();
        if amount > 0.0 && creature.get_age(time) >= MATURE_AGE {
            creature.lose_energy(amount * time_step * FIGHT_ENERGY);

            let self_x = creature.get_px();
            let self_y = creature.get_py();

            let mut colliders = creature.get_colliders(sbip);

            // Remove self
            colliders.remove_softbody(self.clone());

            for collider in colliders {
                let mut col = collider.borrow_mut();
                let distance = SoftBody::distance(self_x, self_y, col.get_px(), col.get_py());
                let combined_radius = creature.get_radius() * FIGHT_RANGE + col.get_radius();

                if distance < combined_radius {
                    // collider was hit, remove energy
                    col.lose_energy(amount * INJURED_ENERGY * time_step);
                }
            }
        }
    }

    /// Checks for collision and adjusts velocity if that's the case.
    ///
    /// TODO: clean up the many uses of `borrow()`
    pub fn collide(&self, sbip: &SoftBodiesInPositions) {
        let mut colliders = self.borrow().get_colliders(sbip);

        // Remove self
        colliders.remove_softbody(self.clone());

        let self_px = self.borrow().get_px();
        let self_py = self.borrow().get_py();
        let self_radius = self.borrow().get_radius();
        let self_mass = self.borrow().get_mass();

        for collider_rc in colliders {
            let collider = collider_rc.borrow();

            let (collider_px, collider_py) = (collider.get_px(), collider.get_py());
            let distance = SoftBody::distance(self_px, self_py, collider_px, collider_py);

            let combined_radius = self_radius + collider.get_radius();

            if distance < combined_radius {
                let force = combined_radius * COLLISION_FORCE;

                let add_vx = (self_px - collider_px) / distance * force / self_mass;
                let add_vy = (self_py - collider_py) / distance * force / self_mass;

                let mut self_mut_deref = self.borrow_mut();
                self_mut_deref.add_vx(add_vx);
                self_mut_deref.add_vy(add_vy);
            }
        }
    }

    /// This function requires a reference to a `Board`.
    /// This is usually impossible so you'll have to turn to `unsafe`.
    pub fn return_to_earth(
        &mut self,
        time: f64,
        board_size: BoardSize,
        terrain: &mut Terrain,
        climate: &Climate,
        sbip: &mut SoftBodiesInPositions,
    ) {
        // To keep the borrowchecker happy.
        {
            let self_deref = self.borrow_mut();

            for _i in 0..PIECES {
                let tile_pos = self_deref.get_random_covered_tile(board_size);
                terrain.add_food_or_nothing_at(tile_pos, self_deref.get_energy() / PIECES as f64);

                terrain.update_at(tile_pos, time, climate);
            }
        }

        self.remove_from_sbip(sbip);
    }

    fn wants_primary_birth(&self, time: f64) -> bool {
        let temp = self.borrow();

        temp.get_energy() > SAFE_SIZE
            && temp.brain.wants_birth() > 0.0
            && temp.get_age(time) > MATURE_AGE
    }

    /// Returns a new creature if there's a birth, otherwise returns `None`
    // TODO: cleanup
    pub fn try_reproduce(
        &mut self,
        time: f64,
        sbip: &mut SoftBodiesInPositions,
        board_size: BoardSize,
    ) -> Option<HLSoftBody> {
        if self.wants_primary_birth(time) {
            let self_px = self.borrow().get_px();
            let self_py = self.borrow().get_py();
            let self_radius = self.borrow().get_radius();

            let mut colliders = self.borrow().get_colliders(sbip);

            // Remove self
            colliders.remove_softbody(self.clone());

            let mut parents: Vec<HLSoftBody> = colliders
                .into_iter()
                .filter(|rc_soft| {
                    let c = rc_soft.borrow();
                    let dist = SoftBody::distance(self_px, self_py, c.get_px(), c.get_py());
                    let combined_radius = self_radius * FIGHT_RANGE + c.get_radius();

                    c.brain.wants_help_birth() > -1.0 // must be a willing creature
                            && dist < combined_radius // must be close enough

                    // TODO: find out if this addition to the Processing code works
                    // && c.get_age(time) >= MATURE_AGE // creature must be old enough
                    // && c.base.get_energy() > SAFE_SIZE
                })
                .collect();

            parents.push(self.clone());

            let available_energy = parents
                .iter()
                .fold(0.0, |acc, c| acc + c.borrow().get_baby_energy());

            if available_energy > BABY_SIZE {
                let energy = BABY_SIZE;

                // Giving birth costs energy
                parents.iter_mut().for_each(|c| {
                    let mut c = c.borrow_mut();

                    let energy_to_lose = energy * (c.get_baby_energy() / available_energy);
                    c.lose_energy(energy_to_lose);
                });

                let sb = HLSoftBody::from(Creature::new_baby(parents, energy, time));

                sb.set_sbip(sbip, board_size);
                sb.set_sbip(sbip, board_size);

                // Hooray! Return the little baby!
                Some(sb)
            } else {
                // There isn't enough energy available
                None
            }
        } else {
            // This creature can't give birth because of age, energy or because it doesn't want to.
            return None;
        }
    }
}

pub type SoftBody = Creature;

impl SoftBody {
    /// Wrapper function.
    pub fn new_random_creature(board_size: BoardSize, time: f64) -> SoftBody {
        Creature::new_random(board_size, time)
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
}

// Here are all the functions only applicable to `Creature`s.
impl SoftBody {
    /// Performs the energy requirement to keep living.
    pub fn metabolize(&mut self, time_step: f64, time: f64) {
        // TODO: fix ugly code.
        let age = AGE_FACTOR * (time - self.get_birth_time());
        let creature = self;
        let energy_to_lose = creature.get_energy() * METABOLISM_ENERGY * age * time_step;
        creature.lose_energy(energy_to_lose);

        // Creature should die if it doesn't have enough energy, this is done by `Board`.
    }
}
