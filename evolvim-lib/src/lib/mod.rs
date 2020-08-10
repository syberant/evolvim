//! Evolvim

// TODO: ensure documentation for everything, use this to generate warnings
// #![warn(missing_docs)]
// Use this to generate errors
// #![deny(missing_docs)]

// #![deny(unsafe_code)]

// Force the explicit marking of trait objects with the dyn syntax
#![deny(bare_trait_objects)]

extern crate nalgebra;
extern crate ncollide2d;
extern crate noise;
extern crate nphysics2d;
extern crate rand;
#[cfg(multithreading)]
extern crate rayon;
extern crate serde;
extern crate specs;

#[macro_use]
extern crate serde_derive;

pub mod brain;
pub mod climate;
pub mod constants;
pub mod ecs_board;
pub mod neat;
pub mod softbody;
pub mod systems;
pub mod terrain;
pub mod time;

pub use self::brain::*;
pub use self::climate::Climate;
pub use self::softbody::*;
pub use self::terrain::*;
