use super::LineLength;
use crate::{Position, Selection, SelectionDeltas, SelectionStorage};

impl Selection {
    pub(super) fn move_left(&self, line_lengths: &impl LineLength, n: usize) -> Selection {
        todo!()
    }

    pub(super) fn move_right(&self, line_lengths: &impl LineLength, n: usize) -> Selection {
        todo!()
    }

    pub(super) fn move_up(&self, line_lengths: &impl LineLength, n: usize) -> Selection {
        todo!()
    }

    pub(super) fn move_down(&self, line_lengths: &impl LineLength, n: usize) -> Selection {
        todo!()
    }
}

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
