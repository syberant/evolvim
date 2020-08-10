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

    fn use_output(&self, env: &mut EnvironmentMut<Self>, time_step: f64)
    where
        Self: std::marker::Sized;
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
    fn recombination_infinite_parents(parents: &[&crate::softbody::SoftBody<Self>]) -> Self
    where
        Self: NeuralNet + std::marker::Sized;
}

pub trait ProvideInformation {
    fn get_raw_values(&self) -> Vec<String> {
        vec![String::from(
            "This struct has not yet implemented it's own information system.",
        )]
    }

    fn get_keys(&self) -> Vec<String> {
        vec![String::from("warning")]
    }

    fn get_ordered_key_value_pairs(&self) -> Vec<(String, String)> {
        let values = self.get_raw_values();
        let keys = self.get_keys();
        assert!(values.len() == keys.len(), "The amount of values ({}) and keys ({}) in the implementation of ProvideInformation does not match.", values.len(), keys.len());

        // Zip the two iterators
        keys.into_iter().zip(values.into_iter()).collect()
    }
}
