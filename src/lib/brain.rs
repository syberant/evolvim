extern crate nalgebra;

use self::allocator::Allocator;
use self::dimension::DimName;
use self::nalgebra::*;

pub type BrainOutput<'a> = &'a [FPN];
pub type BrainInput = [FPN; BRAIN_INPUT_SIZE];

type FPN = f64;
type LayerLength = usize;

/// The amount of neurons in the input layer. The bias is not included.
const BRAIN_INPUT_SIZE: LayerLength = 10;
/// The amount of neurons in the hidden layer. The bias is not included.
const _HIDDEN_LAYER_SIZE: LayerLength = 10;
/// The amount of neurons in the output layer.
const _BRAIN_OUTPUT_SIZE: LayerLength = 10;

pub struct Brain {
    // This dimension should be equal to BRAIN_INPUT_SIZE + 1.
    a_1: RowVectorN<FPN, U11>,
    // These dimensions should be equal to BRAIN_INPUT_SIZE + 1 by HIDDEN_LAYER_SIZE.
    theta_1: MatrixMN<FPN, U11, U10>,
    // This dimension should be equal to HIDDEN_LAYER_SIZE + 1.
    a_2: RowVectorN<FPN, U11>,
    // These dimensions should be equal to HIDDEN_LAYER_SIZE + 1 by OUTPUT_LAYER_SIZE.
    theta_2: MatrixMN<FPN, U11, U10>,
    // This dimension should be equal to OUTPUT_LAYER_SIZE.
    a_3: RowVectorN<FPN, U10>,
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

    fn load_input(&mut self, input: BrainInput) {
        // TODO: fix this ugly code.
        self.a_1 = <MatrixMN<FPN, U1, U10>>::from_row_slice(&input).insert_column(0, 0.0);
    }

    fn get_output(&self) -> BrainOutput {
        return self.a_3.as_slice();
    }

    // TODO: see if I can speed this up a little with clever memory management.
    fn feed_forward(&mut self) {
        let mut z_2 = self.a_1 * self.theta_1;
        // Perform sigmoid function
        Brain::sigmoid(&mut z_2);
        // Add bias.
        self.a_2 = z_2.insert_column(0, 0.0);

        let mut z_3 = self.a_2 * self.theta_2;
        // Perform sigmoid function
        Brain::sigmoid(&mut z_3);
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

    pub fn evolve() -> Self {
        unimplemented!();
    }
}
