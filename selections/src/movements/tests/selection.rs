use super::utils::TestLineLengths;
use crate::Selection;

mod move_left_one_line {
    use super::*;
    use crate::Position;

    #[test]
    fn forward() {
        let mut line_lengths = TestLineLengths::new();
        line_lengths.set(0, 20);
        let selection = Selection::new(Position::new(0, 5), Position::new(0, 10)).move_left(
            &line_lengths,
            2,
            false,
        );
        assert_eq!(
            selection,
            Selection::new(Position::new(0, 8), Position::new(0, 8))
        );
    }

    #[test]
    fn forward_extend() {
        let mut line_lengths = TestLineLengths::new();
        line_lengths.set(0, 20);
        let selection = Selection::new(Position::new(0, 5), Position::new(0, 10)).move_left(
            &line_lengths,
            2,
            true,
        );
        assert_eq!(
            selection,
            Selection::new(Position::new(0, 5), Position::new(0, 8))
        );
    }

    #[test]
    fn backward() {
        let mut line_lengths = TestLineLengths::new();
        line_lengths.set(0, 20);
        let selection = Selection::new(Position::new(0, 10), Position::new(0, 5)).move_left(
            &line_lengths,
            2,
            false,
        );
        assert_eq!(
            selection,
            Selection::new(Position::new(0, 3), Position::new(0, 3))
        );
    }

    #[test]
    fn backward_extend() {
        let mut line_lengths = TestLineLengths::new();
        line_lengths.set(0, 20);
        let selection = Selection::new(Position::new(0, 10), Position::new(0, 5)).move_left(
            &line_lengths,
            2,
            true,
        );
        assert_eq!(
            selection,
            Selection::new(Position::new(0, 10), Position::new(0, 3))
        );
    }

    #[test]
    fn change_direction() {
        let mut line_lengths = TestLineLengths::new();
        line_lengths.set(0, 20);
        let selection = Selection::new(Position::new(0, 5), Position::new(0, 10)).move_left(
            &line_lengths,
            6,
            true,
        );
        assert_eq!(
            selection,
            Selection::new(Position::new(0, 5), Position::new(0, 4))
        );
    }

    #[test]
    fn empty_buffer() {
        let line_lengths = TestLineLengths::new();
        let selection = Selection::new(Position::new(0, 0), Position::new(0, 0)).move_left(
            &line_lengths,
            2,
            true,
        );
        assert_eq!(
            selection,
            Selection::new(Position::new(0, 0), Position::new(0, 0))
        );
    }

    #[test]
    fn hit_buffer_beginning() {
        let mut line_lengths = TestLineLengths::new();
        line_lengths.set(0, 20);
        let selection = Selection::new(Position::new(0, 5), Position::new(0, 10)).move_left(
            &line_lengths,
            69,
            false,
        );
        assert_eq!(
            selection,
            Selection::new(Position::new(0, 0), Position::new(0, 0))
        );
    }
}

mod move_left_multiple_lines {
    use super::*;
    use crate::Position;

    #[test]
    fn forward() {
        let mut line_lengths = TestLineLengths::new();
        line_lengths.set(0, 20);
        line_lengths.set(1, 20);
        let selection = Selection::new(Position::new(1, 5), Position::new(1, 10)).move_left(
            &line_lengths,
            11,
            false,
        );
        assert_eq!(
            selection,
            Selection::new(Position::new(0, 20), Position::new(0, 20))
        );
    }

    #[test]
    fn forward_extend() {
        let mut line_lengths = TestLineLengths::new();
        line_lengths.set(0, 20);
        line_lengths.set(1, 20);
        let selection = Selection::new(Position::new(0, 5), Position::new(1, 10)).move_left(
            &line_lengths,
            11,
            true,
        );
        assert_eq!(
            selection,
            Selection::new(Position::new(0, 5), Position::new(0, 20))
        );
    }

    #[test]
    fn backward() {
        let mut line_lengths = TestLineLengths::new();
        line_lengths.set(0, 20);
        line_lengths.set(1, 20);
        let selection = Selection::new(Position::new(1, 10), Position::new(1, 5)).move_left(
            &line_lengths,
            11,
            false,
        );
        assert_eq!(
            selection,
            Selection::new(Position::new(0, 15), Position::new(0, 15))
        );
    }

    #[test]
    fn backward_extend() {
        let mut line_lengths = TestLineLengths::new();
        line_lengths.set(0, 20);
        line_lengths.set(1, 20);
        let selection = Selection::new(Position::new(1, 10), Position::new(1, 5)).move_left(
            &line_lengths,
            11,
            true,
        );
        assert_eq!(
            selection,
            Selection::new(Position::new(1, 10), Position::new(0, 15))
        );
    }

    #[test]
    fn change_direction() {
        let mut line_lengths = TestLineLengths::new();
        line_lengths.set(0, 20);
        line_lengths.set(1, 20);
        let selection = Selection::new(Position::new(1, 5), Position::new(1, 10)).move_left(
            &line_lengths,
            11,
            true,
        );
        assert_eq!(
            selection,
            Selection::new(Position::new(1, 5), Position::new(0, 20))
        );
    }

    #[test]
    fn hit_buffer_beginning() {
        let mut line_lengths = TestLineLengths::new();
        line_lengths.set(0, 20);
        line_lengths.set(1, 20);
        line_lengths.set(2, 5);
        let selection = Selection::new(Position::new(2, 1), Position::new(2, 4)).move_left(
            &line_lengths,
            69,
            false,
        );
        assert_eq!(
            selection,
            Selection::new(Position::new(0, 0), Position::new(0, 0))
        );
    }
}

mod move_right_one_line {
    use super::*;
    use crate::Position;

    #[test]
    fn forward() {
        let mut line_lengths = TestLineLengths::new();
        line_lengths.set(0, 20);
        let selection = Selection::new(Position::new(0, 5), Position::new(0, 10)).move_right(
            &line_lengths,
            2,
            false,
        );
        assert_eq!(
            selection,
            Selection::new(Position::new(0, 12), Position::new(0, 12))
        );
    }

    #[test]
    fn forward_extend() {
        let mut line_lengths = TestLineLengths::new();
        line_lengths.set(0, 20);
        let selection = Selection::new(Position::new(0, 5), Position::new(0, 10)).move_right(
            &line_lengths,
            2,
            true,
        );
        assert_eq!(
            selection,
            Selection::new(Position::new(0, 5), Position::new(0, 12))
        );
    }

    #[test]
    fn backward() {
        let mut line_lengths = TestLineLengths::new();
        line_lengths.set(0, 20);
        let selection = Selection::new(Position::new(0, 10), Position::new(0, 5)).move_right(
            &line_lengths,
            2,
            false,
        );
        assert_eq!(
            selection,
            Selection::new(Position::new(0, 7), Position::new(0, 7))
        );
    }

    #[test]
    fn backward_extend() {
        let mut line_lengths = TestLineLengths::new();
        line_lengths.set(0, 20);
        let selection = Selection::new(Position::new(0, 10), Position::new(0, 5)).move_right(
            &line_lengths,
            2,
            true,
        );
        assert_eq!(
            selection,
            Selection::new(Position::new(0, 10), Position::new(0, 7))
        );
    }

    #[test]
    fn change_direction() {
        let mut line_lengths = TestLineLengths::new();
        line_lengths.set(0, 20);
        let selection = Selection::new(Position::new(0, 10), Position::new(0, 5)).move_right(
            &line_lengths,
            6,
            true,
        );
        assert_eq!(
            selection,
            Selection::new(Position::new(0, 10), Position::new(0, 11))
        );
    }

    #[test]
    fn empty_buffer() {
        let line_lengths = TestLineLengths::new();
        let selection = Selection::new(Position::new(0, 0), Position::new(0, 0)).move_right(
            &line_lengths,
            2,
            true,
        );
        assert_eq!(
            selection,
            Selection::new(Position::new(0, 0), Position::new(0, 0))
        );
    }

    #[test]
    fn hit_buffer_end() {
        let mut line_lengths = TestLineLengths::new();
        line_lengths.set(0, 20);
        let selection = Selection::new(Position::new(0, 5), Position::new(0, 10)).move_right(
            &line_lengths,
            69,
            false,
        );
        assert_eq!(
            selection,
            Selection::new(Position::new(0, 20), Position::new(0, 20))
        );
    }
}

mod move_right_multiple_lines {
    use super::*;
    use crate::Position;

    #[test]
    fn forward() {
        let mut line_lengths = TestLineLengths::new();
        line_lengths.set(0, 20);
        line_lengths.set(1, 20);
        let selection = Selection::new(Position::new(0, 5), Position::new(0, 10)).move_right(
            &line_lengths,
            11,
            false,
        );
        assert_eq!(
            selection,
            Selection::new(Position::new(1, 0), Position::new(1, 0))
        );
    }

    #[test]
    fn forward_extend() {
        let mut line_lengths = TestLineLengths::new();
        line_lengths.set(0, 20);
        line_lengths.set(1, 20);
        let selection = Selection::new(Position::new(0, 5), Position::new(0, 10)).move_right(
            &line_lengths,
            11,
            true,
        );
        assert_eq!(
            selection,
            Selection::new(Position::new(0, 5), Position::new(1, 0))
        );
    }

    #[test]
    fn backward() {
        let mut line_lengths = TestLineLengths::new();
        line_lengths.set(0, 20);
        line_lengths.set(1, 20);
        let selection = Selection::new(Position::new(0, 10), Position::new(0, 5)).move_right(
            &line_lengths,
            17,
            false,
        );
        assert_eq!(
            selection,
            Selection::new(Position::new(1, 1), Position::new(1, 1))
        );
    }

    #[test]
    fn backward_extend() {
        let mut line_lengths = TestLineLengths::new();
        line_lengths.set(0, 20);
        line_lengths.set(1, 20);
        let selection = Selection::new(Position::new(0, 10), Position::new(0, 5)).move_right(
            &line_lengths,
            17,
            true,
        );
        assert_eq!(
            selection,
            Selection::new(Position::new(0, 10), Position::new(1, 1))
        );
    }

    #[test]
    fn change_direction() {
        let mut line_lengths = TestLineLengths::new();
        line_lengths.set(0, 20);
        line_lengths.set(1, 20);
        let selection = Selection::new(Position::new(0, 10), Position::new(0, 5)).move_right(
            &line_lengths,
            16,
            true,
        );
        assert_eq!(
            selection,
            Selection::new(Position::new(0, 10), Position::new(1, 0))
        );
    }

    #[test]
    fn hit_buffer_end() {
        let mut line_lengths = TestLineLengths::new();
        line_lengths.set(0, 20);
        line_lengths.set(1, 20);
        line_lengths.set(2, 5);
        let selection = Selection::new(Position::new(1, 1), Position::new(1, 4)).move_right(
            &line_lengths,
            69,
            false,
        );
        assert_eq!(
            selection,
            Selection::new(Position::new(2, 5), Position::new(2, 5))
        );
    }
}
