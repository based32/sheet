//! Test for movement API of [SelectionStorage].
//! As isolated selection movements are implemented and tested separately for
//! each case, tests in this module will assume that individual selection is
//! moved correctly and will check only how it affects [SelectionStorage] state
//! rather than forward/backward/multiline combinations.

use crate::{
    test_utils::{selections_test, TestLineLengths},
    Position,
};

#[rustfmt::skip]
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

    #[test]
    fn backward_same_line_extend_no_overlap() {
        selections_test! {
            [
                (0, 0) - (0, 0),
                (1, 2) - (0, 5),
            ],
            storage -> {
                let line_lengths = TestLineLengths::new();
                storage.move_left_single(line_lengths, &Position::new(0, 5), 1, true)
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
                storage.move_left_single(line_lengths, &Position::new(0, 5), 6, true)
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
    fn overlap_one_edge_no_extend() {
        selections_test! {
            [
                (0, 0) - (0, 5),
                (0, 10) - (0, 15),
            ],
            storage -> {
                let mut line_lengths = TestLineLengths::new();
		line_lengths.set(0, 10);
                storage.move_left_single(line_lengths, &Position::new(0, 10), 12, false)
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
                storage.move_left_single(line_lengths, &Position::new(0, 10), 10, true)
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
                storage.move_left_single(line_lengths, &Position::new(1, 3), 69, true)
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

#[rustfmt::skip]
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
                storage.move_right_single(line_lengths, &Position::new(0, 0), 5, false)
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
                storage.move_right_single(line_lengths, &Position::new(0, 1), 1, true)
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
		
                storage.move_right_single(line_lengths, &Position::new(0, 0), 12, false)
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
		
                storage.move_right_single(line_lengths, &Position::new(0, 5), 12, false)
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
            ],
            storage -> {
                let mut line_lengths = TestLineLengths::new();
		line_lengths.set(0, 10);
                storage.move_left_single(line_lengths, &Position::new(0, 10), 12, false)
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
                storage.move_left_single(line_lengths, &Position::new(0, 10), 10, true)
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
                    storage.move_left_single(line_lengths, &Position::new(1, 3), 69, true)
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
