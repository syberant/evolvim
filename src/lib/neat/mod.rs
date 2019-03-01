mod genome;
mod input;
mod output;
mod phenotype;

use genome::Genome;
use phenotype::NeuralNet;

pub struct NeatBrain {
    genome: Genome,
    net: NeuralNet,
}

impl From<Genome> for NeatBrain {
    fn from(genome: Genome) -> Self {
        let net = (&genome).into();

        NeatBrain { genome, net }
    }
}

impl crate::brain::NeuralNet for NeatBrain {
    fn load_input(&mut self, env: &crate::brain::Environment) {
        self.net.load_input(env);
    }

    fn run(&mut self) {
        self.net.run_calculations();
    }

    fn use_output(&self, env: &mut crate::brain::EnvironmentMut, time_step: f64) {
        self.net.use_output(env, time_step);
    }
}

impl crate::brain::Intentions for NeatBrain {
    fn wants_birth(&self) -> f64 {
        unimplemented!()
    }

    fn wants_help_birth(&self) -> f64 {
        unimplemented!()
    }
}

impl crate::brain::GenerateRandom for NeatBrain {
    fn new_random() -> Self {
        Genome::new_fully_linked().into()
    }
}

impl crate::brain::RecombinationTwoParents for NeatBrain {
    fn recombination_two_parents(parent_a: &Self, parent_b: &Self) -> Self {
        let genome = Genome::new_from_2(&parent_a.genome, &parent_b.genome);
        genome.into()
    }
}

impl crate::brain::RecombinationInfinite for NeatBrain {
    fn recombination_infinite_parents(parents: &Vec<crate::softbody::HLSoftBody<Self>>) -> Self {
        use crate::brain::RecombinationTwoParents;

        if parents.len() == 1 {
            // Only mutate this genome

            let parent = parents[0].borrow();
            // Make a copy of the parent genome
            let mut genome = parent.brain.genome.clone();
            // Mutate it
            genome.mutate();
            // Generate a phenotype and return a NeatBrain
            genome.into()
        } else {
            NeatBrain::recombination_two_parents(
                &parents[0].borrow().brain,
                &parents[1].borrow().brain,
            )
        }
    }
}
