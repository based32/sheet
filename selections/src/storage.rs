//! [SelectionStorage] declaration and implementation.

mod batch;
mod delete;
mod get;
mod insert;
mod movement;
mod query;
#[cfg(test)]
mod test_movement_single;

use self::batch::{InsertCommand, MoveCommand};
pub use self::batch::{MovementDirection, SelectionCommandsBatch};

use super::*;

/// Multiselection storage.
pub struct SelectionStorage {
    /// Selections sorted by `from` position in the buffer.
    selections: Vec<Selection>,
    // TODO: active
}

impl SelectionStorage {
    /// Create selection storage with an initial selection in the buffer
    /// beginning.
    pub fn new() -> Self {
        SelectionStorage {
            selections: vec![Selection::default()],
        }
    }

    #[cfg(test)]
    /// Create selection storage with no default selection.
    fn new_empty() -> Self {
        SelectionStorage {
            selections: Vec::new(),
        }
    }

    #[cfg(debug_assertions)]
    /// Verify if selections are kept sorted and with no overlaps.
    fn is_state_correct(&self) -> bool {
        self.selections
            .as_slice()
            .windows(2)
            .all(|w| w[0].to < w[1].from)
            && !self.selections.is_empty()
    }

    /// Apply batch of operations to the selection storage.
    pub fn apply_batch<'a, R, I>(&mut self, batch: SelectionCommandsBatch<I>) -> SelectionDeltas
    where
        I: IntoIterator<Item = &'a Position>,
    {
        // TODO do something more clever

        let mut deltas_vec = Vec::new();

        if let Some(to_delete) = batch.to_delete {
            for id in to_delete {
                deltas_vec.extend(self.delete_internal(id).into_iter());
            }
        }

        if let Some(MoveCommand {
            query,
            direction,
            extending,
        }) = batch.to_move
        {
            // plan for movement:
            // 1. If moving towards buffer end then start processing from the last one
            // 2. Deleted selections add to deltas
            // 3. Updated old states put into temp
            // 4. Updated new states put into temp as ids
        }

        for insert in batch.to_insert {
            // plan for insertion:
            // 1. Put overwritten into deltas
            // 2. Remove it from updated temp
            // 3. Keep inserted ids in a separate temp
        }

        // Add to deltas from updated temp and inserted temp

        SelectionDeltas::from_iter(deltas_vec.into_iter())
    }
}

impl Default for SelectionStorage {
    fn default() -> Self {
        Self::new()
    }
}
