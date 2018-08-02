extern crate nalgebra;

use self::allocator::Allocator;
use self::dimension::DimName;
use self::nalgebra::*;

pub type BrainOutput<'a> = &'a [FPN];
pub type BrainInput = [FPN; 10];

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
        // TODO: fix this ugly code.
        self.a_1 = <MatrixMN<FPN, U1, U10>>::from_row_slice(&input).insert_column(0, 0.0);
    }

    fn get_output(&self) -> BrainOutput {
        return self.a_3.as_slice();
    }

    // TODO: see if I can speed this up a little with clever memory management.
    pub fn feed_forward(&mut self) {
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

    /// Returns a brain with completely random weights.
    pub fn new_random() -> Self {
        Brain {
            // Empty input
            a_1: <MatrixMN<FPN, U1, InputLayerSizePlusBias>>::zeros(),
            // Initialize random weights between [-0.5, 0.5].
            theta_1: <MatrixMN<FPN, InputLayerSizePlusBias, HiddenLayerSize>>::new_random()
                - <MatrixMN<FPN, InputLayerSizePlusBias, HiddenLayerSize>>::from_element(0.5),
            // Empty hidden layer
            a_2: <MatrixMN<FPN, U1, HiddenLayerSizePlusBias>>::zeros(),
            // Initilaize random weights between [-0.5, 0.5].
            theta_2: <MatrixMN<FPN, HiddenLayerSizePlusBias, OutputLayerSize>>::new_random()
                - <MatrixMN<FPN, HiddenLayerSizePlusBias, OutputLayerSize>>::from_element(0.5),
            // Empty output
            a_3: <MatrixMN<FPN, U1, OutputLayerSize>>::zeros(),
        }
    }

    pub fn evolve() -> Self {
        unimplemented!();
    }
}
