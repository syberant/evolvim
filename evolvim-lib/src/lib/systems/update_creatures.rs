use super::*;

pub struct UpdateCreatures;

impl<'a> System<'a> for UpdateCreatures {
    type SystemData = (
        WriteExpect<'a, nphysics2d::world::World<f64>>,
        ReadExpect<'a, Time>,
        WriteExpect<'a, Terrain>,
        ReadExpect<'a, Climate>,
        ReadExpect<'a, BoardSize>,
        WriteStorage<'a, Creature<Brain>>,
    );

    fn run(
        &mut self,
        (mut world, time, mut terrain, climate, board_size, mut creatures): Self::SystemData,
    ) {
        use crate::brain::Environment;
        use crate::brain::EnvironmentMut;
        use crate::brain::NeuralNet;
        use specs::Join;

        for (c) in (&mut creatures).join() {
            c.record_energy();
            c.metabolize(0.001, time.0);

            let env = Environment::new(&terrain, &c.base);
            c.brain.run_with(&env);

            let handle = c.get_handle();
            let mut env_mut = EnvironmentMut::new(
                &mut terrain,
                &mut c.base,
                handle,
                *board_size,
                time.0,
                &climate,
                &mut world,
            );

            c.brain.use_output(&mut env_mut, 0.001);
        }
    }
}