use std::collections::BTreeSet;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn sum_of_multiples_from_factors(limit: u32, factors: &[u32]) -> u32 {
    factors
        .iter()
        .filter(|&&factor| factor != 0)
        .flat_map(|&factor| multiples(limit, factor))
        .collect::<BTreeSet<_>>()
        .into_iter()
        .sum()
}

fn multiples(limit: u32, factor: u32) -> impl Iterator<Item = u32> {
    (factor..)
        .step_by(factor as usize)
        .take_while(move |&n| n < limit)
}

pub fn sum_of_multiples_from_range(limit: u32, factors: &[u32]) -> u32 {
    (1..limit)
        .filter(|&n| factors.iter().any(|&f| f != 0 && n % f == 0))
        .sum()
}

fn from_factors_benchmark(c: &mut Criterion) {
    c.bench_function("from_factors: much_larger_factors", |b| {
        b.iter(|| sum_of_multiples_from_factors(black_box(10000), black_box(&[43, 47])))
    });
    c.bench_function(
        "from_factors: solutions_using_include_exclude_must_extend_to_cardinality_greater_than_3",
        |b| {
            b.iter(|| sum_of_multiples_from_factors(black_box(10000), black_box(&[2, 3, 5, 7, 11])))
        },
    );
}

fn from_range_benchmark(c: &mut Criterion) {
    c.bench_function("from_range: much_larger_factors", |b| {
        b.iter(|| sum_of_multiples_from_range(black_box(10000), black_box(&[43, 47])))
    });
    c.bench_function(
        "from_range: solutions_using_include_exclude_must_extend_to_cardinality_greater_than_3",
        |b| b.iter(|| sum_of_multiples_from_range(black_box(10000), black_box(&[2, 3, 5, 7, 11]))),
    );
}

criterion_group!(benches, from_factors_benchmark, from_range_benchmark);
criterion_main!(benches);
