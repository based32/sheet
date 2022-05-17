use crate::{movements::tests::utils::TestLineLengths, util::selections_test, Position};

mod left {
    use super::*;

    #[test]
    fn single_no_extend_forward_same_line() {
        selections_test! {
            [
                (0, 0) - (0, 0),
                (0, 1) - (1, 2),
            ],
            storage -> {
                let line_lengths = TestLineLengths::default();
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
