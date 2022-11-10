use std::mem;

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

        let Some(idx_old) = self.find_index_by_id(id) else {
	    return Default::default();
	};

        let mut selection_new = self.selections[idx_old].move_left(&line_lengths, n, extend);

        let idx_new =
            self.find_overlapping_indicies_exlude(&selection_new.from, &selection_new.to, idx_old);

        let deltas = match idx_new {
            Err(idx) if idx == idx_old => {
                // Updated selection stays at the same index and no collisions to solve:
                let selection_old = mem::replace(&mut self.selections[idx_old], selection_new);
                let mut deltas = SelectionDeltas::default();
                deltas.push_updated(selection_old, &self.selections[idx]);
                deltas
            }
            Err(idx) => {
                // Still no collisions, but index is changed and in case of
                // movement to the left it will be less than before, than means
                // a subset of all selections between old and new indexes
                // should be moved to the right.
                debug_assert!(idx < idx_old);
                // Replace old selection with a new one and do rotation so new selections will
                // be on the right of updated selection in selections storage sorted vec.
                let selection_old = mem::replace(&mut self.selections[idx_old], selection_new);
                self.selections[idx..=idx_old].rotate_right(1);
                let mut deltas = SelectionDeltas::default();
                deltas.push_updated(selection_old, &self.selections[idx]);
                deltas
            }
            Ok(range) => {
                // Collisions found, overwritten selections will be removed.
                let start_idx = *range.start();
                let end_idx = *range.end();
                let mut deltas = SelectionDeltas::default();

                // In case of `extend` overwritten selection must become a part of the updated
                // selection.
                if extend && self.selections[start_idx].from < selection_new.from {
                    selection_new.from = self.selections[start_idx].from.clone();
                }

                // Update a selection leaving it at the same place, as those on the left will
                // be removed anyway.
                let selection_old = mem::replace(&mut self.selections[idx_old], selection_new);
                for s in self.selections.drain(start_idx..=end_idx) {
                    deltas.push_deleted(s);
                }
                deltas.push_updated(selection_old, &self.selections[start_idx]);
                deltas
            }
        };

        debug_assert!(self.is_state_correct());
        deltas
    }

    /// Moves a selection identified by `id` (its `from` component) on `n`
    /// columns right. Will be narrowed to length of 1 character if `extend`
    /// is `false`.
    pub fn move_right_single(
        &mut self,
        line_lengths: impl LineLength,
        id: &Position,
        n: usize,
        extend: bool,
    ) -> SelectionDeltas {
        if n == 0 {
            return Default::default();
        }

        let Some(idx_old) = self.find_index_by_id(id) else {
	    return Default::default();
	};

        let mut selection_new = self.selections[idx_old].move_right(&line_lengths, n, extend);

        let idx_new =
            self.find_overlapping_indicies_exlude(&selection_new.from, &selection_new.to, idx_old);

        let deltas = match idx_new {
            Err(idx) if idx == idx_old => {
                // Updated selection stays at the same index and no collisions to solve:
                let selection_old = mem::replace(&mut self.selections[idx_old], selection_new);
                let mut deltas = SelectionDeltas::default();
                deltas.push_updated(selection_old, &self.selections[idx]);
                deltas
            }
            Err(idx) => {
                // Still no collisions, but index is changed and in case of
                // movement to the right it will be greater than before, than means
                // a subset of all selections between old and new indexes
                // should be moved to the left.
                //
                // The difference between left and right movement is that right border of
                // rotation during left movement was an index of actual selection (previous
                // state), but in case of right rotation right border is found via
                // `find_overlapping_indicies_exlude` which points to insertion position, which
                // is the index of selection to insert _before_ it (or may be out of bounds if
                // insertion should happen to the vector's end).

                debug_assert!(idx > idx_old);
                // Replace old selection with a new one and do rotation so new selections will
                // be on the left of updated selection in selections storage sorted vec.
                // Note that `idx` is not included this time, explanation is above.
                let selection_old = mem::replace(&mut self.selections[idx_old], selection_new);
                self.selections[idx_old..idx].rotate_left(1);
                let mut deltas = SelectionDeltas::default();
                deltas.push_updated(selection_old, &self.selections[idx - 1]);
                deltas
            }
            Ok(range) => {
                // Collisions found, overwritten selections will be removed.
                let start_idx = *range.start();
                let end_idx = *range.end();
                let mut deltas = SelectionDeltas::default();

                // In case of `extend` overwritten selection must become a part of the updated
                // selection.
                if extend && self.selections[end_idx].to > selection_new.to {
                    selection_new.to = self.selections[end_idx].to.clone();
                }

                // Update a selection leaving it at the same place, as those on the right will
                // be removed anyway.
                let selection_old = mem::replace(&mut self.selections[idx_old], selection_new);
                for s in self.selections.drain(start_idx..=end_idx) {
                    deltas.push_deleted(s);
                }
                deltas.push_updated(selection_old, &self.selections[start_idx]);
                deltas
            }
        };

        debug_assert!(self.is_state_correct());
        deltas
    }
}
