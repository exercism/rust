use ocr_numbers as ocr;

#[test]
fn input_with_lines_not_multiple_of_four_is_error() {
    #[rustfmt::skip]
    let input = " _ \n".to_string() +
                "| |\n" +
                "   ";

    assert_eq!(Err(ocr::Error::InvalidRowCount(3)), ocr::convert(&input));
}

#[test]
#[ignore]
fn input_with_columns_not_multiple_of_three_is_error() {
    #[rustfmt::skip]
    let input = "    \n".to_string() +
                "   |\n" +
                "   |\n" +
                "    ";

    assert_eq!(Err(ocr::Error::InvalidColumnCount(4)), ocr::convert(&input));
}

#[test]
#[ignore]
fn unrecognized_characters_return_question_mark() {
    #[rustfmt::skip]
    let input = "   \n".to_string() +
                "  _\n" +
                "  |\n" +
                "   ";

    assert_eq!(Ok("?".to_string()), ocr::convert(&input));
}

#[test]
#[ignore]
fn recognizes_0() {
    #[rustfmt::skip]
    let input = " _ \n".to_string() +
                "| |\n" +
                "|_|\n" +
                "   ";

    assert_eq!(Ok("0".to_string()), ocr::convert(&input));
}

#[test]
#[ignore]
fn recognizes_1() {
    #[rustfmt::skip]
    let input = "   \n".to_string() +
                "  |\n" +
                "  |\n" +
                "   ";

    assert_eq!(Ok("1".to_string()), ocr::convert(&input));
}

#[test]
#[ignore]
fn recognizes_2() {
    #[rustfmt::skip]
    let input = " _ \n".to_string() +
                " _|\n" +
                "|_ \n" +
                "   ";

    assert_eq!(Ok("2".to_string()), ocr::convert(&input));
}

#[test]
#[ignore]
fn recognizes_3() {
    #[rustfmt::skip]
    let input = " _ \n".to_string() +
                " _|\n" +
                " _|\n" +
                "   ";

    assert_eq!(Ok("3".to_string()), ocr::convert(&input));
}

#[test]
#[ignore]
fn recognizes_4() {
    #[rustfmt::skip]
    let input = "   \n".to_string() +
                "|_|\n" +
                "  |\n" +
                "   ";

    assert_eq!(Ok("4".to_string()), ocr::convert(&input));
}

#[test]
#[ignore]
fn recognizes_5() {
    #[rustfmt::skip]
    let input = " _ \n".to_string() +
                "|_ \n" +
                " _|\n" +
                "   ";

    assert_eq!(Ok("5".to_string()), ocr::convert(&input));
}

#[test]
#[ignore]
fn recognizes_6() {
    #[rustfmt::skip]
    let input = " _ \n".to_string() +
                "|_ \n" +
                "|_|\n" +
                "   ";

    assert_eq!(Ok("6".to_string()), ocr::convert(&input));
}

#[test]
#[ignore]
fn recognizes_7() {
    #[rustfmt::skip]
    let input = " _ \n".to_string() +
                "  |\n" +
                "  |\n" +
                "   ";

    assert_eq!(Ok("7".to_string()), ocr::convert(&input));
}

#[test]
#[ignore]
fn recognizes_8() {
    #[rustfmt::skip]
    let input = " _ \n".to_string() +
                "|_|\n" +
                "|_|\n" +
                "   ";

    assert_eq!(Ok("8".to_string()), ocr::convert(&input));
}

#[test]
#[ignore]
fn recognizes_9() {
    #[rustfmt::skip]
    let input = " _ \n".to_string() +
                "|_|\n" +
                " _|\n" +
                "   ";

    assert_eq!(Ok("9".to_string()), ocr::convert(&input));
}

#[test]
#[ignore]
fn recognizes_110101100() {
    #[rustfmt::skip]
    let input = "       _     _        _  _ \n".to_string() +
                "  |  || |  || |  |  || || |\n" +
                "  |  ||_|  ||_|  |  ||_||_|\n" +
                "                           ";

    assert_eq!(Ok("110101100".to_string()), ocr::convert(&input));
}

#[test]
#[ignore]
fn replaces_only_garbled_numbers_with_question_mark() {
    #[rustfmt::skip]
    let input = "       _     _           _ \n".to_string() +
                "  |  || |  || |     || || |\n" +
                "  |  | _|  ||_|  |  ||_||_|\n" +
                "                           ";

    assert_eq!(Ok("11?10?1?0".to_string()), ocr::convert(&input));
}

#[test]
#[ignore]
fn recognizes_string_of_decimal_numbers() {
    #[rustfmt::skip]
    let input = "    _  _     _  _  _  _  _  _ \n".to_string() +
                "  | _| _||_||_ |_   ||_||_|| |\n" +
                "  ||_  _|  | _||_|  ||_| _||_|\n" +
                "                              ";

    assert_eq!(Ok("1234567890".to_string()), ocr::convert(&input));
}

#[test]
#[ignore]
fn numbers_across_multiple_lines_are_joined_by_commas() {
    #[rustfmt::skip]
    let input = "    _  _ \n".to_string() +
                "  | _| _|\n" +
                "  ||_  _|\n" +
                "         \n" +
                "    _  _ \n" +
                "|_||_ |_ \n" +
                "  | _||_|\n" +
                "         \n" +
                " _  _  _ \n" +
                "  ||_||_|\n" +
                "  ||_| _|\n" +
                "         ";
    assert_eq!(Ok("123,456,789".to_string()), ocr::convert(&input));
}
