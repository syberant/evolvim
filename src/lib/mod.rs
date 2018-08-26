//! Evolvim

// TODO: ensure documentation for everything, use this to generate warnings
// #![warn(missing_docs)]
// Use this to generate errors
// #![deny(missing_docs)]

pub mod board;
pub mod brain;
pub mod climate;
pub mod constants;
pub mod graphics;
pub mod sbip;
pub mod softbody;
pub mod terrain;

pub use board::*;
pub use brain::*;
pub use climate::Climate;
pub use sbip::*;
pub use softbody::*;
pub use terrain::*;
