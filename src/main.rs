extern crate lib_evolvim;
use lib_evolvim::Board;

fn main() {
    let mut board = Board::default();
    const YEARS: usize = 1000;

    for _i in 0..YEARS {
        println!("{}", board.get_population_size());
        for _j in 0..1000 {
            board.update(0.001);
        }
    }
}
