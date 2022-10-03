#![deny(missing_docs)]

//! Selection storage library.

mod deltas;
mod get;
mod insert;
mod movements;
mod util;

pub use deltas::{SelectionDelta, SelectionDeltas};
use intrusive_collections::{intrusive_adapter, KeyAdapter, RBTree, RBTreeLink};

/// Coordinates in a document.
#[derive(Debug, PartialEq, Eq, Ord, Clone)]
pub struct Position {
    line: usize,
    column: usize,
    sticky_column: Option<usize>,
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(match self.line.cmp(&other.line) {
            std::cmp::Ordering::Equal => self.column.cmp(&other.column),
            other => other,
        })
    }
}

impl Position {
    /// Creates new position with `line` and `column`.
    pub fn new(line: usize, column: usize) -> Self {
        Position {
            line,
            column,
            sticky_column: None,
        }
    }

    /// Creates new position with `line`, `column` and `sticky_column`.
    pub fn new_with_sticky(line: usize, column: usize, sticky_column: usize) -> Self {
        Position {
            line,
            column,
            sticky_column: Some(sticky_column),
        }
    }

    /// Removes sticky column
    fn remove_sticky(mut self) -> Self {
        self.sticky_column = None;
        self
    }
}

/// Direction of a selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectionDirection {
    /// Means cursor is after selection
    Forward,
    /// Means cursor is before selection
    Backward,
}

/// Selection is a pair of coordinates in a document.
#[derive(Debug, Clone)]
pub struct Selection {
    from: Position,
    to: Position,
    direction: SelectionDirection,
    link: RBTreeLink,
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
            link: Default::default(),
        }
    }
}

impl Selection {
    /// Build new selection from two positions setting proper direction
    /// depending on order.
    fn new(mut from: Position, mut to: Position) -> Self {
        let direction = if from > to {
            std::mem::swap(&mut from, &mut to);
            SelectionDirection::Backward
        } else {
            SelectionDirection::Forward
        };
        Selection {
            from,
            to,
            direction,
            ..Default::default()
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

// TODO: an allocation for each selection may be too much, consider using
// vector/arena for selections and references for a tree items
intrusive_adapter!(SelectionAdapter = Box<Selection> : Selection { link: RBTreeLink });

impl<'a> KeyAdapter<'a> for SelectionAdapter {
    type Key = &'a Position;

    fn get_key(&self, s: &'a Selection) -> Self::Key {
        &s.from
    }
}

/// Multiselection storage which guarantees no overlaps.
pub struct SelectionStorage {
    tree: RBTree<SelectionAdapter>,
}

impl SelectionStorage {
    /// Create selection storage with an initial selection in the document
    /// beginning.
    pub fn new() -> Self {
        let mut tree = RBTree::new(SelectionAdapter::new());
        tree.insert(Box::new(Selection::default()));
        SelectionStorage { tree }
    }

    #[cfg(test)]
    /// Create selection storage with no default selection.
    fn new_empty() -> Self {
        let tree = RBTree::new(SelectionAdapter::new());
        SelectionStorage { tree }
    }
}
