extern crate lib_evolvim;

// use lib_evolvim::graphics::*;
use lib_evolvim::*;

#[test]
fn test_board_update() {
    let mut board = Board::default();

    board.update(0.001);
}

#[test]
fn test_board_default_intialise() {
    let _board = Board::default();
}
