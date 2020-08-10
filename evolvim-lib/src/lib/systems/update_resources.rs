use super::*;

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