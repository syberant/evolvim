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