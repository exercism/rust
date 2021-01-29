use two_bucket::{solve, Bucket, BucketStats};

#[test]
fn test_case_1() {
    assert_eq!(
        solve(3, 5, 1, &Bucket::One),
        Some(BucketStats {
            moves: 4,
            goal_bucket: Bucket::One,
            other_bucket: 5,
        })
    );
}

#[test]
#[ignore]
fn test_case_2() {
    assert_eq!(
        solve(3, 5, 1, &Bucket::Two),
        Some(BucketStats {
            moves: 8,
            goal_bucket: Bucket::Two,
            other_bucket: 3,
        })
    );
}

#[test]
#[ignore]
fn test_case_3() {
    assert_eq!(
        solve(7, 11, 2, &Bucket::One),
        Some(BucketStats {
            moves: 14,
            goal_bucket: Bucket::One,
            other_bucket: 11,
        })
    );
}

#[test]
#[ignore]
fn test_case_4() {
    assert_eq!(
        solve(7, 11, 2, &Bucket::Two),
        Some(BucketStats {
            moves: 18,
            goal_bucket: Bucket::Two,
            other_bucket: 7,
        })
    );
}

#[test]
#[ignore]
fn goal_equal_to_start_bucket() {
    assert_eq!(
        solve(1, 3, 3, &Bucket::Two),
        Some(BucketStats {
            moves: 1,
            goal_bucket: Bucket::Two,
            other_bucket: 0,
        })
    );
}

#[test]
#[ignore]
fn goal_equal_to_other_bucket() {
    assert_eq!(
        solve(2, 3, 3, &Bucket::One),
        Some(BucketStats {
            moves: 2,
            goal_bucket: Bucket::Two,
            other_bucket: 2,
        })
    );
}

#[test]
#[ignore]
fn not_possible_to_reach_the_goal() {
    assert_eq!(solve(6, 15, 5, &Bucket::One), None);
}

#[test]
#[ignore]
fn with_same_buckets_but_different_goal_then_it_is_possible() {
    assert_eq!(
        solve(6, 15, 9, &Bucket::One),
        Some(BucketStats {
            moves: 10,
            goal_bucket: Bucket::Two,
            other_bucket: 0,
        })
    );
}
