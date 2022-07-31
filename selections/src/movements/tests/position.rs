use super::utils::TestLineLengths;
use crate::Position;

mod move_left {
    use super::*;

    #[test]
    fn one_line() {
        let line_lengths = TestLineLengths::default();
        let pos = Position::new(1, 10).move_left(&line_lengths, 5);
        assert_eq!(pos, Position::new(1, 5));
    }

    #[test]
    fn multiple_lines() {
        let mut line_lengths = TestLineLengths::default();
        line_lengths.set(0, 10);
        line_lengths.set(1, 20);

        let pos = Position::new(1, 15).move_left(&line_lengths, 16);
        assert_eq!(pos, Position::new(0, 9));

        // xxxxxxxxx_ - 10
        // xxxxxxxxxxxxxxxxxxx_ - 20
        // xxxxx|x| - 6
        let pos = Position::new(2, 5).move_left(&line_lengths, 32);
        assert_eq!(pos, Position::new(0, 3));
    }

    #[test]
    fn hit_buffer_beginning() {
        let mut line_lengths = TestLineLengths::default();
        line_lengths.set(0, 10);
        line_lengths.set(1, 20);

        let pos = Position::new(2, 5).move_left(&line_lengths, 69);
        assert_eq!(pos, Position::new(0, 0));
    }

    #[test]
    fn test_empty_buffer() {
        let line_lengths = TestLineLengths::default();
        let pos = Position::new(0, 0).move_left(&line_lengths, 69);
        assert_eq!(pos, Position::new(0, 0));
    }
}

mod move_right {
    use super::*;

    #[test]
    fn one_line() {
        let mut line_lengths = TestLineLengths::default();
        line_lengths.set(1, 30);
        let pos = Position::new(1, 10).move_right(&line_lengths, 5);
        assert_eq!(pos, Position::new(1, 15));
    }

    #[test]
    fn multiple_lines() {
        let mut line_lengths = TestLineLengths::default();
        line_lengths.set(0, 10);
        line_lengths.set(1, 20);
        line_lengths.set(2, 6);

        let pos = Position::new(0, 5).move_right(&line_lengths, 15);
        assert_eq!(pos, Position::new(1, 10));

        // xxxxxxxxx_ - 10
        // xxxxxxxxxxxxxxxxxxx_ - 20
        // xxxxx|x| - 6
        let pos = Position::new(0, 0).move_right(&line_lengths, 32);
        assert_eq!(pos, Position::new(2, 2));
    }

    // #[test]
    // fn hit_buffer_beginning() {
    //     let mut line_lengths = TestLineLengths::default();
    //     line_lengths.set(0, 10);
    //     line_lengths.set(1, 20);

    //     let pos = Position::new(2, 5).move_left(&line_lengths, 69);
    //     assert_eq!(pos, Position::new(0, 0));
    // }

    // #[test]
    // fn test_empty_buffer() {
    //     let line_lengths = TestLineLengths::default();
    //     let pos = Position::new(0, 0).move_left(&line_lengths, 69);
    //     assert_eq!(pos, Position::new(0, 0));
    // }
}
