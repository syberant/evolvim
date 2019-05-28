use self::constants::*;
use super::*;

mod creature;
mod rock;

pub use self::creature::*;
pub use self::rock::*;
use nphysics2d::object::BodyHandle;
type World = nphysics2d::world::World<f64>;
type RigidBody = nphysics2d::object::RigidBody<f64>;

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

impl<B: 'static> HLSoftBody<B> {
    pub fn borrow<'a>(&self, world: &'a World) -> &'a SoftBody<B> {
        let rigid_body = world.rigid_body(self.0).unwrap();
        let data = rigid_body.user_data().unwrap();

        let sb = data.downcast_ref::<SoftBody<B>>().unwrap();

        sb
    }

    /// Wrapper function
    pub fn borrow_mut<'a>(&self, world: &'a mut World) -> &'a mut SoftBody<B> {
        let rigid_body = world.rigid_body_mut(self.0).unwrap();
        let data = rigid_body.user_data_mut().unwrap();

        let sb = data.downcast_mut::<SoftBody<B>>().unwrap();

        sb
    }

    /// Returns a boolean indicating whether this `HLSoftBody` is currently borrowed, useful for debugging.
    pub fn can_borrow_mut(&self) -> bool {
        unimplemented!()
    }

    /// Consume this thing and return the value it holds
    pub fn into_inner(self) -> SoftBody<B> {
        unimplemented!()
    }

    pub fn get_body<'a>(&self, world: &'a World) -> &'a RigidBody {
        world.rigid_body(self.0).unwrap()
    }

    pub fn from_creature(creature: SoftBody<B>, world: &mut World) -> Self {
        HLSoftBody(
            make_physics_creature(world, &creature),
            std::marker::PhantomData,
        )
    }

    /// This function requires a reference to a `Board`.
    /// This is usually impossible so you'll have to turn to `unsafe`.
    pub fn return_to_earth(
        &mut self,
        time: f64,
        board_size: BoardSize,
        terrain: &mut Terrain,
        climate: &Climate,
        world: &mut World,
    ) {
        // To keep the borrowchecker happy.
        {
            let self_deref = self.borrow_mut(world);

            for _i in 0..PIECES {
                let tile_pos = self_deref.get_random_covered_tile(board_size);
                terrain.add_food_or_nothing_at(tile_pos, self_deref.get_energy() / PIECES as f64);

                terrain.update_at(tile_pos, time, climate);
            }
        }

        // SBIP automatically get's wiped every update.
    }
}

impl<B: NeuralNet + Intentions + RecombinationInfinite + 'static> HLSoftBody<B> {
    /// Returns a new creature if there's a birth, otherwise returns `None`
    // TODO: cleanup
    pub fn try_reproduce(
        &mut self,
        time: f64,
        board_size: BoardSize,
        world: &mut World,
    ) -> Option<HLSoftBody<B>> {
        if self.borrow(world).wants_primary_birth(time) {
            let self_px = self.borrow(world).get_px();
            let self_py = self.borrow(world).get_py();
            let self_radius = self.borrow(world).get_radius();

            let mut colliders: SoftBodiesAt<B> = unimplemented!();

            // Remove self
            colliders.remove_softbody(self.clone());

            let mut parents: Vec<HLSoftBody<B>> = colliders
                .into_iter()
                .filter(|rc_soft| {
                    let c = rc_soft.borrow(world);
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
                .fold(0.0, |acc, c| acc + c.borrow(world).get_baby_energy());

            if available_energy > BABY_SIZE {
                let energy = BABY_SIZE;

                // Giving birth costs energy
                parents.iter_mut().for_each(|c| {
                    let mut c = c.borrow_mut(world);

                    let energy_to_lose = energy * (c.get_baby_energy() / available_energy);
                    c.lose_energy(energy_to_lose);
                });
                let par: Vec<&SoftBody<B>> = parents.iter().map(|c| c.borrow(world)).collect();

                let sb = HLSoftBody::from_creature(Creature::new_baby(&par, energy, time), world);

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
