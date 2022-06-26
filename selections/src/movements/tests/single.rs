use crate::{util::selections_test, Position};

#[test]
fn test_move_left_single_no_extend_forward_same_line() {
    selections_test! {
        [
            (0, 0) - (0, 0),
            (0, 1) - (1, 2),
        ],
        storage -> {
            storage.move_left_single(&Position::new(0, 1), 1, false)
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
