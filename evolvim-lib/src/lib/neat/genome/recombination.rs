use super::gene::NodeGene;
use super::utils::{RecombinationGeneTypes, RecombinationGenomesIterator};
use super::Genome;

impl Genome {
    /// Multipoint crossover:
    /// - matching genes: average the weight
    /// - disjoint genes: always include
    /// - excess genes: always include
    pub fn new_from_2(parent_a: &Genome, parent_b: &Genome) -> Genome {
        let mut genome = Genome {
            node_genome: Vec::new(),
            connection_genome: Vec::new(),
        };

        use RecombinationGeneTypes::*;
        for g in RecombinationGenomesIterator::new(parent_a, parent_b) {
            match g {
                Matching(a, b) => {
                    genome.connection_genome.push(a.clone());
                    genome.connection_genome.last_mut().unwrap().weight += b.weight;
                    genome.connection_genome.last_mut().unwrap().weight /= 2.0;
                }
                Disjoint(_, gene) => {
                    genome.connection_genome.push(gene.clone());
                }
                Excess(_, gene) => {
                    genome.connection_genome.push(gene.clone());
                }
            }
        }

        // Make the node genome
        genome.generate_nodes_from_connections(&parent_a.node_genome, &parent_b.node_genome);

        return genome;
    }

    fn generate_nodes_from_connections(
        &mut self,
        parent_a: &Vec<NodeGene>,
        parent_b: &Vec<NodeGene>,
    ) {
        use std::collections::HashSet;

        let mut neuron_ids = HashSet::new();

        for i in &self.connection_genome {
            neuron_ids.insert(i.from);
            neuron_ids.insert(i.to);
        }

        for a in parent_a {
            if neuron_ids.remove(&a.id) {
                self.node_genome.push(a.clone());
            }
        }

        for b in parent_b {
            if neuron_ids.remove(&b.id) {
                self.node_genome.push(b.clone());
            }
        }
    }
}
