use rectangles::count;

#[test]
fn test_zero_area_1() {
    let lines = &[];
    assert_eq!(0, count(lines))
}

#[test]
#[ignore]
fn test_zero_area_2() {
    let lines = &[""];
    assert_eq!(0, count(lines))
}

#[test]
#[ignore]
fn test_empty_area() {
    let lines = &[" "];
    assert_eq!(0, count(lines))
}

#[test]
#[ignore]
fn test_one_rectangle() {
    #[rustfmt::skip]
    let lines = &[
        "+-+",
        "| |",
        "+-+",
    ];
    assert_eq!(1, count(lines))
}

#[test]
#[ignore]
fn test_two_rectangles_no_shared_parts() {
    #[rustfmt::skip]
    let lines = &[
        "  +-+",
        "  | |",
        "+-+-+",
        "| |  ",
        "+-+  ",
    ];
    assert_eq!(2, count(lines))
}

#[test]
#[ignore]
fn test_five_rectangles_three_regions() {
    #[rustfmt::skip]
    let lines = &[
        "  +-+",
        "  | |",
        "+-+-+",
        "| | |",
        "+-+-+",
    ];
    assert_eq!(5, count(lines))
}

#[test]
#[ignore]
fn rectangle_of_height_1() {
    #[rustfmt::skip]
    let lines = &[
        "+--+",
        "+--+",
    ];
    assert_eq!(1, count(lines))
}

#[test]
#[ignore]
fn rectangle_of_width_1() {
    #[rustfmt::skip]
    let lines = &[
        "++",
        "||",
        "++",
    ];
    assert_eq!(1, count(lines))
}

#[test]
#[ignore]
fn unit_square() {
    #[rustfmt::skip]
    let lines = &[
        "++",
        "++",
    ];
    assert_eq!(1, count(lines))
}

#[test]
#[ignore]
fn test_incomplete_rectangles() {
    #[rustfmt::skip]
    let lines = &[
        "  +-+",
        "    |",
        "+-+-+",
        "| | -",
        "+-+-+",
    ];
    assert_eq!(1, count(lines))
}

#[test]
#[ignore]
fn test_complicated() {
    let lines = &[
        "+------+----+",
        "|      |    |",
        "+---+--+    |",
        "|   |       |",
        "+---+-------+",
    ];
    assert_eq!(3, count(lines))
}

#[test]
#[ignore]
fn test_not_so_complicated() {
    let lines = &[
        "+------+----+",
        "|      |    |",
        "+------+    |",
        "|   |       |",
        "+---+-------+",
    ];
    assert_eq!(2, count(lines))
}

#[test]
#[ignore]
fn test_large_input_with_many_rectangles() {
    let lines = &[
        "+---+--+----+",
        "|   +--+----+",
        "+---+--+    |",
        "|   +--+----+",
        "+---+--+--+-+",
        "+---+--+--+-+",
        "+------+  | |",
        "          +-+",
    ];
    assert_eq!(60, count(lines))
}

#[test]
#[ignore]
fn test_three_rectangles_no_shared_parts() {
    #[rustfmt::skip]
    let lines = &[
        "  +-+  ",
        "  | |  ",
        "+-+-+-+",
        "| | | |",
        "+-+ +-+",
    ];
    assert_eq!(3, count(lines))
}
