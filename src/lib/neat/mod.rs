extern crate rand;

mod gene;
mod recombination;
mod mutation;

use self::gene::{NodeGene, ConnectionGene, Id, NodeType};

const AMOUNT_INPUT: usize = 3;
const AMOUNT_OUTPUT: usize = 2;

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
        rand::random::<f64>() * 2.0 - 1.0
    }

    fn add_node(&mut self, node_type: NodeType) {
        let id = self.next_node_id();

        self.node_genome.push(NodeGene {
            node_type,
            id,
        });
    }

    // fn add_connection(&mut self)

    fn add_random_link(&mut self, from: Id, to: Id) {
        let weight = Self::get_random_weight();
        self.mutate_add_connection(from, to, weight);
    }

    pub fn new_fully_linked() -> Self {
        let mut genome = Genome {
            node_genome: Vec::new(),
            connection_genome: Vec::new(),
            node_id: 0,
        };

        for _i in 0..AMOUNT_INPUT {
            genome.add_node(NodeType::Sensor);
        }

        for _i in 0..AMOUNT_OUTPUT {
            genome.add_node(NodeType::Output);
            
            let to = genome.node_genome.last().unwrap().id;
            for i in 0..AMOUNT_INPUT {
                let from = genome.node_genome[i].id;

                genome.add_random_link(from, to);
            }
        }

        return genome;
    }

    // pub fn mutate(&mut self) {
    //     let gene_count = self.connection_genome.len();

    // }
}

impl Genome {
    pub fn log_nodes(&self) {
        for n in &self.node_genome {
            println!("node {} is {:?}", n.id, n.node_type);
        }
    }

    pub fn log_connections(&self) {
        for n in &self.connection_genome {
            println!("innovation {}: from {} to {} with weight {}", n.innovation_number, n.from, n.to, n.weight);
        }
    }
}