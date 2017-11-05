extern crate ocr_numbers as ocr;

#[test]
#[cfg_attr(rustfmt, rustfmt_skip)]
fn input_with_lines_not_multiple_of_four_is_error() {
    let input = " _ \n".to_string() +
                "| |\n" +
                "   ";

    assert!(ocr::convert(&input).is_err());
}

#[test]
#[ignore]
#[cfg_attr(rustfmt, rustfmt_skip)]
fn input_with_columns_not_multiple_of_three_is_error() {
    let input = "    \n".to_string() +
                "   |\n" +
                "   |\n" +
                "    ";

    assert!(ocr::convert(&input).is_err());
}


#[test]
#[ignore]
#[cfg_attr(rustfmt, rustfmt_skip)]
fn unrecognized_chararcters_return_question_mark() {
    let input = "   \n".to_string() +
                "  _\n" +
                "  |\n" +
                "   ";

    assert_eq!(Ok("?".to_string()), ocr::convert(&input));
}

#[test]
#[ignore]
#[cfg_attr(rustfmt, rustfmt_skip)]
fn recognizes_0() {
    let input = " _ \n".to_string() +
                "| |\n" +
                "|_|\n" +
                "   ";

    assert_eq!(Ok("0".to_string()), ocr::convert(&input));
}

#[test]
#[ignore]
#[cfg_attr(rustfmt, rustfmt_skip)]
fn recognizes_1() {
    let input = "   \n".to_string() +
                "  |\n" +
                "  |\n" +
                "   ";

    assert_eq!(Ok("1".to_string()), ocr::convert(&input));
}

#[test]
#[ignore]
#[cfg_attr(rustfmt, rustfmt_skip)]
fn recognizes_2() {
    let input = " _ \n".to_string() +
                " _|\n" +
                "|_ \n" +
                "   ";

    assert_eq!(Ok("2".to_string()), ocr::convert(&input));
}

#[test]
#[ignore]
#[cfg_attr(rustfmt, rustfmt_skip)]
fn recognizes_3() {
    let input = " _ \n".to_string() +
                " _|\n" +
                " _|\n" +
                "   ";

    assert_eq!(Ok("3".to_string()), ocr::convert(&input));
}

#[test]
#[ignore]
#[cfg_attr(rustfmt, rustfmt_skip)]
fn recognizes_4() {
    let input = "   \n".to_string() +
                "|_|\n" +
                "  |\n" +
                "   ";

    assert_eq!(Ok("4".to_string()), ocr::convert(&input));
}

#[test]
#[ignore]
#[cfg_attr(rustfmt, rustfmt_skip)]
fn recognizes_5() {
    let input = " _ \n".to_string() +
                "|_ \n" +
                " _|\n" +
                "   ";

    assert_eq!(Ok("5".to_string()), ocr::convert(&input));
}

#[test]
#[ignore]
#[cfg_attr(rustfmt, rustfmt_skip)]
fn recognizes_6() {
    let input = " _ \n".to_string() +
                "|_ \n" +
                "|_|\n" +
                "   ";

    assert_eq!(Ok("6".to_string()), ocr::convert(&input));
}

#[test]
#[ignore]
#[cfg_attr(rustfmt, rustfmt_skip)]
fn recognizes_7() {
    let input = " _ \n".to_string() +
                "  |\n" +
                "  |\n" +
                "   ";

    assert_eq!(Ok("7".to_string()), ocr::convert(&input));
}

#[test]
#[ignore]
#[cfg_attr(rustfmt, rustfmt_skip)]
fn recognizes_8() {
    let input = " _ \n".to_string() +
                "|_|\n" +
                "|_|\n" +
                "   ";

    assert_eq!(Ok("8".to_string()), ocr::convert(&input));
}

#[test]
#[ignore]
#[cfg_attr(rustfmt, rustfmt_skip)]
fn recognizes_9() {
    let input = " _ \n".to_string() +
                "|_|\n" +
                " _|\n" +
                "   ";

    assert_eq!(Ok("9".to_string()), ocr::convert(&input));
}

#[test]
#[ignore]
#[cfg_attr(rustfmt, rustfmt_skip)]
fn recognizes_110101100() {
    let input = "       _     _        _  _ \n".to_string() +
                "  |  || |  || |  |  || || |\n" +
                "  |  ||_|  ||_|  |  ||_||_|\n" +
                "                           ";

    assert_eq!(Ok("110101100".to_string()), ocr::convert(&input));
}

#[test]
#[ignore]
#[cfg_attr(rustfmt, rustfmt_skip)]
fn replaces_only_garbled_numbers_with_question_mark() {
    let input = "       _     _           _ \n".to_string() +
                "  |  || |  || |     || || |\n" +
                "  |  | _|  ||_|  |  ||_||_|\n" +
                "                           ";

    assert_eq!(Ok("11?10?1?0".to_string()), ocr::convert(&input));
}

#[test]
#[ignore]
#[cfg_attr(rustfmt, rustfmt_skip)]
fn recognizes_string_of_decimal_numbers() {
    let input = "    _  _     _  _  _  _  _  _ \n".to_string() +
                "  | _| _||_||_ |_   ||_||_|| |\n" +
                "  ||_  _|  | _||_|  ||_| _||_|\n" +
                "                              ";

    assert_eq!(Ok("1234567890".to_string()), ocr::convert(&input));
}

#[test]
#[ignore]
#[cfg_attr(rustfmt, rustfmt_skip)]
fn numbers_across_multiple_lines_are_joined_by_commas() {
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
