#[cfg(test)]
mod tests;

use crate::{Position, SelectionDeltas, SelectionStorage};

/// Source of line lengthes for a buffer.
pub trait LineLength {
    /// Returns a length for a line specified by its index.
    fn get_len(&self, line: usize) -> usize;
}

impl Position {
    fn move_left(&self, line_lengths: impl LineLength, mut n: usize) -> Position {
        let mut new_pos = self.clone();
        while n > 0 {
            if new_pos.column < n {
                // If there is no space to move left then the position is a beginning of a buffer.
                if new_pos.line == 0 {
                    new_pos.column = 0;
                    break;
                }

                n -= new_pos.column;
                new_pos.line -= 1;
                new_pos.column = line_lengths.get_len(new_pos.line);
            }
        }
        new_pos
    }

    fn move_right(&self, line_lengths: impl LineLength, n: usize) -> Position {
        todo!()
    }

    fn move_up(&self, line_lengths: impl LineLength, n: usize) -> Position {
        todo!()
    }

    fn move_down(&self, line_lengths: impl LineLength, n: usize) -> Position {
        todo!()
    }
}

impl SelectionStorage {
    /// Moves a selection identified by `pos` as its `from` position on `n`
    /// columns. Will be narrowed to length of 1 character if `extend` is
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
