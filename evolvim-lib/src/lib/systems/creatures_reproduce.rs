use super::*;

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