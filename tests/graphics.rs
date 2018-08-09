extern crate lib_evolvim;

use lib_evolvim::graphics::*;
// use lib_evolvim::*;

#[test]
fn test_graphics_from_hsba() {
    // White because brightness == 1
    assert!([1., 1., 1., 1.] == from_hsba([0., 0., 1., 1.]));

    // Black because brightness == 0
    assert!([0., 0., 0., 1.] == from_hsba([1., 1., 0., 1.]));
}

#[test]
fn test_ui_mouse_to_board_coordinate() {
    let mouse = MouseCoordinate::new(333.01, 866.99);
    assert!(mouse.into_board_coordinate(0.1, 0.3, 10.0) == (33, 86));
}
