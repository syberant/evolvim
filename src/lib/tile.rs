use super::*;

const FOOD_GROWTH_RATE: f64 = 1.0;
const MAX_GROWTH_LEVEL: f64 = 3.0;

pub enum Tile {
    Water,
    Land(LandTile),
}

impl Tile {
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
    pub fn update(&mut self, board: &Board) {
        match self {
            Tile::Water => {}
            Tile::Land(t) => t.update(board),
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
}

pub struct LandTile {
    fertility: f64,
    food_level: f64,

    last_update_time: f64,
}

impl LandTile {
    /// Update this tile
    ///
    /// NOTE: code was almost directly copied from carykh's original Processing version and is pretty messy.
    fn update(&mut self, board: &Board) {
        // TODO: clean up this mess!
        let time = board.get_time();

        if time - self.last_update_time > 0.00001 {
            let growth_change = board.get_growth_over_time_range(self.last_update_time);

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
        }

        self.last_update_time = time;
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
