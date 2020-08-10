use crate::ecs_board::BoardSize;
use crate::brain::Brain;
use crate::climate::Climate;
use crate::softbody::Creature;
use crate::terrain::Terrain;
use crate::time::Time;
use specs::{Entities, LazyUpdate, Read, ReadExpect, ReadStorage, System, WriteExpect, WriteStorage};

mod update_resources;
mod update_creatures;
mod remove_dead_creatures;
mod creatures_reproduce;
mod refill_creatures;
mod physics_step;

pub use update_resources::UpdateResources;
pub use update_creatures::UpdateCreatures;
pub use remove_dead_creatures::RemoveDeadCreatures;
pub use creatures_reproduce::CreaturesReproduce;
pub use refill_creatures::RefillCreatures;
pub use physics_step::PhysicsStep;