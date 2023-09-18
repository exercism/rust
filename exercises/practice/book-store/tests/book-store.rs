#[test]
fn only_a_single_book() {
    let input = &[1];
    let output = book_store::lowest_price(input);
    let expected = 800;
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn two_of_the_same_book() {
    let input = &[2, 2];
    let output = book_store::lowest_price(input);
    let expected = 1600;
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn empty_basket() {
    let input = &[];
    let output = book_store::lowest_price(input);
    let expected = 0;
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn two_different_books() {
    let input = &[1, 2];
    let output = book_store::lowest_price(input);
    let expected = 1520;
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn three_different_books() {
    let input = &[1, 2, 3];
    let output = book_store::lowest_price(input);
    let expected = 2160;
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn four_different_books() {
    let input = &[1, 2, 3, 4];
    let output = book_store::lowest_price(input);
    let expected = 2560;
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn five_different_books() {
    let input = &[1, 2, 3, 4, 5];
    let output = book_store::lowest_price(input);
    let expected = 3000;
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn two_groups_of_four_is_cheaper_than_group_of_five_plus_group_of_three() {
    let input = &[1, 1, 2, 2, 3, 3, 4, 5];
    let output = book_store::lowest_price(input);
    let expected = 5120;
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn two_groups_of_four_is_cheaper_than_groups_of_five_and_three() {
    let input = &[1, 1, 2, 3, 4, 4, 5, 5];
    let output = book_store::lowest_price(input);
    let expected = 5120;
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn group_of_four_plus_group_of_two_is_cheaper_than_two_groups_of_three() {
    let input = &[1, 1, 2, 2, 3, 4];
    let output = book_store::lowest_price(input);
    let expected = 4080;
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn two_each_of_first_four_books_and_one_copy_each_of_rest() {
    let input = &[1, 1, 2, 2, 3, 3, 4, 4, 5];
    let output = book_store::lowest_price(input);
    let expected = 5560;
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn two_copies_of_each_book() {
    let input = &[1, 1, 2, 2, 3, 3, 4, 4, 5, 5];
    let output = book_store::lowest_price(input);
    let expected = 6000;
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn three_copies_of_first_book_and_two_each_of_remaining() {
    let input = &[1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 1];
    let output = book_store::lowest_price(input);
    let expected = 6800;
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn three_each_of_first_two_books_and_two_each_of_remaining_books() {
    let input = &[1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 1, 2];
    let output = book_store::lowest_price(input);
    let expected = 7520;
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn four_groups_of_four_are_cheaper_than_two_groups_each_of_five_and_three() {
    let input = &[1, 1, 2, 2, 3, 3, 4, 5, 1, 1, 2, 2, 3, 3, 4, 5];
    let output = book_store::lowest_price(input);
    let expected = 10240;
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn check_that_groups_of_four_are_created_properly_even_when_there_are_more_groups_of_three_than_groups_of_five(
) {
    let input = &[
        1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 4, 4, 5, 5,
    ];
    let output = book_store::lowest_price(input);
    let expected = 14560;
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn one_group_of_one_and_four_is_cheaper_than_one_group_of_two_and_three() {
    let input = &[1, 1, 2, 3, 4];
    let output = book_store::lowest_price(input);
    let expected = 3360;
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn one_group_of_one_and_two_plus_three_groups_of_four_is_cheaper_than_one_group_of_each_size() {
    let input = &[1, 2, 2, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 5];
    let output = book_store::lowest_price(input);
    let expected = 10000;
    assert_eq!(output, expected);
}
