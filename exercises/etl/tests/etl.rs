use etl;

use std::collections::BTreeMap;

#[test]
fn test_transform_one_value() {
    let input = input_from(&[(1, vec!['A'])]);

    let expected = expected_from(&[('a', 1)]);

    assert_eq!(expected, etl::transform(&input));
}

#[test]
#[ignore]
fn test_transform_more_values() {
    let input = input_from(&[(1, vec!['A', 'E', 'I', 'O', 'U'])]);

    let expected = expected_from(&[('a', 1), ('e', 1), ('i', 1), ('o', 1), ('u', 1)]);

    assert_eq!(expected, etl::transform(&input));
}

#[test]
#[ignore]
fn test_more_keys() {
    let input = input_from(&[(1, vec!['A', 'E']), (2, vec!['D', 'G'])]);

    let expected = expected_from(&[('a', 1), ('e', 1), ('d', 2), ('g', 2)]);

    assert_eq!(expected, etl::transform(&input));
}

#[test]
#[ignore]
fn test_full_dataset() {
    let input = input_from(&[
        (1, vec!['A', 'E', 'I', 'O', 'U', 'L', 'N', 'R', 'S', 'T']),
        (2, vec!['D', 'G']),
        (3, vec!['B', 'C', 'M', 'P']),
        (4, vec!['F', 'H', 'V', 'W', 'Y']),
        (5, vec!['K']),
        (8, vec!['J', 'X']),
        (10, vec!['Q', 'Z']),
    ]);

    let expected = expected_from(&[
        ('a', 1),
        ('b', 3),
        ('c', 3),
        ('d', 2),
        ('e', 1),
        ('f', 4),
        ('g', 2),
        ('h', 4),
        ('i', 1),
        ('j', 8),
        ('k', 5),
        ('l', 1),
        ('m', 3),
        ('n', 1),
        ('o', 1),
        ('p', 3),
        ('q', 10),
        ('r', 1),
        ('s', 1),
        ('t', 1),
        ('u', 1),
        ('v', 4),
        ('w', 4),
        ('x', 8),
        ('y', 4),
        ('z', 10),
    ]);

    assert_eq!(expected, etl::transform(&input));
}

fn input_from(v: &[(i32, Vec<char>)]) -> BTreeMap<i32, Vec<char>> {
    v.iter().cloned().collect()
}

fn expected_from(v: &[(char, i32)]) -> BTreeMap<char, i32> {
    v.iter().cloned().collect()
}
