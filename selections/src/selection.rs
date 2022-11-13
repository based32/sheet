//! [Selection] declaration and movement implementation.

mod movement;
#[cfg(test)]
mod test_movement;

use super::Position;

/// Selection is a pair of coordinates in a document.
#[derive(Debug, Clone, PartialEq, Eq)]
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
    pub fn new(anchor: Position, cursor: Position) -> Self {
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

    #[cfg(test)]
    /// Equality check but ignoring sticky columns.
    pub(crate) fn weak_eq(&self, other: &Self) -> bool {
        self.from.weak_eq(&other.from) && self.to.weak_eq(&other.to)
    }

    /// Get selection's anchor.
    #[inline]
    pub fn anchor(&self) -> &Position {
        match self.direction {
            SelectionDirection::Forward => &self.from,
            SelectionDirection::Backward => &self.to,
        }
    }

    /// Get selection's cursor.
    #[inline]
    pub fn cursor(&self) -> &Position {
        match self.direction {
            SelectionDirection::Forward => &self.to,
            SelectionDirection::Backward => &self.from,
        }
    }

    /// Get selections's id (a `from` component).
    #[inline]
    pub fn id(&self) -> &Position {
        &self.from
    }
}
