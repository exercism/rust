use two_bucket::*;

#[test]
fn measure_using_bucket_one_of_size_3_and_bucket_two_of_size_5_start_with_bucket_one() {
    let output = solve(3, 5, 1, &Bucket::One);
    let expected = Some(BucketStats {
        moves: 4,
        goal_bucket: Bucket::One,
        other_bucket: 5,
    });
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn measure_using_bucket_one_of_size_3_and_bucket_two_of_size_5_start_with_bucket_two() {
    let output = solve(3, 5, 1, &Bucket::Two);
    let expected = Some(BucketStats {
        moves: 8,
        goal_bucket: Bucket::Two,
        other_bucket: 3,
    });
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn measure_using_bucket_one_of_size_7_and_bucket_two_of_size_11_start_with_bucket_one() {
    let output = solve(7, 11, 2, &Bucket::One);
    let expected = Some(BucketStats {
        moves: 14,
        goal_bucket: Bucket::One,
        other_bucket: 11,
    });
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn measure_using_bucket_one_of_size_7_and_bucket_two_of_size_11_start_with_bucket_two() {
    let output = solve(7, 11, 2, &Bucket::Two);
    let expected = Some(BucketStats {
        moves: 18,
        goal_bucket: Bucket::Two,
        other_bucket: 7,
    });
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn measure_one_step_using_bucket_one_of_size_1_and_bucket_two_of_size_3_start_with_bucket_two() {
    let output = solve(1, 3, 3, &Bucket::Two);
    let expected = Some(BucketStats {
        moves: 1,
        goal_bucket: Bucket::Two,
        other_bucket: 0,
    });
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn measure_using_bucket_one_of_size_2_and_bucket_two_of_size_3_start_with_bucket_one_and_end_with_bucket_two(
) {
    let output = solve(2, 3, 3, &Bucket::One);
    let expected = Some(BucketStats {
        moves: 2,
        goal_bucket: Bucket::Two,
        other_bucket: 2,
    });
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn not_possible_to_reach_the_goal() {
    let output = solve(6, 15, 5, &Bucket::One);
    let expected = None;
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn with_the_same_buckets_but_a_different_goal_then_it_is_possible() {
    let output = solve(6, 15, 9, &Bucket::One);
    let expected = Some(BucketStats {
        moves: 10,
        goal_bucket: Bucket::Two,
        other_bucket: 0,
    });
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn goal_larger_than_both_buckets_is_impossible() {
    let output = solve(5, 7, 8, &Bucket::One);
    let expected = None;
    assert_eq!(output, expected);
}
