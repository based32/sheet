//! [SelectionStorage] declaration and implementation.

mod batch;
mod delete;
mod get;
mod insert;
mod movement;
mod query;
#[cfg(test)]
mod test_movement_single;

pub use self::batch::{MovementDirection, SelectionCommandsBatch, SelectionsQuery};

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
    pub fn apply_batch<'a, R, I>(&mut self, batch: SelectionCommandsBatch<R, I>) -> SelectionDeltas
    where
        I: Iterator<Item = &'a Position>,
    {
        // TODO do something more clever
        
        let mut deltas_vec = Vec::new();

        if let Some(to_delete) = batch.to_delete {
            match to_delete {
                SelectionsQuery::Exact(iter) => {
                    for id in iter {
                        deltas_vec.extend(self.delete_internal(id).into_iter());
                    }
                }
                SelectionsQuery::Range(range) => {}
            }
        }
        SelectionDeltas::from_iter(deltas_vec.into_iter())
    }
}

impl Default for SelectionStorage {
    fn default() -> Self {
        Self::new()
    }
}
