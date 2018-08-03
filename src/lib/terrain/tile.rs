use super::*;

const FOOD_GROWTH_RATE: f64 = 1.0;
const MAX_GROWTH_LEVEL: f64 = 3.0;
const FOOD_SENSITIVITY: f64 = 0.3;

pub enum Tile {
    Water,
    Land(LandTile),
}

impl Tile {
    pub fn new(fertility: f64, food_type: f64) -> Self {
        if fertility < 1.0 {
            // Tile is land
            let t = LandTile::new(fertility, food_type);

            Tile::Land(t)
        } else {
            // Tile is water
            Tile::Water
        }
    }

    pub fn is_water(&self) -> bool {
        match self {
            Tile::Water => true,
            Tile::Land(_) => false,
        }
    }

    /// Get the `food_level` of this tile, returns 0 if it is water.
    pub fn get_food_level(&self) -> f64 {
        match self {
            Tile::Water => 0.0,
            Tile::Land(t) => t.food_level,
        }
    }

    /// Update this tile
    pub fn update(&mut self, time: f64, climate: &Climate) {
        match self {
            Tile::Water => {}
            Tile::Land(t) => t.update(time, climate),
        }
    }

    /// Adds the given value to the food level if it's possible.
    ///
    /// This does nothing for water tiles.
    pub fn add_food_or_nothing(&mut self, food_to_add: f64) {
        match self {
            Tile::Water => {}
            Tile::Land(t) => t.add_food(food_to_add),
        }
    }

    /// Removes the given value from the food level.
    ///
    /// This panics for water tiles since you should never try gaining food from them.
    pub fn remove_food(&mut self, food_to_remove: f64) {
        match self {
            Tile::Water => if food_to_remove > 0.0 {
                panic!("You called `remove_food` on a water tile, water tiles don't have any food and should not be eaten.")
            },
            Tile::Land(t) => t.remove_food(food_to_remove),
        }
    }

    pub fn get_food_multiplier(&self, hue: f64) -> Option<f64> {
        match self {
            // Tile::Water => panic!("You called `get_food_multiplier` on a water tile, water tiles don't have any food and should not be eaten."),
            Tile::Water => None,
            Tile::Land(t) => Some(t.get_food_multiplier(hue)),
        }
    }
}

pub struct LandTile {
    fertility: f64,
    food_level: f64,
    food_type: f64,

    last_update_time: f64,
}

impl LandTile {
    /// Creates a new tile with the given `fertility` and `food_type`.
    ///
    /// Begins with `food_level` set to `fertility` and `last_update_time` set to `0`.
    pub fn new(fertility: f64, food_type: f64) -> Self {
        LandTile {
            fertility,
            food_level: fertility,
            food_type,

            last_update_time: 0.0,
        }
    }

    /// Update this tile
    ///
    /// NOTE: code was almost directly copied from carykh's original Processing version and is pretty messy.
    fn update(&mut self, time: f64, climate: &Climate) {
        // TODO: clean up this mess!
        if time - self.last_update_time > 0.00001 {
            let growth_change = climate.get_growth_over_time_range(time, self.last_update_time);

            if growth_change <= 0.0 {
                let food_to_remove =
                    self.food_level - self.food_level * (growth_change * FOOD_GROWTH_RATE).exp();
                self.remove_food(food_to_remove);
            } else if self.food_level < MAX_GROWTH_LEVEL {
                let new_dist_to_max = (MAX_GROWTH_LEVEL - self.food_level)
                    * (-growth_change * self.fertility * FOOD_GROWTH_RATE).exp();

                let food_to_add = MAX_GROWTH_LEVEL - new_dist_to_max - self.food_level;
                self.add_food(food_to_add);
            }

            self.last_update_time = time;
        }
    }

    pub fn get_food_multiplier(&self, hue: f64) -> f64 {
        return 1.0 - (self.food_type - hue).abs() / FOOD_SENSITIVITY;
    }

    /// Subtracts the given amount of food from `self.food_level` and makes sure it can't get negative.
    ///
    /// This takes the maximum of 0 and `food_level` after subtraction.
    ///
    /// NOTE: Doesn't call `update()` like in carykh's Processing code.
    fn remove_food(&mut self, food_to_remove: f64) {
        self.food_level = 0f64.max(self.food_level - food_to_remove);
    }

    /// Adds the given amount of food from `self.food_level` and makes sure it can't get negative.
    ///
    /// This takes the maximum of 0 and `food_level` after adding.
    ///
    /// NOTE: Doesn't call `update()` like in carykh's Processing code.
    pub fn add_food(&mut self, food_to_add: f64) {
        self.food_level = 0f64.max(self.food_level + food_to_add);
    }
}
