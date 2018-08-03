extern crate lib_evolvim;
extern crate piston_window;

use lib_evolvim::graphics::Drawable;
use lib_evolvim::Board;
use piston_window::*;

fn main() {
    let mut board = Board::default();
    let time = board.get_time();
    board.terrain.update_all(time, &board.climate);

    let playspeed = 10;

    let mut window: PistonWindow = WindowSettings::new("Hello Piston!", [1000, 1000])
        .exit_on_esc(true)
        .build()
        .unwrap();

    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics| {
            clear([1.0; 4], graphics);

            let time = board.get_time();
            board.terrain.update_all(time, &board.climate);
            board.terrain.draw(context, graphics);

            for _i in 0..playspeed {
                board.update(0.001);
            }
        });

        window.set_title(format!(
            "Population size: {}, year: {:.1}, season: {}.",
            board.get_population_size(),
            board.get_time(),
            board.get_season()
        ));
    }
}
