//! Helper functions to query [SelectionStorage] for things such as overlaps.

use std::{borrow::Cow, cmp::Ordering, ops::RangeInclusive};

use super::SelectionStorage;
use crate::Position;

pub(super) type SelectionIndex = usize;

pub(super) type SelectionIndexRange = RangeInclusive<SelectionIndex>;

/// Just like [Position], but ignoring sticky column.
#[derive(Debug, Eq, PartialOrd, Ord)]
pub(crate) struct PositionQuery<'a>(Cow<'a, Position>);

impl PartialEq for PositionQuery<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.0.weak_eq(&other.0)
    }
}

impl<'a> From<&'a Position> for PositionQuery<'a> {
    fn from(position: &'a Position) -> Self {
        PositionQuery(Cow::Borrowed(position))
    }
}

impl<'a> From<Position> for PositionQuery<'a> {
    fn from(position: Position) -> Self {
        PositionQuery(Cow::Owned(position))
    }
}

impl SelectionStorage {
    /// Find a Selection by its `from` component.
    pub(crate) fn find_index_by_id(&self, from: PositionQuery) -> Option<SelectionIndex> {
        self.selections
            .binary_search_by(|s| PositionQuery::from(&s.from).cmp(&from))
            .ok()
    }

    /// Find the range of indicies of Selections that overlaps with the provided
    /// one. `Err` case means no overlaps and points to insertion position.
    pub(crate) fn find_overlapping_indicies(
        &self,
        from: PositionQuery,
        to: PositionQuery,
    ) -> Result<SelectionIndexRange, SelectionIndex> {
        if self.selections.is_empty() {
            return Err(0);
        }

        // Get index of a selection that overlaps with `from`, or point at possibly
        // overlapping selection (more on that later).
        let from_idx_result = self.selections.binary_search_by(|s| {
            if from >= PositionQuery::from(&s.from) && from <= PositionQuery::from(&s.to) {
                Ordering::Equal
            } else {
                PositionQuery::from(&s.from).cmp(&from)
            }
        });

        // Same as above, but for `to`.
        let to_idx_result = self.selections.binary_search_by(|s| {
            if to >= PositionQuery::from(&s.from) && to <= PositionQuery::from(&s.to) {
                Ordering::Equal
            } else {
                PositionQuery::from(&s.to).cmp(&to)
            }
        });

        // Binary search returns Result because it could either find an index of a value
        // in a vector or an index to insert an object we're searching for. That said,
        // we're able to find a range of overlapping selections even if both ends won't
        // collide with any, but the interval itself may overlap.
        match (from_idx_result, to_idx_result) {
            // If both ends didn't collide with anything and point to the same place it means there
            // are no overlaps:
            (Err(from_idx), Err(to_idx)) if from_idx == to_idx => Err(from_idx),
            // Otherwise there must be collisions, on `from_idx` position it will be either a match
            // (Ok) or the query result range beginning, and match for `to_idx` must be
            // greater than one for `from_idx`, so it will be actually a range.
            //
            // `from_idx` in case of Err will point
            // _after_ rightmost selection with its `from_idx` less than query's `from_idx`, that's
            // why it needs to be decremented.
            (Ok(from_idx) | Err(from_idx), Err(to_idx)) => Ok(from_idx..=(to_idx - 1)),
            // In case of Ok it's exact match and no indexing tricks required.
            (Ok(from_idx) | Err(from_idx), Ok(to_idx)) => Ok(from_idx..=to_idx),
        }
    }

    /// Find the range of indicies of Selections that overlaps with the provided
    /// one, but excluding an index provided.
    pub(crate) fn find_overlapping_indicies_exlude(
        &self,
        from: PositionQuery,
        to: PositionQuery,
        exclude: SelectionIndex,
    ) -> Result<SelectionIndexRange, SelectionIndex> {
        self.find_overlapping_indicies(from, to).and_then(|range| {
            let start = range.start();
            let end = range.end();

            if start == end && start == &exclude {
                Err(exclude)
            } else if start == &exclude {
                Ok(start + 1..=*end)
            } else if end == &exclude {
                Ok(*start..=*end - 1)
            } else {
                Ok(range)
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::Selection;

    #[test]
    fn find_overlapping_indicies() {
        let storage = SelectionStorage {
            selections: vec![
                Selection::new(Position::new(0, 5), Position::new(1, 10)),
                Selection::new(Position::new(1, 20), Position::new(2, 10)),
                Selection::new(Position::new(2, 20), Position::new(4, 10)),
                Selection::new(Position::new(4, 20), Position::new(7, 11)),
            ],
        };

        // Overlap on the right side:
        assert_eq!(
            storage.find_overlapping_indicies(
                Position::new(1, 15).into(),
                Position::new(1, 30).into()
            ),
            Ok(1..=1)
        );

        // Overlap on the left side:
        assert_eq!(
            storage
                .find_overlapping_indicies(Position::new(1, 8).into(), Position::new(1, 16).into()),
            Ok(0..=0)
        );

        // Overlap on both sides:
        assert_eq!(
            storage
                .find_overlapping_indicies(Position::new(0, 5).into(), Position::new(1, 20).into()),
            Ok(0..=1)
        );

        // No overlaps in between:
        assert_eq!(
            storage.find_overlapping_indicies(
                Position::new(1, 15).into(),
                Position::new(1, 17).into()
            ),
            Err(1),
        );

        // No overlaps before selections:
        assert_eq!(
            storage
                .find_overlapping_indicies(Position::new(0, 0).into(), Position::new(0, 3).into()),
            Err(0)
        );

        // No overlaps after selections:
        assert_eq!(
            storage.find_overlapping_indicies(
                Position::new(8, 12).into(),
                Position::new(13, 37).into()
            ),
            Err(4)
        );

        // Large selection overlaps all:
        assert_eq!(
            storage.find_overlapping_indicies(
                Position::new(0, 0).into(),
                Position::new(13, 37).into()
            ),
            Ok(0..=3),
        );

        // Query selection absorbs another one:
        assert_eq!(
            storage
                .find_overlapping_indicies(Position::new(0, 3).into(), Position::new(1, 15).into()),
            Ok(0..=0),
        );

        // Query selection will be absorbed:
        assert_eq!(
            storage
                .find_overlapping_indicies(Position::new(0, 7).into(), Position::new(0, 8).into()),
            Ok(0..=0),
        );

        // Overlap on the left side and then absorb on right:
        assert_eq!(
            storage.find_overlapping_indicies(
                Position::new(1, 8).into(),
                Position::new(69, 69).into()
            ),
            Ok(0..=3)
        );
    }
}
