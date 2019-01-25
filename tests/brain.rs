extern crate lib_evolvim;

// use lib_evolvim::graphics::*;
use lib_evolvim::*;

#[test]
fn test_brain_evolve() {
    let c_1 = HLSoftBody::from(Creature::new_random((100, 100), 0.0));
    let c_2 = HLSoftBody::from(Creature::new_random((100, 100), 0.0));

    let _new_brain = Brain::evolve(&vec![c_1, c_2]);
}
