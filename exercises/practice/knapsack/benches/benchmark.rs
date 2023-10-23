#![feature(test)]
extern crate knapsack;
extern crate test;

use knapsack::Item;
use std::cmp;
use std::hint::black_box;
use test::Bencher;

#[bench]
fn bench_fifteen(b: &mut Bencher) {
    b.iter(|| knapsack::maximum_value(MAX_WEIGHT, black_box(&ITEMS)));
}
#[bench]
fn bench_fifteen_brute_force(b: &mut Bencher) {
    b.iter(|| maximum_value(MAX_WEIGHT, black_box(&ITEMS)));
}

/// Simple and inefficient brute force. Can it be beat?
pub fn maximum_value(max_weight: u32, items: &[Item]) -> u32 {
    if items.len() == 0 {
        return 0;
    }

    // Put the item in the knapsack (if fits)
    let mut value_taking = 0;
    if items[0].weight <= max_weight {
        value_taking = items[0].value + maximum_value(max_weight - items[0].weight, &items[1..]);
    }

    // Don't put the item in the knapsack
    let value_not_taking = maximum_value(max_weight, &items[1..]);

    // Return the value of the richest knapsack
    cmp::max(value_taking, value_not_taking)
}

const MAX_WEIGHT: u32 = 750;
const ITEMS: [Item; 15] = [
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
