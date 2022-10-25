//! Operations to get selections using iterators.

use core::slice;
use std::cmp;

use super::SelectionStorage;
use crate::{Position, Selection};

/// Iterator over selections.
type SelectionsIter<'a> = slice::Iter<'a, Selection>;

impl SelectionStorage {
    /// Returns iterator over all selections in the storage.
    pub fn iter_all(&self) -> SelectionsIter {
        self.selections.iter()
    }

    /// Returns iterator over selections starting from `line`
    pub fn iter_from_line(&self, line: usize) -> SelectionsIter {
        let line_beginning = Position::new(line, 0);

        // Will point either on a selection that overlaps with the beginning of the line
        // or one after it, if any.
        let start_idx_result = self.selections.binary_search_by(|s| {
            if s.to >= line_beginning && s.from <= line_beginning {
                cmp::Ordering::Equal
            } else {
                s.to.cmp(&line_beginning)
            }
        });

        match start_idx_result {
            Ok(idx) | Err(idx) if idx != self.selections.len() => self.selections[idx..].iter(),
            _ => [].iter(),
        }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::{Position, Selection};

    #[test]
    fn test_iter_all() {
        let mut storage = SelectionStorage::new();
        storage.insert(Selection::new(Position::new(2, 0), Position::new(2, 10)));
        storage.insert(Selection::new(Position::new(2, 15), Position::new(2, 20)));

        let mut iter = storage.iter_all();
        let expected = [
            Selection {
                from: Position::new(0, 0),
                to: Position::new(0, 0),
                ..Default::default()
            },
            Selection {
                from: Position::new(2, 0),
                to: Position::new(2, 10),
                ..Default::default()
            },
            Selection {
                from: Position::new(2, 15),
                to: Position::new(2, 20),
                ..Default::default()
            },
        ];
        for right in expected.iter() {
            assert_eq!(iter.next(), Some(right));
        }
        assert!(iter.next().is_none());
    }

    #[test]
    fn test_iter_from_line() {
        let mut storage = SelectionStorage::new();
        storage.insert(Selection::new(Position::new(1, 10), Position::new(1, 15)));
        storage.insert(Selection::new(Position::new(1, 20), Position::new(2, 1)));
        storage.insert(Selection::new(Position::new(2, 15), Position::new(2, 20)));
        storage.insert(Selection::new(Position::new(3, 5), Position::new(3, 26)));

        // Line beginning overlap case:
        let mut iter = storage.iter_from_line(2);
        let expected = [
            Selection {
                from: Position::new(1, 20),
                to: Position::new(2, 1),
                ..Default::default()
            },
            Selection {
                from: Position::new(2, 15),
                to: Position::new(2, 20),
                ..Default::default()
            },
            Selection {
                from: Position::new(3, 5),
                to: Position::new(3, 26),
                ..Default::default()
            },
        ];
        for right in expected.iter() {
            assert_eq!(iter.next(), Some(right));
        }
        assert!(iter.next().is_none());

        // No line beginning overlap case:
        let mut iter = storage.iter_from_line(3);
        let expected = [Selection {
            from: Position::new(3, 5),
            to: Position::new(3, 26),
            ..Default::default()
        }];

        for right in expected.iter() {
            assert_eq!(iter.next(), Some(right));
        }
        assert!(iter.next().is_none());

        // Empty case:
        let mut iter = storage.iter_from_line(420);
        assert!(iter.next().is_none());
    }
}
