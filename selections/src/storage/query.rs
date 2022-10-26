//! Helper functions to query [SelectionStorage] for things such as overlaps.

use std::{cmp::Ordering, ops::RangeInclusive};

use super::SelectionStorage;
use crate::{Position, Selection};

type SelectionIndex = usize;

type SelectionIndexRange = RangeInclusive<SelectionIndex>;

impl SelectionStorage {
    /// Find the index of a Selection that includes the Position.
    fn find_including_index(&self, pos: &Position) -> Option<SelectionIndex> {
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

    /// Find the range of indicies of Selections that overlaps with the provided
    /// one.
    fn find_overlapping_indicies(&self, selection: &Selection) -> Option<SelectionIndexRange> {
        if self.selections.is_empty() {
            return None;
        }

        // Get index of a selection that overlaps with `from`, or point at possibly
        // overlapping selection (more on that later).
        let from_result = self.selections.binary_search_by(|s| {
            if selection.from >= s.from && selection.from <= s.to {
                Ordering::Equal
            } else {
                s.from.cmp(&selection.from)
            }
        });

        // Same as above, but for `to`.
        let to_result = self.selections.binary_search_by(|s| {
            if selection.to >= s.from && selection.to <= s.to {
                Ordering::Equal
            } else {
                s.to.cmp(&selection.to)
            }
        });

        // Binary search returns Result because it could either find an index of a value
        // in a vector or an index to insert an object we're searching for. That said,
        // we're able to find a range of overlapping selections even if both ends won't
        // collide with any, but the interval itself may overlap.
        match (from_result, to_result) {
            // If both ends didn't collide with anything and point to the same place it means there
            // are no overlaps:
            (Err(from), Err(to)) if from == to => None,
            // Otherwise there must be collisions, on `from` position it will be either a match (Ok)
            // or the query result range beginning, and match for `to` must be greater than one for
            // `from`, so it will be actually a range.
            //
            // `from` in case of Err will point
            // _after_ rightmost selection with its `from` less than query's `from`, that's why it
            // needs to be decremented.
            (Ok(from) | Err(from), Err(to)) => Some(from..=(to - 1)),
            // In case of Ok it's exact match and no indexing tricks required.
            (Ok(from) | Err(from), Ok(to)) => Some(from..=to),
        }
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
            storage.find_overlapping_indicies(&Selection::new(
                Position::new(1, 15),
                Position::new(1, 30),
            )),
            Some(1..=1)
        );

        // Overlap on the left side:
        assert_eq!(
            storage.find_overlapping_indicies(&Selection::new(
                Position::new(1, 8),
                Position::new(1, 16),
            )),
            Some(0..=0)
        );

        // Overlap on both sides:
        assert_eq!(
            storage.find_overlapping_indicies(&Selection::new(
                Position::new(0, 5),
                Position::new(1, 20),
            )),
            Some(0..=1)
        );

        // No overlaps in between:
        assert_eq!(
            storage.find_overlapping_indicies(&Selection::new(
                Position::new(1, 15),
                Position::new(1, 17),
            )),
            None
        );

        // No overlaps before selections:
        assert_eq!(
            storage.find_overlapping_indicies(&Selection::new(
                Position::new(0, 0),
                Position::new(0, 3),
            )),
            None
        );

        // No overlaps after selections:
        assert_eq!(
            storage.find_overlapping_indicies(&Selection::new(
                Position::new(4, 20),
                Position::new(13, 37),
            )),
            None
        );

        // Large selection overlaps all:
        assert_eq!(
            storage.find_overlapping_indicies(&Selection::new(
                Position::new(0, 0),
                Position::new(13, 37),
            )),
            Some(0..=1),
        );

        // Query selection absorbs another one:
        assert_eq!(
            storage.find_overlapping_indicies(&Selection::new(
                Position::new(0, 3),
                Position::new(1, 15),
            )),
            Some(0..=0),
        );

        // Query selection will be absorbed:
        assert_eq!(
            storage.find_overlapping_indicies(&Selection::new(
                Position::new(0, 7),
                Position::new(0, 8),
            )),
            Some(0..=0),
        );

        // Overlap on the left side and then absorb on right:
        assert_eq!(
            storage.find_overlapping_indicies(&Selection::new(
                Position::new(1, 8),
                Position::new(16, 20),
            )),
            Some(0..=1)
        );
    }
}
