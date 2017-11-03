extern crate two_bucket;

use two_bucket::{solve, BucketStats, Bucket};

#[test]
fn test_case_1() {
    assert_eq!(solve(3, 5, 1, &Bucket::One),
               BucketStats {
                   moves: 4,
                   goal_bucket: Bucket::One,
                   other_bucket: 5,
               });
}

#[test]
#[ignore]
fn test_case_2() {
    assert_eq!(solve(3, 5, 1, &Bucket::Two),
               BucketStats {
                   moves: 8,
                   goal_bucket: Bucket::Two,
                   other_bucket: 3,
               });
}

#[test]
#[ignore]
fn test_case_3() {
    assert_eq!(solve(7, 11, 2, &Bucket::One),
               BucketStats {
                   moves: 14,
                   goal_bucket: Bucket::One,
                   other_bucket: 11,
               });
}

#[test]
#[ignore]
fn test_case_4() {
    assert_eq!(solve(7, 11, 2, &Bucket::Two),
               BucketStats {
                   moves: 18,
                   goal_bucket: Bucket::Two,
                   other_bucket: 7,
               });
}

#[test]
#[ignore]
fn test_case_5() {
    assert_eq!(solve(1, 3, 3, &Bucket::Two),
               BucketStats {
                   moves: 1,
                   goal_bucket: Bucket::Two,
                   other_bucket: 0,
               });
}

#[test]
#[ignore]
fn test_case_6() {
    assert_eq!(solve(2, 3, 3, &Bucket::One),
               BucketStats {
                   moves: 2,
                   goal_bucket: Bucket::Two,
                   other_bucket: 2,
               });
}
