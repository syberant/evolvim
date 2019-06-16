extern crate clap;
extern crate lib_evolvim;
extern crate piston_window;
extern crate specs;

mod graphics;

use self::graphics::View;
use clap::{App, Arg};
use lib_evolvim::ecs_board::ECSBoard;
use piston_window::*;

// type BrainType = lib_evolvim::neat::NeatBrain;
type BrainType = lib_evolvim::brain::Brain;

fn main() {
    let matches = App::new("Evolvim - GUI launched via CLI")
        .version(clap::crate_version!())
        .author("Sybrand Aarnoutse")
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .value_name("FILE")
                .takes_value(true)
                .help("The output file, save to this when done"),
        )
        .arg(
            Arg::with_name("input")
                .short("i")
                .long("input")
                .value_name("FILE")
                .takes_value(true)
                .help("The input file, start with this as board"),
        )
        .arg(
            Arg::with_name("save")
                .short("s")
                .long("save")
                .takes_value(false)
                .conflicts_with("output")
                .requires("input")
                .help("Saves to the input file when done"),
        )
        .get_matches();

    let mut view = View::default();
    if let Some(filename) = matches.value_of("input") {
        // view.board = Board::<BrainType>::load_from(filename).unwrap();
    }

    let output_file = if matches.is_present("save") {
        matches.value_of("input")
    } else {
        matches.value_of("output")
    };

    let time = view.board.get_time();
    // view.board.update(0.001);
    // view.board.terrain.update_all(time, &view.board.climate);

    let mut playspeed = 1;

    let mut window: PistonWindow = WindowSettings::new("Hello Piston!", [1000, 900])
        .exit_on_esc(true)
        .build()
        .unwrap();

    window.set_max_fps(20);

    let byte_font = include_bytes!("../../assets/default-font.ttf");
    let factory = window.factory.clone();
    let text_settings = TextureSettings::new();
    let mut glyphs = Glyphs::from_bytes(byte_font, factory, text_settings).unwrap();

    while let Some(event) = window.next() {
        // Render
        event.update(|_args| {
            for _i in 0..playspeed {
                // view.board.update(0.001);
                view.board.run();
            }
        });

        // Draw
        window.draw_2d(&event, |context, graphics| {
            clear([1.0; 4], graphics);

            view.prepare_for_drawing();
            view.draw(context, graphics, &mut glyphs);
        });

        // Match some events
        event.mouse_relative(|x, y| view.on_mouse_move(x, y));
        event.mouse_cursor(|x, y| view.update_mouse(x, y));

        // Match some button presses
        if let Some(button) = event.press_args() {
            use Button::Keyboard;

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
                Keyboard(Key::D) => {
                    view.switch_display_mode();
                }
                Keyboard(Key::O) => {
                    // view.board.select_oldest();
                }
                Keyboard(Key::B) => {
                    // view.board.select_biggest();
                }
                Keyboard(Key::Q) => {
                    // view.board.selected_creature.deselect();
                }
                // Keyboard(Key::S) => {
                //     view.board.save_to("test.bin").unwrap();
                // }
                _ => (),
            }
        }

        if let Event::Input(input) = event {
            use self::mouse::MouseButton::*;
            use self::Button::Mouse;
            use self::ButtonState::*;
            use self::Input::*;

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

    if let Some(filename) = output_file {
        // view.board.save_to(filename).unwrap();
    }
}
