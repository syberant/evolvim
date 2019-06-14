use crate::board::BoardSize;
use crate::brain::Brain;
use crate::constants;
use crate::softbody::Creature;

pub struct ECSBoard {
    world: specs::World,
}

impl ECSBoard {
    pub fn init(board_size: BoardSize, noise_step_size: f64) -> Self {
        let mut world = specs::World::new();

        // Register components
        world.register::<Creature<Brain>>();

        // Initialise resources
        let terrain = crate::terrain::Terrain::generate_perlin(board_size, noise_step_size);
        world.add_resource(terrain);

        let (min_temp, max_temp) = (constants::DEFAULT_MIN_TEMP, constants::DEFAULT_MAX_TEMP);
        let climate = crate::climate::Climate::new(min_temp, max_temp);
        world.add_resource(climate);

        let mut physics_world = nphysics2d::world::World::<f64>::new();
        physics_world.set_timestep(0.001);
        world.add_resource(physics_world);

        let time = crate::time::Time::default();
        world.add_resource(time);

        // Return the world
        ECSBoard { world }
    }

    pub fn run(&mut self) {
        use specs::RunNow;
        use crate::systems::*;

        let mut res_up = UpdateResources;
        let mut creat_up = UpdateCreatures;
        let mut rm_dead_creat = RemoveDeadCreatures;
        let mut creat_rep = CreaturesReproduce;
        let mut refill_creat = RefillCreatures;
        let mut physics = PhysicsStep;

        res_up.run_now(&mut self.world.res);
        creat_up.run_now(&mut self.world.res);
        rm_dead_creat.run_now(&mut self.world.res);
        creat_rep.run_now(&mut self.world.res);
        refill_creat.run_now(&mut self.world.res);
        physics.run_now(&mut self.world.res);

        // Synchronize deletions and insertions
        self.world.maintain();
    }
}