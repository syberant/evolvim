use super::*;

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