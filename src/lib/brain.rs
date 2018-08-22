//! Contains functionality for the intelligence of creatures.
//!
//! Uses a neural network implemented with a linear algebra crate to make it efficient.

extern crate nalgebra;
extern crate rand;

use self::allocator::Allocator;
use self::dimension::DimName;
use self::nalgebra::*;
use self::rand::Rng;
use super::*;
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

impl Brain {
    pub fn run(&mut self, input: BrainInput) -> BrainOutput {
        // Load the input into the net.
        self.load_input(input);

        // Perform feed-forwarding.
        self.feed_forward();

        // Return the output.
        return self.get_output();
    }

    pub fn load_input(&mut self, input: BrainInput) {
        let memory = self.get_output()[0];
        // TODO: fix this ugly code.
        self.a_1 = <MatrixMN<FPN, U1, U9>>::from_row_slice(&input)
            .insert_column(0, 0.0)
            .insert_column(1, memory);
    }

    pub fn get_output(&self) -> BrainOutput {
        return self.a_3.as_slice();
    }

    // TODO: see if I can speed this up a little with clever memory management.
    pub fn feed_forward(&mut self) {
        let mut z_2 = self.a_1 * self.theta_1;
        // Perform sigmoid function
        Brain::sigmoid(&mut z_2);
        // Add bias.
        self.a_2 = z_2.insert_column(0, 0.0);

        let z_3 = self.a_2 * self.theta_2;
        // // Perform sigmoid function, wasn't done in original Processing code.
        // Brain::sigmoid(&mut z_3);

        // Don't need to add bias here.
        self.a_3 = z_3;
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

    /// Returns a brain with completely random weights.
    pub fn new_random() -> Self {
        Brain {
            // Empty input
            a_1: <RowVectorN<FPN, InputLayerSizePlusBias>>::zeros(),
            // Initialize random weights between [-0.5, 0.5].
            theta_1: <MatrixMN<FPN, InputLayerSizePlusBias, HiddenLayerSize>>::new_random()
                - <MatrixMN<FPN, InputLayerSizePlusBias, HiddenLayerSize>>::from_element(0.5),
            // Empty hidden layer
            a_2: <RowVectorN<FPN, HiddenLayerSizePlusBias>>::zeros(),
            // Initialize random weights between [-0.5, 0.5].
            theta_2: <MatrixMN<FPN, HiddenLayerSizePlusBias, OutputLayerSize>>::new_random()
                - <MatrixMN<FPN, HiddenLayerSizePlusBias, OutputLayerSize>>::from_element(0.5),
            // Empty output
            a_3: <RowVectorN<FPN, OutputLayerSize>>::zeros(),
        }
    }

    /// Equivalent to the Processing function, also includes the mutateAxon function.
    ///
    /// TODO: improve performance via vectorization.
    /// TODO: understand formulae and improve them or come up with my own
    pub fn evolve(parents: &Vec<HLSoftBody>) -> Self {
        let a_1 = <RowVectorN<FPN, InputLayerSizePlusBias>>::zeros();
        let a_2 = <RowVectorN<FPN, HiddenLayerSizePlusBias>>::zeros();
        let a_3 = <RowVectorN<FPN, OutputLayerSize>>::zeros();

        let mut theta_1 = <MatrixMN<FPN, InputLayerSizePlusBias, HiddenLayerSize>>::zeros();
        let mut theta_2 = <MatrixMN<FPN, HiddenLayerSizePlusBias, OutputLayerSize>>::zeros();

        let mut rng = rand::thread_rng();
        let random_rotation: f64 = rng.gen();
        let amount_parents = parents.len() as f64;

        for y in 0..theta_1.nrows() {
            for z in 0..theta_1.ncols() {
                // BRAIN_HEIGHT = 11; x = 0; BRAIN_WIDTH = 3;
                let axon_angle =
                    PI + (((y + z) as f64 - 11.0) / 2.0).atan2(0.0 - 3.0 / 2.0) / (2.0 * PI);

                let parent_id =
                    (((axon_angle + random_rotation) % 1.0) * amount_parents).floor() as usize;

                assert!(parent_id < amount_parents as usize);

                let r = (rng.gen::<f64>() * 2.0 - 1.0).powi(9);
                let mutate_multi = rng.gen::<f64>().powi(9);
                let mutability = rng.gen::<f64>().powi(14);

                theta_1[(y, z)] = parents[parent_id].borrow().get_creature().brain.theta_1[(y, z)]
                    + r * mutability / mutate_multi;
            }
        }

        for y in 0..theta_2.nrows() {
            for z in 0..theta_2.ncols() {
                // BRAIN_HEIGHT = 11; x = 1; BRAIN_WIDTH = 3;
                let axon_angle =
                    PI + (((y + z) as f64 - 11.0) / 2.0).atan2(1.0 - 3.0 / 2.0) / (2.0 * PI);

                let parent_id =
                    (((axon_angle + random_rotation) % 1.0) * amount_parents).floor() as usize;

                assert!(parent_id < amount_parents as usize);

                let r = (rng.gen::<f64>() * 2.0 - 1.0).powi(9);
                let mutate_multi = rng.gen::<f64>().powi(9);
                let mutability = rng.gen::<f64>().powi(14);

                theta_2[(y, z)] = parents[parent_id].borrow().get_creature().brain.theta_2[(y, z)]
                    + r * mutability / mutate_multi;
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

// All functions to retrieve intentions
impl Brain {
    pub fn wants_birth(&self) -> f64 {
        self.get_output()[5]
    }

    pub fn wants_to_eat(&self) -> f64 {
        self.get_output()[3]
    }

    pub fn wants_mouth_hue(&self) -> f64 {
        self.get_output()[6]
    }

    pub fn wants_acceleration(&self) -> f64 {
        self.get_output()[1]
    }

    pub fn wants_turning(&self) -> f64 {
        self.get_output()[2]
    }
}
