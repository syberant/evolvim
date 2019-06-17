use crate::brain::Brain;
use crate::constants;
use crate::softbody::Creature;

pub type BoardSize = (usize, usize);
pub type BoardCoordinate = (usize, usize);
#[derive(Clone)]
pub struct BoardPreciseCoordinate(pub f64, pub f64);

impl BoardPreciseCoordinate {
    pub fn unpack(&self) -> (f64, f64) {
        return (self.0, self.1);
    }
}

impl From<BoardPreciseCoordinate> for BoardCoordinate {
    fn from(bpc: BoardPreciseCoordinate) -> BoardCoordinate {
        let (x, y) = bpc.unpack();

        (x.floor() as usize, y.floor() as usize)
    }
}

pub struct ECSBoard<'a, 'b> {
    world: specs::World,
    dispatcher: specs::Dispatcher<'a, 'b>,
}

impl<'a, 'b> ECSBoard<'a, 'b> {
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

        world.add_resource(board_size);

        // Make the world
        let mut board = ECSBoard {
            world,
            dispatcher: specs::DispatcherBuilder::new().build(),
        };

        // Build the dispatcher
        board.custom_dispatcher(specs::DispatcherBuilder::new());

        return board;
    }

    pub fn run(&mut self) {
        self.dispatcher.dispatch(&mut self.world.res);

        // Synchronize deletions and insertions
        self.world.maintain();
    }

    pub fn custom_dispatcher(&mut self, builder: specs::DispatcherBuilder<'a, 'b>) {
        use crate::systems::*;

        self.dispatcher = builder
            .with(UpdateResources, "update_resources", &[])
            .with(UpdateCreatures, "update_creatures", &["update_resources"])
            .with(RemoveDeadCreatures, "remove_dead_creatures", &["update_creatures"])
            // .with(CreaturesReproduce, "creatures_reproduce", &["remove_dead_creatures"])
            // .with(RefillCreatures, "refill_creatures", &["creatures_reproduce"])
            .with(RefillCreatures, "refill_creatures", &["remove_dead_creatures"])
            .with(PhysicsStep, "physics_step", &["refill_creatures"])
            .build();
    }

    pub fn get_ecs(&self) -> &specs::World {
        &self.world
    }

    pub fn get_time(&self) -> f64 {
        self.world.read_resource::<crate::time::Time>().0
    }

    /// Returns a `String` representing the current season.
    ///
    /// Can be either "Winter", "Spring", "Summer" or "Autumn".
    pub fn get_season(&self) -> String {
        const SEASONS: [&str; 4] = ["Winter", "Spring", "Summer", "Autumn"];
        let season: usize = ((self.get_time() % 1.0) * 4.0).floor() as usize;

        return SEASONS[season].to_string();
    }

    pub fn get_population_size(&self) -> usize {
        use specs::Join;

        self.world.read_storage::<Creature<Brain>>().join().count()
    }

    pub fn get_board_size(&self) -> BoardSize {
        *self.world.read_resource::<BoardSize>()
    }

    pub fn get_board_width(&self) -> usize {
        self.get_board_size().0
    }

    pub fn get_board_height(&self) -> usize {
        self.get_board_size().1
    }
}