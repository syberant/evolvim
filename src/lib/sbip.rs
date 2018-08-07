//! Used for collision detection.
//!
//! `SoftBodiesInPositions` contains references to all `SoftBody`'s.
//! It is of critical importance that these are **always** valid!
//! Seriously, otherwise the application will just crash.
//!
//! Please don't mess with this module if you don't understand it: it will save you a lot of frustration!

use super::*;

pub type SoftBodiesAt = std::collections::HashSet<*const SoftBody>;

/// Contains a list of every `SoftBody` in a given coordinate.
pub struct SoftBodiesInPositions(Vec<Vec<SoftBodiesAt>>);

impl SoftBodiesInPositions {
    pub fn new_allocated(board_size: BoardSize) -> Self {
        let (board_width, board_height) = board_size;

        let allocated_cell = SoftBodiesAt::with_capacity(2);
        let allocated_column = std::iter::repeat(allocated_cell)
            .take(board_height)
            .collect::<Vec<SoftBodiesAt>>();
        let allocated_rows = std::iter::repeat(allocated_column)
            .take(board_width)
            .collect();

        return SoftBodiesInPositions(allocated_rows);
    }

    pub fn get_soft_bodies_at(&self, x: usize, y: usize) -> &SoftBodiesAt {
        return &self.0[x][y];
    }

    pub fn add_soft_body_at(&mut self, x: usize, y: usize, body: &SoftBody) {
        self.0[x][y].insert(body as *const SoftBody);
    }

    pub fn remove_soft_body_at(&mut self, x: usize, y: usize, body: &SoftBody) {
        self.0[x][y].remove(&(body as *const SoftBody));
    }
}
