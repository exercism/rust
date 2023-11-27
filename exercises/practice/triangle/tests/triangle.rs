mod equilateral {
    use triangle::Triangle;

    #[test]
    fn all_sides_are_equal() {
        let input = [2, 2, 2];
        let output = Triangle::build(input).unwrap();
        assert!(output.is_equilateral());
    }

    #[test]
    #[ignore]
    fn any_side_is_unequal() {
        let input = [2, 3, 2];
        let output = Triangle::build(input).unwrap();
        assert!(!output.is_equilateral());
    }

    #[test]
    #[ignore]
    fn no_sides_are_equal() {
        let input = [5, 4, 6];
        let output = Triangle::build(input).unwrap();
        assert!(!output.is_equilateral());
    }

    #[test]
    #[ignore]
    #[cfg(feature = "generic")]
    fn sides_may_be_floats() {
        let input = [0.5, 0.5, 0.5];
        let output = Triangle::build(input).unwrap();
        assert!(output.is_equilateral());
    }
}

mod isosceles {
    use triangle::Triangle;

    #[test]
    #[ignore]
    fn last_two_sides_are_equal() {
        let input = [3, 4, 4];
        let output = Triangle::build(input).unwrap();
        assert!(output.is_isosceles());
    }

    #[test]
    #[ignore]
    fn first_two_sides_are_equal() {
        let input = [4, 4, 3];
        let output = Triangle::build(input).unwrap();
        assert!(output.is_isosceles());
    }

    #[test]
    #[ignore]
    fn first_and_last_sides_are_equal() {
        let input = [4, 3, 4];
        let output = Triangle::build(input).unwrap();
        assert!(output.is_isosceles());
    }

    #[test]
    #[ignore]
    fn equilateral_triangles_are_also_isosceles() {
        let input = [4, 4, 4];
        let output = Triangle::build(input).unwrap();
        assert!(output.is_isosceles());
    }

    #[test]
    #[ignore]
    fn no_sides_are_equal() {
        let input = [2, 3, 4];
        let output = Triangle::build(input).unwrap();
        assert!(!output.is_isosceles());
    }

    #[test]
    #[ignore]
    #[cfg(feature = "generic")]
    fn sides_may_be_floats() {
        let input = [0.5, 0.4, 0.5];
        let output = Triangle::build(input).unwrap();
        assert!(output.is_isosceles());
    }
}

mod scalene {
    use triangle::Triangle;

    #[test]
    #[ignore]
    fn no_sides_are_equal() {
        let input = [5, 4, 6];
        let output = Triangle::build(input).unwrap();
        assert!(output.is_scalene());
    }

    #[test]
    #[ignore]
    fn all_sides_are_equal() {
        let input = [4, 4, 4];
        let output = Triangle::build(input).unwrap();
        assert!(!output.is_scalene());
    }

    #[test]
    #[ignore]
    fn first_and_second_sides_are_equal() {
        let input = [4, 4, 3];
        let output = Triangle::build(input).unwrap();
        assert!(!output.is_scalene());
    }

    #[test]
    #[ignore]
    fn first_and_third_sides_are_equal() {
        let input = [3, 4, 3];
        let output = Triangle::build(input).unwrap();
        assert!(!output.is_scalene());
    }

    #[test]
    #[ignore]
    fn second_and_third_sides_are_equal() {
        let input = [4, 3, 3];
        let output = Triangle::build(input).unwrap();
        assert!(!output.is_scalene());
    }

    #[test]
    #[ignore]
    #[cfg(feature = "generic")]
    fn sides_may_be_floats() {
        let input = [0.5, 0.4, 0.6];
        let output = Triangle::build(input).unwrap();
        assert!(output.is_scalene());
    }
}

mod invalid {
    use triangle::Triangle;

    #[test]
    #[ignore]
    fn all_zero_sides_is_not_a_triangle() {
        let input = [0, 0, 0];
        let output = Triangle::build(input);
        assert!(output.is_none());
    }

    #[test]
    #[ignore]
    fn first_triangle_inequality_violation() {
        let input = [1, 1, 3];
        let output = Triangle::build(input);
        assert!(output.is_none());
    }

    #[test]
    #[ignore]
    fn second_triangle_inequality_violation() {
        let input = [1, 3, 1];
        let output = Triangle::build(input);
        assert!(output.is_none());
    }

    #[test]
    #[ignore]
    fn third_triangle_inequality_violation() {
        let input = [3, 1, 1];
        let output = Triangle::build(input);
        assert!(output.is_none());
    }

    #[test]
    #[ignore]
    fn may_not_violate_triangle_inequality() {
        let input = [7, 3, 2];
        let output = Triangle::build(input);
        assert!(output.is_none());
    }
}
