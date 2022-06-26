#[cfg(test)]
mod tests;

use crate::{Position, SelectionDeltas, SelectionStorage};

impl SelectionStorage {
    /// Moves a selection identified by `pos` as its `from` position on `n`
    /// columns. Will be narrowed to length of 1 character if `extend` is
    /// `false`.
    pub fn move_left_single(&mut self, pos: &Position, n: usize, extend: bool) -> SelectionDeltas {
        todo!()
    }
}
