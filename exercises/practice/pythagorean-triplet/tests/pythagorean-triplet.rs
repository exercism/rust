use pythagorean_triplet::find;
use std::collections::HashSet;

fn process_tripletswithsum_case(sum: u32, expected: &[[u32; 3]]) {
    let triplets = find(sum);

    if !expected.is_empty() {
        let expected: HashSet<_> = expected.iter().cloned().collect();

        assert_eq!(expected, triplets);
    } else {
        assert!(triplets.is_empty());
    }
}

#[test]
fn test_triplets_whose_sum_is_12() {
    process_tripletswithsum_case(12, &[[3, 4, 5]]);
}

#[test]
#[ignore]
fn test_triplets_whose_sum_is_108() {
    process_tripletswithsum_case(108, &[[27, 36, 45]]);
}

#[test]
#[ignore]
fn test_triplets_whose_sum_is_1000() {
    process_tripletswithsum_case(1000, &[[200, 375, 425]]);
}

#[test]
#[ignore]
fn test_no_matching_triplets_for_1001() {
    process_tripletswithsum_case(1001, &[]);
}

#[test]
#[ignore]
fn test_returns_all_matching_triplets() {
    process_tripletswithsum_case(90, &[[9, 40, 41], [15, 36, 39]]);
}

#[test]
#[ignore]
fn test_several_matching_triplets() {
    process_tripletswithsum_case(
        840,
        &[
            [40, 399, 401],
            [56, 390, 394],
            [105, 360, 375],
            [120, 350, 370],
            [140, 336, 364],
            [168, 315, 357],
            [210, 280, 350],
            [240, 252, 348],
        ],
    );
}

#[test]
#[ignore]
fn test_triplets_for_large_number() {
    process_tripletswithsum_case(
        30_000,
        &[
            [1200, 14_375, 14_425],
            [1875, 14_000, 14_125],
            [5000, 12_000, 13_000],
            [6000, 11_250, 12_750],
            [7500, 10_000, 12_500],
        ],
    );
}
