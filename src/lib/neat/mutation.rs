use super::{Genome, get_next_node_id};
use super::gene::{NodeType, NodeGene};

const CHANCE_MUTATE_NEW_LINK: f64 = 0.1;
const CHANCE_MUTATE_LINK_TO_NODE: f64 = 0.05;
const CHANCE_MUTATE_TWEAK_WEIGHT: f64 = 0.6;
const CHANCE_MUTATE_RANDOM_WEIGHT: f64 = 0.2;
const CHANCE_MUTATE_TOGGLE_ENABLED: f64 = 0.05;

impl Genome {
    pub fn mutate(&mut self) {
        use MutationType::*;
        use rand::distributions::Distribution;

        enum MutationType {
            AddConnection,
            ConnectionToNode,
            TweakWeight,
            RandomizeWeight,
            ToggleEnabled,
        }

        impl MutationType {
            const fn get_choices() -> [Self; 5] {
                [
                    AddConnection,
                    ConnectionToNode,
                    TweakWeight,
                    RandomizeWeight,
                    ToggleEnabled
                ]
            }

            const fn get_weights() -> [f64; 5] {
                [
                    CHANCE_MUTATE_NEW_LINK,
                    CHANCE_MUTATE_LINK_TO_NODE,
                    CHANCE_MUTATE_TWEAK_WEIGHT,
                    CHANCE_MUTATE_RANDOM_WEIGHT,
                    CHANCE_MUTATE_TOGGLE_ENABLED,
                ]
            }
        }


        let dist = rand::distributions::WeightedIndex::new(&MutationType::get_weights()).unwrap();
        let mut rng = rand::thread_rng();
        let times = self.connection_genome.len() / 2;

        for _i in 0..times {
            match MutationType::get_choices()[dist.sample(&mut rng)] {
                AddConnection => self.mutate_add_connection(),
                ConnectionToNode => self.mutate_connection_to_node(),
                TweakWeight => self.mutate_tweak_weight(),
                RandomizeWeight => self.mutate_randomize_weight(),
                ToggleEnabled => self.mutate_toggle_gene(),
            }
        }
    }

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

    pub fn mutate_toggle_gene(&mut self) {
        let connection_id = self.get_random_connection_place();
        // toggle `enabled`
        self.connection_genome[connection_id].toggle_enabled();
    }
}