use super::Genome;
use super::gene::{NodeGene, ConnectionGene, Id, NodeType};

static mut INNOVATION_NUMBER: usize = 0;

fn get_innovation_number() -> usize {
    unsafe {
        INNOVATION_NUMBER += 1;
        return INNOVATION_NUMBER;
    }
}

impl Genome {
    pub fn mutate_add_connection(&mut self, from: Id, to: Id, weight: f64) {
        let connection = ConnectionGene {
            from,
            to,
            weight,

            enabled: true,
            innovation_number: get_innovation_number(),
        };

        self.connection_genome.push(connection);
    }

    pub fn mutate_connection_to_node(&mut self, connection_id: usize) {
        let next_node_id = self.next_node_id();
        let (from, to) = self.connection_genome[connection_id].disable_and_info();

        self.mutate_add_connection(from, next_node_id, Self::get_random_weight());
        self.mutate_add_connection(next_node_id, to, Self::get_random_weight());

        self.node_genome.push(
            NodeGene {
                node_type: NodeType::Hidden,
                id: next_node_id,
            }
        );
    }
}