//! Selection deltas definitions.
use std::{cmp, vec};

use crate::{Position, Selection};

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
        match self.from_pos().partial_cmp(&other.from_pos()) {
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
    fn from_pos(&self) -> &Position {
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
    deltas: Vec<SelectionDelta<'a>>,
}

impl<'a> SelectionDeltas<'a> {
    /// Create empty deltas collection
    pub(crate) fn new() -> Self {
        SelectionDeltas { deltas: Vec::new() }
    }

    /// Create empty deltas collection with size hint
    pub(crate) fn with_capacity(n: usize) -> Self {
        SelectionDeltas {
            deltas: Vec::with_capacity(n),
        }
    }

    /// Adds delta for a deleted selection
    pub(crate) fn push_deleted(&mut self, s: Selection) {
        self.push(SelectionDelta::Deleted(s));
    }

    /// Adds delta for a created selection
    pub(crate) fn push_created(&mut self, s: &'a Selection) {
        self.push(SelectionDelta::Created(s));
    }

    /// Returns iterator over selection deltas keeping their order (in case of
    /// `Updated` it will order by its old state)
    pub fn into_iter(self) -> vec::IntoIter<SelectionDelta<'a>> {
        self.deltas.into_iter()
    }

    /// Puts the delta on the top of vec, reorders if needed.
    fn push(&mut self, delta: SelectionDelta<'a>) {
        if self.deltas.last().map(|last| last < &delta).unwrap_or(true) {
            // If order is maintained just put delta in the end
            self.deltas.push(delta);
        } else {
            self.deltas.push(delta);
            self.deltas.sort(); // TODO no need to check all vector
        }
    }
}
