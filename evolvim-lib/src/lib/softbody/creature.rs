use super::*;

use nphysics2d::object::{BodyHandle, RigidBody};
type World = nphysics2d::world::World<f64>;

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

impl<B: 'static + std::marker::Sync + std::marker::Send> specs::Component for Creature<B> {
    type Storage = specs::VecStorage<Self>;
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
        use rand::Rng;

        let energy = CREATURE_MIN_ENERGY
            + rand::random::<f64>() * (CREATURE_MAX_ENERGY - CREATURE_MIN_ENERGY);
        let base = Rock::new_random(board_size, CREATURE_DENSITY, energy, time);
        let brain = B::new_random();
        
        let mut rng = rand::thread_rng();
        let position = nalgebra::Vector2::<f64>::from_vec(
            vec!(
                rng.gen::<f64>() * board_size.0 as f64,
                rng.gen::<f64>() * board_size.1 as f64,
            )
        );
        let body = make_physics_creature(world, &base, position).into();
        // TODO: add id

        Creature { base, brain, body }
    }
}

impl<B: NeuralNet + RecombinationInfinite> Creature<B> {
    /// Create a new baby, it isn't in `SoftBodiesInPositions` so please fix that.
    /// While you're at it, also add it to `Board.creatures`.
    pub fn new_baby(
        world: &mut World,
        parents: &[&SoftBody<B>],
        energy: f64,
        time: f64,
    ) -> Creature<B> {
        let brain = B::recombination_infinite_parents(parents);
        let base = Rock::new_from_parents(parents, energy, time);
        let body = physics_creature_from_parent(world, &base, parents[0]).into();

        Creature { base, brain, body }
    }
}

impl<B: NeuralNet + RecombinationInfinite> Creature<B> {
    // Returns a new creature if there's a birth, otherwise returns `None`
    // TODO: cleanup
    pub fn try_reproduce(
        &mut self,
        time: f64,
        board_size: BoardSize,
        world: &mut World,
    ) -> Option<Creature<B>> {
        if self.wants_primary_birth(time) {
            // let self_px = self.get_px();
            // let self_py = self.get_py();
            let self_radius = self.get_radius();

            // let mut colliders: SoftBodiesAt<B> = unimplemented!();

            // // Remove self
            // colliders.remove_softbody(self.clone());

            // let mut parents: Vec<HLSoftBody<B>> = colliders
            //     .into_iter()
            //     .filter(|rc_soft| {
            //         let c = rc_soft.borrow(world);
            //         let dist = distance(self_px, self_py, c.get_px(), c.get_py());
            //         let combined_radius = self_radius * FIGHT_RANGE + c.get_radius();

            //         c.brain.wants_help_birth() > -1.0 // must be a willing creature
            //                 && dist < combined_radius // must be close enough

            //         // TODO: find out if this addition to the Processing code works
            //         // && c.get_age(time) >= MATURE_AGE // creature must be old enough
            //         // && c.base.get_energy() > SAFE_SIZE
            //     })
            //     .collect();

            // parents.push(self.clone());

            let parents: Vec<&mut Creature<B>> = unimplemented!();

            let available_energy = parents.iter().fold(0.0, |acc, c| acc + c.get_baby_energy());

            if available_energy > BABY_SIZE {
                let energy = BABY_SIZE;

                // Giving birth costs energy
                parents.iter_mut().for_each(|c| {
                    let energy_to_lose = energy * (c.get_baby_energy() / available_energy);

                    c.lose_energy(energy_to_lose);
                });
                let par: Vec<&SoftBody<B>> = parents.into_iter().map(|c| &*c).collect();

                let sb = Creature::new_baby(world, &par, energy, time);

                // Hooray! Return the little baby!
                Some(sb)
            } else {
                // There isn't enough energy available
                None
            }
        } else {
            // This creature can't give birth because of age, energy or because it doesn't want to.
            return None;
        }
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
        world: &World,
    ) {
        let rg_body = self.get_rigid_body(world);
        let pos = rg_body.position().translation.vector;
        let tile_pos = (pos[0] as usize, pos[1] as usize);
        
        terrain.add_food_or_nothing_at(tile_pos, self.get_energy());
        terrain.update_at(tile_pos, time, climate);
    }

    pub fn should_die(&self) -> bool {
        return self.get_energy() < SAFE_SIZE;
    }

    pub fn get_baby_energy(&self) -> f64 {
        self.base.get_energy() - SAFE_SIZE
    }

    pub fn get_handle(&self) -> BodyHandle {
        self.body.0
    }

    fn get_rigid_body<'a>(&self, world: &'a World) -> &'a RigidBody<f64> {
        world.rigid_body(self.get_handle()).unwrap()
    }
}

fn make_physics_creature(world: &mut World, cr: &Rock, position: nalgebra::Vector2<f64>) -> BodyHandle {
    use ncollide2d::shape::{Ball, ShapeHandle};
    use nphysics2d::object::{ColliderDesc, RigidBodyDesc};

    // let radius = cr.get_radius();
    let radius = 0.3;

    // Create the ColliderDesc
    let shape = ShapeHandle::new(Ball::new(radius));
    let collide_handle = ColliderDesc::new(shape);

    let mass = cr.get_mass();

    // Build the RigidBody
    let rigid_body = RigidBodyDesc::new()
        .mass(mass)
        .translation(position)
        .collider(&collide_handle)
        .build(world);

    return rigid_body.handle();
}

fn physics_creature_from_parent<B>(world: &mut World, cr: &Rock, par: &Creature<B>) -> BodyHandle {
    unimplemented!()
}