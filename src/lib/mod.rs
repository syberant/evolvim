pub mod board;
pub mod brain;
pub mod climate;
pub mod constants;
pub mod sbip;
pub mod softbody;
pub mod tile;

pub use board::*;
pub use brain::*;
pub use climate::Climate;
pub use sbip::*;
pub use softbody::*;
pub use tile::Tile;

pub trait Drawable {
    fn draw(&self);
}

pub enum Dragging {
    Board,
    MinTemperature,
    MaxTemperature,
}
