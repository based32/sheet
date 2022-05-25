use intrusive_collections::{Bound, RBTreeLink};

use super::{Position, SelectionStorage};
use crate::{Selection, SelectionDelta, SelectionDeltas};

impl SelectionStorage {
    /// Insert a selection bounded by `from` and `to` positions. If inserted
    /// selection overlaps with an existing one(s) all will be merged into one.
    pub fn insert(&mut self, from: Position, to: Position) -> SelectionDeltas {
        self.insert_internal(from, to, false)
    }

    /// Insert a selection bounded by `from` and `to` positions. If inserted
    /// selection overlaps with an existing one(s) all will replaced by inserted
    /// one.
    pub fn insert_replacing(&mut self, from: Position, to: Position) -> SelectionDeltas {
        self.insert_internal(from, to, true)
    }

    /// Insert a selection bounded by `from` and `to` positions. If inserted
    /// selection overlaps with an existing one(s) it either will be replaced
    /// (`replace == true`) or merged (`replace == false`).
    fn insert_internal(&mut self, from: Position, to: Position, replace: bool) -> SelectionDeltas {
        // Search for a possible collision
        let (mut new_from, mut new_to) = (None, None);

        let mut deltas = SelectionDeltas::new();

        // Check left neighbor
        let mut left_collision_cursor = self.tree.upper_bound_mut(Bound::Included(&from));
        if let Some(left) = left_collision_cursor.get() {
            if left.from <= from && left.to >= from {
                // Collision with left neighbor
                if !replace {
                    new_from = Some(left.from.clone());
                }
                let deleted = left_collision_cursor.remove().expect("not a null object");
                deltas.add_deleted(deleted);
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
                let deleted = right_collision_cursor.remove().expect("not a null object");
                deltas.add_deleted(deleted);

                // After removal it starts to point to the next item, but for absorbed
                // selections we want to go backwards.
                right_collision_cursor.move_prev();
            }
        }

        // Check absorbed selections
        while let Some(selection) = right_collision_cursor.get() {
            if selection.from >= from && selection.to <= to {
                let deleted = right_collision_cursor.remove().expect("not a null object");
                deltas.add_deleted(deleted);
                right_collision_cursor.move_prev();
            } else {
                break;
            }
        }

        let created = if replace {
            self.tree.insert(Box::new(Selection {
                from,
                to,
                link: RBTreeLink::new(),
            }))
        } else {
            self.tree.insert(Box::new(Selection {
                from: new_from.unwrap_or(from),
                to: new_to.unwrap_or(to),
                link: RBTreeLink::new(),
            }))
        }
        .into_ref()
        .expect("not a null object");
        deltas.add_created(created);

        deltas
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::selections_test;

    #[test]
    fn test_no_collision() {
        selections_test! {
            [],
            storage -> { storage.insert(Position::new(1, 3), Position::new(3, 7)) },
            [
                Created((1, 3) - (3, 7))
            ],
            [(1, 3) - (3, 7)]
        };
    }

    // #[test]
    // fn test_collision_left_merge() {
    //     selections_test! {
    //         [(1, 3) - (3, 7)],
    //         storage -> { storage.insert(Position::new(1, 4), Position::new(4,
    // 5)); },         [(1, 3) - (4, 5)]
    //     };
    // }

    // #[test]
    // fn test_collision_left_replace() {
    //     selections_test! {
    //         [(1, 3) - (3, 7)],
    //         storage -> { storage.insert_replacing(Position::new(1, 4),
    // Position::new(4, 5)); },         [(1, 4) - (4, 5)]
    //     };
    // }

    // #[test]
    // fn test_collision_left_merge_cornercase() {
    //     selections_test! {
    //         [(1, 3) - (3, 7)],
    //         storage -> {
    //             storage.insert(Position::new(3, 7), Position::new(4, 5));
    //             storage.insert(Position::new(4, 6), Position::new(4, 8));
    //         },
    //         [
    //             (1, 3) - (4, 5),
    //             (4, 6) - (4, 8)
    //         ]
    //     };
    // }

    // #[test]
    // fn test_collision_left_replace_cornercase() {
    //     selections_test! {
    //         [(1, 3) - (3, 7)],
    //         storage -> {
    //             storage.insert_replacing(Position::new(3, 7),
    // Position::new(4, 5));
    // storage.insert_replacing(Position::new(4, 6), Position::new(4, 8));
    //         },
    //         [
    //             (3, 7) - (4, 5),
    //             (4, 6) - (4, 8)
    //         ]
    //     };
    // }

    // #[test]
    // fn test_collision_right_merge() {
    //     selections_test! {
    //         [(1, 3) - (3, 7)],
    //         storage -> { storage.insert(Position::new(0, 10),
    // Position::new(1, 5)); },         [(0, 10) - (3, 7)]
    //     };
    // }

    // #[test]
    // fn test_collision_right_replace() {
    //     selections_test! {
    //         [(1, 3) - (3, 7)],
    //         storage -> { storage.insert_replacing(Position::new(0, 10),
    // Position::new(1, 5)); },         [(0, 10) - (1, 5)]
    //     };
    // }

    // #[test]
    // fn test_collision_both_ends_merge() {
    //     selections_test! {
    //         [
    //             (1, 3) - (3, 7),
    //             (4, 3) - (5, 7),
    //         ],
    //         storage -> { storage.insert(Position::new(3, 5), Position::new(4,
    // 7)); },         [(1, 3) - (5, 7)]
    //     };
    // }

    // #[test]
    // fn test_collision_both_ends_replace() {
    //     selections_test! {
    //         [
    //             (1, 3) - (3, 7),
    //             (4, 3) - (5, 7),
    //         ],
    //         storage -> { storage.insert_replacing(Position::new(3, 5),
    // Position::new(4, 7)); },         [(3, 5) - (4, 7),]
    //     };
    // }

    // #[test]
    // fn test_absorbs_multiple_selections() {
    //     selections_test! {
    //         [
    //             (0, 3) - (0, 5),
    //             (1, 3) - (3, 7),
    //             (4, 3) - (5, 7),
    //             (6, 7) - (8, 9)
    //         ],
    //         storage -> { storage.insert(Position::new(0, 10),
    // Position::new(5, 8)); },         [
    //             (0, 3) - (0, 5),
    //             (0, 10) - (5, 8),
    //             (6, 7) - (8, 9)
    //         ]
    //     };
    // }

    // #[test]
    // fn test_absorbs_selections_and_handles_collisions_right_merge() {
    //     selections_test! {
    //         [
    //             (0, 3) - (0, 5),
    //             (1, 3) - (3, 7),
    //             (4, 3) - (5, 7),
    //             (6, 7) - (8, 9)
    //         ],
    //         storage -> { storage.insert(Position::new(0, 10),
    // Position::new(6, 10)); },         [
    //             (0, 3) - (0, 5),
    //             (0, 10) - (8, 9),
    //         ]
    //     };
    // }

    // #[test]
    // fn test_absorbs_selections_and_handles_collisions_right_replace() {
    //     selections_test! {
    //         [
    //             (0, 3) - (0, 5),
    //             (1, 3) - (3, 7),
    //             (4, 3) - (5, 7),
    //             (6, 7) - (8, 9)
    //         ],
    //         storage -> { storage.insert_replacing(Position::new(0, 10),
    // Position::new(6, 10)); },         [
    //             (0, 3) - (0, 5),
    //             (0, 10) - (6, 10),
    //         ]
    //     };
    // }

    // #[test]
    // fn test_absorbs_selections_and_handles_collisions_left_merge() {
    //     selections_test! {
    //         [
    //             (0, 3) - (0, 5),
    //             (1, 3) - (3, 7),
    //             (4, 3) - (5, 7),
    //             (6, 7) - (8, 9)
    //         ],
    //         storage -> { storage.insert(Position::new(0, 4), Position::new(6,
    // 5)); },         [
    //             (0, 3) - (6, 5),
    //             (6, 7) - (8, 9),
    //         ]
    //     };
    // }

    // #[test]
    // fn test_absorbs_selections_and_handles_collisions_left_replace() {
    //     selections_test! {
    //         [
    //             (0, 3) - (0, 5),
    //             (1, 3) - (3, 7),
    //             (4, 3) - (5, 7),
    //             (6, 7) - (8, 9)
    //         ],
    //         storage -> { storage.insert_replacing(Position::new(0, 4),
    // Position::new(6, 5)); },         [
    //             (0, 4) - (6, 5),
    //             (6, 7) - (8, 9),
    //         ]
    //     };
    // }
}
