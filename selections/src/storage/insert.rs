use super::{Position, SelectionStorage};
use crate::{Selection, SelectionDeltas, SelectionDirection};

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
    fn insert_internal(
        &mut self,
        mut from: Position,
        mut to: Position,
        replace: bool,
    ) -> SelectionDeltas {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::selections_test;

    #[test]
    fn insert_reversed() {
        selections_test! {
            [],
            storage -> { storage.insert(Position::new(3, 7), Position::new(1, 3)) },
            [
                Created((3, 7) - (1, 3)),
            ],
            [(3, 7) - (1, 3)]
        };
    }

    #[test]
    fn insert_reversed_merge() {
        selections_test! {
            [(0, 0) - (1, 4)],
            storage -> { storage.insert(Position::new(3, 7), Position::new(1, 3)) },
            [
                Deleted((0, 0) - (1, 4)),
                Created((3, 7) - (0, 0)),
            ],
            [(3, 7) - (0, 0)]
        };
    }

    #[test]
    fn no_collision() {
        selections_test! {
            [],
            storage -> { storage.insert(Position::new(1, 3), Position::new(3, 7)) },
            [
                Created((1, 3) - (3, 7)),
            ],
            [(1, 3) - (3, 7)]
        };
    }

    #[test]
    fn collision_left_merge() {
        selections_test! {
            [(1, 3) - (3, 7)],
            storage -> { storage.insert(Position::new(1, 4), Position::new(4, 5)) },
            [
                Deleted((1, 3) - (3, 7)),
                Created((1, 3) - (4, 5))
            ],
            [(1, 3) - (4, 5)]
        };
    }

    #[test]
    fn collision_left_replace() {
        selections_test! {
            [(1, 3) - (3, 7)],
            storage -> { storage.insert_replacing(Position::new(1, 4), Position::new(4, 5)) },
            [
                Deleted((1, 3) - (3, 7)),
                Created((1, 4) - (4, 5))
            ],
            [(1, 4) - (4, 5)]
        };
    }

    #[test]
    fn collision_left_merge_cornercase() {
        selections_test! {
            [(1, 3) - (3, 7)],
            storage -> {
                storage.insert(Position::new(3, 7), Position::new(4, 5))
            },
            [
                Deleted((1, 3) - (3, 7)),
                Created((1, 3) - (4, 5))
            ],
            [
                (1, 3) - (4, 5),
            ]
        };
    }

    #[test]
    fn collision_left_neighbors_cornercase() {
        selections_test! {
            [(1, 3) - (3, 7)],
            storage -> {
                storage.insert(Position::new(3, 8), Position::new(4, 5))
            },
            [
                Created((3, 8) - (4, 5))

            ],
            [
                (1, 3) - (3, 7),
                (3, 8) - (4, 5)
            ]
        };
    }

    #[test]
    fn collision_left_replace_cornercase() {
        selections_test! {
            [(1, 3) - (3, 7)],
            storage -> {
                storage.insert_replacing(Position::new(3, 7), Position::new(4, 5))
            },
            [
                Deleted((1, 3) - (3, 7)),
                Created((3, 7) - (4, 5)),
            ],
            [
                (3, 7) - (4, 5),
            ]
        };
    }

    #[test]
    fn collision_right_merge() {
        selections_test! {
            [(1, 3) - (3, 7)],
            storage -> { storage.insert(Position::new(0, 10), Position::new(1, 5)) },
            [
                Created((0, 10) - (3, 7)),
                Deleted((1, 3) - (3, 7)),
            ],
            [(0, 10) - (3, 7)]
        };
    }

    #[test]
    fn collision_right_replace() {
        selections_test! {
            [(1, 3) - (3, 7)],
            storage -> { storage.insert_replacing(Position::new(0, 10), Position::new(1, 5)) },
            [
                Created((0, 10) - (1, 5)),
                Deleted((1, 3) - (3, 7)),
            ],
            [(0, 10) - (1, 5)]
        };
    }

    #[test]
    fn collision_both_ends_merge() {
        selections_test! {
            [
                (1, 3) - (3, 7),
                (4, 3) - (5, 7),
            ],
            storage -> { storage.insert(Position::new(3, 5), Position::new(4, 7)) },
            [
                Deleted((1, 3) - (3, 7)),
                Created((1, 3) - (5, 7)),
                Deleted((4, 3) - (5, 7)),
            ],
            [(1, 3) - (5, 7)]
        };
    }

    #[test]
    fn collision_both_ends_replace() {
        selections_test! {
            [
                (1, 3) - (3, 7),
                (4, 3) - (5, 7),
            ],
            storage -> { storage.insert_replacing(Position::new(3, 5), Position::new(4, 7)) },
            [
                Deleted((1, 3) - (3, 7)),
                Created((3, 5) - (4, 7)),
                Deleted((4, 3) - (5, 7)),
            ],
            [(3, 5) - (4, 7)]
        };
    }

    #[test]
    fn absorbs_multiple_selections() {
        selections_test! {
            [
                (0, 3) - (0, 5),
                (1, 3) - (3, 7),
                (4, 3) - (5, 7),
                (6, 7) - (8, 9)
            ],
            storage -> { storage.insert(Position::new(0, 10), Position::new(5, 8)) },
            [
                Created((0, 10) - (5, 8)),
                Deleted((1, 3) - (3, 7)),
                Deleted((4, 3) - (5, 7)),
            ],
            [
                (0, 3) - (0, 5),
                (0, 10) - (5, 8),
                (6, 7) - (8, 9)
            ]
        };
    }

    #[test]
    fn absorbs_selections_and_handles_collisions_right_merge() {
        selections_test! {
            [
                (0, 3) - (0, 5),
                (1, 3) - (3, 7),
                (4, 3) - (5, 7),
                (6, 7) - (8, 9)
            ],
            storage -> { storage.insert(Position::new(0, 10), Position::new(6, 10)) },
            [
                Created((0, 10) - (8, 9)),
                Deleted((1, 3) - (3, 7)),
                Deleted((4, 3) - (5, 7)),
                Deleted((6, 7) - (8, 9))
            ],
            [
                (0, 3) - (0, 5),
                (0, 10) - (8, 9),
            ]
        };
    }

    #[test]
    fn absorbs_selections_and_handles_collisions_right_replace() {
        selections_test! {
            [
                (0, 3) - (0, 5),
                (1, 3) - (3, 7),
                (4, 3) - (5, 7),
                (6, 7) - (8, 9)
            ],
            storage -> { storage.insert_replacing(Position::new(0, 10), Position::new(6, 10)) },
            [
                Created((0, 10) - (6, 10)),
                Deleted((1, 3) - (3, 7)),
                Deleted((4, 3) - (5, 7)),
                Deleted((6, 7) - (8, 9))
            ],
            [
                (0, 3) - (0, 5),
                (0, 10) - (6, 10),
            ]
        };
    }

    #[test]
    fn absorbs_selections_and_handles_collisions_left_merge() {
        selections_test! {
            [
                (0, 3) - (0, 5),
                (1, 3) - (3, 7),
                (4, 3) - (5, 7),
                (6, 7) - (8, 9)
            ],
            storage -> { storage.insert(Position::new(0, 4), Position::new(6, 5)) },
            [
                Deleted((0, 3) - (0, 5)),
                Created((0, 3) - (6, 5)),
                Deleted((1, 3) - (3, 7)),
                Deleted((4, 3) - (5, 7)),
            ],
            [
                (0, 3) - (6, 5),
                (6, 7) - (8, 9),
            ]
        };
    }

    #[test]
    fn absorbs_selections_and_handles_collisions_left_replace() {
        selections_test! {
            [
                (0, 3) - (0, 5),
                (1, 3) - (3, 7),
                (4, 3) - (5, 7),
                (6, 7) - (8, 9)
            ],
            storage -> { storage.insert_replacing(Position::new(0, 4), Position::new(6, 5)) },
            [
                Deleted((0, 3) - (0, 5)),
                Created((0, 4) - (6, 5)),
                Deleted((1, 3) - (3, 7)),
                Deleted((4, 3) - (5, 7)),
            ],
            [
                (0, 4) - (6, 5),
                (6, 7) - (8, 9),
            ]
        };
    }
}
