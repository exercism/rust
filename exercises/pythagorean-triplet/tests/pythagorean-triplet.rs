extern crate pythagorean_triplet;

use pythagorean_triplet::find;

fn process_tripletswithsum_case(sum: u32, expected: Option<Vec<[u32; 3]>>) {
    if let Some(expected_triplets) = expected {
        let triplets = find(sum).unwrap();

        assert!(
            expected_triplets
                .iter()
                .all(|expected_triplet| triplets.contains(expected_triplet))
        );
    } else {
        assert!(find(sum).is_none());
    }
}

#[test]
fn test_several_matching_triplets() {
    process_tripletswithsum_case(
        840,
        Some(vec![
            [40, 399, 401],
            [56, 390, 394],
            [105, 360, 375],
            [120, 350, 370],
            [140, 336, 364],
            [168, 315, 357],
            [210, 280, 350],
            [240, 252, 348],
        ]),
    );
}

#[test]
#[ignore]
fn test_returns_all_matching_triplets() {
    process_tripletswithsum_case(90, Some(vec![[9, 40, 41], [15, 36, 39]]));
}

#[test]
#[ignore]
fn test_triplets_whose_sum_is_108() {
    process_tripletswithsum_case(108, Some(vec![[27, 36, 45]]));
}

#[test]
#[ignore]
fn test_triplets_for_large_number() {
    process_tripletswithsum_case(
        30000,
        Some(vec![
            [1200, 14375, 14425],
            [1875, 14000, 14125],
            [5000, 12000, 13000],
            [6000, 11250, 12750],
            [7500, 10000, 12500],
        ]),
    );
}

#[test]
#[ignore]
fn test_no_matching_triplets_for_1001() {
    process_tripletswithsum_case(1001, None);
}

#[test]
#[ignore]
fn test_triplets_whose_sum_is_1000() {
    process_tripletswithsum_case(1000, Some(vec![[200, 375, 425]]));
}

#[test]
#[ignore]
fn test_triplets_whose_sum_is_12() {
    process_tripletswithsum_case(12, Some(vec![[3, 4, 5]]));
}
