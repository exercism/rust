extern crate complex_number;
use complex_number::Complex;

fn within_range(expected: f64, actual: f64, epsilon: f64) -> bool {
    (expected - actual).abs() < epsilon
}

#[test]
fn test_real_part_of_purely_real_number() {
    let cmplx = Complex::new(1.0, 0.0);
    assert_eq!(cmplx.re(), 1.0);
}

#[test]
#[ignore]
fn test_real_part_of_purely_imaginary_number() {
    let cmplx = Complex::new(0.0, 1.0);
    assert_eq!(cmplx.re(), 0.0);
}

#[test]
#[ignore]
fn test_real_part_of_complex_number() {
    let cmplx = Complex::new(1.0, 2.0);
    assert_eq!(cmplx.re(), 1.0);
}

#[test]
#[ignore]
fn test_imaginary_part_of_purely_real_number() {
    let cmplx = Complex::new(1.0, 0.0);
    assert_eq!(cmplx.im(), 0.0);
}

#[test]
#[ignore]
fn test_imaginary_part_of_purely_imaginary_number() {
    let cmplx = Complex::new(1.0, 0.0);
    assert_eq!(cmplx.im(), 0.0);
}

#[test]
#[ignore]
fn test_imaginary_part_of_complex_number() {
    let cmplx = Complex::new(1.0, 2.0);
    assert_eq!(cmplx.im(), 2.0);
}

#[test]
#[ignore]
fn test_imaginary_unit() {
    let cmplx = Complex::new(0.0, 1.0);
    let result = Complex::new(-1.0, 0.0);
    assert_eq!(cmplx * cmplx, result);
}

#[test]
#[ignore]
fn test_adding_purely_real_numbers() {
    let c1 = Complex::new(1.0, 0.0);
    let c2 = Complex::new(2.0, 0.0);
    let result = Complex::new(3.0, 0.0);
    assert_eq!(c1 + c2, result);
}

#[test]
#[ignore]
fn test_adding_purely_imaginary_numbers() {
    let c1 = Complex::new(0.0, 1.0);
    let c2 = Complex::new(0.0, 2.0);
    let result = Complex::new(0.0, 3.0);
    assert_eq!(c1 + c2, result);
}

#[test]
#[ignore]
fn test_adding_complex_numbers() {
    let c1 = Complex::new(1.0, 2.0);
    let c2 = Complex::new(3.0, 4.0);
    let result = Complex::new(4.0, 6.0);
    assert_eq!(c1 + c2, result);
}

#[test]
#[ignore]
fn test_subtracting_purely_real_numbers() {
    let c1 = Complex::new(1.0, 0.0);
    let c2 = Complex::new(2.0, 0.0);
    let result = Complex::new(-1.0, 0.0);
    assert_eq!(c1 - c2, result);
}

#[test]
#[ignore]
fn test_subtracting_purely_imaginary_numbers() {
    let c1 = Complex::new(0.0, 1.0);
    let c2 = Complex::new(0.0, 2.0);
    let result = Complex::new(0.0, -1.0);
    assert_eq!(c1 - c2, result);
}

#[test]
#[ignore]
fn test_subtracting_complex_numbers() {
    let c1 = Complex::new(1.0, 2.0);
    let c2 = Complex::new(3.0, 4.0);
    let result = Complex::new(-2.0, -2.0);
    assert_eq!(c1 - c2, result);
}

#[test]
#[ignore]
fn test_multiplying_purely_real_numbers() {
    let c1 = Complex::new(1.0, 0.0);
    let c2 = Complex::new(2.0, 0.0);
    let result = Complex::new(2.0, 0.0);
    assert_eq!(c1 * c2, result);
}

#[test]
#[ignore]
fn test_multiplying_purely_imaginary_numbers() {
    let c1 = Complex::new(0.0, 1.0);
    let c2 = Complex::new(0.0, 2.0);
    let result = Complex::new(-2.0, 0.0);
    assert_eq!(c1 * c2, result);
}

#[test]
#[ignore]
fn test_multiplying_complex_numbers() {
    let c1 = Complex::new(1.0, 2.0);
    let c2 = Complex::new(3.0, 4.0);
    let result = Complex::new(-5.0, 10.0);
    assert_eq!(c1 * c2, result);
}

#[test]
#[ignore]
fn test_dividing_purely_real_numbers() {
    let c1 = Complex::new(1.0, 0.0);
    let c2 = Complex::new(2.0, 0.0);
    let result = Complex::new(0.5, 0.0);
    assert_eq!(c1 / c2, result);
}

#[test]
#[ignore]
fn test_dividing_purely_imaginary_numbers() {
    let c1 = Complex::new(0.0, 1.0);
    let c2 = Complex::new(0.0, 2.0);
    let result = Complex::new(0.5, 0.0);
    assert_eq!(c1 / c2, result);
}

#[test]
#[ignore]
fn test_dividing_complex_numbers() {
    let c1 = Complex::new(1.0, 2.0);
    let c2 = Complex::new(3.0, 4.0);
    let result = Complex::new(0.44, 0.08);
    assert_eq!(c1 / c2, result);
}

#[test]
#[ignore]
fn test_absolute_value_of_positive_real_number() {
    let cmplx = Complex::new(5.0, 0.0);
    assert_eq!(cmplx.abs(), 5.0);
}

#[test]
#[ignore]
fn test_absolute_value_of_negative_real_number() {
    let cmplx = Complex::new(-5.0, 0.0);
    assert_eq!(cmplx.abs(), 5.0);
}

#[test]
#[ignore]
fn test_absolute_value_of_positive_imaginary_number() {
    let cmplx = Complex::new(0.0, 5.0);
    assert_eq!(cmplx.abs(), 5.0);
}

#[test]
#[ignore]
fn test_absolute_value_of_negative_imaginary_number() {
    let cmplx = Complex::new(0.0, -5.0);
    assert_eq!(cmplx.abs(), 5.0);
}

#[test]
#[ignore]
fn test_absolute_value_of_complex_number() {
    let cmplx = Complex::new(3.0, 4.0);
    assert_eq!(cmplx.abs(), 5.0);
}

#[test]
#[ignore]
fn test_conjugate_of_purely_real_number() {
    let cmplx = Complex::new(5.0, 0.0);
    let result = Complex::new(5.0, 0.0);
    assert_eq!(cmplx.conjugate(), result);
}

#[test]
#[ignore]
fn test_conjugate_of_purely_imaginary_number() {
    let cmplx = Complex::new(0.0, 5.0);
    let result = Complex::new(0.0, -5.0);
    assert_eq!(cmplx.conjugate(), result);
}

#[test]
#[ignore]
fn test_conjugate_of_complex_number() {
    let cmplx = Complex::new(1.0, 1.0);
    let result = Complex::new(1.0, -1.0);
    assert_eq!(cmplx.conjugate(), result);
}

use std::f64::consts::{E, LN_2, PI};

#[test]
#[ignore]
fn test_eulers_identity() {
    let cmplx = Complex::new(0.0, PI);
    assert!(within_range(-1.0, cmplx.exp().re(), 0.002));
    assert!(within_range(0.0, cmplx.exp().im(), 0.002));
}

#[test]
#[ignore]
fn test_exponential_of_zero() {
    let cmplx = Complex::new(0.0, 0.0);
    assert!(within_range(1.0, cmplx.exp().re(), 0.002));
    assert!(within_range(0.0, cmplx.exp().im(), 0.002));
}

#[test]
#[ignore]
fn test_exponential_of_purely_real_number() {
    let cmplx = Complex::new(1.0, 0.0);
    assert!(within_range(E, cmplx.exp().re(), 0.002));
    assert!(within_range(0.0, cmplx.exp().im(), 0.002));
}

#[test]
#[ignore]
fn test_exponential_of_complex_number() {
    let cmplx = Complex::new(LN_2, PI);
    assert!(within_range(-2.0, cmplx.exp().re(), 0.002));
    assert!(within_range(0.0, cmplx.exp().im(), 0.002));
}
