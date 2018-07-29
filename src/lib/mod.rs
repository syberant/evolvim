pub mod board;
pub mod creature;
pub mod rock;
pub mod sbip;
pub mod softbody;
pub mod tile;

pub use board::Board;
pub use creature::Creature;
pub use rock::Rock;
pub use sbip::{SoftBodiesAt, SoftBodiesInPositions};
pub use softbody::SoftBody;
pub use tile::Tile;

pub trait Drawable {
    fn draw(&self);
}

pub enum Dragging {
    Board,
    MinTemperature,
    MaxTemperature,
}
