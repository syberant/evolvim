extern crate lib_evolvim;

use lib_evolvim::neat;

#[test]
fn test_construct_random() {
    let gen = neat::Genome::new_fully_linked();

    gen.log_nodes();
    gen.log_connections();
}
