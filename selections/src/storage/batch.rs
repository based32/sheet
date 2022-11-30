use std::ops::RangeBounds;

use crate::{Position, Selection};

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

pub enum SelectionsQuery<R, I> {
    Range(R),
    Exact(I),
}

pub enum MovementDirection {
    Left,
    Right,
    Up,
    Down,
}
