#![deny(missing_docs)]

//! Selection storage library.

mod get;
mod insert;
mod movements;
mod util;

use std::collections::BTreeSet;

use intrusive_collections::{intrusive_adapter, KeyAdapter, RBTree, RBTreeLink};

/// Coordinates in a document.
#[derive(Debug, PartialEq, Eq, Ord, Clone)]
pub struct Position {
    line: usize,
    column: usize,
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
        Position { line, column }
    }
}

/// Direction of selections in the selection storage.
#[derive(Debug, Clone, Copy)]
pub enum SelectionDirection {
    /// Means each cursor is after selection
    Forward,
    /// Means each cursor is before selection
    Backward,
}

/// Selection is a pair of coordinates in a document.
#[derive(Debug)]
pub struct Selection {
    from: Position,
    to: Position,
    direction: SelectionDirection,
    link: RBTreeLink,
}

impl PartialEq for Selection {
    fn eq(&self, other: &Self) -> bool {
        self.from == other.from && self.to == other.to
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
    #[cfg(test)]
    /// Build new selection from two positions setting proper direction depending on order.
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

/// Info on created/deleted/updated selection.
#[derive(Debug, PartialEq, Eq)]
pub enum SelectionDelta<'a> {
    /// Selection was created
    Created(&'a Selection),
    /// Selection was deleted
    Deleted(Box<Selection>),
    /// Selection was updated
    Updated {
        /// Old selection state
        old: Box<Selection>,
        /// New selection state
        new: &'a Selection,
    },
}

/// Wrapper to override selection delta comparison to hold a proper order within
/// `SelectionDeltas`.
#[derive(Debug, PartialEq, Eq)]
struct SelectionDeltaWrapper<'a>(SelectionDelta<'a>);

impl SelectionDelta<'_> {
    /// Shortcut to get `from` coordinate required for comparison
    fn from_pos(&self) -> &Position {
        match self {
            SelectionDelta::Created(Selection { from, .. }) => from,
            SelectionDelta::Deleted(s) | SelectionDelta::Updated { old: s, .. } => &s.from,
        }
    }
}

impl PartialOrd for SelectionDeltaWrapper<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.0.from_pos().partial_cmp(&other.0.from_pos()) {
            Some(std::cmp::Ordering::Equal) => {
                if matches!(self.0, SelectionDelta::Deleted(_)) {
                    Some(std::cmp::Ordering::Less)
                } else {
                    Some(std::cmp::Ordering::Greater)
                }
            }
            other => other,
        }
    }
}

impl Ord for SelectionDeltaWrapper<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).expect("total ordering is defined")
    }
}

/// Collection of selection deltas.
#[derive(Debug)]
pub struct SelectionDeltas<'a> {
    selections: BTreeSet<SelectionDeltaWrapper<'a>>,
}

impl<'a> SelectionDeltas<'a> {
    /// Create empty deltas collection
    fn new() -> Self {
        SelectionDeltas {
            selections: BTreeSet::new(),
        }
    }

    /// Adds delta for a deleted selection
    fn add_deleted(&mut self, s: Box<Selection>) {
        self.selections
            .insert(SelectionDeltaWrapper(SelectionDelta::Deleted(s)));
    }

    /// Adds delta for a created selection
    fn add_created(&mut self, s: &'a Selection) {
        self.selections
            .insert(SelectionDeltaWrapper(SelectionDelta::Created(s)));
    }

    /// Returns iterator over selection deltas keeping their order (in case of
    /// `Updated` it will order by its old state)
    pub fn into_iter(self) -> impl Iterator<Item = SelectionDelta<'a>> {
        self.selections.into_iter().map(|x| x.0)
    }
}
