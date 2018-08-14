extern crate lib_evolvim;

// use lib_evolvim::graphics::*;
use lib_evolvim::*;

#[test]
fn test_brain_evolve() {
    let brain_1 = Brain::new_random();
    let brain_2 = Brain::new_random();

    let _new_brain = Brain::evolve(vec![&brain_1, &brain_2]);

    // make sure we didn't move brain_1
    assert!(brain_1.get_output().len() > 0);
}
