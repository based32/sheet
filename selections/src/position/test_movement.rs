use super::Position;
use crate::test_utils::TestLineLengths;

mod move_left {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn one_line() {
        let line_lengths = TestLineLengths::new();
        let pos = Position::new(1, 10).move_left(&line_lengths, 5);
        assert_eq!(pos, Position::new(1, 5));
    }

    #[test]
    fn multiple_lines() {
        let mut line_lengths = TestLineLengths::new();
        line_lengths.set(0, 10);
        line_lengths.set(1, 20);
        line_lengths.set(2, 6);
        // xxxxxxxxxx_ - 10
        // xxxxxxxxxxxxxxxxxxxx_ - 20
        // xxxxxx - 6

        let pos = Position::new(1, 15).move_left(&line_lengths, 16);
        assert_eq!(pos, Position::new(0, 10));

        let pos = Position::new(2, 5).move_left(&line_lengths, 32);
        assert_eq!(pos, Position::new(0, 5));
    }

    #[test]
    fn multiple_lines_with_empty() {
        let mut line_lengths = TestLineLengths::new();
        line_lengths.set(0, 10);
        line_lengths.set(1, 0);
        line_lengths.set(2, 20);
        line_lengths.set(3, 6);
        // xxxxxxxxxx_ - 10
        // _ - 0
        // xxxxxxxxxxxxxxxxxxxx_ - 20
        // xxxxxx - 6

        let pos = Position::new(2, 15).move_left(&line_lengths, 16);
        assert_eq!(pos, Position::new(1, 0));

        let pos = Position::new(3, 5).move_left(&line_lengths, 32);
        assert_eq!(pos, Position::new(0, 6));
    }

    #[test]
    fn hit_buffer_beginning() {
        let mut line_lengths = TestLineLengths::new();
        line_lengths.set(0, 10);
        line_lengths.set(1, 20);

        let pos = Position::new(2, 5).move_left(&line_lengths, 69);
        assert_eq!(pos, Position::new(0, 0));
    }

    #[test]
    fn empty_buffer() {
        let line_lengths = TestLineLengths::new();
        let pos = Position::new(0, 0).move_left(&line_lengths, 69);
        assert_eq!(pos, Position::new(0, 0));
    }
}

mod move_right {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn one_line() {
        let mut line_lengths = TestLineLengths::new();
        line_lengths.set(1, 30);
        let pos = Position::new(1, 10).move_right(&line_lengths, 5);
        assert_eq!(pos, Position::new(1, 15));
    }

    #[test]
    fn multiple_lines() {
        let mut line_lengths = TestLineLengths::new();
        line_lengths.set(0, 10);
        line_lengths.set(1, 20);
        line_lengths.set(2, 6);
        // xxxxxxxxxx_ - 10
        // xxxxxxxxxxxxxxxxxxxx_ - 20
        // xxxxxx - 6

        let pos = Position::new(0, 5).move_right(&line_lengths, 15);
        assert_eq!(pos, Position::new(1, 9));

        let pos = Position::new(0, 0).move_right(&line_lengths, 32);
        assert_eq!(pos, Position::new(2, 0));
    }

    #[test]
    fn multiple_lines_with_empty() {
        let mut line_lengths = TestLineLengths::new();
        line_lengths.set(0, 10);
        line_lengths.set(1, 0);
        line_lengths.set(2, 20);
        line_lengths.set(3, 6);
        // xxxxxxxxxx_ - 10
        // _ - 0
        // xxxxxxxxxxxxxxxxxxxx_ - 20
        // xxxxxx - 6

        let pos = Position::new(0, 5).move_right(&line_lengths, 15);
        assert_eq!(pos, Position::new(2, 8));

        let pos = Position::new(0, 0).move_right(&line_lengths, 32);
        assert_eq!(pos, Position::new(2, 20));
    }

    #[test]
    fn hit_buffer_end() {
        let mut line_lengths = TestLineLengths::new();
        line_lengths.set(0, 10);
        line_lengths.set(1, 20);
        line_lengths.set(2, 6);

        let pos = Position::new(0, 5).move_right(&line_lengths, 69);
        assert_eq!(pos, Position::new(2, 6)); // Hitting a fake newline symbol
    }

    #[test]
    fn hit_buffer_end_with_newline() {
        let mut line_lengths = TestLineLengths::new();
        line_lengths.set(0, 10);
        line_lengths.set(1, 20);
        line_lengths.set(2, 6);
        line_lengths.set(3, 0);

        let pos = Position::new(0, 5).move_right(&line_lengths, 69);
        assert_eq!(pos, Position::new(3, 0));
    }

    #[test]
    fn empty_buffer() {
        let line_lengths = TestLineLengths::new();
        let pos = Position::new(0, 0).move_right(&line_lengths, 69);
        assert_eq!(pos, Position::new(0, 0));
    }
}

mod move_up {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn one_line() {
        let mut line_lengths = TestLineLengths::new();
        line_lengths.set(0, 30);
        line_lengths.set(1, 30);
        let pos = Position::new(1, 10).move_up(&line_lengths, 1);
        assert_eq!(pos, Position::new(0, 10));
    }

    #[test]
    fn multiple_lines() {
        let mut line_lengths = TestLineLengths::new();
        line_lengths.set(0, 10);
        line_lengths.set(1, 20);
        line_lengths.set(2, 6);

        let pos = Position::new(2, 6).move_up(&line_lengths, 2);
        assert_eq!(pos, Position::new(0, 6));
    }

    #[test]
    fn multiple_lines_with_empty() {
        let mut line_lengths = TestLineLengths::new();
        line_lengths.set(0, 10);
        line_lengths.set(1, 0);
        line_lengths.set(2, 20);
        line_lengths.set(3, 6);

        let pos = Position::new(3, 6).move_up(&line_lengths, 3);
        assert_eq!(pos, Position::new(0, 6));
    }

    #[test]
    fn hit_buffer_beginning() {
        let mut line_lengths = TestLineLengths::new();
        line_lengths.set(0, 10);
        line_lengths.set(1, 20);
        line_lengths.set(2, 6);

        let pos = Position::new(2, 5).move_up(&line_lengths, 69);
        assert_eq!(pos, Position::new(0, 5));
    }

    #[test]
    fn empty_buffer() {
        let line_lengths = TestLineLengths::new();
        let pos = Position::new(0, 0).move_up(&line_lengths, 69);
        assert_eq!(pos, Position::new(0, 0));
    }

    #[test]
    fn sticky_column() {
        let mut line_lengths = TestLineLengths::new();
        line_lengths.set(0, 10);
        line_lengths.set(1, 0);
        line_lengths.set(2, 20);
        line_lengths.set(3, 6);

        let pos = Position::new(3, 6).move_up(&line_lengths, 2);
        assert_eq!(pos, Position::new_with_sticky(1, 0, 6));
    }

    #[test]
    fn sticky_column_separate_movements() {
        let mut line_lengths = TestLineLengths::new();
        line_lengths.set(0, 10);
        line_lengths.set(1, 0);
        line_lengths.set(2, 20);
        line_lengths.set(3, 6);

        let pos = Position::new(3, 6).move_up(&line_lengths, 2);
        assert_eq!(pos, Position::new_with_sticky(1, 0, 6));

        let pos2 = pos.move_up(&line_lengths, 1);
        assert_eq!(pos2, Position::new(0, 6));
    }

    #[test]
    fn no_sticky_column_edge_case() {
        let mut line_lengths = TestLineLengths::new();
        line_lengths.set(0, 10);
        line_lengths.set(1, 5);
        line_lengths.set(2, 0);
        line_lengths.set(3, 5);

        let pos = Position::new(3, 5).move_up(&line_lengths, 2);
        assert_eq!(pos, Position::new(1, 5));
    }
}

mod move_down {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn one_line() {
        let mut line_lengths = TestLineLengths::new();
        line_lengths.set(0, 30);
        line_lengths.set(1, 30);
        let pos = Position::new(0, 10).move_down(&line_lengths, 1);
        assert_eq!(pos, Position::new(1, 10));
    }

    #[test]
    fn multiple_lines() {
        let mut line_lengths = TestLineLengths::new();
        line_lengths.set(0, 10);
        line_lengths.set(1, 20);
        line_lengths.set(2, 6);

        let pos = Position::new(0, 6).move_down(&line_lengths, 2);
        assert_eq!(pos, Position::new(2, 6));
    }

    #[test]
    fn multiple_lines_with_empty() {
        let mut line_lengths = TestLineLengths::new();
        line_lengths.set(0, 10);
        line_lengths.set(1, 0);
        line_lengths.set(2, 20);
        line_lengths.set(3, 6);

        let pos = Position::new(0, 6).move_down(&line_lengths, 3);
        assert_eq!(pos, Position::new(3, 6));
    }

    #[test]
    fn hit_buffer_end() {
        let mut line_lengths = TestLineLengths::new();
        line_lengths.set(0, 10);
        line_lengths.set(1, 20);
        line_lengths.set(2, 6);

        let pos = Position::new(0, 5).move_down(&line_lengths, 69);
        assert_eq!(pos, Position::new(2, 5));
    }

    #[test]
    fn empty_buffer() {
        let line_lengths = TestLineLengths::new();
        let pos = Position::new(0, 0).move_down(&line_lengths, 69);
        assert_eq!(pos, Position::new(0, 0));
    }

    #[test]
    fn sticky_column() {
        let mut line_lengths = TestLineLengths::new();
        line_lengths.set(0, 10);
        line_lengths.set(1, 0);
        line_lengths.set(2, 20);
        line_lengths.set(3, 6);

        let pos = Position::new(0, 6).move_down(&line_lengths, 1);
        assert_eq!(pos, Position::new_with_sticky(1, 0, 6));
    }

    #[test]
    fn sticky_column_separate_movements() {
        let mut line_lengths = TestLineLengths::new();
        line_lengths.set(0, 10);
        line_lengths.set(1, 0);
        line_lengths.set(2, 20);
        line_lengths.set(3, 6);

        let pos = Position::new(0, 6).move_down(&line_lengths, 1);
        assert_eq!(pos, Position::new_with_sticky(1, 0, 6));

        let pos2 = pos.move_down(&line_lengths, 1);
        assert_eq!(pos2, Position::new(2, 6));
    }

    #[test]
    fn no_sticky_column_edge_case() {
        let mut line_lengths = TestLineLengths::new();
        line_lengths.set(0, 10);
        line_lengths.set(1, 5);
        line_lengths.set(2, 0);
        line_lengths.set(3, 5);

        let pos = Position::new(1, 5).move_down(&line_lengths, 2);
        assert_eq!(pos, Position::new(3, 5));
    }
}
