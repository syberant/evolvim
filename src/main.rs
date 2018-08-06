extern crate lib_evolvim;
extern crate piston_window;

use lib_evolvim::graphics::{from_hsba, View};
// use lib_evolvim::Board;
use piston_window::*;

fn main() {
    let mut view = View::default();
    let time = view.board.get_time();
    view.board.update(0.001);
    view.board.terrain.update_all(time, &view.board.climate);

    let mut playspeed = 1;

    assert!([1., 1., 1., 1.] == from_hsba([0., 0., 1., 1.]));
    assert!([0., 0., 0., 1.] == from_hsba([1., 1., 0., 1.]));

    let mut window: PistonWindow = WindowSettings::new("Hello Piston!", [1000, 800])
        .exit_on_esc(true)
        .build()
        .unwrap();

    while let Some(event) = window.next() {
        // Render
        event.update(|_args| {
            for _i in 0..playspeed {
                view.board.update(0.001);
            }
        });

        // Draw
        window.draw_2d(&event, |context, graphics| {
            clear([1.0; 4], graphics);

            view.board.prepare_for_drawing();

            view.draw(context, graphics);
        });

        // Match some events
        event.mouse_relative(|x, y| view.on_mouse_move(x, y));

        // Match some button presses
        if let Some(button) = event.press_args() {
            use piston_window::Button::Keyboard;
            use piston_window::Key;

            match button {
                Keyboard(Key::Up) => {
                    if playspeed > 0 {
                        playspeed *= 2;
                    } else {
                        playspeed = 1;
                    }
                }
                Keyboard(Key::Down) => {
                    if playspeed > 1 {
                        playspeed /= 2;
                    } else {
                        playspeed = 0;
                    }
                }
                _ => (),
            }
        }

        if let Event::Input(input) = event {
            use mouse::MouseButton::*;
            use Button::Mouse;
            use ButtonState::*;
            use Input::*;

            match input {
                Button(b_args) => match b_args.button {
                    Mouse(m_args) => match m_args {
                        Left => match b_args.state {
                            Press => view.on_mouse_press(),
                            Release => view.on_mouse_release(),
                        },
                        _ => {}
                    },
                    _ => {}
                },
                _ => {}
            }
        }

        window.set_title(format!(
            "Population size: {}, year: {:.2}, season: {}.",
            view.board.get_population_size(),
            view.board.get_time(),
            view.board.get_season()
        ));
    }
}
