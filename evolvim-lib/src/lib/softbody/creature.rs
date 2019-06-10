use super::*;

pub const MINIMUM_SURVIVABLE_SIZE: f64 = 0.06;

#[derive(Clone)]
struct Body(BodyHandle);

impl From<BodyHandle> for Body {
    fn from(b: BodyHandle) -> Body {
        Body(b)
    }
}

impl<'de> serde::Deserialize<'de> for Body {
    fn deserialize<D: serde::Deserializer<'de>>(_deserializer: D) -> Result<Body, D::Error> {
        unimplemented!()
    }
}

impl serde::Serialize for Body {
    fn serialize<S: serde::Serializer>(&self, _serializer: S) -> Result<S::Ok, S::Error> {
        unimplemented!()
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Creature<B> {
    pub base: Rock,
    pub brain: B,
    body: Body,
}

impl<B> std::ops::Deref for Creature<B> {
    type Target = Rock;

    fn deref(&self) -> &Rock {
        &self.base
    }
}

impl<B> std::ops::DerefMut for Creature<B> {
    fn deref_mut(&mut self) -> &mut Rock {
        &mut self.base
    }
}

impl<B: GenerateRandom> Creature<B> {
    pub fn new_random(world: &mut World, board_size: BoardSize, time: f64) -> Self {
        let energy = CREATURE_MIN_ENERGY
            + rand::random::<f64>() * (CREATURE_MAX_ENERGY - CREATURE_MIN_ENERGY);
        let base = Rock::new_random(board_size, CREATURE_DENSITY, energy, time);
        let brain = B::new_random();
        let body = make_physics_creature(world, &base).into();
        // TODO: add id

        Creature { base, brain, body }
    }
}

impl<B: NeuralNet + RecombinationInfinite> Creature<B> {
    /// Create a new baby, it isn't in `SoftBodiesInPositions` so please fix that.
    /// While you're at it, also add it to `Board.creatures`.
    pub fn new_baby(world: &mut World, parents: &[&SoftBody<B>], energy: f64, time: f64) -> Creature<B> {
        let brain = B::recombination_infinite_parents(parents);
        let base = Rock::new_from_parents(parents, energy, time);
        let body = make_physics_creature(world, &base).into();

        Creature { base, brain, body }
    }
}

impl<B> Creature<B> {
    // The `Creature` version of `apply_motions`, this is different to the `Rock` version.
    pub fn apply_motions(&mut self, time_step: f64, terrain: &Terrain, board_size: BoardSize) {
        if self.is_on_water(terrain, board_size) {
            let energy_to_lose = time_step * SWIM_ENERGY * self.get_energy();
            self.lose_energy(energy_to_lose);
        }

        // self.base.apply_motions(time_step, board_size);
    }

    pub fn return_to_earth(
        &self,
        time: f64,
        board_size: BoardSize,
        terrain: &mut Terrain,
        climate: &Climate,
        world: &mut World,
    ) {
        for _i in 0..PIECES {
            let tile_pos = self.get_random_covered_tile(board_size);

            terrain.add_food_or_nothing_at(tile_pos, self.get_energy() / PIECES as f64);
            terrain.update_at(tile_pos, time, climate);
        }
    }

    pub fn should_die(&self) -> bool {
        return self.get_energy() < SAFE_SIZE;
    }

    pub fn get_baby_energy(&self) -> f64 {
        self.base.get_energy() - SAFE_SIZE
    }
}

fn make_physics_creature(world: &mut World, cr: &Rock) -> BodyHandle {
    use nalgebra::Vector2;
    use ncollide2d::shape::{Ball, ShapeHandle};
    use nphysics2d::object::{ColliderDesc, RigidBodyDesc};

    let radius = cr.get_radius();

    // Create the ColliderDesc
    let shape = ShapeHandle::new(Ball::new(radius));
    let collide_handle = ColliderDesc::new(shape);

    let mass = cr.get_mass();
    let position = Vector2::new(cr.get_px(), cr.get_py());

    // Build the RigidBody
    let rigid_body = RigidBodyDesc::new()
        .mass(mass)
        .translation(position)
        .collider(&collide_handle)
        .build(world);

    return rigid_body.handle();
}