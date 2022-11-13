//! Test for movement API of [SelectionStorage].
//! As isolated selection movements are implemented and tested separately for
//! each case, tests in this module will assume that individual selection is
//! moved correctly and will check only how it affects [SelectionStorage] state
//! rather than forward/backward/multiline combinations.

use crate::{
    test_utils::{selections_test, TestLineLengths},
    Position,
};

mod left_single {
    // It's the first time when all [SelectionDelta] varians are checked, some
    // cominations of selections' tests will be used to ensure [selections_test]
    // macro works fine. Other submodules not necessarily cover it.

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
                storage.move_left_single(&line_lengths, &Position::new(0, 1), 1, false)
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
                storage.move_left_single(&line_lengths, &Position::new(0, 5), 10, true)
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
    fn backward_same_line_no_extend_no_overlap() {
        selections_test! {
            [
                (0, 0) - (0, 0),
                (1, 2) - (0, 5),
            ],
            storage -> {
                let line_lengths = TestLineLengths::new();
                storage.move_left_single(&line_lengths, &Position::new(0, 5), 1, false)
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

    #[test]
    fn backward_same_line_extend_no_overlap() {
        selections_test! {
            [
                (0, 0) - (0, 0),
                (1, 2) - (0, 5),
            ],
            storage -> {
                let line_lengths = TestLineLengths::new();
                storage.move_left_single(&line_lengths, &Position::new(0, 5), 1, true)
            },
            [
                Updated {
                    old: (1, 2) - (0, 5),
                    new: (1, 2) - (0, 4),
                }
            ],
            [
                (0, 0) - (0, 0),
                (1, 2) - (0, 4),
            ]
        };
    }

    #[test]
    fn forward_same_line_extend_reverse_no_overlap() {
        selections_test! {
            [
                (0, 0) - (0, 0),
                (0, 5) - (0, 10),
            ],
            storage -> {
                let line_lengths = TestLineLengths::new();
                storage.move_left_single(&line_lengths, &Position::new(0, 5), 6, true)
            },
            [
                Updated {
                    old: (0, 5) - (0, 10),
                    new: (0, 5) - (0, 4),
                }
            ],
            [
                (0, 0) - (0, 0),
                (0, 5) - (0, 4),
            ]
        };
    }

    #[test]
    fn step_over_no_overlap() {
        selections_test! {
            [
                (0, 0) - (0, 0),
                (0, 5) - (0, 10),
                (0, 15) - (0, 15),
            ],
            storage -> {
                let line_lengths = TestLineLengths::new();
                storage.move_left_single(&line_lengths, &Position::new(0, 15), 12, false)
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
    fn overlap_one_edge_no_extend() {
        selections_test! {
            [
                (0, 0) - (0, 5),
                (0, 10) - (0, 15),
            ],
            storage -> {
                let mut line_lengths = TestLineLengths::new();
                line_lengths.set(0, 10);
                storage.move_left_single(&line_lengths, &Position::new(0, 10), 12, false)
            },
            [
                Deleted((0, 0) - (0, 5)),
                Updated {
                    old: (0, 10) - (0, 15),
                    new: (0, 3) - (0, 3),
                }
            ],
            [
                (0, 3) - (0, 3),
            ]
        };
    }

    #[test]
    fn overlap_one_edge_extend() {
        selections_test! {
            [
                (0, 0) - (0, 5),
                (0, 10) - (1, 2),
            ],
            storage -> {
                let mut line_lengths = TestLineLengths::new();
                line_lengths.set(0, 10);
                storage.move_left_single(&line_lengths, &Position::new(0, 10), 10, true)
            },
            [
                Deleted((0, 0) - (0, 5)),
                Updated {
                    old: (0, 10) - (1, 2),
                    new: (0, 10) - (0, 0),
                }
            ],
            [
                (0, 10) - (0, 0),
            ]
        };
    }

    #[test]
    fn overlap_many() {
        selections_test! {
            [
                (0, 0) - (0, 0),
                (0, 3) - (0, 8),
                (0, 15) - (1, 2),
                (3, 7) - (1, 3),
                (4, 20) - (13, 37),
            ],
            storage -> {
                let mut line_lengths = TestLineLengths::new();
                line_lengths.set(0, 10);
                line_lengths.set(1, 10);
                line_lengths.set(2, 10);
                line_lengths.set(3, 10);
                storage.move_left_single(&line_lengths, &Position::new(1, 3), 69, true)
            },
            [
                Deleted((0, 0) - (0, 0)),
                Deleted((0, 3) - (0, 8)),
                Deleted((0, 15) - (1, 2)),
                Updated {
                    old: (3, 7) - (1, 3),
                    new: (3, 7) - (0, 0),
                },
            ],
            [
                (3, 7) - (0, 0),
                (4, 20) - (13, 37),
            ]
        }
    }
}

mod right_single {
    use super::*;

    #[test]
    fn no_changes_in_order() {
        selections_test! {
            [
                (0, 0) - (0, 0),
                (0, 10) - (1, 2),
            ],
            storage -> {
                let mut line_lengths = TestLineLengths::new();
                line_lengths.set(0, 69);
                storage.move_right_single(&line_lengths, &Position::new(0, 0), 5, false)
            },
            [
                Updated {
                    old: (0, 0) - (0, 0),
                    new: (0, 5) - (0, 5),
                }
            ],
            [
                (0, 5) - (0, 5),
                (0, 10) - (1, 2),
            ]
        };
    }

    #[test]
    fn no_changes_in_order_last() {
        selections_test! {
            [
                (0, 0) - (0, 0),
                (0, 1) - (1, 2),
            ],
            storage -> {
                let mut line_lengths = TestLineLengths::new();
                line_lengths.set(0, 69);
                line_lengths.set(1, 69);
                storage.move_right_single(&line_lengths, &Position::new(0, 1), 1, true)
            },
            [
                Updated {
                    old: (0, 1) - (1, 2),
                    new: (0, 1) - (1, 3),
                }
            ],
            [
                (0, 0) - (0, 0),
                (0, 1) - (1, 3),
            ]
        };
    }

    #[test]
    fn step_over_no_overlap() {
        selections_test! {
            [
                (0, 0) - (0, 0),
                (0, 5) - (0, 10),
                (0, 15) - (0, 15),
            ],
            storage -> {
                let mut line_lengths = TestLineLengths::new();
                line_lengths.set(0, 30);

                storage.move_right_single(&line_lengths, &Position::new(0, 0), 12, false)
            },
            [
                Updated {
                    old: (0, 0) - (0, 0),
                    new: (0, 12) - (0, 12),
                }
            ],
            [
                (0, 5) - (0, 10),
                (0, 12) - (0, 12),
                (0, 15) - (0, 15),
            ]
        };
    }

    #[test]
    fn step_over_no_overlap_last() {
        selections_test! {
            [
                (0, 0) - (0, 0),
                (0, 5) - (0, 10),
                (0, 15) - (0, 15),
            ],
            storage -> {
                let mut line_lengths = TestLineLengths::new();
                line_lengths.set(0, 30);

                storage.move_right_single(&line_lengths, &Position::new(0, 5), 12, false)
            },
            [
                Updated {
                    old: (0, 5) - (0, 10),
                    new: (0, 22) - (0, 22),
                }
            ],
            [
                (0, 0) - (0, 0),
                (0, 15) - (0, 15),
                (0, 22) - (0, 22),
            ]
        };
    }

    #[test]
    fn overlap_one_edge_no_extend() {
        selections_test! {
            [
                (0, 0) - (0, 5),
                (0, 10) - (0, 15),
                (1, 3) - (3, 7),
            ],
            storage -> {
                let mut line_lengths = TestLineLengths::new();
                line_lengths.set(0, 20);
                line_lengths.set(1, 20);
                line_lengths.set(2, 10);
                line_lengths.set(3, 10);
                storage.move_right_single(&line_lengths, &Position::new(0, 0), 8, false)
            },
            [
                Updated {
                    old: (0, 0) - (0, 5),
                    new: (0, 13) - (0, 13),
                },
                Deleted((0, 10) - (0, 15)),
            ],
            [
                (0, 13) - (0, 13),
                (1, 3) - (3, 7),
            ]
        };
    }

    #[test]
    fn overlap_one_edge_extend() {
        selections_test! {
            [
                (0, 0) - (0, 5),
                (0, 10) - (0, 15),
                (1, 3) - (3, 7),
            ],
            storage -> {
                let mut line_lengths = TestLineLengths::new();
                line_lengths.set(0, 20);
                line_lengths.set(1, 20);
                line_lengths.set(2, 10);
                line_lengths.set(3, 10);
                storage.move_right_single(&line_lengths, &Position::new(0, 10), 9, true)
            },
            [
                Updated {
                    old: (0, 10) - (0, 15),
                    new: (0, 10) - (3,7),
                },
                Deleted((1, 3) - (3, 7)),
            ],
            [
                (0, 0) - (0, 5),
                (0, 10) - (3, 7),
            ]
        };
    }

    #[test]
    fn overlap_many() {
        selections_test! {
            [
                (0, 0) - (0, 0),
                (0, 8) - (0, 3),
                (0, 15) - (1, 2),
                (3, 7) - (1, 3),
                (4, 20) - (5, 37),
            ],
            storage -> {
                let mut line_lengths = TestLineLengths::new();
                line_lengths.set(0, 20);
                line_lengths.set(1, 20);
                line_lengths.set(2, 20);
                line_lengths.set(3, 20);
                line_lengths.set(4, 20);
                line_lengths.set(5, 40);
                storage.move_right_single(&line_lengths, &Position::new(0, 3), 420, true)
            },
            [
                Updated {
                    old: (0, 8) - (0, 3),
                    new: (0, 8) - (5, 40),
                },
                Deleted((0, 15) - (1, 2)),
                Deleted((3, 7) - (1, 3)),
                Deleted((4, 20) - (5, 37)),
            ],
            [
                (0, 0) - (0, 0),
                (0, 8) - (5, 40),
            ]
        }
    }
}

mod up_single {
    use super::*;

    #[test]
    fn no_changes_in_order() {
        selections_test! {
            [
                (0, 0) - (0, 0),
                (2, 10) - (3, 2),
                (5, 5) - (5, 6),
            ],
            storage -> {
                let mut line_lengths = TestLineLengths::new();
                line_lengths.set(0, 69);
                line_lengths.set(1, 69);
                line_lengths.set(2, 69);
                line_lengths.set(3, 69);
                storage.move_up_single(&line_lengths, &Position::new(2, 10), 1, false)
            },
            [
                Updated {
                    old: (2, 10) - (3, 2),
                    new: (2, 2) - (2, 2),
                }
            ],
            [
                (0, 0) - (0, 0),
                (2, 2) - (2, 2),
                (5, 5) - (5, 6),
            ]
        };
    }

    #[test]
    fn no_changes_in_order_last() {
        selections_test! {
            [
                (0, 0) - (0, 0),
                (2, 1) - (2, 0),
            ],
            storage -> {
                let mut line_lengths = TestLineLengths::new();
                line_lengths.set(0, 69);
                line_lengths.set(1, 69);
                line_lengths.set(2, 69);
                storage.move_up_single(&line_lengths, &Position::new(2, 0), 1, true)
            },
            [
                Updated {
                    old: (2, 1) - (2, 0),
                    new: (2, 1) - (1, 0),
                }
            ],
            [
                (0, 0) - (0, 0),
                (2, 1) - (1, 0),
            ]
        };
    }

    #[test]
    fn step_over_no_overlap() {
        selections_test! {
            [
                (0, 0) - (0, 0),
                (1, 5) - (1, 10),
                (2, 15) - (2, 15),
            ],
            storage -> {
                let mut line_lengths = TestLineLengths::new();
                line_lengths.set(0, 30);
                line_lengths.set(1, 30);
                line_lengths.set(2, 30);

                storage.move_up_single(&line_lengths, &Position::new(2, 15), 2, false)
            },
            [
                Updated {
                    old: (2, 15) - (2, 15),
                    new: (0, 15) - (0, 15),
                }
            ],
            [
                (0, 0) - (0, 0),
                (0, 15) - (0, 15),
                (1, 5) - (1, 10),
            ]
        };
    }

    #[test]
    fn step_over_no_overlap_first() {
        selections_test! {
            [
                (1, 5) - (1, 10),
                (2, 15) - (2, 15),
            ],
            storage -> {
                let mut line_lengths = TestLineLengths::new();
                line_lengths.set(0, 30);
                line_lengths.set(1, 30);
                line_lengths.set(2, 30);

                storage.move_up_single(&line_lengths, &Position::new(2, 15), 12, false)
            },
            [
                Updated {
                    old: (2, 15) - (2, 15),
                    new: (0, 15) - (0, 15),
                }
            ],
            [
                (0, 15) - (0, 15),
                (1, 5) - (1, 10),
            ]
        };
    }

    #[test]
    fn overlap_one_edge_no_extend() {
        selections_test! {
            [
                (0, 0) - (0, 5),
                (0, 10) - (0, 15),
                (3, 7) - (1, 13),
            ],
            storage -> {
                let mut line_lengths = TestLineLengths::new();
                line_lengths.set(0, 20);
                line_lengths.set(1, 20);
                line_lengths.set(2, 10);
                line_lengths.set(3, 10);
                storage.move_up_single(&line_lengths, &Position::new(1, 13), 1, false)
            },
            [
                Deleted((0, 10) - (0, 15)),
                Updated {
                    old: (3, 7) - (1, 13),
                    new: (0, 13) - (0, 13),
                },
            ],
            [
                (0, 0) - (0, 5),
                (0, 13) - (0, 13),
            ]
        };
    }

    #[test]
    fn overlap_one_edge_extend() {
        selections_test! {
            [
                (0, 0) - (0, 5),
                (0, 10) - (0, 15),
                (1, 3) - (3, 12),
            ],
            storage -> {
                let mut line_lengths = TestLineLengths::new();
                line_lengths.set(0, 20);
                line_lengths.set(1, 20);
                line_lengths.set(2, 10);
                line_lengths.set(3, 10);
                storage.move_up_single(&line_lengths, &Position::new(1, 3), 3, true)
            },
            [
                Deleted((0, 10) - (0, 15)),
                Updated {
                    old: (1, 3) - (3, 12),
                    new: (1, 3) - (0, 10),
                },
            ],
            [
                (0, 0) - (0, 5),
                (1, 3) - (0, 10),
            ]
        };
    }

    #[test]
    fn overlap_many() {
        selections_test! {
            [
                (0, 5) - (0, 7),
                (0, 8) - (0, 12),
                (0, 15) - (1, 2),
                (3, 7) - (1, 6),
                (4, 20) - (5, 37),
            ],
            storage -> {
                let mut line_lengths = TestLineLengths::new();
                line_lengths.set(0, 20);
                line_lengths.set(1, 20);
                line_lengths.set(2, 20);
                line_lengths.set(3, 20);
                line_lengths.set(4, 20);
                line_lengths.set(5, 40);
                storage.move_up_single(&line_lengths, &Position::new(1, 6), 420, true)
            },
            [
                Deleted((0, 5) - (0, 7)),
                Deleted((0, 8) - (0, 12)),
                Deleted((0, 15) - (1, 2)),
                Updated {
                    old: (3, 7) - (1, 6),
                    new: (3, 7) - (0, 5),
                },

            ],
            [
                (3, 7) - (0, 5),
                (4, 20) - (5, 37),
            ]
        }
    }

    #[test]
    fn overlap_inherits_sticky_column() {
        selections_test! {
            [
                (2, 20) - (1, 5) sticky 10,
                (3, 40) - (3, 15),
            ],
            storage -> {
                let mut line_lengths = TestLineLengths::new();
                line_lengths.set(0, 20);
                line_lengths.set(1, 5);
                line_lengths.set(2, 30);
                line_lengths.set(3, 50);

                storage.move_up_single(&line_lengths, &Position::new(3, 15), 1, true);
                storage.move_up_single(&line_lengths, &Position::new(1, 5), 1, true)
            },
            [
                Updated {
                    old: (3, 40) - (1, 5) sticky 10,
                    new: (3, 40) - (0, 10),
                }
            ],
            [
                (3, 40) - (0, 10),
            ]
        }
    }
}

mod down_single {
    use super::*;

    #[test]
    fn no_changes_in_order() {
        selections_test! {
            [
                (0, 0) - (0, 0),
                (2, 10) - (3, 2),
                (5, 5) - (5, 6),
            ],
            storage -> {
                let mut line_lengths = TestLineLengths::new();
                line_lengths.set(0, 69);
                line_lengths.set(1, 69);
                line_lengths.set(2, 69);
                line_lengths.set(3, 69);
                line_lengths.set(4, 69);
                line_lengths.set(5, 69);
                storage.move_down_single(&line_lengths, &Position::new(2, 10), 1, false)
            },
            [
                Updated {
                    old: (2, 10) - (3, 2),
                    new: (4, 2) - (4, 2),
                }
            ],
            [
                (0, 0) - (0, 0),
                (4, 2) - (4, 2),
                (5, 5) - (5, 6),
            ]
        };
    }

    #[test]
    fn no_changes_in_order_last() {
        selections_test! {
            [
                (0, 0) - (0, 0),
                (2, 1) - (2, 0),
            ],
            storage -> {
                let mut line_lengths = TestLineLengths::new();
                line_lengths.set(0, 69);
                line_lengths.set(1, 69);
                line_lengths.set(2, 69);
                line_lengths.set(3, 69);
                storage.move_down_single(&line_lengths, &Position::new(2, 0), 1, true)
            },
            [
                Updated {
                    old: (2, 1) - (2, 0),
                    new: (2, 1) - (3, 0),
                }
            ],
            [
                (0, 0) - (0, 0),
                (2, 1) - (3, 0),
            ]
        };
    }

    #[test]
    fn step_over_no_overlap() {
        selections_test! {
            [
                (0, 0) - (0, 0),
                (1, 5) - (1, 10),
                (2, 15) - (2, 15),
            ],
            storage -> {
                let mut line_lengths = TestLineLengths::new();
                line_lengths.set(0, 30);
                line_lengths.set(1, 30);
                line_lengths.set(2, 30);
                line_lengths.set(3, 30);
                storage.move_down_single(&line_lengths, &Position::new(1, 5), 2, false)
            },
            [
                Updated {
                    old: (1, 5) - (1, 10),
                    new: (3, 10) - (3, 10),
                }
            ],
            [
                (0, 0) - (0, 0),
                (2, 15) - (2, 15),
                (3, 10) - (3, 10),
            ]
        };
    }

    #[test]
    fn step_over_no_overlap_last() {
        selections_test! {
            [
                (1, 5) - (1, 10),
                (2, 15) - (2, 15),
            ],
            storage -> {
                let mut line_lengths = TestLineLengths::new();
                line_lengths.set(0, 30);
                line_lengths.set(1, 30);
                line_lengths.set(2, 30);
                line_lengths.set(3, 30);
                line_lengths.set(4, 30);

                storage.move_down_single(&line_lengths, &Position::new(1, 5), 12, false)
            },
            [
                Updated {
                    old: (1, 5) - (1, 10),
                    new: (4, 10) - (4, 10),
                }
            ],
            [
                (2, 15) - (2, 15),
                (4, 10) - (4, 10),
            ]
        };
    }

    #[test]
    fn overlap_one_edge_no_extend() {
        selections_test! {
            [
                (0, 0) - (0, 5),
                (0, 10) - (0, 15),
                (3, 7) - (1, 13),
            ],
            storage -> {
                let mut line_lengths = TestLineLengths::new();
                line_lengths.set(0, 20);
                line_lengths.set(1, 20);
                line_lengths.set(2, 10);
                line_lengths.set(3, 10);
                storage.move_down_single(&line_lengths, &Position::new(0, 10), 1, false)
            },
            [
                Updated {
                    old: (0, 10) - (0, 15),
                    new: (1, 15) - (1, 15),
                },
                Deleted((3, 7) - (1, 13)),
            ],
            [
                (0, 0) - (0, 5),
                (1, 15) - (1, 15),
            ]
        };
    }

    #[test]
    fn overlap_one_edge_extend() {
        selections_test! {
            [
                (0, 0) - (0, 5),
                (0, 10) - (0, 15),
                (2, 3) - (3, 12),
            ],
            storage -> {
                let mut line_lengths = TestLineLengths::new();
                line_lengths.set(0, 20);
                line_lengths.set(1, 20);
                line_lengths.set(2, 10);
                line_lengths.set(3, 10);
                storage.move_down_single(&line_lengths, &Position::new(0, 10), 2, true)
            },
            [
                Updated {
                    old: (0, 10) - (0, 15),
                    new: (0, 10) - (3, 12),
                },
                Deleted((2, 3) - (3, 12)),
            ],
            [
                (0, 0) - (0, 5),
                (0, 10) - (3, 12),
            ]
        };
    }

    #[test]
    fn overlap_many() {
        selections_test! {
            [
                (0, 5) - (0, 7),
                (0, 8) - (0, 12),
                (0, 15) - (1, 2),
                (3, 7) - (1, 6),
                (4, 20) - (5, 37),
            ],
            storage -> {
                let mut line_lengths = TestLineLengths::new();
                line_lengths.set(0, 20);
                line_lengths.set(1, 20);
                line_lengths.set(2, 20);
                line_lengths.set(3, 20);
                line_lengths.set(4, 20);
                line_lengths.set(5, 40);
                line_lengths.set(6, 10);
                storage.move_down_single(&line_lengths, &Position::new(0, 8), 420, true)
            },
            [
                Updated {
                    old: (0, 8) - (0, 12),
                    new: (0, 8) - (6, 10) sticky 12,
                },
                Deleted((0, 15) - (1, 2)),
                Deleted((3, 7) - (1, 6)),
                Deleted((4, 20) - (5, 37)),
            ],
            [
                (0, 5) - (0, 7),
                (0, 8) - (6, 10) sticky 12,
            ]
        }
    }

    #[test]
    fn overlap_inherits_sticky_column() {
        selections_test! {
            [
                (2, 20) - (2, 10),
                (3, 8) - (4, 20) sticky 25,
            ],
            storage -> {
                let mut line_lengths = TestLineLengths::new();
                line_lengths.set(0, 20);
                line_lengths.set(1, 5);
                line_lengths.set(2, 30);
                line_lengths.set(3, 50);
                line_lengths.set(4, 20);
                line_lengths.set(5, 30);

                storage.move_down_single(&line_lengths, &Position::new(2, 10), 1, true);
                storage.move_down_single(&line_lengths, &Position::new(2, 20), 1, true)
            },
            [
                Updated {
                    old: (2, 20) - (4, 20) sticky 25,
                    new: (2, 20) - (5, 25),
                }
            ],
            [
                (2, 20) - (5, 25),
            ]
        }
    }
}
