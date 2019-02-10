use super::gene::ConnectionGene;
use super::Genome;
use std::iter::Peekable;

pub enum Parent {
    A,
    B,
}

pub enum RecombinationGeneTypes<'a> {
    Matching(&'a ConnectionGene, &'a ConnectionGene),
    Disjoint(Parent, &'a ConnectionGene),
    Excess(Parent, &'a ConnectionGene),
}

pub struct RecombinationGenomesIterator<'a> {
    parent_a: Peekable<std::slice::Iter<'a, ConnectionGene>>,
    parent_b: Peekable<std::slice::Iter<'a, ConnectionGene>>,
}

impl<'a> RecombinationGenomesIterator<'a> {
    pub fn new(a: &'a Genome, b: &'a Genome) -> Self {
        RecombinationGenomesIterator {
            parent_a: a.connection_genome.iter().peekable(),
            parent_b: b.connection_genome.iter().peekable(),
        }
    }
}

impl<'a> Iterator for RecombinationGenomesIterator<'a> {
    type Item = RecombinationGeneTypes<'a>;

    // TODO: make some use of if let to remove unneccessary .unwrap()'s
    fn next(&mut self) -> Option<RecombinationGeneTypes<'a>> {
        use RecombinationGeneTypes::*;
        use Parent::*;

        let gene_a = self.parent_a.peek();
        let gene_b = self.parent_b.peek();

        if gene_a.is_none() {
            if let Some(b) = self.parent_b.next() {
                return Some(
                    Excess(B, b)
                );
            } else {
                return None;
            }
        } else if gene_b.is_none() {
            if let Some(a) = self.parent_a.next() {
                return Some(
                    Excess(A, a)
                );
            } else {
                return None;
            };
        }

        use std::cmp::Ordering;
        match gene_a
            .unwrap()
            .innovation_number
            .cmp(&gene_b.unwrap().innovation_number)
        {
            Ordering::Equal => {
                // matching gene
                return Some(Matching(
                    self.parent_a.next().unwrap(), self.parent_b.next().unwrap()
                ));
            }
            Ordering::Less => {
                // disjoint gene from parent A
                return Some(Disjoint(A, self.parent_a.next().unwrap()));
            }
            Ordering::Greater => {
                // disjoint gene from parent B
                return Some(Disjoint(B, self.parent_b.next().unwrap()));
            }
        }
    }
}
