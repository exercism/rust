use std::collections::BTreeMap;

#[test]
fn single_letter() {
    let input = BTreeMap::from([(1, vec!['A'])]);

    let expected = BTreeMap::from([('a', 1)]);

    assert_eq!(expected, etl::transform(&input));
}

#[test]
#[ignore]
fn single_score_with_multiple_letters() {
    let input = BTreeMap::from([(1, vec!['A', 'E', 'I', 'O', 'U'])]);

    let expected = BTreeMap::from([('a', 1), ('e', 1), ('i', 1), ('o', 1), ('u', 1)]);

    assert_eq!(expected, etl::transform(&input));
}

#[test]
#[ignore]
fn multiple_scores_with_multiple_letters() {
    let input = BTreeMap::from([(1, vec!['A', 'E']), (2, vec!['D', 'G'])]);

    let expected = BTreeMap::from([('a', 1), ('d', 2), ('e', 1), ('g', 2)]);

    assert_eq!(expected, etl::transform(&input));
}

#[test]
#[ignore]
fn multiple_scores_with_differing_numbers_of_letters() {
    let input = BTreeMap::from([
        (1, vec!['A', 'E', 'I', 'O', 'U', 'L', 'N', 'R', 'S', 'T']),
        (2, vec!['D', 'G']),
        (3, vec!['B', 'C', 'M', 'P']),
        (4, vec!['F', 'H', 'V', 'W', 'Y']),
        (5, vec!['K']),
        (8, vec!['J', 'X']),
        (10, vec!['Q', 'Z']),
    ]);

    let expected = BTreeMap::from([
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
