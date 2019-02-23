mod generate;

use super::input::{Environment, InputType};
use super::output::OutputType;

// TODO: use unsafe pointers to speed things up
pub struct NeuralNet {
    nodes: Vec<Node>,

    outputs: Vec<Output>,
    inputs: Vec<Input>,
}

impl NeuralNet {
    pub fn load_input(&mut self, env: &Environment) {
        for input in &self.inputs {
            input.load_into(&mut self.nodes, env);
        }
    }

    pub fn get_output(&self) -> &[Output] {
        unimplemented!()
    }

    pub fn run_calculations(&mut self) {
        for i in 0..self.nodes.len() {
            self.calc_neuron(i);
        }
    }

    fn calc_neuron(&mut self, id: usize) {
        let value = self.nodes[id].perform_sigmoid();

        // Reset counter
        self.nodes[id].value = 0.0;

        // unsafe here is necessary
        // it is safe because we use .connections with the immutable reference and .value with the mutable one
        for i in unsafe { &(*(&self.nodes[id] as *const Node)).connections } {
            self.nodes[i.to_index].value += value * i.weight;
        }
    }
}

pub struct Output {
    node_index: usize,
    output_type: OutputType,
}

impl Output {
    pub fn new(node_index: usize, output_type: OutputType) -> Self {
        Output {
            node_index,
            output_type,
        }
    }
}

struct Input {
    node_index: usize,
    input_type: InputType,
}

impl Input {
    pub fn load_into(&self, nodes: &mut [Node], env: &Environment) {
        let data = self.input_type.get_data(env);
        nodes[self.node_index].add_to_value(data);
    }

    pub fn new(node_index: usize, input_type: InputType) -> Self {
        Input {
            node_index,
            input_type,
        }
    }
}

#[derive(Clone)]
struct Node {
    pub value: f64,
    pub connections: Vec<Connection>,
}

impl Node {
    pub fn add_to_value(&mut self, n: f64) {
        self.value += n;
    }

    pub fn perform_sigmoid(&mut self) -> f64 {
        return sigmoid(self.value);
    }

    pub fn empty() -> Self {
        Node {
            value: 0.0,
            connections: Vec::new(),
        }
    }
}

#[derive(Clone)]
struct Connection {
    to_index: usize,
    weight: f64,
}

impl Connection {
    pub fn new(to_index: usize, weight: f64) -> Self {
        Connection { to_index, weight }
    }
}

fn sigmoid(n: f64) -> f64 {
    1.0 / (1.0 + n.exp())
}
