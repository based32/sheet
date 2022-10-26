//! [Selection] declaration and movement implementation.

mod movement;
#[cfg(test)]
mod test_movement;

use std::mem;

use super::Position;

/// Selection is a pair of coordinates in a document.
#[derive(Debug, Clone)]
pub struct Selection {
    pub(crate) from: Position,
    pub(crate) to: Position,
    pub(crate) direction: SelectionDirection,
}

/// Direction of a selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectionDirection {
    /// Means cursor is after selection
    Forward,
    /// Means cursor is before selection
    Backward,
}

impl PartialEq for Selection {
    fn eq(&self, other: &Self) -> bool {
        self.from == other.from && self.to == other.to && self.direction == other.direction
    }
}

impl Eq for Selection {}

impl Default for Selection {
    fn default() -> Self {
        Selection {
            from: Position::new(0, 0),
            to: Position::new(0, 0),
            direction: SelectionDirection::Forward,
        }
    }
}

impl Selection {
    /// Build new selection from two positions setting proper direction
    /// depending on order.
    pub fn new(mut anchor: Position, mut cursor: Position) -> Self {
        if anchor > cursor {
            Selection {
                from: cursor,
                to: anchor,
                direction: SelectionDirection::Backward,
            }
        } else {
            Selection {
                from: anchor,
                to: cursor,
                direction: SelectionDirection::Forward,
            }
        }
    }

    /// Get selection's left coordinates.
    #[inline]
    pub fn from(&self) -> &Position {
        match self.direction {
            SelectionDirection::Forward => &self.from,
            SelectionDirection::Backward => &self.to,
        }
    }

    /// Get selection's right coordinates (greater than or equal to its left
    /// coordinates).
    #[inline]
    pub fn to(&self) -> &Position {
        match self.direction {
            SelectionDirection::Forward => &self.to,
            SelectionDirection::Backward => &self.from,
        }
    }
}
