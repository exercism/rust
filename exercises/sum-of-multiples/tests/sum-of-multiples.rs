extern crate sum_of_multiples;

use sum_of_multiples::*;

#[test]
fn multiples_one() {
    assert_eq!(0, sum_of_multiples(1, &vec![3, 5]))
}

#[test]
#[ignore]
fn multiples_two() {
    assert_eq!(3, sum_of_multiples(4, &vec![3, 5]))
}

#[test]
#[ignore]
fn multiples_three() {
    assert_eq!(23, sum_of_multiples(10, &vec![3, 5]))
}

#[test]
#[ignore]
fn multiples_four() {
    assert_eq!(2318, sum_of_multiples(100, &vec![3, 5]))
}

#[test]
#[ignore]
fn multiples_five() {
    assert_eq!(233168, sum_of_multiples(1000, &vec![3, 5]))
}

#[test]
#[ignore]
fn multiples_six() {
    assert_eq!(51, sum_of_multiples(20, &vec![7, 13, 17]))
}

#[test]
#[ignore]
fn multiples_seven() {
    assert_eq!(30, sum_of_multiples(15, &vec![4, 6]))
}

#[test]
#[ignore]
fn multiples_eight() {
    assert_eq!(4419, sum_of_multiples(150, &vec![5, 6, 8]))
}

#[test]
#[ignore]
fn multiples_nine() {
    assert_eq!(275, sum_of_multiples(51, &vec![5, 25]))
}

#[test]
#[ignore]
fn multiples_ten() {
    assert_eq!(2203160, sum_of_multiples(10000, &vec![43, 47]))
}

#[test]
#[ignore]
fn multiples_eleven() {
    assert_eq!(4950, sum_of_multiples(100, &vec![1]))
}

#[test]
#[ignore]
fn multiples_twelve() {
    assert_eq!(0, sum_of_multiples(10000, &vec![]))
}
