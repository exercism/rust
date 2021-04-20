mod divmod {
    //! tests of divmod
    //!
    //! note that we're only testing positive quantities; no need to get into the mod/rem distinction

    use low_power_embedded_game::divmod;

    #[test]
    fn example() {
        assert_eq!(divmod(10, 3), (3, 1));
    }

    #[test]
    #[ignore]
    fn powerup() {
        assert_eq!(divmod(100, 3), (33, 1));
    }

    #[test]
    #[ignore]
    fn less() {
        assert_eq!(divmod(3, 10), (0, 3));
    }

    #[test]
    #[ignore]
    fn eq() {
        assert_eq!(divmod(3, 3), (1, 0));
    }

    #[test]
    #[ignore]
    fn multiple() {
        assert_eq!(divmod(9, 3), (3, 0));
    }
}

mod evens {
    use low_power_embedded_game::evens;

    #[test]
    #[ignore]
    fn simple_i32() {
        let out: Vec<i32> = evens(0..).take(5).collect();
        assert_eq!(out, &[0, 2, 4, 6, 8]);
    }

    #[test]
    #[ignore]
    fn reverse_i32() {
        let out: Vec<i32> = evens((0..=10).rev()).collect();
        assert_eq!(out, &[10, 8, 6, 4, 2, 0]);
    }

    #[test]
    #[ignore]
    fn offset_i32() {
        let out: Vec<i32> = evens(1..).take(5).collect();
        assert_eq!(out, &[1, 3, 5, 7, 9]);
    }

    #[test]
    #[ignore]
    fn strs() {
        let input = "You really must never be above joking.".split_whitespace();
        let expected: Vec<_> = "You must be joking.".split_whitespace().collect();
        let out: Vec<_> = evens(input).collect();
        assert_eq!(out, expected);
    }
}

mod manhattan {
    use low_power_embedded_game::Position;

    #[test]
    #[ignore]
    fn origin() {
        assert_eq!(Position(0, 0).manhattan(), 0);
    }

    #[test]
    #[ignore]
    fn q1_unit() {
        assert_eq!(Position(1, 1).manhattan(), 2);
    }

    #[test]
    #[ignore]
    fn q2_unit() {
        assert_eq!(Position(1, -1).manhattan(), 2);
    }

    #[test]
    #[ignore]
    fn q3_unit() {
        assert_eq!(Position(-1, -1).manhattan(), 2);
    }

    #[test]
    #[ignore]
    fn q4_unit() {
        assert_eq!(Position(-1, 1).manhattan(), 2);
    }

    #[test]
    #[ignore]
    fn relative_prime() {
        assert_eq!(Position(30, 70).manhattan(), 100);
    }
}
