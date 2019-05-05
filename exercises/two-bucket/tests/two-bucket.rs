use two_bucket::{solve, Bucket, BucketStats};

#[test]
fn test_case_1() {
    assert_eq!(
        BucketStats {
            moves: 4,
            goal_bucket: Bucket::One,
            other_bucket: 5,
        },
        solve(3, 5, 1, &Bucket::One)
    );
}

#[test]
#[ignore]
fn test_case_2() {
    assert_eq!(
        BucketStats {
            moves: 8,
            goal_bucket: Bucket::Two,
            other_bucket: 3,
        },
        solve(3, 5, 1, &Bucket::Two)
    );
}

#[test]
#[ignore]
fn test_case_3() {
    assert_eq!(
        BucketStats {
            moves: 14,
            goal_bucket: Bucket::One,
            other_bucket: 11,
        },
        solve(7, 11, 2, &Bucket::One)
    );
}

#[test]
#[ignore]
fn test_case_4() {
    assert_eq!(
        BucketStats {
            moves: 18,
            goal_bucket: Bucket::Two,
            other_bucket: 7,
        },
        solve(7, 11, 2, &Bucket::Two)
    );
}

#[test]
#[ignore]
fn goal_equal_to_start_bucket() {
    assert_eq!(
        BucketStats {
            moves: 1,
            goal_bucket: Bucket::Two,
            other_bucket: 0,
        },
        solve(1, 3, 3, &Bucket::Two)
    );
}

#[test]
#[ignore]
fn goal_equal_to_other_bucket() {
    assert_eq!(
        BucketStats {
            moves: 2,
            goal_bucket: Bucket::Two,
            other_bucket: 2,
        },
        solve(2, 3, 3, &Bucket::One)
    );
}
