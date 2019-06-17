use self::constants::*;
use super::*;
use crate::ecs_board::BoardSize;

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
