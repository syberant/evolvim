use super::*;

pub struct PhysicsStep;

impl<'a> System<'a> for PhysicsStep {
    type SystemData = (WriteExpect<'a, nphysics2d::world::World<f64>>);

    fn run(&mut self, mut world: Self::SystemData) {
        world.step();
    }
}