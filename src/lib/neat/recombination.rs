use super::Genome;
// use super::gene::{NodeGene, ConnectionGene};

impl Genome {
    pub fn new_from_2(parent_a: &Genome, parent_b: &Genome) -> Genome {
        let mut genome = Genome {
            node_genome: Vec::new(),
            connection_genome: Vec::new(),
        };
        
        let mut iter_b = parent_b.connection_genome.iter();
        let mut maybe_gene_b = iter_b.next();
        for gene_a in &parent_a.connection_genome {
            match maybe_gene_b {
                None => genome.connection_genome.push(gene_a.clone()),
                Some(gene_b) => {
                    // Inherit randomly
                    let chosen_gene = unimplemented!();
                    genome.connection_genome.push(chosen_gene);
                    maybe_gene_b = iter_b.next();
                }
            }
        }
        while maybe_gene_b.is_some() {
            genome.connection_genome.push(maybe_gene_b.unwrap().clone());
            maybe_gene_b = iter_b.next();
        }

        // Make the node genome
        unimplemented!(); // TODO!

        return genome;
    }
}