extern crate lib_evolvim;
extern crate clap;

use lib_evolvim::Board;
use clap::{Arg, App};

fn main() {
    let matches = App::new("Evolvim - cli")
        .version(clap::crate_version!())
        .author("Sybrand Aarnoutse")
        .arg(Arg::with_name("output")
            .short("o")
            .long("output")
            .value_name("FILE")
            .takes_value(true)
            .help("The output file, save to this when done"))
        .arg(Arg::with_name("input")
            .short("i")
            .long("input")
            .value_name("FILE")
            .takes_value(true)
            .help("The input file, start with this as board"))
        .arg(Arg::with_name("save")
            .short("s")
            .long("save")
            .takes_value(false)
            .conflicts_with("output")
            .requires("input")
            .help("Saves to the input file when done"))
        .arg(Arg::with_name("iterations")
            .short("u")
            .long("updates")
            .value_name("YEARS")
            .takes_value(true)
            .help("Amount of years to simulate"))
        .arg(Arg::with_name("info")
            .long("info")
            .takes_value(false)
            .help("Output a summary of this world"))
        .get_matches();


    let output_file = if matches.is_present("save") {
        matches.value_of("input")
    } else {
        matches.value_of("output")
    };

    let mut board = if let Some(name) = matches.value_of("input") {
        Board::load_from(name).unwrap()
    } else {
        Board::default()
    };

    if let Some(years) = matches.value_of("iterations") {
        let years: usize = years.parse().unwrap();

        for _j in 0..years {
            println!("Simulating year {}...", board.get_time() as usize);
            print!("\x1B[1A");
            for _i in 0..1000 {
                board.update(0.001);
            }

        }
    }

    if matches.is_present("info") {
        println!("Year: {}", board.get_time() as usize);
        println!("Population: {}", board.creatures.len());
    }

    if let Some(name) = output_file {
        board.save_to(name).unwrap();
    }
}