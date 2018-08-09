use super::*;
use std::ops::Range;

/// The view part of MVC (Model-View-Controller), currently takes on jobs for the controller too.
///
/// TODO: Provide adequate error handling when the mouse leaves the window.
///
/// TODO 2: Move the "controller" parts over to another struct.
pub struct View {
    precise_x: f64,
    precise_y: f64,

    max_x: usize,
    max_y: usize,

    tiles_on_height: usize,
    tiles_on_width: usize,

    _base_tile_width: f64,
    tile_width: f64,

    pub board: Board,

    pub mouse: MouseCoordinate,

    drag: Dragging,
}

impl Default for View {
    fn default() -> Self {
        let base_tile_width = 100.0;
        View {
            precise_x: 0.0,
            precise_y: 0.0,

            max_x: 100,
            max_y: 100,

            tiles_on_height: 10,
            tiles_on_width: 10,

            _base_tile_width: base_tile_width,
            tile_width: base_tile_width,

            board: Board::default(),

            mouse: MouseCoordinate::new(0.0, 0.0),

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
                self.change_precise_x(-change_x / MOUSE_SPEED);
                self.change_precise_y(-change_y / MOUSE_SPEED);
            }
            _ => {}
        }
    }

    pub fn update_mouse(&mut self, x: f64, y: f64) {
        self.mouse = MouseCoordinate::new(x, y);
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
    pub fn draw<C, G>(&self, context: Context, graphics: &mut G, glyphs: &mut C)
    where
        C: CharacterCache,
        C::Error: std::fmt::Debug,
        G: Graphics<Texture = C::Texture>,
    {
        self.board.terrain.draw(context, graphics, glyphs, &self);

        for c in &self.board.creatures {
            c.get_creature().base.draw(context, graphics, &self);
        }

        if let Some(c_pointer) = self.board.selected_creature {
            unsafe {
                (*c_pointer).brain.draw(context, graphics, glyphs, &self);
            }
        }
    }
}
