//! Contains utilities for displaying our awesome world on a screen.
//!
//! Note that you can implement the graphics yourself too!
//! This part of the crate provides a working version and is a good start, but I'm sure somebody can do better!
//! Just use the rest of this crate and ignore this module; although you could draw some inspiration from it.

extern crate graphics;

pub mod ui;
pub mod view;
pub use self::ui::{Dragging, MouseCoordinate};
pub use self::view::View;

use self::graphics::character::CharacterCache;
use self::graphics::rectangle;
use self::graphics::text::Text;
use self::graphics::types::Color;
use self::graphics::{Context, Graphics, Transformed};

// use super::constants::*;
use super::*;

// pub trait Drawable {
//     fn draw(&self, context: Context, g2d: &mut G2d);
// }

/// Converts hsba (Hue, Saturation, Brightness, Alpha) into rgba (Red, Green, Blue, Alpha)
///
/// All input values should range from 0 to 1. All output values will range from 0 to 1.
///
/// Formulae from [here](https://en.wikipedia.org/wiki/HSL_and_HSV#From_HSV)
pub fn from_hsba(hsba: [f32; 4]) -> Color {
    let [hue, sat, bri, alpha] = hsba;

    // Chroma
    let c = bri * sat;
    // H' = hue * 360 / 60 = hue * 6
    let mut h = hue * 6.0;
    let x = c * (1.0 - (h % 2.0 - 1.0).abs());

    if h == 0.0 {
        h = 1.0;
    }

    let (r, g, b): (f32, f32, f32) = match h.ceil() as usize {
        1 => (c, x, 0.0),
        2 => (x, c, 0.0),
        3 => (0.0, c, x),
        4 => (0.0, x, c),
        5 => (x, 0.0, c),
        6 => (c, 0.0, x),
        // Value should not be larger than 6 --> hue should not be larger than 1
        _ => panic!(),
    };

    let m = bri - c;

    return [r + m, g + m, b + m, alpha];
}

impl Terrain {
    pub fn draw<C, G>(&self, context: Context, graphics: &mut G, glyphs: &mut C, view: &View)
    where
        C: CharacterCache,
        C::Error: std::fmt::Debug,
        G: Graphics<Texture = C::Texture>,
    {
        let size = view.get_tile_size();
        let transform = context
            .transform
            .trans(-view.get_precise_x() * size, -view.get_precise_y() * size);

        for x in view.get_x_range() {
            for y in view.get_y_range() {
                let tile = self.get_tile_at((x, y));

                let rect = [x as f64 * size, y as f64 * size, size, size];

                let color = from_hsba(tile.get_hsba_color());

                rectangle(color, rect, transform, graphics);
            }
        }

        // Draw text for `Tile` under cursor
        let tile_pos = view.mouse.into_board_coordinate(
            view.get_precise_x(),
            view.get_precise_y(),
            view.get_tile_size(),
        );
        let tile = self.get_tile_at(tile_pos);

        let text = &format!("{:.0}", tile.get_food_level() * 100.0);

        Text::new(12)
            .draw(
                text,
                glyphs,
                &context.draw_state,
                transform.trans(tile_pos.0 as f64 * size, tile_pos.1 as f64 * size),
                graphics,
            )
            .expect("Your font doesn't seem to be working... Could not draw text.");
    }
}

impl Creature {
    pub fn draw<G>(&self, context: Context, graphics: &mut G, view: &View)
    where
        G: Graphics,
    {
        unimplemented!();
    }
}
