use super::*;
use constants::*;

mod creature;
mod rock;

pub use self::creature::*;
pub use self::rock::*;
use std::cell::{Ref, RefCell, RefMut};
use std::ops::Range;
use std::rc::Rc;

const COLLISION_FORCE: f64 = 0.01;
const PIECES: usize = 20;
const AGE_FACTOR: f64 = 1.0;
const MATURE_AGE: f64 = 0.01;

/// Our `safe` saviour! Provides multiple references to a `SoftBody`.
pub type RcSoftBody = Rc<RefCell<SoftBody>>;

/// Higher-Level SoftBody
///
/// This is a wrapper struct providing some useful functions.
///
/// TODO: come up with a better name.
pub struct HLSoftBody(RcSoftBody);

impl From<RcSoftBody> for HLSoftBody {
    fn from(val: RcSoftBody) -> Self {
        return HLSoftBody(val);
    }
}

impl Into<RcSoftBody> for HLSoftBody {
    fn into(self) -> RcSoftBody {
        return self.0;
    }
}

impl From<SoftBody> for HLSoftBody {
    fn from(sb: SoftBody) -> HLSoftBody {
        HLSoftBody::from(Rc::new(RefCell::new(sb)))
    }
}

// impl Deref for HLSoftBody {
//     type Target = SoftBody;

//     fn deref(&self) -> &SoftBody {
//         self.0.borrow().deref()
//     }
// }

// impl DerefMut for HLSoftBody {
//     fn deref_mut(&mut self) -> &mut SoftBody {
//         self.0.borrow_mut().deref_mut()
//     }
// }

impl PartialEq<HLSoftBody> for HLSoftBody {
    fn eq(&self, rhs: &HLSoftBody) -> bool {
        Rc::ptr_eq(self.value_ref(), rhs.value_ref())
    }
}

impl PartialEq<RcSoftBody> for HLSoftBody {
    fn eq(&self, rhs: &RcSoftBody) -> bool {
        Rc::ptr_eq(self.value_ref(), rhs)
    }
}

impl HLSoftBody {
    /// Wrapper function
    pub fn borrow(&self) -> Ref<SoftBody> {
        return self.0.borrow();
    }

    /// Wrapper function
    pub fn borrow_mut(&self) -> RefMut<SoftBody> {
        return self.0.borrow_mut();
    }

    /// Returns a boolean indicating whether this `HLSoftBody` is currently borrowed.
    pub fn can_borrow_mut(&self) -> bool {
        return self.0.try_borrow_mut().is_ok();
    }

    /// Returns a reference to the underlying `RcSoftBody`
    pub fn value_ref(&self) -> &RcSoftBody {
        return &self.0;
    }

    /// Get a clone of the internal `RcSoftBody`
    pub fn value_clone(&self) -> RcSoftBody {
        return Rc::clone(&self.0);
    }

    /// Calls the same function on all types and updates `SoftBodiesInPositions` by calling `set_sbip`.
    pub fn apply_motions(
        &self,
        time_step: f64,
        board_size: BoardSize,
        terrain: &Terrain,
        sbip: &mut SoftBodiesInPositions,
    ) {
        use std::ops::DerefMut;

        match self.borrow_mut().deref_mut() {
            SoftBody::Rock(b) => b.apply_motions(time_step, board_size),
            SoftBody::Creature(c) => c.apply_motions(time_step, terrain, board_size),
        };

        self.set_sbip(sbip, board_size);
    }

    /// Updates `SoftBodiesInPositions` and updates itself by calling `update_sbip_variables()`.
    pub fn set_sbip(&self, sbip: &mut SoftBodiesInPositions, board_size: BoardSize) {
        // TODO: Look for optimizations here by cleaning and filling sbip more intelligently.

        let mut self_borrow = self.borrow_mut();

        self_borrow.update_sbip_variables(board_size);

        if self_borrow.moved_between_tiles() {
            for x in self_borrow.previous_x_range() {
                for y in self_borrow.previous_y_range() {
                    // Prevents deleting tiles we are currently in.
                    if !self_borrow.is_in_tile(x, y) {
                        sbip.remove_soft_body_at(x, y, self.value_clone());
                    }
                }
            }

            for x in self_borrow.current_x_range() {
                for y in self_borrow.current_y_range() {
                    // Prevents duplicate entries.
                    if !self_borrow.was_in_tile(x, y) {
                        sbip.add_soft_body_at(x, y, self.value_clone());
                    }
                }
            }
        }
    }

    /// Checks for collision and adjusts velocity if that's the case.
    ///
    /// TODO: clean up the many uses of `borrow()`
    pub fn collide(&self, sbip: &SoftBodiesInPositions) {
        let mut colliders = self.borrow().get_colliders(sbip);

        // Remove self
        colliders.remove_softbody(self.value_clone());

        let self_px = self.borrow().get_px();
        let self_py = self.borrow().get_py();
        let self_radius = self.borrow().get_radius();
        let self_mass = self.borrow().get_mass();

        for collider_rc in colliders {
            let collider = collider_rc.borrow();

            let (collider_px, collider_py) = (collider.get_px(), collider.get_py());
            let distance = SoftBody::distance(self_px, self_py, collider_px, collider_py);

            let combined_radius = self_radius + collider.get_radius();

            if distance < combined_radius {
                let force = combined_radius * COLLISION_FORCE;

                let add_vx = ((self_px - collider_px) / distance) * force * self_mass;
                let add_vy = ((self_py - collider_py) / distance) * force * self_mass;

                let mut self_mut_deref = self.borrow_mut();
                self_mut_deref.add_vx(add_vx);
                self_mut_deref.add_vy(add_vy);
            }
        }

        // TODO: translate this from Processing to Rust
        // fight_level = 0;
    }

    /// This function requires a reference to a `Board`.
    /// This is usually impossible so you'll have to turn to `unsafe`.
    pub fn return_to_earth(&mut self, board: &mut Board, board_size: BoardSize) {
        let time = board.get_time();

        // To make the borrow-checker happy.
        {
            let terrain = &mut board.terrain;
            let sbip = &mut board.soft_bodies_in_positions;

            let mut self_deref = self.borrow_mut();

            for _i in 0..PIECES {
                let tile_pos = self_deref.get_random_covered_tile(board_size);
                terrain.add_food_or_nothing_at(tile_pos, self_deref.get_energy() / PIECES as f64);

                // TODO: check if this is neccessary and fix this mess!
                terrain.update_at(tile_pos, time, &board.climate);
            }

            self_deref.remove_from_sbip(sbip, self.value_clone());
        }

        // Unselect this creature if it was selected.
        board.unselect_if_dead(self.value_ref());
    }

    /// Returns a new creature if there's a birth, otherwise returns `None`
    // TODO: cleanup
    pub fn try_reproduce(
        &mut self,
        time: f64,
        sbip: &mut SoftBodiesInPositions,
        board_size: BoardSize,
    ) -> Option<HLSoftBody> {
        if self.borrow().get_creature().get_energy() > SAFE_SIZE
            && self.borrow().get_creature().brain.wants_birth() > 0.0
            && self.borrow().get_creature().get_age(time) > MATURE_AGE
        {
            use std::ops::Deref;

            let self_px = self.borrow().get_px();
            let self_py = self.borrow().get_py();
            let self_radius = self.borrow().get_radius();

            let mut parents: Vec<HLSoftBody> = self
                .borrow()
                .get_colliders(sbip)
                .into_iter()
                .filter(|rc_soft| {
                    match rc_soft.borrow().deref() {
                        SoftBody::Creature(c) => {
                            let dist = SoftBody::distance(
                                self_px,
                                self_py,
                                c.base.get_px(),
                                c.base.get_py(),
                            );
                            let combined_radius = self_radius * FIGHT_RANGE + c.base.get_radius();

                            c.brain.wants_birth() > -1.0 // must be a willing creature
                            && dist < combined_radius // must be close enough

                            // TODO: find out if this addition to the Processing code works
                            // && c.get_age(time) >= MATURE_AGE // creature must be old enough
                            // && c.base.get_energy() > SAFE_SIZE
                        }
                        // Only creatures can be parents
                        _ => false,
                    }
                })
                .map(|rc_soft| HLSoftBody::from(Rc::clone(&rc_soft)))
                .collect();

            let available_energy = parents.iter().fold(0.0, |acc, c| {
                acc + c.borrow().get_creature().get_baby_energy()
            });

            if available_energy > BABY_SIZE {
                let energy = BABY_SIZE;

                // Giving birth costs energy
                parents.iter_mut().for_each(|c| {
                    let mut c_ref_mut = c.borrow_mut();
                    let c = c_ref_mut.get_creature_mut();

                    let energy_to_lose = energy * (c.get_baby_energy() / available_energy);
                    c.lose_energy(energy_to_lose);
                });

                let sb = HLSoftBody::from(SoftBody::Creature(Creature::new_baby(
                    parents, energy, time,
                )));

                sb.set_sbip(sbip, board_size);
                sb.set_sbip(sbip, board_size);

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

pub enum SoftBody {
    Rock(Rock),
    Creature(Creature),
}

impl SoftBody {
    /// Returns true if this `SoftBody` is a creature and false otherwise.
    pub fn is_creature(&self) -> bool {
        match self {
            SoftBody::Creature(_) => true,
            _ => false,
        }
    }

    /// Returns true if this `SoftBody` is a rock and false otherwise.
    pub fn is_rock(&self) -> bool {
        match self {
            SoftBody::Rock(_) => true,
            _ => false,
        }
    }

    /// Wrapper function.
    pub fn new_random_creature(board_size: BoardSize, time: f64) -> SoftBody {
        SoftBody::Creature(Creature::new_random(board_size, time))
    }

    /// Checks if the center is inside of the world, possibly corrects it and returns it.
    pub fn check_center_x(x: usize, board_width: usize) -> usize {
        return x.max(0).min(board_width - 1);
    }

    /// Checks if the center is inside of the world, possibly corrects it and returns it.
    pub fn check_center_y(y: usize, board_height: usize) -> usize {
        return y.max(0).min(board_height - 1);
    }

    /// Completely removes this `SoftBody` from `sbip`.
    ///
    /// NOTE: `SoftBody` is added again when `set_sbip` is called.
    pub fn remove_from_sbip(&mut self, sbip: &mut SoftBodiesInPositions, self_ref: RcSoftBody) {
        for x in self.current_x_range() {
            for y in self.current_y_range() {
                sbip.remove_soft_body_at(x, y, Rc::clone(&self_ref));
            }
        }
    }

    /// Returns the distance between two points.
    ///
    /// Uses the Pythagorean theorem: A^2 + B^2 = C^2.
    pub fn distance(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
        ((x1 - x2).powi(2) + (y1 - y2).powi(2)).sqrt()
    }
}

// Here are all the functions only applicable to `Creature`s.
impl SoftBody {
    /// Returns a reference to the `Creature` that was hidden in this `SoftBody` or `panic!`s.
    pub fn get_creature(&self) -> &Creature {
        match self {
            SoftBody::Creature(c) => c,
            _ => panic!("This `SoftBody` is not a `Creature`! It looks like you accidentally called `get_creature`!"),
        }
    }

    /// Returns a mutable reference to the `Creature` that was hidden in this `SoftBody` or `panic!`s.
    pub fn get_creature_mut(&mut self) -> &mut Creature {
        match self {
            SoftBody::Creature(c) => c,
            _ => panic!("This `SoftBody` is not a `Creature`! It looks like you accidentally called `get_creature_mut`!"),
        }
    }

    /// Wrapper function.
    pub fn get_birth_time(&self) -> f64 {
        return self.get_creature().get_birth_time();
    }

    /// Parts of this function are unsafe. Only mess with them if you know what you're doing!
    pub fn use_brain(
        &mut self,
        time_step: f64,
        use_output: bool,
        // The following are parts of a `Board`.
        time: f64,
        board_size: BoardSize,
        terrain: &mut Terrain,
        climate: &Climate,
    ) {
        let input = self.get_input(terrain);
        let creature = self.get_creature_mut();
        creature.brain.run(input);

        if use_output {
            creature
                .base
                .accelerate(creature.brain.wants_acceleration(), time_step);
            creature
                .base
                .turn(creature.brain.wants_turning(), time_step);

            // TODO: clean this mess.
            let tile_pos = creature.base.get_random_covered_tile(board_size);
            let tile = terrain.get_tile_at_mut(tile_pos);

            let eat_amount = creature.brain.wants_to_eat();
            creature.eat(eat_amount, time_step, time, climate, tile);

            // Fight
            // unimplemented!();

            // Reproducing is done elsewhere

            let mouth_hue = creature.brain.wants_mouth_hue();
            creature.set_mouth_hue(mouth_hue);
        }
    }

    /// Gets the input for the brain of the creature.
    ///
    /// TODO: improve!
    fn get_input(&self, terrain: &Terrain) -> BrainInput {
        let mut input = [0.0; 9];

        let creature = self.get_creature();
        input[0] = creature.get_energy();
        input[1] = creature.get_mouth_hue();
        let pos = BoardPreciseCoordinate(self.get_px(), self.get_py()).into();
        let tile = terrain.get_tile_at(pos);
        let c = tile.get_hsba_color();
        input[2] = c[0] as f64;
        input[3] = c[1] as f64;
        input[4] = c[2] as f64;

        return input;
    }

    /// Performs the energy requirement to keep living.
    pub fn metabolize(&mut self, time_step: f64, board: &Board) {
        // TODO: fix ugly code.
        let age = AGE_FACTOR * (board.get_time() - self.get_birth_time());
        let creature = self.get_creature_mut();
        let energy_to_lose = creature.get_energy() * METABOLISM_ENERGY * age * time_step;
        creature.lose_energy(energy_to_lose);

        // Creature should die if it doesn't have enough energy, this is done by `Board`.
    }

    /// Wrapper function
    pub fn should_die(&self) -> bool {
        return self.get_creature().should_die();
    }
}

// Here are all the functions which merely call the same function on the underlying types.
impl SoftBody {
    fn get_random_covered_tile(&self, board_size: BoardSize) -> BoardCoordinate {
        match self {
            SoftBody::Rock(b) => b.get_random_covered_tile(board_size),
            SoftBody::Creature(c) => c.base.get_random_covered_tile(board_size),
        }
    }

    /// Returns `true` if this `SoftBody` has moved between tiles since the last update.
    ///
    /// Used to determine if `SoftBodiesInPosisitions` should be updated and `set_sbip` should be called.
    ///
    /// Wrapper function.
    fn moved_between_tiles(&self) -> bool {
        match self {
            SoftBody::Rock(b) => b.moved_between_tiles(),
            SoftBody::Creature(c) => c.base.moved_between_tiles(),
        }
    }

    /// Wrapper function.
    fn is_in_tile(&self, x: usize, y: usize) -> bool {
        match self {
            SoftBody::Rock(b) => b.is_in_tile(x, y),
            SoftBody::Creature(c) => c.base.is_in_tile(x, y),
        }
    }

    /// Wrapper function.
    fn was_in_tile(&self, x: usize, y: usize) -> bool {
        match self {
            SoftBody::Rock(b) => b.was_in_tile(x, y),
            SoftBody::Creature(c) => c.base.was_in_tile(x, y),
        }
    }

    /// Wrapper function.
    fn previous_x_range(&self) -> Range<usize> {
        match self {
            SoftBody::Rock(b) => b.previous_x_range(),
            SoftBody::Creature(c) => c.base.previous_x_range(),
        }
    }

    /// Wrapper function.
    fn previous_y_range(&self) -> Range<usize> {
        match self {
            SoftBody::Rock(b) => b.previous_y_range(),
            SoftBody::Creature(c) => c.base.previous_y_range(),
        }
    }

    /// Wrapper function.
    fn current_x_range(&self) -> Range<usize> {
        match self {
            SoftBody::Rock(b) => b.current_x_range(),
            SoftBody::Creature(c) => c.base.current_x_range(),
        }
    }

    /// Wrapper function.
    fn current_y_range(&self) -> Range<usize> {
        match self {
            SoftBody::Rock(b) => b.current_y_range(),
            SoftBody::Creature(c) => c.base.current_y_range(),
        }
    }

    /// Wrapper function.
    fn get_colliders(&self, sbip: &SoftBodiesInPositions) -> SoftBodiesAt {
        match self {
            SoftBody::Rock(b) => b.get_colliders(sbip),
            SoftBody::Creature(c) => c.base.get_colliders(sbip),
        }
    }

    /// Wrapper function.
    fn update_sbip_variables(&mut self, board_size: BoardSize) {
        match self {
            SoftBody::Rock(b) => b.update_sbip_variables(board_size),
            SoftBody::Creature(c) => c.base.update_sbip_variables(board_size),
        };
    }

    /// Wrapper function.
    pub fn get_px(&self) -> f64 {
        match self {
            SoftBody::Rock(b) => b.get_px(),
            SoftBody::Creature(c) => c.base.get_px(),
        }
    }

    /// Wrapper function.
    pub fn get_py(&self) -> f64 {
        match self {
            SoftBody::Rock(b) => b.get_py(),
            SoftBody::Creature(c) => c.base.get_py(),
        }
    }

    /// Wrapper function.
    pub fn get_radius(&self) -> f64 {
        match self {
            SoftBody::Rock(b) => b.get_radius(),
            SoftBody::Creature(c) => c.base.get_radius(),
        }
    }

    /// Wrapper function.
    fn get_mass(&self) -> f64 {
        match self {
            SoftBody::Rock(b) => b.get_mass(),
            SoftBody::Creature(c) => c.base.get_mass(),
        }
    }

    /// Wrapper function.
    fn get_energy(&self) -> f64 {
        match self {
            SoftBody::Rock(b) => b.get_energy(),
            SoftBody::Creature(c) => c.get_energy(),
        }
    }

    /// Wrapper function.
    fn add_vx(&mut self, value_to_add: f64) {
        match self {
            SoftBody::Rock(b) => b.add_vx(value_to_add),
            SoftBody::Creature(c) => c.base.add_vx(value_to_add),
        }
    }

    /// Wrapper function.
    fn add_vy(&mut self, value_to_add: f64) {
        match self {
            SoftBody::Rock(b) => b.add_vy(value_to_add),
            SoftBody::Creature(c) => c.base.add_vy(value_to_add),
        }
    }
}
