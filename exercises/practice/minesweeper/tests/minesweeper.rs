use minesweeper::*;

#[test]
fn no_rows() {
    let input = &[];
    let expected: &[&str] = &[];
    let actual = annotate(input);
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
fn no_columns() {
    let input = &[""];
    let expected = &[""];
    let actual = annotate(input);
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
fn no_mines() {
    #[rustfmt::skip]
    let (input, expected) = (&[
        "   ",
        "   ",
        "   ",
    ], &[
        "   ",
        "   ",
        "   ",
    ]);
    let actual = annotate(input);
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
fn minefield_with_only_mines() {
    #[rustfmt::skip]
    let (input, expected) = (&[
        "***",
        "***",
        "***",
    ], &[
        "***",
        "***",
        "***",
    ]);
    let actual = annotate(input);
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
fn mine_surrounded_by_spaces() {
    #[rustfmt::skip]
    let (input, expected) = (&[
        "   ",
        " * ",
        "   ",
    ], &[
        "111",
        "1*1",
        "111",
    ]);
    let actual = annotate(input);
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
fn space_surrounded_by_mines() {
    #[rustfmt::skip]
    let (input, expected) = (&[
        "***",
        "* *",
        "***",
    ], &[
        "***",
        "*8*",
        "***",
    ]);
    let actual = annotate(input);
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
fn horizontal_line() {
    let input = &[" * * "];
    let expected = &["1*2*1"];
    let actual = annotate(input);
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
fn horizontal_line_mines_at_edges() {
    let input = &["*   *"];
    let expected = &["*1 1*"];
    let actual = annotate(input);
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
fn vertical_line() {
    #[rustfmt::skip]
    let (input, expected) = (&[
        " ",
        "*",
        " ",
        "*",
        " ",
    ], &[
        "1",
        "*",
        "2",
        "*",
        "1",
    ]);
    let actual = annotate(input);
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
fn vertical_line_mines_at_edges() {
    #[rustfmt::skip]
    let (input, expected) = (&[
        "*",
        " ",
        " ",
        " ",
        "*",
    ], &[
        "*",
        "1",
        " ",
        "1",
        "*",
    ]);
    let actual = annotate(input);
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
fn cross() {
    #[rustfmt::skip]
    let (input, expected) = (&[
        "  *  ",
        "  *  ",
        "*****",
        "  *  ",
        "  *  ",
    ], &[
        " 2*2 ",
        "25*52",
        "*****",
        "25*52",
        " 2*2 ",
    ]);
    let actual = annotate(input);
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
fn large_minefield() {
    #[rustfmt::skip]
    let (input, expected) = (&[
        " *  * ",
        "  *   ",
        "    * ",
        "   * *",
        " *  * ",
        "      ",
    ], &[
        "1*22*1",
        "12*322",
        " 123*2",
        "112*4*",
        "1*22*2",
        "111111",
    ]);
    let actual = annotate(input);
    assert_eq!(actual, expected);
}
