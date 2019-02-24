use crate::softbody::Rock;
use crate::{BoardSize, Climate, Terrain};

pub struct EnvironmentMut<'a> {
    pub terrain: &'a mut Terrain,
    pub this_body: &'a mut Rock,
    pub board_size: BoardSize,
    pub time: f64,
    pub climate: &'a Climate,
}

impl<'a> EnvironmentMut<'a> {
    pub fn new(
        terrain: &'a mut Terrain,
        this_body: &'a mut Rock,
        board_size: BoardSize,
        time: f64,
        climate: &'a Climate,
    ) -> Self {
        EnvironmentMut {
            terrain,
            this_body,
            board_size,
            time,
            climate,
        }
    }
}

pub struct Environment<'a> {
    pub terrain: &'a Terrain,
    pub this_body: &'a Rock,
}

impl<'a> Environment<'a> {
    pub fn new(terrain: &'a Terrain, this_body: &'a Rock) -> Self {
        Environment { terrain, this_body }
    }
}
