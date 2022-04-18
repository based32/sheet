use intrusive_collections::{rbtree::Cursor, Bound};

use super::{SelectionAdapter, SelectionStorage};
use crate::{Position, Selection};

/// Iterator over selections.
pub struct SelectionsIter<'a> {
    cursor: Cursor<'a, SelectionAdapter>,
    done: bool,
}

impl<'a> Iterator for SelectionsIter<'a> {
    type Item = &'a Selection;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            None
        } else {
            let value = self.cursor.get();
            self.cursor.move_next();
            if value.is_none() {
                self.done = true;
            }
            value
        }
    }
}

impl SelectionStorage {
    /// Returns iterator over all selections in the storage.
    pub fn iter_all(&self) -> SelectionsIter {
        SelectionsIter {
            cursor: self.tree.front(),
            done: false,
        }
    }

    /// Returns iterator over selections starting from `line`
    pub fn iter_from_line(&self, line: usize) -> SelectionsIter {
        let mut cursor = self
            .tree
            .upper_bound(Bound::Included(&Position::new(line, 0)));
        if let Some(selection) = cursor.get() {
            if selection.to.line < line {
                // Selection ends before lower bound so it should be skipped
                cursor.move_next();
            }
        }
        SelectionsIter {
            cursor,
            done: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Selection;

    #[test]
    fn test_iter_all() {
        let mut storage = SelectionStorage::new();
        storage.insert(Position::new(2, 0), Position::new(2, 10));
        storage.insert(Position::new(2, 15), Position::new(2, 20));

        let mut iter = storage.iter_all();
        let expected = [
            Selection {
                from: Position::new(0, 0),
                to: Position::new(0, 1),
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
        assert!(iter.done);
    }

    #[test]
    fn test_iter_from_line() {
        let mut storage = SelectionStorage::new();
        storage.insert(Position::new(1, 10), Position::new(1, 15));
        storage.insert(Position::new(1, 20), Position::new(2, 1));
        storage.insert(Position::new(2, 15), Position::new(2, 20));
        storage.insert(Position::new(3, 5), Position::new(3, 26));

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
        assert!(iter.done);
    }
}
