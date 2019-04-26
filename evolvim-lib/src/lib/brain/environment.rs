use crate::softbody::{HLSoftBody, Rock};
use crate::{BoardSize, Climate, SoftBodiesInPositions, Terrain};

pub struct EnvironmentMut<'a, B> {
    pub terrain: &'a mut Terrain,
    pub this_body: &'a mut Rock,
    pub board_size: BoardSize,
    pub time: f64,
    pub climate: &'a Climate,
    pub sbip: &'a SoftBodiesInPositions<B>,
    pub self_pointer: HLSoftBody<B>,
}

impl<'a, B> EnvironmentMut<'a, B> {
    pub fn new(
        terrain: &'a mut Terrain,
        this_body: &'a mut Rock,
        board_size: BoardSize,
        time: f64,
        climate: &'a Climate,
        sbip: &'a SoftBodiesInPositions<B>,
        self_pointer: HLSoftBody<B>,
    ) -> Self {
        EnvironmentMut {
            terrain,
            this_body,
            board_size,
            time,
            climate,
            sbip,
            self_pointer,
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
