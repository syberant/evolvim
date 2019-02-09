use super::Genome;
use super::gene::NodeGene;
use super::rand;

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
        
        let mut genes_a = parent_a.connection_genome.iter().peekable();
        let mut genes_b = parent_b.connection_genome.iter().peekable();

        loop {
            let gene_a = genes_a.peek();
            let gene_b = genes_b.peek();

            if gene_a.is_none() {
                for g in genes_b {
                    // excess gene from parent B
                    genome.connection_genome.push(g.clone());
                }
                break;
            } else if gene_b.is_none() {
                for g in genes_a {
                    // excess gene from parent A
                    genome.connection_genome.push(g.clone());
                }
                break;
            }

            use std::cmp::Ordering;
            match gene_a.unwrap().innovation_number.cmp(&gene_b.unwrap().innovation_number) {
                Ordering::Equal => {
                    // matching gene, averaging weight
                    let weight_b = genes_b.next().unwrap().weight;
                    genome.connection_genome.push(genes_a.next().unwrap().clone());
                    genome.connection_genome.last_mut().unwrap().weight += weight_b;
                    genome.connection_genome.last_mut().unwrap().weight /= 2.0;
                },
                Ordering::Less => {
                    // disjoint gene from parent A
                    genome.connection_genome.push(genes_a.next().unwrap().clone());
                },
                Ordering::Greater => {
                    // disjoint gene from parent B
                    genome.connection_genome.push(genes_b.next().unwrap().clone());
                }
            }
        }

        // Make the node genome
        genome.generate_nodes_from_connections(&parent_a.node_genome, &parent_b.node_genome);

        return genome;
    }

    fn generate_nodes_from_connections(&mut self, parent_a: &Vec<NodeGene>, parent_b: &Vec<NodeGene>) {
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