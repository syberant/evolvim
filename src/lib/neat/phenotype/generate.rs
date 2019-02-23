use super::super::genome::{Genome, Id, NodeType};
use super::{Connection, NeuralNet, Node};
use std::collections::HashMap;

// TODO: clean this HORRIFIC code up...
impl From<&Genome> for NeuralNet {
    fn from(genome: &Genome) -> Self {
        let node_gen = genome.get_node_genome();
        let mut nodes: Vec<Node> = std::iter::repeat(Node::empty())
            .take(node_gen.len())
            .collect();
        let mut inputs = Vec::new();
        let mut outputs = Vec::new();
        let mut lookup: HashMap<Id, usize> = HashMap::new();

        let mut counter = 0;
        for i in node_gen {
            lookup.insert(i.id, counter);

            match i.node_type {
                NodeType::Sensor => {
                    inputs.push(super::Input::new(
                        counter,
                        super::super::input::InputType::Bias(1.0),
                    ));
                }
                NodeType::Output => {
                    outputs.push(super::Output::new(
                        counter,
                        super::super::output::OutputType::Test,
                    ));
                }
                _ => {}
            }

            counter += 1;
        }

        for con in genome.get_connection_genome().iter().filter(|c| c.enabled) {
            let from = get_usize_from_id(&lookup, con.from);
            let to = get_usize_from_id(&lookup, con.to);

            nodes[from]
                .connections
                .push(Connection::new(to, con.weight));
        }

        NeuralNet {
            nodes,
            inputs,
            outputs,
        }
    }
}

fn get_usize_from_id(m: &HashMap<Id, usize>, id: Id) -> usize {
    *m.get(&id).unwrap()
}
