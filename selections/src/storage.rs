//! [SelectionStorage] declaration and implementation.

mod get;
//mod insert;
mod movement;
#[cfg(test)]
mod test_movement;

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
}
