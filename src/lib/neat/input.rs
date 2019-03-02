use crate::brain::Environment;

#[derive(Debug)]
pub enum InputType {
    Eye(Eye),
    Bias(f64),
    MouthHue,
    Energy,
}

impl InputType {
    pub fn get_data(&self, env: &Environment) -> f64 {
        use InputType::*;

        match &self {
            Bias(v) => *v,
            Eye(s) => s.get_data(env),
            MouthHue => env.this_body.get_mouth_hue(),
            Energy => env.this_body.get_energy(),
        }
    }
}

#[derive(Debug)]
struct Eye {
    relative_distance: f64,
    angle: f64,
    what_to_look_for: EyeType,
}

#[derive(Debug)]
enum EyeType {
    FoodLevel,
    FoodColor,
    TileFertility,
}

impl Eye {
    pub fn get_data(&self, env: &Environment) -> f64 {
        use crate::board::BoardPreciseCoordinate;
        use EyeType::*;

        let real_angle = self.angle + env.this_body.get_rotation();
        let x = real_angle.cos() * self.relative_distance;
        let y = real_angle.sin() * self.relative_distance;
        let pos = BoardPreciseCoordinate(x + env.this_body.get_px(), y + env.this_body.get_py());

        let tile = env.terrain.get_tile_at(pos.into());
        match self.what_to_look_for {
            FoodLevel => tile.get_food_level(),
            FoodColor => tile.get_food_type(),
            TileFertility => tile.get_fertility(),
        }
    }
}
