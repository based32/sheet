use crate::{
    test_utils::{selections_test, TestLineLengths},
    Position,
};

#[rustfmt::skip]
mod left_single {
    use super::*;

    #[test]
    fn forward_same_line_no_extend_no_overlap() {
        selections_test! {
            [
                (0, 0) - (0, 0),
                (0, 1) - (1, 2),
            ],
            storage -> {
                let line_lengths = TestLineLengths::new();
                storage.move_left_single(line_lengths, &Position::new(0, 1), 1, false)
            },
            [
                Updated {
                    old: (0, 1) - (1, 2),
                    new: (1, 1) - (1, 1),
                }
            ],
            [
                (0, 0) - (0, 0),
                (1, 1) - (1, 1),
            ]
        };
    }

    #[test]
    fn forward_same_line_extend_no_overlap() {
        selections_test! {
            [
                (0, 0) - (0, 0),
                (0, 5) - (1, 20),
            ],
            storage -> {
                let line_lengths = TestLineLengths::new();
                storage.move_left_single(line_lengths, &Position::new(0, 5), 10, true)
            },
            [
                Updated {
                    old: (0, 5) - (1, 20),
                    new: (0, 5) - (1, 10),
                }
            ],
            [
                (0, 0) - (0, 0),
                (0, 5) - (1, 10),
            ]
        };
    }

    #[test]
    fn forward_same_line_step_over_no_overlap() {
        selections_test! {
            [
                (0, 0) - (0, 0),
                (0, 5) - (0, 10),
		(0, 15) - (0, 15),
            ],
            storage -> {
                let line_lengths = TestLineLengths::new();
                storage.move_left_single(line_lengths, &Position::new(0, 15), 12, false)
            },
            [
                Updated {
                    old: (0, 15) - (0, 15),
                    new: (0, 3) - (0, 3),
                }
            ],
            [
                (0, 0) - (0, 0),
		(0, 3) - (0, 3),
                (0, 5) - (0, 10),
            ]
        };
    }

    #[test]
    fn backward_same_line_no_extend_no_overlap() {
        selections_test! {
            [
                (0, 0) - (0, 0),
                (1, 2) - (0, 5),
            ],
            storage -> {
                let line_lengths = TestLineLengths::new();
                storage.move_left_single(line_lengths, &Position::new(0, 5), 1, false)
            },
            [
                Updated {
                    old: (1, 2) - (0, 5),
                    new: (0, 4) - (0, 4),
                }
            ],
            [
                (0, 0) - (0, 0),
		(0, 4) - (0, 4),
            ]
        };
    }
}
