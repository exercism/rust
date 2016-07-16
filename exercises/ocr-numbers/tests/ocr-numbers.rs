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

    assert_eq!("?", ocr::convert(&input).unwrap());
}

#[test]
#[ignore]
#[cfg_attr(rustfmt, rustfmt_skip)]
fn recognizes_0() {
    let input = " _ \n".to_string() +
                "| |\n" +
                "|_|\n" +
                "   ";

    assert_eq!("0", ocr::convert(&input).unwrap());
}

#[test]
#[ignore]
#[cfg_attr(rustfmt, rustfmt_skip)]
fn recognizes_1() {
    let input = "   \n".to_string() +
                "  |\n" +
                "  |\n" +
                "   ";

    assert_eq!("1", ocr::convert(&input).unwrap());
}

#[test]
#[ignore]
#[cfg_attr(rustfmt, rustfmt_skip)]
fn recognizes_2() {
    let input = " _ \n".to_string() +
                " _|\n" +
                "|_ \n" +
                "   ";

    assert_eq!("2", ocr::convert(&input).unwrap());
}

#[test]
#[ignore]
#[cfg_attr(rustfmt, rustfmt_skip)]
fn recognizes_3() {
    let input = " _ \n".to_string() +
                " _|\n" +
                " _|\n" +
                "   ";

    assert_eq!("3", ocr::convert(&input).unwrap());
}

#[test]
#[ignore]
#[cfg_attr(rustfmt, rustfmt_skip)]
fn recognizes_4() {
    let input = "   \n".to_string() +
                "|_|\n" +
                "  |\n" +
                "   ";

    assert_eq!("4", ocr::convert(&input).unwrap());
}

#[test]
#[ignore]
#[cfg_attr(rustfmt, rustfmt_skip)]
fn recognizes_5() {
    let input = " _ \n".to_string() +
                "|_ \n" +
                " _|\n" +
                "   ";

    assert_eq!("5", ocr::convert(&input).unwrap());
}

#[test]
#[ignore]
#[cfg_attr(rustfmt, rustfmt_skip)]
fn recognizes_6() {
    let input = " _ \n".to_string() +
                "|_ \n" +
                "|_|\n" +
                "   ";

    assert_eq!("6", ocr::convert(&input).unwrap());
}

#[test]
#[ignore]
#[cfg_attr(rustfmt, rustfmt_skip)]
fn recognizes_7() {
    let input = " _ \n".to_string() +
                "  |\n" +
                "  |\n" +
                "   ";

    assert_eq!("7", ocr::convert(&input).unwrap());
}

#[test]
#[ignore]
#[cfg_attr(rustfmt, rustfmt_skip)]
fn recognizes_8() {
    let input = " _ \n".to_string() +
                "|_|\n" +
                "|_|\n" +
                "   ";

    assert_eq!("8", ocr::convert(&input).unwrap());
}

#[test]
#[ignore]
#[cfg_attr(rustfmt, rustfmt_skip)]
fn recognizes_9() {
    let input = " _ \n".to_string() +
                "|_|\n" +
                " _|\n" +
                "   ";

    assert_eq!("9", ocr::convert(&input).unwrap());
}

#[test]
#[ignore]
#[cfg_attr(rustfmt, rustfmt_skip)]
fn recognizes_110101100() {
    let input = "       _     _        _  _ \n".to_string() +
                "  |  || |  || |  |  || || |\n" +
                "  |  ||_|  ||_|  |  ||_||_|\n" +
                "                           ";

    assert_eq!("110101100", ocr::convert(&input).unwrap());
}

#[test]
#[ignore]
#[cfg_attr(rustfmt, rustfmt_skip)]
fn replaces_only_garbled_numbers_with_question_mark() {
    let input = "       _     _           _ \n".to_string() +
                "  |  || |  || |     || || |\n" +
                "  |  | _|  ||_|  |  ||_||_|\n" +
                "                           ";

    assert_eq!("11?10?1?0", ocr::convert(&input).unwrap());
}

#[test]
#[ignore]
#[cfg_attr(rustfmt, rustfmt_skip)]
fn recognizes_string_of_decimal_numbers() {
    let input = "    _  _     _  _  _  _  _  _ \n".to_string() +
                "  | _| _||_||_ |_   ||_||_|| |\n" +
                "  ||_  _|  | _||_|  ||_| _||_|\n" +
                "                              ";

    assert_eq!("1234567890", ocr::convert(&input).unwrap());
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
    assert_eq!("123,456,789", ocr::convert(&input).unwrap());
}
