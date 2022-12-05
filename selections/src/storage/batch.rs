use crate::Selection;

/// Batch allows to apply multiple operations simultaneously for the selection storage.
pub struct SelectionCommandsBatch<I> {
    pub(crate) to_delete: Option<I>,
    pub(crate) to_move: Option<MoveCommand<I>>,
    pub(crate) to_insert: Vec<InsertCommand>,
}

pub(crate) struct MoveCommand<I> {
    pub(crate) query: I,
    pub(crate) direction: MovementDirection,
    pub(crate) extending: bool,
}

pub(crate) struct InsertCommand {
    selection: Selection,
    replacing: bool,
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
