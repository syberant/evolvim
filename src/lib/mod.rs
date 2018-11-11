//! Evolvim

// TODO: ensure documentation for everything, use this to generate warnings
// #![warn(missing_docs)]
// Use this to generate errors
// #![deny(missing_docs)]

#![deny(unsafe_code)]

#[macro_use]
extern crate serde_derive;
extern crate serde;

pub mod board;
pub mod brain;
pub mod climate;
pub mod constants;
pub mod sbip;
pub mod softbody;
pub mod terrain;

pub use self::board::*;
pub use self::brain::*;
pub use self::climate::Climate;
pub use self::sbip::*;
pub use self::softbody::*;
pub use self::terrain::*;
