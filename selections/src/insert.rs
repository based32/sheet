use intrusive_collections::{Bound, RBTreeLink};

use super::{Position, SelectionStorage};
use crate::Selection;

impl SelectionStorage {
    /// Insert a selection bounded by `from` and `to` positions. If inserted
    /// selection overlaps with an existing one(s) all will be merged into one.
    pub fn insert(&mut self, from: Position, to: Position) {
        self.insert_internal(from, to, false);
    }

    /// Insert a selection bounded by `from` and `to` positions. If inserted
    /// selection overlaps with an existing one(s) all will replaced by inserted one.
    pub fn insert_replacing(&mut self, from: Position, to: Position) {
        self.insert_internal(from, to, true);
    }

    /// Insert a selection bounded by `from` and `to` positions. If inserted
    /// selection overlaps with an existing one(s) it either will be replaced
    /// (`replace == true`) or merged (`replace == false`).
    fn insert_internal(&mut self, from: Position, to: Position, replace: bool) {
        // Search for a possible collision
        let (mut new_from, mut new_to) = (None, None);
        // Check left neighbor
        let mut left_collision_cursor = self.tree.upper_bound_mut(Bound::Included(&from));
        if let Some(left) = left_collision_cursor.get() {
            if left.from <= from && left.to >= from {
                // Collision with left neighbor
                if !replace {
                    new_from = Some(left.from.clone());
                }
                left_collision_cursor.remove();
            }
        }
        // Check right neighbor
        let mut right_collision_cursor = self.tree.upper_bound_mut(Bound::Included(&to));
        if let Some(right) = right_collision_cursor.get() {
            if right.from <= to && right.to >= to {
                // Collision with right neighbor
                if !replace {
                    new_to = Some(right.to.clone());
                }
                right_collision_cursor.remove();
            }
        }
        if replace {
            self.tree.insert(Box::new(Selection {
                from,
                to,
                link: RBTreeLink::new(),
            }));
        } else {
            self.tree.insert(Box::new(Selection {
                from: new_from.unwrap_or(from),
                to: new_to.unwrap_or(to),
                link: RBTreeLink::new(),
            }));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insertion_no_collision() {
        let mut storage = SelectionStorage::new();

        storage.insert(Position::new(1, 3), Position::new(3, 7));

        let mut iter = storage.iter_all();
        let expected = [
            Selection {
                from: Position::new(0, 0),
                to: Position::new(0, 1),
                ..Default::default()
            },
            Selection {
                from: Position::new(1, 3),
                to: Position::new(3, 7),
                ..Default::default()
            },
        ];
        for right in expected.iter() {
            assert_eq!(iter.next(), Some(right));
        }
        assert!(iter.next().is_none());
    }

    #[test]
    fn test_insertion_collision_left_merge() {
        let mut storage = SelectionStorage::new();
        storage.insert(Position::new(1, 3), Position::new(3, 7));

        storage.insert(Position::new(1, 4), Position::new(4, 5));

        let mut iter = storage.iter_all();
        let expected = [
            Selection {
                from: Position::new(0, 0),
                to: Position::new(0, 1),
                ..Default::default()
            },
            Selection {
                from: Position::new(1, 3),
                to: Position::new(4, 5),
                ..Default::default()
            },
        ];
        for right in expected.iter() {
            assert_eq!(iter.next(), Some(right));
        }
        assert!(iter.next().is_none());
    }

    #[test]
    fn test_insertion_collision_left_replace() {
        let mut storage = SelectionStorage::new();
        storage.insert(Position::new(1, 3), Position::new(3, 7));

        storage.insert_replacing(Position::new(1, 4), Position::new(4, 5));

        let mut iter = storage.iter_all();
        let expected = [
            Selection {
                from: Position::new(0, 0),
                to: Position::new(0, 1),
                ..Default::default()
            },
            Selection {
                from: Position::new(1, 4),
                to: Position::new(4, 5),
                ..Default::default()
            },
        ];
        for right in expected.iter() {
            assert_eq!(iter.next(), Some(right));
        }
        assert!(iter.next().is_none());
    }

    #[test]
    fn test_insertion_collision_right_merge() {
        let mut storage = SelectionStorage::new();
        storage.insert(Position::new(1, 3), Position::new(3, 7));

        storage.insert(Position::new(0, 10), Position::new(1, 5));

        let mut iter = storage.iter_all();
        let expected = [
            Selection {
                from: Position::new(0, 0),
                to: Position::new(0, 1),
                ..Default::default()
            },
            Selection {
                from: Position::new(0, 10),
                to: Position::new(3, 7),
                ..Default::default()
            },
        ];
        for right in expected.iter() {
            assert_eq!(iter.next(), Some(right));
        }
        assert!(iter.next().is_none());
    }

    #[test]
    fn test_insertion_collision_right_replace() {
        let mut storage = SelectionStorage::new();
        storage.insert(Position::new(1, 3), Position::new(3, 7));

        storage.insert_replacing(Position::new(0, 10), Position::new(1, 5));

        let mut iter = storage.iter_all();
        let expected = [
            Selection {
                from: Position::new(0, 0),
                to: Position::new(0, 1),
                ..Default::default()
            },
            Selection {
                from: Position::new(0, 10),
                to: Position::new(1, 5),
                ..Default::default()
            },
        ];
        for right in expected.iter() {
            assert_eq!(iter.next(), Some(right));
        }
        assert!(iter.next().is_none());
    }

    #[test]
    fn test_insertion_collision_both_ends_merge() {
        let mut storage = SelectionStorage::new();
        storage.insert(Position::new(1, 3), Position::new(3, 7));
        storage.insert(Position::new(4, 3), Position::new(5, 7));

        storage.insert(Position::new(3, 5), Position::new(4, 7));

        let mut iter = storage.iter_all();
        let expected = [
            Selection {
                from: Position::new(0, 0),
                to: Position::new(0, 1),
                ..Default::default()
            },
            Selection {
                from: Position::new(1, 3),
                to: Position::new(5, 7),
                ..Default::default()
            },
        ];
        for right in expected.iter() {
            assert_eq!(iter.next(), Some(right));
        }
        assert!(iter.next().is_none());
    }

    #[test]
    fn test_insertion_collision_both_ends_replace() {
        let mut storage = SelectionStorage::new();
        storage.insert(Position::new(1, 3), Position::new(3, 7));
        storage.insert(Position::new(4, 3), Position::new(5, 7));

        storage.insert_replacing(Position::new(3, 5), Position::new(4, 7));

        let mut iter = storage.iter_all();
        let expected = [
            Selection {
                from: Position::new(0, 0),
                to: Position::new(0, 1),
                ..Default::default()
            },
            Selection {
                from: Position::new(3, 5),
                to: Position::new(4, 7),
                ..Default::default()
            },
        ];
        for right in expected.iter() {
            assert_eq!(iter.next(), Some(right));
        }
        assert!(iter.next().is_none());
    }
}
