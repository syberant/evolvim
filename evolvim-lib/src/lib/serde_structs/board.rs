use crate::board::Board;
use crate::terrain::Terrain;
use crate::softbody::SoftBody;
use crate::climate::Climate;
use super::version::Version;

use serde_derive::{Deserialize, Serialize};
use crate::brain::NeuralNet;

#[derive(Deserialize, Serialize)]
pub struct BoardSerde<B: NeuralNet> {
    // Fields not in the board
    version: Version,

    // Fields relevant for the board itself.
    board_width: usize,
    board_height: usize,
    pub terrain: Terrain,

    // Fields relevant for the creatures.
    creature_minimum: usize,
    // pub soft_bodies_in_positions: SoftBodiesInPositions<B>,
    pub creatures: Vec<SoftBody<B>>,
    creature_id_up_to: usize,
    // _creature_rank_metric: usize,

    // Fields relevant for time or history
    year: f64,

    // Fields relevant for temperature
    pub climate: Climate,

    // Miscelanious
    // pub selected_creature: SelectedCreature<B>,
}

impl<B: NeuralNet> From<Board<B>> for BoardSerde<B> {
    fn from(bd: Board<B>) -> BoardSerde<B> {
        let (board_width, board_height) = bd.get_board_size();
        let creature_minimum = bd.get_creature_minimum();
        let creature_id_up_to = bd.get_creature_id_up_to();
        let year = bd.get_time();

        let creatures: Vec<SoftBody<B>> = bd.creatures.into_iter().map(|c| c.into_inner()).collect();

        BoardSerde {
            version: Version::current_version(),

            board_width,
            board_height,
            terrain: bd.terrain,

            creature_minimum,
            creatures,
            creature_id_up_to,
            
            year,

            climate: bd.climate,
        }
    }
}

impl<B: NeuralNet> From<BoardSerde<B>> for Board<B> {
    fn from(bs: BoardSerde<B>) -> Board<B> {
        use crate::board::SelectedCreature;
        use crate::sbip::SoftBodiesInPositions;
        use crate::softbody::HLSoftBody;

        if !bs.version.is_compatible_with_current() {
            panic!("File from version {} can not be used with current version ({}).",
                    bs.version,
                    Version::current_version()
            );
        }

        let board_size = (bs.board_width, bs.board_height);
        let mut soft_bodies_in_positions = SoftBodiesInPositions::new_allocated(board_size);
        let creatures: Vec<HLSoftBody<B>> = bs.creatures.into_iter()
            .map(|c| HLSoftBody::from(c)).collect();

        for c in &creatures {
            c.set_sbip(&mut soft_bodies_in_positions, board_size);
            c.set_sbip(&mut soft_bodies_in_positions, board_size);
        }

        Board::new(
            bs.board_width,
            bs.board_height,
            bs.terrain,

            bs.creature_minimum,
            soft_bodies_in_positions,
            creatures,
            bs.creature_id_up_to,

            bs.year,

            bs.climate,

            SelectedCreature::default(),
        )
    }
}