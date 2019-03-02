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
        // Preallocate the memory so we don't have to reallocate and make the *mut-pointers invalid.
        let mut outputs = Vec::with_capacity(
            node_gen
                .iter()
                .filter(|node| {
                    if let NodeType::Output(_) = node.node_type {
                        true
                    } else {
                        false
                    }
                })
                .count(),
        );
        let mut lookup: HashMap<Id, usize> = HashMap::new();

        let mut counter = 0;
        for i in node_gen {
            lookup.insert(i.id, counter);

            match &i.node_type {
                NodeType::Sensor(in_type) => {
                    inputs.push(super::Input::new(
                        counter,
                        in_type.clone(),
                    ));
                }
                NodeType::Output(out_type) => {
                    outputs.push(super::Output::new(counter, out_type.clone()));
                    let to: *mut f64 = &mut outputs.last_mut().unwrap().value;
                    nodes[counter]
                        .connections
                        .push(unsafe { Connection::new(to, 1.0) });
                }
                _ => {}
            }

            counter += 1;
        }

        for con in genome.get_connection_genome().iter().filter(|c| c.enabled) {
            let from = get_usize_from_id(&lookup, con.from);
            let to = &mut nodes[get_usize_from_id(&lookup, con.to)].value as *mut f64;

            nodes[from]
                .connections
                .push(unsafe { Connection::new(to, con.weight) });
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
