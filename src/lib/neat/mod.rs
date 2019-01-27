mod gene;
// mod genome;
mod mutation;

use self::gene::{NodeGene, ConnectionGene, Id};

pub struct Genome {
    node_genome: Vec<NodeGene>,
    connection_genome: Vec<ConnectionGene>,

    node_id: Id,
}

impl Genome {
    pub fn next_node_id(&mut self) -> Id {
        self.node_id += 1;
        return self.node_id;
    }

    pub fn get_random_weight() -> f64 {
        unimplemented!()
    }
}