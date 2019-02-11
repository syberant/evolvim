use super::utils::{RecombinationGeneTypes, RecombinationGenomesIterator};
use super::Genome;

// URGENT TODO: change these
const COEFFICIENT_MATCHING: f64 = 1.0;
const COEFFICIENT_DISJOINT: f64 = 1.0;
const COEFFICIENT_EXCESS: f64 = 1.0;

impl Genome {
    pub fn genetical_distance(&self, other: &Genome) -> f64 {
        use RecombinationGeneTypes::*;

        let iter = RecombinationGenomesIterator::new(&self, other);

        let mut weight_differences = 0.0;
        let mut counter_matching = 0;
        let mut counter_disjoint = 0;
        let mut counter_excess = 0;
        for g in iter {
            match g {
                Matching(a, b) => {
                    counter_matching += 1;
                    weight_differences += (a.weight - b.weight).abs();
                }
                Disjoint(_, _) => counter_disjoint += 1,
                Excess(_, _) => counter_excess += 1,
            }
        }

        let length = (counter_matching + counter_disjoint + counter_excess) as f64;

        return COEFFICIENT_MATCHING * weight_differences / counter_matching as f64
            + COEFFICIENT_DISJOINT * counter_disjoint as f64 / length
            + COEFFICIENT_EXCESS * counter_excess as f64 / length;
    }
}
