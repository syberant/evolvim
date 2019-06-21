extern crate lib_evolvim;

// use lib_evolvim::graphics::*;
use lib_evolvim::*;

#[test]
fn test_brain_evolve() {
    let mut world = nphysics2d::world::World::<f64>::new();

    let c_1 = Creature::new_random(&mut world, (100, 100), 0.0);
    let c_2 = Creature::new_random(&mut world, (100, 100), 0.0);

    let _new_brain = Brain::recombination_infinite_parents(&[&c_1, &c_2]);
}
