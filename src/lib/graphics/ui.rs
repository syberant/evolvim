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

    pub fn into_board_precise_coordinate(self, base_x: f64, base_y: f64) -> BoardPreciseCoordinate {
        let x = base_x + self.0;
        let y = base_y + self.1;

        return BoardPreciseCoordinate(x, y);
    }

    pub fn into_board_coordinate(self, base_x: f64, base_y: f64) -> BoardCoordinate {
        return BoardCoordinate::from(self.into_board_precise_coordinate(base_x, base_y));
    }
}
