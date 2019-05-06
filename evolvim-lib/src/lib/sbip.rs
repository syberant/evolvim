//! Used for collision detection.
//!
//! `SoftBodiesInPositions` contains references to all `SoftBody`'s.
//! It is of critical importance that these are **always** valid!
//! Seriously, otherwise the application will just crash.
//!
//! Please don't mess with this module if you don't understand it: it will save you a lot of frustration!

use super::*;
use std::ops::Range;

pub trait SoftBodyBucket<B> {
    fn remove_softbody(&mut self, body: HLSoftBody<B>);

    fn add_softbody(&mut self, body: HLSoftBody<B>);
}

pub type SoftBodiesAt<B> = Vec<HLSoftBody<B>>;

impl<B> SoftBodyBucket<B> for SoftBodiesAt<B> {
    fn remove_softbody(&mut self, body: HLSoftBody<B>) {
        // WARNING: Only removes one instance
        for i in 0..self.len() {
            if self[i] == body {
                self.remove(i);
                break;
            }
        }
    }

    /// Adds the given `HLSoftBody`, prevents duplicates.
    fn add_softbody(&mut self, body: HLSoftBody<B>) {
        for i in 0..self.len() {
            if self[i] == body {
                return;
            }
        }

        self.push(body);
    }
}

/// Contains a list of every `SoftBody` in a given coordinate.
pub struct SoftBodiesInPositions<B>(Vec<Vec<SoftBodiesAt<B>>>);

impl<B> SoftBodiesInPositions<B> {
    pub fn new_allocated(board_size: BoardSize) -> Self {
        let (board_width, board_height) = board_size;

        let allocated_cell = SoftBodiesAt::with_capacity(2);
        let allocated_column = std::iter::repeat(allocated_cell)
            .take(board_height)
            .collect::<Vec<SoftBodiesAt<B>>>();
        let allocated_rows = std::iter::repeat(allocated_column)
            .take(board_width)
            .collect();

        return SoftBodiesInPositions(allocated_rows);
    }

    /// Wipes all references but leaves allocations in place
    pub fn wipe(&mut self) {
        for i in &mut self.0 {
            for j in i {
                j.clear();
            }
        }
    }

    pub fn get_soft_bodies_at(&self, x: usize, y: usize) -> &SoftBodiesAt<B> {
        return &self.0[x][y];
    }

    pub fn add_soft_body_at(&mut self, x: usize, y: usize, body: HLSoftBody<B>) {
        self.0[x][y].push(body);
    }

    /// NOTE: only removes one instance of `body`.
    pub fn remove_soft_body_at(&mut self, x: usize, y: usize, body: HLSoftBody<B>) {
        self.0[x][y].remove_softbody(body);
    }

    pub fn get_soft_bodies_in(
        &self,
        x_range: Range<usize>,
        y_range: Range<usize>,
    ) -> SoftBodiesAt<B> {
        let mut soft_body_bucket = Vec::new();

        for x in x_range {
            for y in y_range.clone() {
                for i in self.get_soft_bodies_at(x, y) {
                    soft_body_bucket.add_softbody(i.clone());
                }
            }
        }

        return soft_body_bucket;
    }
}
