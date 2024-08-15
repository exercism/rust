use knapsack::*;

#[test]
fn no_items() {
    let max_weight = 100;
    let items = [];
    let output = maximum_value(max_weight, &items);
    let expected = 0;
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn one_item_too_heavy() {
    let max_weight = 10;
    let items = [Item {
        weight: 100,
        value: 1,
    }];
    let output = maximum_value(max_weight, &items);
    let expected = 0;
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn five_items_cannot_be_greedy_by_weight() {
    let max_weight = 10;
    let items = [
        Item {
            weight: 2,
            value: 5,
        },
        Item {
            weight: 2,
            value: 5,
        },
        Item {
            weight: 2,
            value: 5,
        },
        Item {
            weight: 2,
            value: 5,
        },
        Item {
            weight: 10,
            value: 21,
        },
    ];
    let output = maximum_value(max_weight, &items);
    let expected = 21;
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn five_items_cannot_be_greedy_by_value() {
    let max_weight = 10;
    let items = [
        Item {
            weight: 2,
            value: 20,
        },
        Item {
            weight: 2,
            value: 20,
        },
        Item {
            weight: 2,
            value: 20,
        },
        Item {
            weight: 2,
            value: 20,
        },
        Item {
            weight: 10,
            value: 50,
        },
    ];
    let output = maximum_value(max_weight, &items);
    let expected = 80;
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn example_knapsack() {
    let max_weight = 10;
    let items = [
        Item {
            weight: 5,
            value: 10,
        },
        Item {
            weight: 4,
            value: 40,
        },
        Item {
            weight: 6,
            value: 30,
        },
        Item {
            weight: 4,
            value: 50,
        },
    ];
    let output = maximum_value(max_weight, &items);
    let expected = 90;
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn test_8_items() {
    let max_weight = 104;
    let items = [
        Item {
            weight: 25,
            value: 350,
        },
        Item {
            weight: 35,
            value: 400,
        },
        Item {
            weight: 45,
            value: 450,
        },
        Item {
            weight: 5,
            value: 20,
        },
        Item {
            weight: 25,
            value: 70,
        },
        Item {
            weight: 3,
            value: 8,
        },
        Item {
            weight: 2,
            value: 5,
        },
        Item {
            weight: 2,
            value: 5,
        },
    ];
    let output = maximum_value(max_weight, &items);
    let expected = 900;
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn test_15_items() {
    let max_weight = 750;
    let items = [
        Item {
            weight: 70,
            value: 135,
        },
        Item {
            weight: 73,
            value: 139,
        },
        Item {
            weight: 77,
            value: 149,
        },
        Item {
            weight: 80,
            value: 150,
        },
        Item {
            weight: 82,
            value: 156,
        },
        Item {
            weight: 87,
            value: 163,
        },
        Item {
            weight: 90,
            value: 173,
        },
        Item {
            weight: 94,
            value: 184,
        },
        Item {
            weight: 98,
            value: 192,
        },
        Item {
            weight: 106,
            value: 201,
        },
        Item {
            weight: 110,
            value: 210,
        },
        Item {
            weight: 113,
            value: 214,
        },
        Item {
            weight: 115,
            value: 221,
        },
        Item {
            weight: 118,
            value: 229,
        },
        Item {
            weight: 120,
            value: 240,
        },
    ];
    let output = maximum_value(max_weight, &items);
    let expected = 1458;
    assert_eq!(output, expected);
}
