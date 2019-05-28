use crate::softbody::{HLSoftBody, Rock};
use crate::{BoardSize, Climate, SoftBodiesInPositions, Terrain};

pub struct EnvironmentMut<'a, B> {
    pub terrain: &'a mut Terrain,
    pub this_body: &'a mut Rock,
    pub board_size: BoardSize,
    pub time: f64,
    pub climate: &'a Climate,
    pub self_pointer: HLSoftBody<B>,
    pub world: &'a mut nphysics2d::world::World<f64>,
}

impl<'a, B> EnvironmentMut<'a, B> {
    pub fn new(
        terrain: &'a mut Terrain,
        this_body: &'a mut Rock,
        board_size: BoardSize,
        time: f64,
        climate: &'a Climate,
        self_pointer: HLSoftBody<B>,
        world: &'a mut nphysics2d::world::World<f64>,
    ) -> Self {
        EnvironmentMut {
            terrain,
            this_body,
            board_size,
            time,
            climate,
            self_pointer,
            world,
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
