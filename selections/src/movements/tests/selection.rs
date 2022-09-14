use super::utils::TestLineLengths;
use crate::Selection;

mod move_left {
    use super::*;
    use crate::Position;

    #[test]
    fn one_line_forward() {
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
    fn one_line_forward_extend() {
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
    fn one_line_backward() {
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
    fn one_line_backward_extend() {
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
}
