extern crate lib_evolvim;
extern crate piston_window;

use lib_evolvim::*;
use piston_window::*;

fn main() {
    let mut board = Board::<Brain>::default();

    let mut window: PistonWindow = WindowSettings::new("Hello Piston!", [1000, 800])
        .exit_on_esc(true)
        .build()
        .unwrap();

    while let Some(event) = window.next() {
        // Draw
        window.draw_2d(&event, |context, graphics| {
            clear([1.0; 4], graphics);

            for x in 0..100 {
                for y in 0..100 {
                    let size = 10.0;
                    let tile = board.terrain.get_tile_at((x, y));

                    let rect = [x as f64 * size, y as f64 * size, size, size];

                    let color = match tile.is_water() {
                        false => [
                            // tile.get_food_type() as f32,
                            0.0,
                            tile.get_fertility() as f32,
                            0.0,
                            1.0,
                        ],
                        true => [0.0, 0.0, 1.0, 1.0],
                    };

                    rectangle(color, rect, context.transform, graphics);
                }
            }
        });

        if let Some(button) = event.press_args() {
            use piston_window::Button::Keyboard;
            use piston_window::Key;

            match button {
                Keyboard(Key::Up) => {
                    board = Board::default();
                }
                _ => (),
            }
        }
    }
}
