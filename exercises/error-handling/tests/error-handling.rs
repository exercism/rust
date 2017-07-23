extern crate error_handling as eh;

use eh::IceCream;
use eh::IceCream::*;
use eh::IceCreamTruck;

#[test]
fn test_chocolate_decode() {
    assert_eq!(IceCream::get_flavor("Chocolate"), Ok(Chocolate));
    assert_eq!(IceCream::get_flavor("chocolate"), Ok(Chocolate));
    assert_eq!(IceCream::get_flavor("choc"), Ok(Chocolate));
    assert_eq!(IceCream::get_flavor("C"), Ok(Chocolate));
    assert_eq!(IceCream::get_flavor("c"), Ok(Chocolate));
    assert!(IceCream::get_flavor("no flavor").is_err());
}

#[test]
#[ignore]
fn test_strawberry_decode() {
    assert_eq!(IceCream::get_flavor("Strawberry"), Ok(Strawberry));
    assert_eq!(IceCream::get_flavor("strawberry"), Ok(Strawberry));
    assert_eq!(IceCream::get_flavor("straw"), Ok(Strawberry));
    assert_eq!(IceCream::get_flavor("S"), Ok(Strawberry));
    assert_eq!(IceCream::get_flavor("s"), Ok(Strawberry));
    assert!(IceCream::get_flavor("no flavor").is_err());
}

#[test]
#[ignore]
fn test_vanilla_decode() {
    assert_eq!(IceCream::get_flavor("Vanilla"), Ok(Vanilla));
    assert_eq!(IceCream::get_flavor("vanilla"), Ok(Vanilla));
    assert_eq!(IceCream::get_flavor("V"), Ok(Vanilla));
    assert_eq!(IceCream::get_flavor("v"), Ok(Vanilla));
    assert_eq!(IceCream::get_flavor("plain"), Ok(Vanilla));
    assert!(IceCream::get_flavor("no flavor").is_err());
}

#[test]
#[ignore]
fn test_buy_chocolate_sufficient_qty() {
    let mut ict = IceCreamTruck::new(3, 1, 1);
    assert_eq!(ict.buy(2, Chocolate), Some(2));
    assert_eq!(ict.chocolate, 1);
}

#[test]
#[ignore]
fn test_buy_chocolate_reduced_qty() {
    let mut ict = IceCreamTruck::new(1, 0, 0);
    assert_eq!(ict.buy(2, Chocolate), Some(1));
    assert_eq!(ict.chocolate, 0);
}

#[test]
#[ignore]
fn test_buy_chocolate_no_qty() {
    let mut ict = IceCreamTruck::new(0, 1, 1);
    assert_eq!(ict.buy(2, Chocolate), None);
}

#[test]
#[ignore]
fn test_buy_strawberry_sufficient_qty() {
    let mut ict = IceCreamTruck::new(1, 1, 3);
    assert_eq!(ict.buy(2, Strawberry), Some(2));
    assert_eq!(ict.strawberry, 1);
}

#[test]
#[ignore]
fn test_buy_strawberry_reduced_qty() {
    let mut ict = IceCreamTruck::new(3, 3, 1);
    assert_eq!(ict.buy(2, Strawberry), Some(1));
    assert_eq!(ict.strawberry, 0);
}

#[test]
#[ignore]
fn test_buy_strawberry_no_qty() {
    let mut ict = IceCreamTruck::new(1, 1, 0);
    assert_eq!(ict.buy(2, Strawberry), None);   
}

#[test]
#[ignore]
fn test_buy_vanilla_sufficient_qty() {
    let mut ict = IceCreamTruck::new(0, 3, 0);
    assert_eq!(ict.buy(2, Vanilla), Some(2));
    assert_eq!(ict.vanilla, 1);
}

#[test]
#[ignore]
fn test_buy_vanilla_reduced_qty() {
    let mut ict = IceCreamTruck::new(0, 1, 0);
    assert_eq!(ict.buy(2, Vanilla), Some(1));
    assert_eq!(ict.vanilla, 0);
}

#[test]
#[ignore]
fn test_buy_vanilla_no_qty() {
    let mut ict = IceCreamTruck::new(1, 0, 1);
    assert_eq!(ict.buy(2, Vanilla), None);
}

#[test]
#[ignore]
fn test_process_order_empty() {
    let mut ict = IceCreamTruck::new(5, 4, 6);
    assert_eq!(ict.process_order(vec![]), vec![]);
}

#[test]
#[ignore]
fn test_process_order_one() {
    let mut ict = IceCreamTruck::new(5, 4, 6);
    assert_eq!(ict.process_order(vec![("straw", 2)]), vec![Some(2)]);
}

#[test]
#[ignore]
fn test_process_order_large() {
    let mut ict = IceCreamTruck::new(5, 4, 6);
    let order = vec![("straw", 2),
                     ("plain", 1),
                     ("cookie dough", 3),
                     ("C", 6),
                     ("S", 5),
                     ("Vanilla", 3),
                     ("Chocolate", 2),
                     ("vanilla", 1),
                     ("s", 4)];
    let expected = vec![Some(2), Some(1), None, Some(5), Some(4),
                        Some(3), None, None, None];
    assert_eq!(ict.process_order(order), expected);
}
