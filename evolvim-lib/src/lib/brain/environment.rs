use crate::softbody::{HLSoftBody, Rock};
use crate::{BoardSize, Climate, Terrain};
use nphysics2d::object::BodyHandle;

pub struct EnvironmentMut<'a, B> {
    pub terrain: &'a mut Terrain,
    pub this_body: &'a mut Rock,
    pub handle: BodyHandle,
    pub board_size: BoardSize,
    pub time: f64,
    pub climate: &'a Climate,
    pub world: &'a mut nphysics2d::world::World<f64>,
    phantom: std::marker::PhantomData<B>,
}

impl<'a, B> EnvironmentMut<'a, B> {
    pub fn new(
        terrain: &'a mut Terrain,
        this_body: &'a mut Rock,
        handle: BodyHandle,
        board_size: BoardSize,
        time: f64,
        climate: &'a Climate,
        world: &'a mut nphysics2d::world::World<f64>,
    ) -> Self {
        EnvironmentMut {
            terrain,
            this_body,
            handle,
            board_size,
            time,
            climate,
            world,
            phantom: std::marker::PhantomData,
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
