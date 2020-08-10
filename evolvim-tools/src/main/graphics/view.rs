use super::*;
use crate::BrainType;
use std::ops::Range;
use lib_evolvim::ecs_board::ECSBoard;

/// The view part of MVC (Model-View-Controller), currently takes on jobs for the controller too.
///
/// TODO: Provide adequate error handling when the mouse leaves the window.
///
/// TODO 2: Move the "controller" parts over to another struct.
pub struct View<'a, 'b> {
    precise_x: f64,
    precise_y: f64,

    max_x: usize,
    max_y: usize,

    tiles_on_height: usize,
    tiles_on_width: usize,

    _base_tile_width: f64,
    tile_width: f64,

    pub board: ECSBoard<'a, 'b>,

    pub mouse: MouseCoordinate,

    drag: Dragging,
    mode: DisplayMode,
}

impl<'a, 'b> Default for View<'a, 'b> {
    fn default() -> Self {
        let board = ECSBoard::init((100, 100), 0.1);

        let base_tile_width = 100.0;

        View {
            precise_x: 0.0,
            precise_y: 0.0,

            max_x: board.get_board_width(),
            max_y: board.get_board_height(),

            tiles_on_height: 9,
            tiles_on_width: 10,

            _base_tile_width: base_tile_width,
            tile_width: base_tile_width,

            board,

            mouse: MouseCoordinate::new(0.0, 0.0),

            drag: Dragging::None,
            mode: DisplayMode::default(),
        }
    }
}

impl<'a, 'b> View<'a, 'b> {
    pub fn on_mouse_release(&mut self) {
        use self::Dragging::*;

        self.drag = None;

        // if let Some(exact_pos) = self.mouse.into_board_precise_coordinate(
        //     self.get_precise_x(),
        //     self.get_precise_y(),
        //     self.get_tile_size(),
        //     self.board.get_board_size(),
        // ) {
        //     let (x, y) = BoardCoordinate::from(exact_pos.clone());
        //     let soft_bodies = self.board.soft_bodies_in_positions.get_soft_bodies_at(x, y);
        //     let world = &self.board.world;

        //     for c_ref in soft_bodies {
        //         let c = c_ref.borrow(world);

        //         let px = c.get_px();
        //         let py = c.get_py();
        //         let radius = c.get_radius();

        //         let dist = lib_evolvim::softbody::distance(exact_pos.0, exact_pos.1, px, py);

        //         if dist < radius {
        //             self.board.selected_creature.select(c_ref.clone());
        //             break;
        //         }
        //     }
        // }
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

    pub fn switch_display_mode(&mut self) {
        use self::DisplayMode::*;

        self.mode = match self.mode {
            Normal => Tiles,
            Tiles => None,
            None => Normal,
        };
    }
}

impl<'a, 'b> View<'a, 'b> {
    pub fn get_tile_size(&self) -> f64 {
        return self.tile_width;
    }

    pub fn get_x(&self) -> usize {
        return self.precise_x.floor().max(0.0) as usize;
    }

    pub fn get_y(&self) -> usize {
        return self.precise_y.floor().max(0.0) as usize;
    }

    pub fn get_precise_x(&self) -> f64 {
        return self.precise_x;
    }

    pub fn get_precise_y(&self) -> f64 {
        return self.precise_y;
    }

    fn set_precise_x(&mut self, val: f64) {
        self.precise_x = val.max(0.0).min((self.max_x - self.tiles_on_width) as f64);
    }

    fn set_precise_y(&mut self, val: f64) {
        self.precise_y = val.max(0.0).min((self.max_y - self.tiles_on_height) as f64);
    }

    fn change_precise_x(&mut self, change: f64) {
        let val = self.precise_x + change;
        self.set_precise_x(val);
    }

    fn change_precise_y(&mut self, change: f64) {
        let val = self.precise_y + change;
        self.set_precise_y(val);
    }

    pub fn get_x_range(&self) -> Range<usize> {
        self.get_x()..(self.get_x() + self.tiles_on_width + 1).min(self.max_x)
    }

    pub fn get_y_range(&self) -> Range<usize> {
        self.get_y()..(self.get_y() + self.tiles_on_height + 1).min(self.max_y)
    }
}

impl<'a, 'b> View<'a, 'b> {
    pub fn prepare_for_drawing(&mut self) {
        if self.mode == DisplayMode::Normal || self.mode == DisplayMode::Tiles {
            let time = self.board.get_time();
            let x_range = self.get_x_range();
            let y_range = self.get_y_range();

            let bd = self.board.get_ecs();
            let mut terrain = bd.write_resource::<lib_evolvim::Terrain>();
            let climate = bd.read_resource::<lib_evolvim::Climate>();
            terrain.update_all_at(time, &climate, x_range, y_range);
            // self.board.terrain.update_all(time, &self.board.climate);

            // if self.board.selected_creature.0.is_some() {
            //     let pos = {
            //         let world = &self.board.world;
            //         let c = &self.board.selected_creature.0.as_ref().unwrap();
            //         let c = c.borrow(world);

            //         c.get_position()
            //     };

            //     let tw = self.tiles_on_width;
            //     let th = self.tiles_on_height;

            //     self.set_precise_x(pos.0 - tw as f64 * 0.5);
            //     self.set_precise_y(pos.1 - th as f64 * 0.5);
            // }
        }
    }

    pub fn draw<C, G>(&self, context: Context, graphics: &mut G, glyphs: &mut C)
    where
        C: CharacterCache,
        C::Error: Debug,
        G: Graphics<Texture = C::Texture>,
    {
        use self::DisplayMode::*;

        match self.mode {
            Normal => {
                // Draw terrain
                let bd = self.board.get_ecs();
                let terrain = bd.read_resource::<lib_evolvim::Terrain>();
                draw_terrain(&terrain, context, graphics, glyphs, &self);

                // Draw creatures
                let ps_world = bd.read_resource::<nphysics2d::world::World<f64>>();
                use specs::Join;

                for c in bd.read_storage::<Creature<Brain>>().join() {
                    let body = ps_world.rigid_body(c.get_handle()).unwrap();
                    draw_body(body, context, graphics, &self);
                }

                // if let Some(ref c) = self.board.selected_creature.0 {
                //     let creature = c.borrow(world);

                //     draw_details_creature(&creature, context, graphics, glyphs, &self);
                // }
            }
            Tiles => {
                let bd = self.board.get_ecs();
                let terrain = bd.read_resource::<lib_evolvim::Terrain>();
                draw_terrain(&terrain, context, graphics, glyphs, &self);
            }
            None => {}
        }
    }
}

#[derive(PartialEq)]
pub enum DisplayMode {
    /// Normal display mode, like evolv.io had.
    Normal,
    /// Only display tiles.
    Tiles,
    /// Doesn't display anything, lets the simulation go faster because there is no rendering.
    None,
}

impl Default for DisplayMode {
    fn default() -> Self {
        DisplayMode::Normal
    }
}
