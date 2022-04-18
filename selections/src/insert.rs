use super::{Position, SelectionStorage};

impl SelectionStorage {
    /// Insert a selection bounded by `from` and `to` positions. If inserted
    /// selection overlaps with an existing one it either will be replaced
    /// (`replace == true`) or merged (`replace == false`).
    pub fn insert(&mut self, from: Position, to: Position, replace: bool) {}
}
