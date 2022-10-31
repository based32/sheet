//! Helper functions to query [SelectionStorage] for things such as overlaps.

use std::{cmp::Ordering, ops::RangeInclusive};

use super::SelectionStorage;
use crate::{Position, Selection};

type SelectionIndex = usize;

type SelectionIndexRange = RangeInclusive<SelectionIndex>;

impl SelectionStorage {
    /// Find the index of a Selection that includes the Position.
    pub(crate) fn find_including_index(&self, pos: &Position) -> Option<SelectionIndex> {
        self.selections
            .binary_search_by(|s| {
                if &s.from <= pos && &s.to >= pos {
                    Ordering::Equal
                } else {
                    s.from.cmp(pos)
                }
            })
            .ok()
    }

    /// Find a Selection by its `from` component.
    pub(crate) fn find_index_by_id(&self, from: &Position) -> Option<SelectionIndex> {
        self.selections.binary_search_by(|s| s.from.cmp(from)).ok()
    }

    /// Find the range of indicies of Selections that overlaps with the provided
    /// one. `Err` case means no overlaps and points to insertion position.
    pub(crate) fn find_overlapping_indicies(
        &self,
        from: &Position,
        to: &Position,
    ) -> Result<SelectionIndexRange, SelectionIndex> {
        if self.selections.is_empty() {
            return Err(0);
        }

        // Get index of a selection that overlaps with `from`, or point at possibly
        // overlapping selection (more on that later).
        let from_idx_result = self.selections.binary_search_by(|s| {
            if from >= &s.from && from <= &s.to {
                Ordering::Equal
            } else {
                s.from.cmp(&from)
            }
        });

        // Same as above, but for `to`.
        let to_idx_result = self.selections.binary_search_by(|s| {
            if to >= &s.from && to <= &s.to {
                Ordering::Equal
            } else {
                s.to.cmp(&to)
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
        from: &Position,
        to: &Position,
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
                Ok(*start..=*end + 1)
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
    fn find_including_index() {
        let storage = SelectionStorage {
            selections: vec![
                Selection::new(Position::new(0, 0), Position::new(1, 10)),
                Selection::new(Position::new(1, 20), Position::new(2, 10)),
            ],
        };

        // Should hit first selection somewhere in a middle:
        assert_eq!(storage.find_including_index(&Position::new(0, 20)), Some(0));

        // Should hit second selection somewhere in a middle:
        assert_eq!(storage.find_including_index(&Position::new(1, 30)), Some(1));

        // Should hit first selection in its end:
        assert_eq!(storage.find_including_index(&Position::new(1, 10)), Some(0));

        // Should not hit anything:
        assert_eq!(storage.find_including_index(&Position::new(1, 11)), None);
    }

    #[test]
    fn find_overlapping_indicies() {
        let storage = SelectionStorage {
            selections: vec![
                Selection::new(Position::new(0, 5), Position::new(1, 10)),
                Selection::new(Position::new(1, 20), Position::new(2, 10)),
            ],
        };

        // Overlap on the right side:
        assert_eq!(
            storage.find_overlapping_indicies(&Position::new(1, 15), &Position::new(1, 30),),
            Ok(1..=1)
        );

        // Overlap on the left side:
        assert_eq!(
            storage.find_overlapping_indicies(&Position::new(1, 8), &Position::new(1, 16),),
            Ok(0..=0)
        );

        // Overlap on both sides:
        assert_eq!(
            storage.find_overlapping_indicies(&Position::new(0, 5), &Position::new(1, 20),),
            Ok(0..=1)
        );

        // No overlaps in between:
        assert_eq!(
            storage.find_overlapping_indicies(&Position::new(1, 15), &Position::new(1, 17),),
            Err(1),
        );

        // No overlaps before selections:
        assert_eq!(
            storage.find_overlapping_indicies(&Position::new(0, 0), &Position::new(0, 3),),
            Err(0)
        );

        // No overlaps after selections:
        assert_eq!(
            storage.find_overlapping_indicies(&Position::new(4, 20), &Position::new(13, 37),),
            Err(2)
        );

        // Large selection overlaps all:
        assert_eq!(
            storage.find_overlapping_indicies(&Position::new(0, 0), &Position::new(13, 37),),
            Ok(0..=1),
        );

        // Query selection absorbs another one:
        assert_eq!(
            storage.find_overlapping_indicies(&Position::new(0, 3), &Position::new(1, 15),),
            Ok(0..=0),
        );

        // Query selection will be absorbed:
        assert_eq!(
            storage.find_overlapping_indicies(&Position::new(0, 7), &Position::new(0, 8),),
            Ok(0..=0),
        );

        // Overlap on the left side and then absorb on right:
        assert_eq!(
            storage.find_overlapping_indicies(&Position::new(1, 8), &Position::new(16, 20),),
            Ok(0..=1)
        );
    }
}
