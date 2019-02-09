use super::{Genome, get_next_node_id};
use super::gene::{NodeType, NodeGene};

impl Genome {
    pub fn mutate_add_connection(&mut self) {
        let from = self.get_random_node_id();
        let to = self.get_random_node_id();
        let weight = Self::get_random_weight();
        
        self.add_connection(from, to, weight);
    }

    pub fn mutate_connection_to_node(&mut self) {
        let connection_id = self.get_random_connection_place();
        let next_node_id = get_next_node_id();
        let (from, to) = self.connection_genome[connection_id].disable_and_info();

        self.add_connection(from, next_node_id, Self::get_random_weight());
        self.add_connection(next_node_id, to, Self::get_random_weight());

        self.node_genome.push(
            NodeGene {
                node_type: NodeType::Hidden,
                id: next_node_id,
            }
        );
    }

    pub fn mutate_tweak_weight(&mut self) {
        let connection_id = self.get_random_connection_place();
        self.connection_genome[connection_id].weight *= Self::get_random_weight_multiplier();
    }

    pub fn mutate_randomize_weight(&mut self) {
        let connection_id = self.get_random_connection_place();
        self.connection_genome[connection_id].weight = Self::get_random_weight();
    }
}