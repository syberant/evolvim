mod generate;

use super::input::InputType;
use super::output::OutputType;

// TODO: use unsafe pointers or something to speed things up
#[derive(Debug)]
pub struct NeuralNet {
    nodes: Box<[Node]>,

    outputs: Box<[Output]>,
    inputs: Vec<Input>,
}

impl NeuralNet {
    pub fn load_input(&mut self, env: &crate::brain::Environment) {
        for input in &self.inputs {
            input.load_into(&mut self.nodes, env);
        }
    }

    pub fn use_output(
        &self,
        env: &mut crate::brain::EnvironmentMut<super::NeatBrain>,
        time_step: f64,
    ) {
        for output in self.outputs.iter() {
            output.use_output(env, time_step);
        }
    }

    pub fn run_calculations(&mut self) {
        for n in self.outputs.iter_mut() {
            // Reset the value to 0
            n.value = 0.0;
        }

        for n in self.nodes.iter_mut() {
            n.calc();
        }
    }
}

#[derive(Debug)]
struct Output {
    node_index: usize,
    value: f64,
    output_type: OutputType,
}

impl Output {
    fn use_output(&self, env: &mut crate::brain::EnvironmentMut<super::NeatBrain>, time_step: f64) {
        self.output_type.use_output(self.value, env, time_step);
    }

    pub fn new(node_index: usize, output_type: OutputType) -> Self {
        Output {
            node_index,
            value: 0.0,
            output_type,
        }
    }
}

#[derive(Debug)]
struct Input {
    node_index: usize,
    input_type: InputType,
}

impl Input {
    pub fn load_into(&self, nodes: &mut [Node], env: &crate::brain::Environment) {
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

#[derive(Clone, Debug)]
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

    pub fn calc(&mut self) {
        let sig_value = self.perform_sigmoid();

        self.value = 0.0;

        for c in &self.connections {
            unsafe {
                *c.to += c.weight * sig_value;
            }
        }
    }

    pub fn empty() -> Self {
        Node {
            value: 0.0,
            connections: Vec::new(),
        }
    }
}

#[derive(Clone, Debug)]
struct Connection {
    to: *mut f64,
    weight: f64,
}

impl Connection {
    /// Weighted connection to another point in memory
    ///
    /// This is unsafe, to use this you must manually guarantee that the pointer stays valid
    /// at least until we destroy this Neural Network struct.
    pub unsafe fn new(to: *mut f64, weight: f64) -> Self {
        Connection { to, weight }
    }
}

fn sigmoid(n: f64) -> f64 {
    1.0 / (1.0 + n.exp())
}
