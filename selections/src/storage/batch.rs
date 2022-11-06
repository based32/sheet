use crate::{Position, Selection};

enum MovementDirection {
    Left,
    Right,
    Up,
    Down,
}

enum Operation {
    Insert(Selection),
    Move {
        id: Position,
        n: usize,
        extend: bool,
        direction: MovementDirection,
    },
    Delete(Position),
}
