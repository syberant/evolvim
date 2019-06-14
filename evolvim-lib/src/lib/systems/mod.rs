use crate::board::BoardSize;
use crate::brain::Brain;
use crate::climate::Climate;
use crate::softbody::Creature;
use crate::terrain::Terrain;
use crate::time::Time;
use specs::{Entities, LazyUpdate, Read, ReadExpect, ReadStorage, System, WriteExpect, WriteStorage};

pub struct UpdateResources;

impl<'a> System<'a> for UpdateResources {
    type SystemData = (
        WriteExpect<'a, Terrain>,
        WriteExpect<'a, Time>,
        WriteExpect<'a, Climate>,
    );

    fn run(&mut self, (mut terrain, mut year, mut climate): Self::SystemData) {
        let time_step = 0.001;

        year.0 += time_step;
        climate.update(year.0);

        let temp_change_into_frame =
            climate.get_temperature() - climate.get_growth_rate(year.0 - time_step);
        let temp_change_out_of_frame =
            climate.get_growth_rate(year.0 + time_step) - climate.get_temperature();

        if temp_change_into_frame * temp_change_out_of_frame < 0.0 {
            // Temperature change flipped direction
            terrain.update_all(year.0, &climate);
        }
    }
}

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

            let mut env_mut = EnvironmentMut::new(
                &mut terrain,
                &mut c.base,
                *board_size,
                time.0,
                &climate,
                &mut world,
            );

            c.brain.use_output(&mut env_mut, 0.001);
        }
    }
}

pub struct RemoveDeadCreatures;

impl<'a> System<'a> for RemoveDeadCreatures {
    type SystemData = (
        WriteExpect<'a, nphysics2d::world::World<f64>>,
        ReadExpect<'a, Time>,
        WriteExpect<'a, Terrain>,
        ReadExpect<'a, Climate>,
        ReadExpect<'a, BoardSize>,
        ReadStorage<'a, Creature<Brain>>,
        Entities<'a>,
    );

    fn run(
        &mut self,
        (mut world, time, mut terrain, climate, board_size, creatures, entities): Self::SystemData,
    ) {
        use specs::Join;

        for (c, e) in (&creatures, &entities).join() {
            if c.should_die() {
                c.return_to_earth(time.0, *board_size, &mut terrain, &climate, &mut world);

                // Remove this entity from nphysics2d::World
                // TODO: optimise this to do all at once or something
                world.remove_bodies(&[c.get_handle()]);

                // Remove this entity from specs::World
                entities.delete(e).unwrap();
            }
        }
    }
}

pub struct CreaturesReproduce;

impl<'a> System<'a> for CreaturesReproduce {
    type SystemData = (
        ReadExpect<'a, BoardSize>,
        ReadExpect<'a, Time>,
        WriteStorage<'a, Creature<Brain>>,
        WriteExpect<'a, nphysics2d::world::World<f64>>,
        Entities<'a>,
        Read<'a, LazyUpdate>,
    );

    fn run(&mut self, (board_size, time, mut creatures, mut world, entities, updater): Self::SystemData) {
        use specs::Join;

        let mut iter = entities.create_iter();

        for c in (&mut creatures).join() {
            let maybe_baby = c.try_reproduce(time.0, *board_size, &mut world);

            if let Some(baby) = maybe_baby {
                // Adding it using the WriteStorage is not possible
                // creatures.insert(iter.next().unwrap(), baby).unwrap();

                // Lazily add it, requires a call to specs::World::maintain() before functioning
                updater.insert(iter.next().unwrap(), baby);
            }
        }
    }
}

pub struct RefillCreatures;

impl<'a> System<'a> for RefillCreatures {
    type SystemData = (
        ReadExpect<'a, BoardSize>,
        ReadExpect<'a, Time>,
        WriteStorage<'a, Creature<Brain>>,
        WriteExpect<'a, nphysics2d::world::World<f64>>,
        Entities<'a>,
    );

    fn run(&mut self, (board_size, time, mut creatures, mut world, entities): Self::SystemData) {
        use specs::Join;

        let num_creatures = creatures.join().count();
        let minimum_creatures = 60;
        let creatures_to_add = if num_creatures >= minimum_creatures {
            0
        } else {
            minimum_creatures - num_creatures
        };

        for e in entities.create_iter().take(creatures_to_add) {
            // Make a new creature and add it to nphysicsd2::World
            let creature = Creature::<Brain>::new_random(&mut world, *board_size, time.0);

            // Add it to specs::World
            creatures.insert(e, creature).unwrap();
        }
    }
}

pub struct PhysicsStep;

impl<'a> System<'a> for PhysicsStep {
    type SystemData = (WriteExpect<'a, nphysics2d::world::World<f64>>);

    fn run(&mut self, mut world: Self::SystemData) {
        world.step();
    }
}