use crate::softbody::Rock;
use crate::Terrain;

pub struct EnvironmentMut<'a> {
    pub terrain: &'a mut Terrain,
    pub this_body: &'a mut Rock,
}

impl<'a> EnvironmentMut<'a> {
    pub fn new(terrain: &'a mut Terrain, this_body: &'a mut Rock) -> Self {
        EnvironmentMut { terrain, this_body }
    }
}

pub struct Environment<'a> {
    pub terrain: &'a Terrain,
    pub this_body: &'a Rock,
}

impl<'a> Environment<'a> {
    pub fn new(terrain: &'a Terrain, this_body: &'a Rock) -> Self {
        Environment { terrain, this_body }
    }
}
