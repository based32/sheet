//! [SelectionStorage] movements implementation module.

use std::{cmp, mem};

use super::{
    query::{SelectionIndex, SelectionIndexRange},
    SelectionStorage,
};
use crate::{LineLength, Position, Selection, SelectionDeltas};

/// Structure that handles corner cases for general single selection movement in
/// [SelectionStorage].
enum MovementDirection {
    Left,
    Right,
    Up,
    Down,
}

impl MovementDirection {
    /// Boolean flag as most of the times behaviour differs whether movement
    /// happens towards buffer beginning or not.
    fn is_towards_beginning(&self) -> bool {
        match self {
            MovementDirection::Left | MovementDirection::Up => true,
            MovementDirection::Right | MovementDirection::Down => false,
        }
    }

    /// Apply appropriate function call to get new selection.
    fn new_selection_state(
        &self,
        selection: &Selection,
        line_length: &impl LineLength,
        n: usize,
        extend: bool,
    ) -> Selection {
        match self {
            MovementDirection::Left => selection.move_left(line_length, n, extend),
            MovementDirection::Right => selection.move_right(line_length, n, extend),
            MovementDirection::Up => selection.move_up(line_length, n, extend),
            MovementDirection::Down => selection.move_down(line_length, n, extend),
        }
    }

    /// Depending on direction of movement in case of no overlaps other
    /// selections between old a new positions should be either moved towards
    /// beginning or end.
    ///
    /// The difference between left and right movement is that right border of
    /// rotation during left movement was an index of actual selection (previous
    /// state), but in case of right rotation right border is found via
    /// [Self::find_overlapping_indicies_exlude] which points to insertion
    /// position, which is the index of selection to insert _before_ it (or
    /// may be out of bounds if insertion should happen to the vector's
    /// end), so that means actual selection index for right border is
    /// decremented in case of movement towards buffer end.
    fn fix_step_over_rotation(
        &self,
        selections: &mut Vec<Selection>,
        old_index: SelectionIndex,
        new_index: SelectionIndex,
    ) {
        if self.is_towards_beginning() {
            selections[new_index..=old_index].rotate_right(1);
        } else {
            selections[old_index..new_index].rotate_left(1);
        }
    }

    /// Extends new selections in case of overlap by it's edge.
    fn fix_extend_overlap(
        &self,
        selections: &Vec<Selection>,
        overlap_indicies: SelectionIndexRange,
        new_selection: &mut Selection,
    ) {
        let start_idx = *overlap_indicies.start();
        let end_idx = *overlap_indicies.end();

        if self.is_towards_beginning() {
            if selections[start_idx].from < new_selection.from {
                new_selection.from = selections[start_idx].from.clone();
            }
        } else {
            if selections[end_idx].to > new_selection.to {
                new_selection.to = selections[end_idx].to.clone();
            }
        }
    }

    /// New index for a selection after rotation depends on movement direction.
    /// As described in [fix_step_over_rotation] docs, movement towards buffer
    /// ends requires index decrement.
    fn get_final_new_index(&self, idx: SelectionIndex) -> SelectionIndex {
        if self.is_towards_beginning() {
            idx
        } else {
            idx - 1
        }
    }
}

impl SelectionStorage {
    /// Moves a selection identified by `id` (its `from` component) on `n`
    /// columns left. Will be narrowed to length of 1 character if `extend`
    /// is `false`.
    pub fn move_left_single(
        &mut self,
        line_lengths: &impl LineLength,
        id: &Position,
        n: usize,
        extend: bool,
    ) -> SelectionDeltas {
        self.move_common(MovementDirection::Left, line_lengths, id, n, extend)
    }

    /// Moves a selection identified by `id` (its `from` component) on `n`
    /// columns right. Will be narrowed to length of 1 character if `extend`
    /// is `false`.
    pub fn move_right_single(
        &mut self,
        line_lengths: &impl LineLength,
        id: &Position,
        n: usize,
        extend: bool,
    ) -> SelectionDeltas {
        self.move_common(MovementDirection::Right, line_lengths, id, n, extend)
    }

    /// Moves a selection identified by `id` (its `from` component) on `n`
    /// lines up. Will be narrowed to length of 1 character if `extend`
    /// is `false`.
    pub fn move_up_single(
        &mut self,
        line_lengths: &impl LineLength,
        id: &Position,
        n: usize,
        extend: bool,
    ) -> SelectionDeltas {
        self.move_common(MovementDirection::Up, line_lengths, id, n, extend)
    }

    /// Moves a selection identified by `id` (its `from` component) on `n`
    /// lines down. Will be narrowed to length of 1 character if `extend`
    /// is `false`.
    pub fn move_down_single(
        &mut self,
        line_lengths: &impl LineLength,
        id: &Position,
        n: usize,
        extend: bool,
    ) -> SelectionDeltas {
        self.move_common(MovementDirection::Down, line_lengths, id, n, extend)
    }

    /// Common logic for movement.
    fn move_common(
        &mut self,
        direction: MovementDirection,
        line_lengths: &impl LineLength,
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

        let mut selection_new =
            direction.new_selection_state(&self.selections[idx_old], line_lengths, n, extend);

        // Find new insertion index or possible overlaps.
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
                // Replace old selection with a new one and do rotation to fix vector order.
                let selection_old = mem::replace(&mut self.selections[idx_old], selection_new);
                direction.fix_step_over_rotation(&mut self.selections, idx_old, idx);
                let mut deltas = SelectionDeltas::default();
                deltas.push_updated(
                    selection_old,
                    &self.selections[direction.get_final_new_index(idx)],
                );
                deltas
            }
            Ok(range) => {
                // Collisions found, overwritten selections will be removed.
                let start_idx = *range.start();
                let end_idx = *range.end();
                let mut deltas = SelectionDeltas::default();

                // With `extend` flag leftmost/rightmost overlapped selection will become a part
                // of new selection.
                if extend {
                    direction.fix_extend_overlap(&self.selections, range, &mut selection_new);
                }

                // Update a selection leaving it at the same place, as others will
                // be removed anyway.
                let selection_old = mem::replace(&mut self.selections[idx_old], selection_new);
                for s in self.selections.drain(start_idx..=end_idx) {
                    deltas.push_deleted(s);
                }
                // After deletion of overlapped selections new selection will be placed at the
                // leftmost index of all involved, so [cmp::min] is legal there.
                deltas.push_updated(
                    selection_old,
                    &self.selections[cmp::min(start_idx, idx_old)],
                );
                deltas
            }
        };

        debug_assert!(self.is_state_correct());
        deltas
    }
}
