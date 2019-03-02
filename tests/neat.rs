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

    for _i in 0..5 {
        gen1.mutate();
        gen2.mutate();
    }

    println!("Parent A:");
    gen1.log_nodes();
    gen1.log_connections();

    println!("Parent B:");
    gen2.log_nodes();
    gen2.log_connections();

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

#[test]
fn test_genetical_distance() {
    let mut gen1 = neat::Genome::new_fully_linked();
    let mut gen2 = neat::Genome::new_fully_linked();

    for _i in 0..5 {
        gen1.mutate();
        gen2.mutate();
    }

    let distance = gen1.genetical_distance(&gen2);
    println!(
        "The distance between two randomly mutated genomes is {}",
        distance
    );
}

#[test]
fn test_generate_phenotype() {
    let gen = neat::Genome::new_fully_linked();
    let _phen: neat::NeuralNet = (&gen).into();
}

#[test]
fn test_run_phenotype() {
    let gen = neat::Genome::new_fully_linked();
    let mut phen: neat::NeuralNet = (&gen).into();

    phen.run_calculations();
}
