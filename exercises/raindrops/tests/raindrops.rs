extern crate raindrops;

#[test]
fn test_1() { assert_eq!("1", raindrops::raindrops(1)); }

#[test]

fn test_3() { assert_eq!("Pling", raindrops::raindrops(3)); }

#[test]

fn test_5() { assert_eq!("Plang", raindrops::raindrops(5)); }

#[test]

fn test_7() { assert_eq!("Plong", raindrops::raindrops(7)); }

#[test]

fn test_6() { assert_eq!("Pling", raindrops::raindrops(6)); }

#[test]

fn test_8() { assert_eq!("8", raindrops::raindrops(8)); }

#[test]

fn test_9() { assert_eq!("Pling", raindrops::raindrops(9)); }

#[test]

fn test_10() { assert_eq!("Plang", raindrops::raindrops(10)); }

#[test]

fn test_14() { assert_eq!("Plong", raindrops::raindrops(14)); }

#[test]

fn test_15() { assert_eq!("PlingPlang", raindrops::raindrops(15)); }

#[test]

fn test_21() { assert_eq!("PlingPlong", raindrops::raindrops(21)); }

#[test]

fn test_25() { assert_eq!("Plang", raindrops::raindrops(25)); }

#[test]

fn test_27() { assert_eq!("Pling", raindrops::raindrops(27)); }

#[test]

fn test_35() { assert_eq!("PlangPlong", raindrops::raindrops(35)); }

#[test]

fn test_49() { assert_eq!("Plong", raindrops::raindrops(49)); }

#[test]

fn test_52() { assert_eq!("52", raindrops::raindrops(52)); }

#[test]

fn test_105() { assert_eq!("PlingPlangPlong", raindrops::raindrops(105)); }

#[test]

fn test_3125() { assert_eq!("Plang", raindrops::raindrops(3125)); }

#[test]

fn test_12121() { assert_eq!("12121", raindrops::raindrops(12121)); }
