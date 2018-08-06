use super::*;
use std::ops::Range;

pub struct View {
    precise_x: f64,
    precise_y: f64,

    max_x: usize,
    max_y: usize,

    tiles_on_height: usize,
    tiles_on_width: usize,

    base_tile_width: f64,
    tile_width: f64,

    pub board: Board,

    drag: Dragging,
}

impl Default for View {
    fn default() -> Self {
        let base_tile_width = 20.0;
        View {
            precise_x: 0.0,
            precise_y: 0.0,

            max_x: 100,
            max_y: 100,

            tiles_on_height: 50,
            tiles_on_width: 50,

            base_tile_width,
            tile_width: base_tile_width,

            board: Board::default(),

            drag: Dragging::None,
        }
    }
}

impl View {
    pub fn on_mouse_release(&mut self) {
        use self::Dragging::*;

        self.drag = None;
    }

    pub fn on_mouse_press(&mut self) {
        use self::Dragging::*;

        self.drag = Board;
    }

    pub fn on_mouse_move(&mut self, change_x: f64, change_y: f64) {
        use self::Dragging::*;

        match self.drag {
            Board => {
                self.change_precise_x(-change_x);
                self.change_precise_y(-change_y);
            }
            _ => {}
        }
    }
}

impl View {
    pub fn get_tile_size(&self) -> f64 {
        return self.tile_width;
    }

    pub fn get_x(&self) -> usize {
        return self.precise_x.floor() as usize;
    }

    pub fn get_y(&self) -> usize {
        return self.precise_y.floor() as usize;
    }

    pub fn get_precise_x(&self) -> f64 {
        return self.precise_x;
    }

    pub fn get_precise_y(&self) -> f64 {
        return self.precise_y;
    }

    fn change_precise_x(&mut self, change: f64) {
        self.precise_x = (self.precise_x + change)
            .max(0.0)
            .min((self.max_x - self.tiles_on_width) as f64);
    }

    fn change_precise_y(&mut self, change: f64) {
        self.precise_y = (self.precise_y + change)
            .max(0.0)
            .min((self.max_y - self.tiles_on_height) as f64);
    }

    pub fn get_x_range(&self) -> Range<usize> {
        assert!(self.get_x() + self.tiles_on_width <= self.max_x);

        Range {
            start: self.get_x(),
            end: self.get_x() + self.tiles_on_width,
        }
    }

    pub fn get_y_range(&self) -> Range<usize> {
        assert!(self.get_y() + self.tiles_on_height <= self.max_y);

        Range {
            start: self.get_y(),
            end: self.get_y() + self.tiles_on_height,
        }
    }

    pub fn prepare_for_drawing(&mut self) {
        let time = self.board.get_time();
        // let x_range = self.get_x_range();
        // let y_range = self.get_y_range();

        // self.board
        //     .terrain
        //     .update_all_at(time, &self.board.climate, x_range, y_range);
        self.board.terrain.update_all(time, &self.board.climate);
    }
}

impl View {
    pub fn draw(&self, context: Context, graphics: &mut G2d) {
        self.board.terrain.draw(context, graphics, &self);
    }
}
