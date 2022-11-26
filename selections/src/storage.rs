//! [SelectionStorage] declaration and implementation.

mod get;
mod insert;
mod movement;
mod query;
#[cfg(test)]
mod test_movement_single;

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
    }
}

impl Default for SelectionStorage {
    fn default() -> Self {
        Self::new()
    }
}
