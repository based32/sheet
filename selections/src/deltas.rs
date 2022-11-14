//! Selection deltas definitions.
use std::{cmp, vec};

use crate::{utils::UniqueSortedVec, Position, Selection};

/// Info on changed selection.
#[derive(Debug, PartialEq, Eq)]
pub enum SelectionDelta<'a> {
    /// Selection was created
    Created(&'a Selection),
    /// Selection was deleted
    Deleted(Selection),
    /// Selection was updated
    Updated {
        /// Old selection state
        old: Selection,
        /// New selection state
        new: &'a Selection,
    },
}

impl PartialOrd for SelectionDelta<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        match self
            .get_from_position()
            .partial_cmp(other.get_from_position())
        {
            Some(std::cmp::Ordering::Equal) => {
                if matches!(self, SelectionDelta::Deleted(_)) {
                    Some(std::cmp::Ordering::Less)
                } else {
                    Some(std::cmp::Ordering::Greater)
                }
            }
            other => other,
        }
    }
}

impl Ord for SelectionDelta<'_> {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.partial_cmp(other).expect("total order defined")
    }
}

impl SelectionDelta<'_> {
    /// Shortcut to get `from` coordinate required for comparison
    fn get_from_position(&self) -> &Position {
        match self {
            SelectionDelta::Created(Selection { from, .. }) => from,
            SelectionDelta::Deleted(s) | SelectionDelta::Updated { old: s, .. } => &s.from,
        }
    }
}

/// Collection of sorted selection deltas.
/// Works better when pushing deltas in order (ordered by `from`, `Deleted`
/// first on collision).
#[derive(Debug)]
pub struct SelectionDeltas<'a> {
    deltas: UniqueSortedVec<SelectionDelta<'a>>,
}

impl Default for SelectionDeltas<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> SelectionDeltas<'a> {
    /// Create empty deltas collection
    pub(crate) fn new() -> Self {
        SelectionDeltas {
            deltas: UniqueSortedVec::new(),
        }
    }

    /// Create empty deltas collection with size hint
    pub(crate) fn with_capacity(n: usize) -> Self {
        SelectionDeltas {
            deltas: UniqueSortedVec::with_capacity(n),
        }
    }

    /// Adds delta for a deleted selection
    pub(crate) fn push_deleted(&mut self, s: Selection) {
        self.deltas.push(SelectionDelta::Deleted(s));
    }

    /// Adds delta for a created selection
    pub(crate) fn push_created(&mut self, s: &'a Selection) {
        self.deltas.push(SelectionDelta::Created(s));
    }

    /// Adds delta for an updated selection
    pub(crate) fn push_updated(&mut self, old: Selection, new: &'a Selection) {
        self.deltas.push(SelectionDelta::Updated { old, new });
    }
}

impl<'a> IntoIterator for SelectionDeltas<'a> {
    type IntoIter = vec::IntoIter<SelectionDelta<'a>>;
    type Item = SelectionDelta<'a>;

    /// Returns iterator over selection deltas keeping their order (in case of
    /// `Updated` it will order by its old state)
    fn into_iter(self) -> Self::IntoIter {
        self.deltas.take().into_iter()
    }
}
