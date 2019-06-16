//! Contains utilities for displaying our awesome world on a screen.
//!
//! Note that you can implement the graphics yourself too!
//! This part of the crate provides a working version and is a good start, but I'm sure somebody can do better!
//! Just use the rest of this crate and ignore this module; although you could draw some inspiration from it.

extern crate graphics;
extern crate lib_evolvim;
extern crate nphysics2d;

pub mod ui;
pub mod view;
pub use self::ui::{Dragging, MouseCoordinate};
pub use self::view::View;

use self::graphics::character::CharacterCache;
use self::graphics::text::Text;
use self::graphics::types::Color;
use self::graphics::{ellipse, rectangle};
use self::graphics::{Context, Graphics, Transformed};
use std::fmt::Debug;

use lib_evolvim::constants::*;
use lib_evolvim::*;

use nphysics2d::object::RigidBody;

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

    assert!(hue <= 1.0, "Hue can't be larger than 1.");
    assert!(sat <= 1.0);
    assert!(bri <= 1.0);
    assert!(alpha <= 1.0);

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
        _ => unreachable!(),
    };

    let m = bri - c;

    return [r + m, g + m, b + m, alpha];
}

pub fn draw_lines<G, C>(
    text_to_draw: Vec<String>,
    line_heigth: f64,
    line_width: f64,
    context: Context,
    text: Text,
    glyphs: &mut C,
    graphics: &mut G,
) where
    G: Graphics<Texture = C::Texture>,
    C: CharacterCache,
    C::Error: Debug,
{
    let buffer = 10.0;
    let mut transform = context.transform.trans(buffer, buffer);

    // Draw white background
    let rect = [
        0.0,
        0.0,
        line_width + 2.0 * buffer,
        line_heigth * text_to_draw.len() as f64 + 2.0 * buffer,
    ];
    rectangle([1.0, 1.0, 1.0, 0.8], rect, context.transform, graphics);

    for i in 0..text_to_draw.len() {
        transform = transform.trans(0.0, line_heigth);

        text.draw(
            &text_to_draw[i],
            glyphs,
            &context.draw_state,
            transform,
            graphics,
        )
        .expect("Your font doesn't seem to be working... Could not draw text.");
    }
}

pub fn draw_terrain<C, G>(
    terrain: &Terrain,
    context: Context,
    graphics: &mut G,
    glyphs: &mut C,
    view: &View,
) where
    C: CharacterCache,
    C::Error: Debug,
    G: Graphics<Texture = C::Texture>,
{
    let size = view.get_tile_size();
    let transform = context
        .transform
        .trans(-view.get_precise_x() * size, -view.get_precise_y() * size);

    let mut shape = rectangle::Rectangle::new([1., 1., 1., 1.]);

    for x in view.get_x_range() {
        for y in view.get_y_range() {
            let tile = terrain.get_tile_at((x, y));

            let rect = [x as f64 * size, y as f64 * size, size, size];

            shape = shape.color(from_hsba(tile.get_hsba_color()));

            shape.draw(rect, &context.draw_state, transform, graphics);
        }
    }

    // Draw text for `Tile` under cursor
    if let Some(tile_pos) = view.mouse.into_board_coordinate(
        view.get_precise_x(),
        view.get_precise_y(),
        view.get_tile_size(),
        view.board.get_board_size(),
    ) {
        let tile = terrain.get_tile_at(tile_pos);

        let text = &format!("{:.0}", tile.get_food_level() * 100.0);

        Text::new(12)
            .draw(
                text,
                glyphs,
                &context.draw_state,
                transform.trans(
                    tile_pos.0 as f64 * size + 0.5 * size,
                    tile_pos.1 as f64 * size + 0.5 * size,
                ),
                graphics,
            )
            .expect("Your font doesn't seem to be working... Could not draw text.");
    }
}

pub fn draw_creature<B: lib_evolvim::brain::NeuralNet, G: Graphics>(
    creature: &Creature<B>,
    context: Context,
    graphics: &mut G,
    view: &View,
) {
    let size = view.get_tile_size();
    let transform = context
        .transform
        .trans(-view.get_precise_x() * size, -view.get_precise_y() * size);

    let radius = creature.get_radius();
    let color = from_hsba([creature.get_mouth_hue() as f32, 1.0, 1.0, 1.0]);

    let rect = [
        // This gives the upper-left corner of the circle so subtract the radius.
        (creature.get_px() - radius) * size,
        (creature.get_py() - radius) * size,
        radius * 2.0 * size,
        radius * 2.0 * size,
    ];

    let ellipse = ellipse::Ellipse::new(color);

    ellipse.draw(rect, &context.draw_state, transform, graphics);
}

pub fn draw_body<G: Graphics>(body: &RigidBody<f64>, context: Context, graphics: &mut G, view: &View) {
    let size = view.get_tile_size();
    let transform = context
        .transform
        .trans(-view.get_precise_x() * size, -view.get_precise_y() * size);

    // let radius = creature.get_radius();
    let radius = 0.3;

    // let color = from_hsba([creature.get_mouth_hue() as f32, 1.0, 1.0, 1.0]);
    let color = from_hsba([0.5, 1.0, 1.0, 1.0]);

    let pos = body.position().translation.vector;
    let x = pos[0];
    let y = pos[1];
    let rect = [
        // This gives the upper-left corner of the circle so subtract the radius.
        (x - radius) * size,
        (y - radius) * size,
        radius * 2.0 * size,
        radius * 2.0 * size,
    ];

    let ellipse = ellipse::Ellipse::new(color);

    ellipse.draw(rect, &context.draw_state, transform, graphics);
}

pub fn draw_details_creature<B, C, G>(
    creature: &Creature<B>,
    context: Context,
    graphics: &mut G,
    glyphs: &mut C,
    view: &View,
) where
    B: lib_evolvim::brain::NeuralNet + DrawableBrain,
    C: CharacterCache,
    C::Error: Debug,
    G: Graphics<Texture = C::Texture>,
{
    let text = Text::new(18);
    let mut text_to_draw = Vec::new();

    text_to_draw.push(format!("Energy: {:.3}", creature.get_energy()));
    let time_step = 0.001;
    text_to_draw.push(format!(
        "Energy D: {:.3}",
        creature.get_energy_change(time_step)
    ));
    text_to_draw.push(format!(
        "Age: {:.3}",
        creature.get_age(view.board.get_time())
    ));
    text_to_draw.push(format!(
        "Pos: ({:.1}, {:.1})",
        creature.get_px(),
        creature.get_py()
    ));
    // text_to_draw.push(format!("Speed: {:.3}", creature.get_total_velocity()));

    draw_lines(
        text_to_draw,
        20.0,
        200.0,
        context.trans(0.0, 300.0),
        text,
        glyphs,
        graphics,
    );

    creature.brain.draw_brain(context, graphics, glyphs);

    let size = view.get_tile_size();
    let transform = context
        .transform
        .trans(-view.get_precise_x() * size, -view.get_precise_y() * size);

    let radius = creature.get_radius() / 2.0;
    let color = [1.0, 1.0, 1.0, 0.5];

    let rect = [
        // This gives the upper-left corner of the circle so subtract the radius.
        (creature.get_px() - radius) * size,
        (creature.get_py() - radius) * size,
        radius * 2.0 * size,
        radius * 2.0 * size,
    ];

    let ellipse = ellipse::Ellipse::new(color);

    ellipse.draw(rect, &context.draw_state, transform, graphics);
}

pub trait DrawableBrain {
    fn draw_brain<C, G>(&self, context: Context, graphics: &mut G, glyphs: &mut C)
    where
        C: CharacterCache,
        C::Error: Debug,
        G: Graphics<Texture = C::Texture>;
}

impl DrawableBrain for lib_evolvim::neat::NeatBrain {
    fn draw_brain<C, G>(&self, context: Context, graphics: &mut G, glyphs: &mut C)
    where
        C: CharacterCache,
        C::Error: Debug,
        G: Graphics<Texture = C::Texture>,
    {
        let text = Text::new(18);
        let info = self
            .get_ordered_key_value_pairs()
            .into_iter()
            .map(|(key, value)| format!("{}: {}", key, value))
            .collect();
        draw_lines(info, 20.0, 100.0, context, text, glyphs, graphics);
    }
}

impl DrawableBrain for Brain {
    fn draw_brain<C, G>(&self, context: Context, graphics: &mut G, glyphs: &mut C)
    where
        C: CharacterCache,
        C::Error: Debug,
        G: Graphics<Texture = C::Texture>,
    {
        let text = Text::new(18);
        let output = self
            .get_output()
            .iter()
            .map(|value| format!("{:.3}", value))
            .collect();
        let hidden = self
            .get_hidden_layer()
            .iter()
            .map(|val| format!("{:.3}", val))
            .collect();
        let input = self
            .get_input_layer()
            .iter()
            .map(|val| format!("{:.3}", val))
            .collect();
        let info = self.intentions();

        draw_lines(input, 20.0, 100.0, context, text, glyphs, graphics);
        draw_lines(
            hidden,
            20.0,
            100.0,
            context.trans(120.0, 0.0),
            text,
            glyphs,
            graphics,
        );
        draw_lines(
            output,
            20.0,
            100.0,
            context.trans(240.0, 0.0),
            text,
            glyphs,
            graphics,
        );
        draw_lines(
            info,
            20.0,
            100.0,
            context.trans(360.0, 0.0),
            text,
            glyphs,
            graphics,
        );
    }
}
