use super::*;

pub const MINIMUM_SURVIVABLE_SIZE: f64 = 0.06;

#[derive(Clone, Serialize, Deserialize)]
pub struct Creature<B> {
    pub base: Rock,
    pub brain: B,
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
    pub fn new_random(board_size: BoardSize, time: f64) -> Self {
        let energy = CREATURE_MIN_ENERGY
            + rand::random::<f64>() * (CREATURE_MAX_ENERGY - CREATURE_MIN_ENERGY);
        let base = Rock::new_random(board_size, CREATURE_DENSITY, energy, time);
        let brain = B::new_random();
        // TODO: add id

        Creature { base, brain }
    }
}

impl<B: NeuralNet + RecombinationInfinite> Creature<B> {
    /// Create a new baby, it isn't in `SoftBodiesInPositions` so please fix that.
    /// While you're at it, also add it to `Board.creatures`.
    pub fn new_baby(parents: &[&SoftBody<B>], energy: f64, time: f64) -> Creature<B> {
        let brain = B::recombination_infinite_parents(parents);
        let base = Rock::new_from_parents(parents, energy, time);

        Creature { base, brain }
    }
}

impl<B> Creature<B> {
    // The `Creature` version of `apply_motions`, this is different to the `Rock` version.
    pub fn apply_motions(&mut self, time_step: f64, terrain: &Terrain, board_size: BoardSize) {
        if self.is_on_water(terrain, board_size) {
            let energy_to_lose = time_step * SWIM_ENERGY * self.get_energy();
            self.lose_energy(energy_to_lose);
        }

        self.base.apply_motions(time_step, board_size);
    }

    pub fn should_die(&self) -> bool {
        return self.get_energy() < SAFE_SIZE;
    }

    pub fn get_baby_energy(&self) -> f64 {
        self.base.get_energy() - SAFE_SIZE
    }
}
