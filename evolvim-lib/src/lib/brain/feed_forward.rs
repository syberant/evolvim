//! Contains functionality for the intelligence of creatures.
//!
//! Uses a neural network implemented with a linear algebra crate to make it efficient.

#![warn(missing_docs)]

use self::allocator::Allocator;
use self::dimension::DimName;
use nalgebra::*;
use rand::Rng;
use std::f64::consts::PI;

pub type BrainOutput<'a> = &'a [FPN];
pub type BrainInput = [FPN; 9];

type FPN = f64;

/// The amount of neurons in the input layer.
type _InputLayerSize = U10;
/// The amount of neurons in the input layer plus the bias node.
type InputLayerSizePlusBias = U11;
/// The amount of neurons in the hidden layer.
type HiddenLayerSize = U10;
/// The amount of neurons in the hidden layer plus the bias node.
type HiddenLayerSizePlusBias = U11;
/// The amount of neurons in the output layer.
type OutputLayerSize = U10;

// const AXON_ANGLES_0: Vec<f64> = get_axon_angles(110, 0);
// const AXON_ANGLES_1: Vec<f64> = get_axon_angles(110, 1);

/// This struct contains all parameters and values necessary for a feed-forward neural network.
///
/// # Usage
/// Give it some input with `load_input()` and perform the calculations with `feed_forward()`, then extract the output with `get_output`.
/// Or just do all three at once with `run`!
///
/// # Processing equivalent
/// *Brain.pde/Brain*, although this doesn't have an `Axon` class/structure to rely on.
#[derive(Clone, Serialize, Deserialize)]
pub struct Brain {
    // This dimension should be equal to InputLayerSize + 1.
    a_1: RowVectorN<FPN, InputLayerSizePlusBias>,
    // These dimensions should be equal to InputLayerSize + 1 by HiddenLayerSize.
    theta_1: MatrixMN<FPN, InputLayerSizePlusBias, HiddenLayerSize>,
    // This dimension should be equal to HiddenLayerSize + 1.
    a_2: RowVectorN<FPN, HiddenLayerSizePlusBias>,
    // These dimensions should be equal to HiddenLayerSize + 1 by OutputLayerSize.
    theta_2: MatrixMN<FPN, HiddenLayerSizePlusBias, OutputLayerSize>,
    // This dimension should be equal to OUTPUT_LAYER_SIZE.
    a_3: RowVectorN<FPN, OutputLayerSize>,
}

impl super::NeuralNet for Brain {
    fn load_input(&mut self, env: &super::Environment) {
        // Load the memory
        self.a_1[0] = self.get_memory();

        // The current energy of the creature
        self.a_1[1] = env.this_body.get_energy();

        // The current mouth hue
        self.a_1[2] = env.this_body.get_mouth_hue();

        // Look directly underneath the creature
        let pos = env.this_body.get_position();
        let tile = env.terrain.get_tile_at(pos.into());
        let colors = tile.get_hsba_color();
        self.a_1[3] = colors[0] as FPN;
        self.a_1[4] = colors[1] as FPN;
        self.a_1[5] = colors[2] as FPN;
    }

    /// Performs feed foward propagation on the neural network.
    // TODO: see if I can speed this up a little with clever memory management.
    fn run(&mut self) {
        let mut z_2 = self.a_1 * self.theta_1;
        // Perform sigmoid function
        Brain::sigmoid(&mut z_2);
        // Add bias.
        self.a_2 = z_2.insert_column(0, 1.0);

        let z_3 = self.a_2 * self.theta_2;
        // // Perform sigmoid function, wasn't done in original Processing code.
        // Brain::sigmoid(&mut z_3);

        // Don't need to add bias here.
        self.a_3 = z_3;
    }

    fn use_output(&self, env: &mut super::EnvironmentMut<Self>, time_step: f64) {
        let acceleration = self.wants_acceleration();
        env.this_body.accelerate(acceleration, time_step);

        let turning = self.wants_turning();
        env.this_body.turn(turning, time_step);

        // TODO: clean this mess.
        let tile_pos = env.this_body.get_random_covered_tile(env.board_size);
        let tile = env.terrain.get_tile_at_mut(tile_pos);
        let eat_amount = self.wants_to_eat();
        env.this_body
            .eat(eat_amount, time_step, env.time, env.climate, tile);

        let mouth_hue = self.wants_mouth_hue();
        env.this_body.set_mouth_hue(mouth_hue);
    }
}

impl super::GenerateRandom for Brain {
    /// Returns a brain with completely random weights.
    fn new_random() -> Self {
        let theta_1 = <MatrixMN<FPN, InputLayerSizePlusBias, HiddenLayerSize>>::new_random()
            - <MatrixMN<FPN, InputLayerSizePlusBias, HiddenLayerSize>>::from_element(0.5);
        let theta_2 = <MatrixMN<FPN, HiddenLayerSizePlusBias, OutputLayerSize>>::new_random()
            - <MatrixMN<FPN, HiddenLayerSizePlusBias, OutputLayerSize>>::from_element(0.5);

        Brain {
            // Empty input
            a_1: <RowVectorN<FPN, InputLayerSizePlusBias>>::zeros(),
            // Initialize random weights between [-0.5, 0.5].
            theta_1,
            // Empty hidden layer
            a_2: <RowVectorN<FPN, HiddenLayerSizePlusBias>>::zeros(),
            // Initialize random weights between [-0.5, 0.5].
            theta_2,
            // Empty output
            a_3: <RowVectorN<FPN, OutputLayerSize>>::zeros(),
        }
    }
}

impl Brain {
    /// # Processing equivalent
    /// *Brain.pde/outputs*, although here only a reference to the output values is returned instead of a copy.
    pub fn get_output(&self) -> BrainOutput {
        return self.a_3.as_slice();
    }

    /// Returns a reference to the hidden layer values.
    pub fn get_hidden_layer(&self) -> &[FPN] {
        self.a_2.as_slice()
    }

    /// Returns a reference to the input layer values.
    pub fn get_input_layer(&self) -> &[FPN] {
        self.a_1.as_slice()
    }

    /// Performs the sigmoid function for every element in the matrix.
    fn sigmoid<R: DimName, C: DimName>(matrix: &mut MatrixMN<FPN, R, C>)
    where
        DefaultAllocator: Allocator<FPN, R, C>,
    {
        for v in matrix.iter_mut() {
            *v = 1.0 / (1.0 + (-*v).exp());
        }
    }
}

impl super::RecombinationInfinite for Brain {
    /// # Processing equivalent
    /// *Brain.pde/evolve*, although the structure of the brain is different and there are no calls to `Axon`s here.
    /// Everything is done in this function.
    ///
    /// TODO: improve performance via vectorization.
    /// TODO: understand formulae and improve them or come up with my own
    fn recombination_infinite_parents(parents: &[&crate::softbody::SoftBody<Brain>]) -> Self {
        let a_1 = <RowVectorN<FPN, InputLayerSizePlusBias>>::zeros();
        let a_2 = <RowVectorN<FPN, HiddenLayerSizePlusBias>>::zeros();
        let a_3 = <RowVectorN<FPN, OutputLayerSize>>::zeros();

        let mut theta_1 = <MatrixMN<FPN, InputLayerSizePlusBias, HiddenLayerSize>>::zeros();
        let mut theta_2 = <MatrixMN<FPN, HiddenLayerSizePlusBias, OutputLayerSize>>::zeros();

        let mut rng = rand::thread_rng();
        let random_rotation: f64 = rng.gen();
        let amount_parents = parents.len() as f64;

        const MUTABILITY: f64 = 0.0005;
        // const MUTATE_MULTI: f64 = 0.5.powi(9);
        const MUTATE_MULTI: f64 = 0.001953125;

        let axon_angles = get_axon_angles(110, 0);
        for y in 0..theta_1.nrows() {
            for z in 0..theta_1.ncols() {
                // BRAIN_HEIGHT = 11; x = 0; BRAIN_WIDTH = 3;
                let axon_angle = axon_angles[y + z];

                let parent_id =
                    (((axon_angle + random_rotation) % 1.0) * amount_parents).floor() as usize;

                let r = (rng.gen::<f64>() * 2.0 - 1.0).powi(9);

                theta_1[(y, z)] =
                    parents[parent_id].brain.theta_1[(y, z)] + r * MUTABILITY / MUTATE_MULTI;
            }
        }

        let axon_angles = get_axon_angles(110, 1);
        for y in 0..theta_2.nrows() {
            for z in 0..theta_2.ncols() {
                // BRAIN_HEIGHT = 11; x = 1; BRAIN_WIDTH = 3;
                let axon_angle = axon_angles[y + z];

                let parent_id =
                    (((axon_angle + random_rotation) % 1.0) * amount_parents).floor() as usize;

                let r = (rng.gen::<f64>() * 2.0 - 1.0).powi(9);

                theta_2[(y, z)] =
                    parents[parent_id].brain.theta_2[(y, z)] + r * MUTABILITY / MUTATE_MULTI;
            }
        }

        Brain {
            a_1,
            theta_1,
            a_2,
            theta_2,
            a_3,
        }
    }
}

impl Brain {
    /// # Processing equivalent
    /// Returns *Brain.pde/outputLabels*.
    pub fn intentions(&self) -> Vec<String> {
        let info = vec![
            "Memory",
            "Acceleration",
            "Turning",
            "Eating",
            "Birth",
            "Mouth hue",
            "Help birth",
        ];

        // Turn it into `String`s
        info.into_iter().map(|val| String::from(val)).collect()
    }
}

impl super::Intentions for Brain {
    fn wants_birth(&self) -> f64 {
        self.get_output()[4]
    }

    fn wants_help_birth(&self) -> f64 {
        self.get_output()[6]
    }
}

#[allow(missing_docs)]
// All functions to retrieve intentions
impl Brain {
    pub fn get_memory(&self) -> f64 {
        self.get_output()[0]
    }

    pub fn wants_acceleration(&self) -> f64 {
        self.get_output()[1]
    }

    pub fn wants_turning(&self) -> f64 {
        self.get_output()[2]
    }

    pub fn wants_to_eat(&self) -> f64 {
        self.get_output()[3]
    }

    pub fn wants_mouth_hue(&self) -> f64 {
        self.get_output()[5]
    }
}

fn get_axon_angles(max: usize, x: usize) -> Vec<f64> {
    let mut vec = Vec::with_capacity(max);
    const BRAIN_WIDTH: f64 = 3.0;
    const BRAIN_HEIGHT: f64 = 11.0;

    for i in 0..max {
        vec.push(
            PI + ((i as f64 - BRAIN_HEIGHT) / 2.0).atan2(x as f64 - BRAIN_WIDTH / 2.0) / (2.0 * PI),
        );
    }

    vec
}
