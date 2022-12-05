//! Deletion operation implementation for [SelectionStorage].

use crate::{Position, Selection, SelectionDeltas, SelectionStorage};

impl SelectionStorage {
    /// Deletes a selection from storage.
    pub fn delete(&mut self, id: &Position) -> SelectionDeltas {
        let mut deltas = self.delete_internal(id);

        if self.selections.is_empty() {
            self.selections.push(Selection::default());
            deltas.push_created(&self.selections[0]);
        }

        debug_assert!(self.is_state_correct());
        deltas
    }

    /// Deletes a selection without creating default selection if deleting last.
    /// Returns deleted selection instead of delta to untie from `self` lifetime.
    pub(crate) fn delete_internal(&mut self, id: &Position) -> SelectionDeltas<'static> {
        let mut deltas = SelectionDeltas::with_capacity(2);

        if let Some(idx) = self.find_index_by_id(id.into()) {
            let deleted = self.selections.remove(idx);
            deltas.push_deleted(deleted);
        }

        deltas
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::selections_test;

    #[test]
    fn simple_deletion() {
        selections_test! {
            [
                (0, 3) - (0, 5),
                (1, 3) - (3, 7),
                (4, 3) - (5, 7),
                (6, 7) - (8, 9)
            ],
            storage -> {
                storage.delete(&Position::new(4, 3))
            },
            [
                Deleted((4, 3) - (5, 7))
            ],
            [
                (0, 3) - (0, 5),
                (1, 3) - (3, 7),
                (6, 7) - (8, 9)
            ]
        };
    }

    #[test]
    fn delete_last() {
        selections_test! {
            [
                (0, 3) - (0, 5)
            ],
            storage -> {
                storage.delete(&Position::new(0, 3))
            },
            [
                Created((0, 0) - (0, 0))
                Deleted((0, 3) - (0, 5))
            ],
            [
                (0, 0) - (0, 0),
            ]
        };
    }
}