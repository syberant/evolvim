extern crate lib_evolvim;

use lib_evolvim::neat;

#[test]
fn test_construct_random() {
    let gen1 = neat::Genome::new_fully_linked();

    println!("Initial random network:");
    gen1.log_nodes();
    gen1.log_connections();
}

#[test]
fn test_recombination() {
    let mut gen1 = neat::Genome::new_fully_linked();
    let mut gen2 = neat::Genome::new_fully_linked();

    for _i in 0..10 {
        gen1.mutate();
        gen2.mutate();
    }

    let baby = neat::Genome::new_from_2(&gen1, &gen2);
    println!("Genome after recombination:");
    baby.log_nodes();
    baby.log_connections();
}

#[test]
fn test_mutation() {
    let mut gen = neat::Genome::new_fully_linked();

    println!("Before mutation:");
    gen.log_nodes();
    gen.log_connections();

    for _i in 0..10 {
        gen.mutate();
    }

    println!("\nAfter mutation:");
    gen.log_nodes();
    gen.log_connections();
}