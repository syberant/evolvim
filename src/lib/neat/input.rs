use crate::Terrain;

pub struct Environment<'a> {
    terrain: &'a Terrain,
}

pub enum InputType {
    Eye(Eye),
    Bias(f64),
}

impl InputType {
    pub fn get_data(&self, env: &Environment) -> f64 {
        use InputType::*;

        match &self {
            Bias(v) => *v,
            Eye(s) => s.get_data(env),
        }
    }
}

struct Eye {
    relative_distance: f64,
    angle: f64,
}

impl Eye {
    pub fn get_data(&self, _env: &Environment) -> f64 {
        unimplemented!()
    }
}
