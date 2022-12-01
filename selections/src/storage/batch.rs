use std::ops::RangeBounds;

use crate::{Position, Selection};

/// Batch allows to apply multiple operations simultaneously for the selection storage.
pub struct SelectionCommandsBatch<R, I> {
    to_delete: Option<SelectionsQuery<R, I>>,
    to_move: Option<MoveCommand<R, I>>,
    to_insert: Vec<Selection>,
}

struct MoveCommand<R, I> {
    query: SelectionsQuery<R, I>,
    direction: MovementDirection,
    extending: bool,
}

struct InsertCommand {
    selection: Selection,
    replacing: bool,
}

/// Allows to be more specific about what selections to apply operations
pub enum SelectionsQuery<R, I> {
    /// A range of selections
    Range(R),
    /// An iterator if selection ids
    Exact(I),
}

/// Possible movement directions for a two dimensional text editor
pub enum MovementDirection {
    /// Left movement
    Left,
    /// Right movement
    Right,
    /// Up movement
    Up,
    /// Down movement
    Down,
}
