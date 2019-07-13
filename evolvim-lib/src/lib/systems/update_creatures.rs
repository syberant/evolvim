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

        for c in (&mut creatures).join() {
            check_position(c, *board_size, &mut world);

            c.record_energy();
            c.metabolize(0.001, time.0);

            let handle = c.get_handle();
            let env = Environment::new(&terrain, &c.base, handle, &world);
            c.brain.run_with(&env);

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

fn check_position(creature: &mut Creature<Brain>, board_size: BoardSize, world: &mut nphysics2d::world::World<f64>) {
    let handle = creature.get_handle();
    let mut rg = world.rigid_body_mut(handle).unwrap();

    let position: crate::ecs_board::BoardPreciseCoordinate = rg.position().into();
    let mut x = position.0.floor() as usize;
    let mut y = position.1.floor() as usize;

    if (x >= board_size.0) {
        // println!("({}, {}) Out of bounds! x coordinate is {}.", x, y, x);

        x = if (x > 2*board_size.0) {0} else {board_size.0 - 1};
        let mut pos = rg.position().clone();
        pos.translation.vector[0] = x as f64;

        rg.set_position(pos);
    }

    if (y >= board_size.1) {
        // println!("({}, {}) Out of bounds! y coordinate is {}.", x, y, y);

        y = if (y > 2*board_size.1) {0} else {board_size.1 - 1};
        let mut pos = rg.position().clone();
        pos.translation.vector[1] = y as f64;

        rg.set_position(pos);
    }
}