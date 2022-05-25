#![deny(missing_docs)]

//! Selection storage library.

mod get;
mod insert;
mod movements;
mod util;

use std::collections::{btree_set::IntoIter, BTreeSet};

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
    link: RBTreeLink,
}

impl PartialEq for Selection {
    fn eq(&self, other: &Self) -> bool {
        self.from == other.from && self.to == other.to
    }
}

impl Default for Selection {
    fn default() -> Self {
        Selection {
            from: Position::new(0, 0),
            to: Position::new(0, 1),
            link: Default::default(),
        }
    }
}

impl Selection {
    /// Get selection's left coordinates.
    #[inline]
    pub fn from(&self) -> &Position {
        &self.from
    }

    /// Get selection's right coordinates.
    #[inline]
    pub fn to(&self) -> &Position {
        &self.to
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
    direction: SelectionDirection,
}

impl SelectionStorage {
    /// Create selection storage with an initial selection in the document
    /// beginning.
    pub fn new() -> Self {
        let mut tree = RBTree::new(SelectionAdapter::new());
        tree.insert(Box::new(Selection {
            from: Position { line: 0, column: 0 },
            to: Position { line: 0, column: 1 },
            link: RBTreeLink::new(),
        }));
        SelectionStorage {
            tree,
            direction: SelectionDirection::Forward,
        }
    }

    #[cfg(test)]
    /// Create selection storage with no default selection.
    fn new_empty() -> Self {
        let tree = RBTree::new(SelectionAdapter::new());
        SelectionStorage {
            tree,
            direction: SelectionDirection::Forward,
        }
    }
}

/// Info on created/deleted/updated selection.
#[derive(Debug, PartialEq)]
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

impl PartialEq for SelectionDeltaWrapper<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.0.from_pos() == other.0.from_pos()
    }
}

impl Eq for SelectionDeltaWrapper<'_> {}

impl PartialOrd for SelectionDeltaWrapper<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.from_pos().partial_cmp(&other.0.from_pos())
    }
}

impl Ord for SelectionDeltaWrapper<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).expect("total ordering is defined")
    }
}

/// Collection of selection deltas.
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
