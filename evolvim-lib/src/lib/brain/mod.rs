pub mod feed_forward;
pub use feed_forward::Brain;

mod environment;
pub use environment::{Environment, EnvironmentMut};

pub trait NeuralNet: Intentions {
    fn load_input(&mut self, env: &Environment);

    fn run(&mut self);

    fn run_with(&mut self, env: &Environment) {
        self.load_input(env);
        self.run();
    }

    fn use_output(&self, env: &mut EnvironmentMut, time_step: f64);
}

pub trait Intentions {
    fn wants_birth(&self) -> f64;
    fn wants_help_birth(&self) -> f64;
}

pub trait GenerateRandom {
    fn new_random() -> Self;
}

pub trait RecombinationTwoParents {
    fn recombination_two_parents(parent_a: &Self, parent_b: &Self) -> Self
    where
        Self: NeuralNet + std::marker::Sized;
}

pub trait RecombinationInfinite {
    fn recombination_infinite_parents(parents: &Vec<crate::softbody::HLSoftBody<Self>>) -> Self
    where
        Self: NeuralNet + std::marker::Sized;
}
