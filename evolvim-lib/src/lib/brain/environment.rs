use crate::ecs_board::{BoardSize, BoardPreciseCoordinate};
use crate::softbody::Rock;
use crate::{Climate, Terrain};
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
    pub handle: BodyHandle,
    pub world: &'a nphysics2d::world::World<f64>,
}

impl<'a> Environment<'a> {
    pub fn new(terrain: &'a Terrain, this_body: &'a Rock, handle: BodyHandle, world: &'a nphysics2d::world::World<f64>) -> Self {
        Environment { terrain, this_body, handle, world }
    }

    /// The rotation of this body in `]-pi; pi]`
    pub fn body_angle(&self) -> f64 {
        let rb = self.world.rigid_body(self.handle).unwrap();

        rb.position().rotation.angle()
    }

    pub fn body_position(&self) -> BoardPreciseCoordinate {
        let rb = self.world.rigid_body(self.handle).unwrap();

        let pos = rb.position().translation.vector;

        BoardPreciseCoordinate(pos[0], pos[1])
    }
}
