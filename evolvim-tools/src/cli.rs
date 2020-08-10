extern crate clap;
extern crate ctrlc;
extern crate lib_evolvim;

use clap::{App, Arg};
use lib_evolvim::ecs_board::ECSBoard;
use std::sync::atomic::Ordering;

// type BrainType = lib_evolvim::neat::NeatBrain;
type BrainType = lib_evolvim::Brain;

fn main() {
    let abort_reader = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let abort_writer = abort_reader.clone();

    ctrlc::set_handler(move || abort_writer.store(true, Ordering::SeqCst))
        .expect("Error setting SIGINT handler! Blame the other crate!");

    let matches = App::new("Evolvim - cli")
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
        .arg(
            Arg::with_name("iterations")
                .short("u")
                .long("updates")
                .value_name("YEARS")
                .takes_value(true)
                .help("Amount of years to simulate"),
        )
        .arg(
            Arg::with_name("info")
                .long("info")
                .takes_value(false)
                .help("Output a summary of this world"),
        )
        .get_matches();

    let output_file = if matches.is_present("save") {
        matches.value_of("input")
    } else {
        matches.value_of("output")
    };

    // let mut board: Board<BrainType> = if let Some(name) = matches.value_of("input") {
    //     Board::<BrainType>::load_from(name).unwrap()
    // } else {
    //     Board::default()
    // };
    let mut board = ECSBoard::init((100, 100), 0.1);

    if let Some(years) = matches.value_of("iterations") {
        let mut years: usize = years.parse().unwrap();

        if years == 0 {
            years = std::usize::MAX;
        }

        for _j in 0..years {
            if abort_reader.load(Ordering::SeqCst) {
                // Ctrl-C was pressed, stop the simulation
                break;
            }

            println!("Simulating year {}...", board.get_time() as usize);
            print!("\x1B[1A");
            for _i in 0..1000 {
                // board.update(0.001);
                board.run();
            }
        }
    }

    // Clear line
    print!("\x1B[2K");

    if matches.is_present("info") {
        println!("Year: {}", board.get_time() as usize);
        println!("Population: {}", board.get_population_size());
    }

    // if let Some(name) = output_file {
    //     board.save_to(name).unwrap();
    // }
}
