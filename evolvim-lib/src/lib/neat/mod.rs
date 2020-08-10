mod genome;
mod input;
mod output;
mod phenotype;

pub use genome::Genome;
pub use phenotype::NeuralNet;

#[derive(Debug)]
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

    fn use_output(&self, env: &mut crate::brain::EnvironmentMut<Self>, time_step: f64) {
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
    fn recombination_infinite_parents(parents: &[&crate::softbody::SoftBody<Self>]) -> Self {
        use crate::brain::RecombinationTwoParents;

        if parents.len() == 1 {
            // Only mutate this genome

            let parent = parents[0];
            // Make a copy of the parent genome
            let mut genome = parent.brain.genome.clone();
            // Mutate it
            genome.mutate();
            // Generate a phenotype and return a NeatBrain
            genome.into()
        } else {
            NeatBrain::recombination_two_parents(
                &parents[0].brain,
                &parents[1].brain,
            )
        }
    }
}

impl crate::brain::ProvideInformation for NeatBrain {
    fn get_keys(&self) -> Vec<String> {
        vec!["nodes".to_string(), "connections".to_string()]
    }

    fn get_raw_values(&self) -> Vec<String> {
        vec![
            format!("{}", self.genome.get_node_genome().len()),
            format!("{}", self.genome.get_connection_genome().len()),
        ]
    }
}

// TODO: serialize the values of the nodes (which allows for memory)
impl serde::Serialize for NeatBrain {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;

        let mut state = serializer.serialize_struct("NeatBrain", 1)?;

        state.serialize_field("genome", &self.genome)?;

        state.end()
    }
}

// TODO: deserialize the values of the nodes (which allows for memory)
impl<'de> serde::Deserialize<'de> for NeatBrain {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<NeatBrain, D::Error> {
        use serde::de::*;

        struct BrainVisitor;

        impl<'de> Visitor<'de> for BrainVisitor {
            type Value = NeatBrain;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct NeatBrain")
            }

            fn visit_seq<V: SeqAccess<'de>>(self, mut seq: V) -> Result<NeatBrain, V::Error> {
                let genome: Genome = seq
                    .next_element()?
                    .ok_or_else(|| Error::invalid_length(0, &self))?;

                Ok(genome.into())
            }
        }

        const FIELDS: &[&str] = &["genome"];
        deserializer.deserialize_struct::<BrainVisitor>("NeatBrain", FIELDS, BrainVisitor)
    }
}
