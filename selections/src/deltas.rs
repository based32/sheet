//! Selection deltas definitions.
use std::collections::BTreeSet;

use crate::{Position, Selection};

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
    pub(crate) fn new() -> Self {
        SelectionDeltas {
            selections: BTreeSet::new(),
        }
    }

    /// Adds delta for a deleted selection
    pub(crate) fn add_deleted(&mut self, s: Box<Selection>) {
        self.selections
            .insert(SelectionDeltaWrapper(SelectionDelta::Deleted(s)));
    }

    /// Adds delta for a created selection
    pub(crate) fn add_created(&mut self, s: &'a Selection) {
        self.selections
            .insert(SelectionDeltaWrapper(SelectionDelta::Created(s)));
    }

    /// Returns iterator over selection deltas keeping their order (in case of
    /// `Updated` it will order by its old state)
    pub fn into_iter(self) -> impl Iterator<Item = SelectionDelta<'a>> {
        self.selections.into_iter().map(|x| x.0)
    }
}
