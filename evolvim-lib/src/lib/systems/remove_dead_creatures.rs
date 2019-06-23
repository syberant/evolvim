use super::*;

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
                c.return_to_earth(time.0, *board_size, &mut terrain, &climate);

                // Remove this entity from nphysics2d::World
                // TODO: optimise this to do all at once or something
                world.remove_bodies(&[c.get_handle()]);

                // Remove this entity from specs::World
                entities.delete(e).unwrap();
            }
        }
    }
}