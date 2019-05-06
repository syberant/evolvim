use self::constants::*;
use super::*;

mod creature;
mod rock;

pub use self::creature::*;
pub use self::rock::*;
use std::cell::{Ref, RefMut};

const COLLISION_FORCE: f64 = 0.01;
const PIECES: usize = 20;
const AGE_FACTOR: f64 = 1.0;
const MATURE_AGE: f64 = 0.01;

/// Higher-Level SoftBody
///
/// This is a wrapper struct providing some useful functions.
///
/// TODO: come up with a better name.
pub struct HLSoftBody<B = Brain>(BodyHandle, std::marker::PhantomData<B>);

impl<B> Clone for HLSoftBody<B> {
    fn clone(&self) -> Self {
        HLSoftBody(self.0.clone(), std::marker::PhantomData)
    }
}

impl<B> PartialEq<HLSoftBody<B>> for HLSoftBody<B> {
    fn eq(&self, rhs: &HLSoftBody<B>) -> bool {
        self.0 == rhs.0
    }
}

impl<B> HLSoftBody<B> {
    /// Wrapper function
    pub fn borrow(&self) -> std::cell::Ref<SoftBody<B>> {
        unimplemented!()
    }

    /// Wrapper function
    pub fn borrow_mut(&self) -> std::cell::RefMut<SoftBody<B>> {
        unimplemented!()
    }

    /// Returns a boolean indicating whether this `HLSoftBody` is currently borrowed, useful for debugging.
    pub fn can_borrow_mut(&self) -> bool {
        unimplemented!()
    }

    /// Consume this thing and return the value it holds
    pub fn into_inner(self) -> SoftBody<B> {
        unimplemented!()
    }

    pub fn from_creature(creature: SoftBody<B>, world: &mut World) -> Self {
        HLSoftBody(
            make_physics_creature(world, &creature),
            std::marker::PhantomData,
        )
    }

    /// Calls the same function on all types and updates `SoftBodiesInPositions` by calling `set_sbip`.
    pub fn apply_motions(
        &self,
        time_step: f64,
        board_size: BoardSize,
        terrain: &Terrain,
        sbip: &mut SoftBodiesInPositions<B>,
    ) {
        use std::ops::DerefMut;

        self.borrow_mut()
            .deref_mut()
            .apply_motions(time_step, terrain, board_size);

        self.set_sbip(sbip, board_size);
    }

    /// Updates `SoftBodiesInPositions` and updates itself by calling `update_sbip_variables()`.
    pub fn set_sbip(&self, sbip: &mut SoftBodiesInPositions<B>, board_size: BoardSize) {
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
    pub fn remove_from_sbip(&mut self, sbip: &mut SoftBodiesInPositions<B>) {
        for x in self.borrow().current_x_range() {
            for y in self.borrow().current_y_range() {
                sbip.remove_soft_body_at(x, y, self.clone());
            }
        }
    }

    /// Checks for collision and adjusts velocity if that's the case.
    ///
    /// TODO: clean up the many uses of `borrow()`
    pub fn collide(&self, sbip: &SoftBodiesInPositions<B>) {
        let mut self_br = self.borrow_mut();
        let mut colliders = self_br.get_colliders(sbip);

        // Remove self, if you don't do this then the program will crash because you're borrowing self twice.
        colliders.remove_softbody(self.clone());

        let self_px = self_br.get_px();
        let self_py = self_br.get_py();
        let self_radius = self_br.get_radius();
        let self_mass = self_br.get_mass();

        for collider_rc in colliders {
            let collider = collider_rc.borrow();

            let (collider_px, collider_py) = (collider.get_px(), collider.get_py());
            let distance = distance(self_px, self_py, collider_px, collider_py);

            let combined_radius = self_radius + collider.get_radius();

            if distance < combined_radius {
                let force = combined_radius * COLLISION_FORCE;

                let add_vx = (self_px - collider_px) / distance * force / self_mass;
                let add_vy = (self_py - collider_py) / distance * force / self_mass;

                // This is where self is needed to be borrowed mutably.
                self_br.add_vx(add_vx);
                self_br.add_vy(add_vy);
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
        sbip: &mut SoftBodiesInPositions<B>,
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
}



impl<B: NeuralNet + Intentions + RecombinationInfinite> HLSoftBody<B> {
    /// Returns a new creature if there's a birth, otherwise returns `None`
    // TODO: cleanup
    pub fn try_reproduce(
        &mut self,
        time: f64,
        sbip: &mut SoftBodiesInPositions<B>,
        board_size: BoardSize,
    ) -> Option<HLSoftBody<B>> {
        if self.borrow().wants_primary_birth(time) {
            let self_px = self.borrow().get_px();
            let self_py = self.borrow().get_py();
            let self_radius = self.borrow().get_radius();

            let mut colliders = self.borrow().get_colliders(sbip);

            // Remove self
            colliders.remove_softbody(self.clone());

            let mut parents: Vec<HLSoftBody<B>> = colliders
                .into_iter()
                .filter(|rc_soft| {
                    let c = rc_soft.borrow();
                    let dist = distance(self_px, self_py, c.get_px(), c.get_py());
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
                let par: Vec<&SoftBody<B>> = parents.iter().map(|c| c.borrow(world)).collect();

                let sb = HLSoftBody::from(Creature::new_baby(&par, energy, time));

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

pub type SoftBody<B = Brain> = Creature<B>;

// Here are all the functions only applicable to `Creature`s.
impl<B: Intentions> SoftBody<B> {
    fn wants_primary_birth(&self, time: f64) -> bool {
        self.get_energy() > SAFE_SIZE
            && self.brain.wants_birth() > 0.0
            && self.get_age(time) > MATURE_AGE
    }
}

impl<B> SoftBody<B> {
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

fn make_physics_creature<B>(world: &mut World, cr: &Creature<B>) -> BodyHandle {
    use nalgebra::Vector2;
    use ncollide2d::shape::{Ball, ShapeHandle};
    use nphysics2d::object::{ColliderDesc, RigidBodyDesc};

    let radius = cr.get_radius();

    // Create the ColliderDesc
    let shape = ShapeHandle::new(Ball::new(radius));
    let collide_handle = ColliderDesc::new(shape);

    let mass = cr.get_mass();
    let position = Vector2::new(cr.get_px(), cr.get_py());

    // Build the RigidBody
    let rigid_body = RigidBodyDesc::new()
        .mass(mass)
        .translation(position)
        .collider(&collide_handle)
        .build(world);

    return rigid_body.handle();
}
