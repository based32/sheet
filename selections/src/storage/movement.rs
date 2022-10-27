use super::SelectionStorage;
use crate::{LineLength, Position, SelectionDeltas};

impl SelectionStorage {
    /// Moves a selection identified by `id` (its `from` component) on `n`
    /// columns left. Will be narrowed to length of 1 character if `extend`
    /// is `false`.
    pub fn move_left_single(
        &mut self,
        line_lengths: impl LineLength,
        id: &Position,
        n: usize,
        extend: bool,
    ) -> SelectionDeltas {
        if n == 0 {
            return Default::default();
        }

        if let Some(idx_old) = self.find_index_by_id(id) {
            let selection_old = &self.selections[idx_old];
            let selection_new = selection_old.move_left(&line_lengths, n, extend);
            let idx_new = self.find_overlapping_indicies(&selection_new.from, &selection_new.to);

            match idx_new {
                Err(idx) if idx == idx_old => {
                    // Updated selection stays at the same index and no collisions to solve:
                    self.selections[idx_old] = selection_new;
                }
                Err(idx) => {
                    // Still no collisions, but index is changed and in case of
                    // movement to the left it will be less than before, than means
                    // a subset of all selections between old and new indexes
                    // should be moved to the right:
		    self.selections[idx..=idx_old].rotate_right(1);
		    self.selections[idx] = selection_new;
                }
            }

	    debug_assert!(self.is_state_correct());
	    todo!()
        } else {
            return Default::default();
        }
    }
}
