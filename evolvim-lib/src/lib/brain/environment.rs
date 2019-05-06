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
    pub world: &'a mut nphysics2d::world::World<f64>,
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
        world: &'a mut nphysics2d::world::World<f64>,
    ) -> Self {
        EnvironmentMut {
            terrain,
            this_body,
            board_size,
            time,
            climate,
            sbip,
            self_pointer,
            world,
        }
    }

    pub fn get_colliders(&self) -> crate::sbip::SoftBodiesAt<B> {
        use crate::sbip::SoftBodyBucket;

        let mut colliders = self.this_body.get_colliders(self.sbip);

        // Remove self
        colliders.remove_softbody(self.self_pointer.clone());

        return colliders;
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
