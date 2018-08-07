use super::*;

#[derive(Debug)]
pub enum Dragging {
    Board,
    MinTemperature,
    MaxTemperature,
    None,
}

pub struct MouseCoordinate(f64, f64);

impl MouseCoordinate {
    pub fn new(x: f64, y: f64) -> Self {
        return MouseCoordinate(x, y);
    }

    /// Converts this into a coordinate of the board.
    ///
    /// # Arguments
    /// `base_x` and `base_y` should be the "start" of the window, i.e. how much `Tile`s you skip displaying.
    ///
    /// `scale` should be the size of a single `Tile` in pixels.
    pub fn into_board_precise_coordinate(
        &self,
        base_x: f64,
        base_y: f64,
        scale: f64,
    ) -> BoardPreciseCoordinate {
        let x = base_x + self.0 / scale;
        let y = base_y + self.1 / scale;

        if x < 0.0 || y < 0.0 {
            panic!("Mouse moved outside of window.");
        }

        return BoardPreciseCoordinate(x, y);
    }

    pub fn into_board_coordinate(&self, base_x: f64, base_y: f64, scale: f64) -> BoardCoordinate {
        return BoardCoordinate::from(self.into_board_precise_coordinate(base_x, base_y, scale));
    }
}
