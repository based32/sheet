use super::LineLength;
use crate::{Position, Selection, SelectionDeltas, SelectionDirection, SelectionStorage};

pub(super) struct FromToPair {
    from: Position,
    to: Position,
}

impl Selection {
    pub(super) fn move_left(
        &self,
        line_lengths: &impl LineLength,
        n: usize,
        extend: bool,
    ) -> FromToPair {
        match self.direction {
            SelectionDirection::Backward => {
                let from = self.from.move_left(line_lengths, n);
                let to = if !extend {
                    self.from.clone()
                } else {
                    self.to.clone()
                };
                FromToPair { from, to }
            }
            SelectionDirection::Forward => {
                let to = self.to.move_left(line_lengths, n);
                let from = if !extend {
                    self.to.clone()
                } else {
                    self.from.clone()
                };
                FromToPair { from, to }
            }
        }
    }

    pub(super) fn move_right(
        &self,
        line_lengths: &impl LineLength,
        n: usize,
        extend: bool,
    ) -> FromToPair {
        match self.direction {
            SelectionDirection::Backward => {
                let from = self.from.move_right(line_lengths, n);
                let to = if !extend {
                    self.from.clone()
                } else {
                    self.to.clone()
                };
                FromToPair { from, to }
            }
            SelectionDirection::Forward => {
                let to = self.to.move_right(line_lengths, n);
                let from = if !extend {
                    self.to.clone()
                } else {
                    self.from.clone()
                };
                FromToPair { from, to }
            }
        }
    }

    pub(super) fn move_up(
        &self,
        line_lengths: &impl LineLength,
        n: usize,
        extend: bool,
    ) -> FromToPair {
        match self.direction {
            SelectionDirection::Backward => {
                let from = self.from.move_up(line_lengths, n);
                let to = if !extend {
                    self.from.clone()
                } else {
                    self.to.clone()
                };
                FromToPair { from, to }
            }
            SelectionDirection::Forward => {
                let to = self.to.move_up(line_lengths, n);
                let from = if !extend {
                    self.to.clone()
                } else {
                    self.from.clone()
                };
                FromToPair { from, to }
            }
        }
    }

    pub(super) fn move_down(
        &self,
        line_lengths: &impl LineLength,
        n: usize,
        extend: bool,
    ) -> FromToPair {
        match self.direction {
            SelectionDirection::Backward => {
                let from = self.from.move_down(line_lengths, n);
                let to = if !extend {
                    self.from.clone()
                } else {
                    self.to.clone()
                };
                FromToPair { from, to }
            }
            SelectionDirection::Forward => {
                let to = self.to.move_down(line_lengths, n);
                let from = if !extend {
                    self.to.clone()
                } else {
                    self.from.clone()
                };
                FromToPair { from, to }
            }
        }
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
