use super::*;

pub type SoftBodiesAt = std::collections::HashSet<*const SoftBody>;

/// Contains a list of every `SoftBody` in a given coordinate.
pub struct SoftBodiesInPositions(Vec<Vec<SoftBodiesAt>>);

impl SoftBodiesInPositions {
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
