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

#[macro_use]
extern crate serde_derive;

pub mod board;
pub mod brain;
pub mod climate;
pub mod constants;
pub mod neat;
pub mod sbip;
pub mod serde_structs;
pub mod softbody;
pub mod terrain;

pub use self::board::*;
pub use self::brain::*;
pub use self::climate::Climate;
pub use self::sbip::*;
pub use self::softbody::*;
pub use self::terrain::*;
