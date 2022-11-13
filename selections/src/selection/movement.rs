//! Movement implementations for a single selection.

use crate::{LineLength, Position, Selection};

impl Selection {
    fn move_generic(&self, extend: bool, move_fn: impl Fn(&Position) -> Position) -> Selection {
        let cursor = move_fn(self.cursor());
        if extend {
            Selection::new(self.anchor().clone(), cursor)
        } else {
            Selection::new(cursor.clone().remove_sticky(), cursor)
        }
    }

    pub(crate) fn move_left(
        &self,
        line_lengths: &impl LineLength,
        n: usize,
        extend: bool,
    ) -> Selection {
        self.move_generic(extend, |p| p.move_left(line_lengths, n))
    }

    pub(crate) fn move_right(
        &self,
        line_lengths: &impl LineLength,
        n: usize,
        extend: bool,
    ) -> Selection {
        self.move_generic(extend, |p| p.move_right(line_lengths, n))
    }

    pub(crate) fn move_up(
        &self,
        line_lengths: &impl LineLength,
        n: usize,
        extend: bool,
    ) -> Selection {
        self.move_generic(extend, |p| p.move_up(line_lengths, n))
    }

    pub(crate) fn move_down(
        &self,
        line_lengths: &impl LineLength,
        n: usize,
        extend: bool,
    ) -> Selection {
        self.move_generic(extend, |p| p.move_down(line_lengths, n))
    }
}
