use crate::{
    test_utils::{selections_test, TestLineLengths},
    Position,
};

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
}
