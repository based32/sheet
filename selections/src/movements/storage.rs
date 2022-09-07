use super::LineLength;
use crate::{Position, SelectionDeltas, SelectionStorage};

impl SelectionStorage {
    /// Moves a selection identified by `pos` as its `from` position on `n`
    /// columns left. Will be narrowed to length of 1 character if `extend` is
    /// `false`.
    pub fn move_left_single(
        &mut self,
        line_lengths: impl LineLength,
        pos: &Position,
        n: u32,
        extend: bool,
    ) -> SelectionDeltas {
        todo!()
    }
}
