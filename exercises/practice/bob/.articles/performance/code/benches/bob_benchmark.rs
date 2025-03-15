use bob_reply::*;
use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};

const BASIC_LOREM_IPSUM_100: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. In arcu justo, facilisis vel leo et sapien";
const BASIC_LOREM_IPSUM_1000: &str = include_str!("./1000_lorem_ipsum.txt");
const BASIC_LOREM_IPSUM_10000: &str = include_str!("./10000_lorem_ipsum.txt");

const BASIC_LOREM_IPSUM_NON_ASCII_100: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing èlit. În arcu justo, facilisis vel leo et sapien";
const BASIC_LOREM_IPSUM_NON_ASCII_1000: &str = include_str!("./1000_lorem_ipsum_non_ascii.txt");
const BASIC_LOREM_IPSUM_NON_ASCII_10000: &str = include_str!("./10000_lorem_ipsum_non_ascii.txt");

pub fn reply_for_normal_question(c: &mut Criterion) {
    let mut group = c.benchmark_group("Reply: \"Sure.\" (normal question)");
    let inputs = vec![
        (0, String::from("?")),
        (1, String::from("a?")),
        (10, String::from("Hello there?")), // ~10
        (100, format!("{BASIC_LOREM_IPSUM_100}?")),
        (1000, format!("{BASIC_LOREM_IPSUM_1000}?")),
        (10000, format!("{BASIC_LOREM_IPSUM_10000}?")),
    ];
    for (i, input) in inputs {
        group.bench_with_input(BenchmarkId::new("Array", i), &input, |b, input| {
            b.iter(|| reply_array(input))
        });
        group.bench_with_input(BenchmarkId::new("If Chain", i), &input, |b, input| {
            b.iter(|| reply_if_chain(input))
        });
        group.bench_with_input(BenchmarkId::new("Match", i), &input, |b, input| {
            b.iter(|| reply_match(input))
        });
        group.bench_with_input(BenchmarkId::new("State Machine", i), &input, |b, input| {
            b.iter(|| reply_state_machine(input))
        });
        group.bench_with_input(
            BenchmarkId::new("State Machine (ascii opt.)", i),
            &input,
            |b, input| b.iter(|| reply_state_machine_ascii_optimized(input)),
        );
    }
}

pub fn reply_for_normal_question_non_ascii(c: &mut Criterion) {
    let mut group = c.benchmark_group("Reply: \"Sure.\" (normal question + non-ascii)");
    let inputs = vec![
        (0, String::from("?")),
        (1, String::from("a?")),
        (10, String::from("Hello there?")), // ~10
        (100, format!("{BASIC_LOREM_IPSUM_NON_ASCII_100}?")),
        (1000, format!("{BASIC_LOREM_IPSUM_NON_ASCII_1000}?")),
        (10000, format!("{BASIC_LOREM_IPSUM_NON_ASCII_10000}?")),
    ];
    for (i, input) in inputs {
        group.bench_with_input(BenchmarkId::new("Array", i), &input, |b, input| {
            b.iter(|| reply_array(input))
        });
        group.bench_with_input(BenchmarkId::new("If Chain", i), &input, |b, input| {
            b.iter(|| reply_if_chain(input))
        });
        group.bench_with_input(BenchmarkId::new("Match", i), &input, |b, input| {
            b.iter(|| reply_match(input))
        });
        group.bench_with_input(BenchmarkId::new("State Machine", i), &input, |b, input| {
            b.iter(|| reply_state_machine(input))
        });
        group.bench_with_input(
            BenchmarkId::new("State Machine (ascii opt.)", i),
            &input,
            |b, input| b.iter(|| reply_state_machine_ascii_optimized(input)),
        );
    }
}

pub fn reply_for_yell(c: &mut Criterion) {
    let mut group = c.benchmark_group("Reply: \"Whoa, chill out!\" (yell)");
    let inputs = vec![
        (1, String::from("A")),
        (10, String::from("HELLO THERE")), // ~10
        (100, format!("{}", BASIC_LOREM_IPSUM_100.to_uppercase())),
        (1000, format!("{}", BASIC_LOREM_IPSUM_1000.to_uppercase())),
        (10000, format!("{}", BASIC_LOREM_IPSUM_10000.to_uppercase())),
    ];
    for (i, input) in inputs {
        group.bench_with_input(BenchmarkId::new("Array", i), &input, |b, input| {
            b.iter(|| reply_array(input))
        });
        group.bench_with_input(BenchmarkId::new("If Chain", i), &input, |b, input| {
            b.iter(|| reply_if_chain(input))
        });
        group.bench_with_input(BenchmarkId::new("Match", i), &input, |b, input| {
            b.iter(|| reply_match(input))
        });
        group.bench_with_input(BenchmarkId::new("State Machine", i), &input, |b, input| {
            b.iter(|| reply_state_machine(input))
        });
        group.bench_with_input(
            BenchmarkId::new("State Machine (ascii opt.)", i),
            &input,
            |b, input| b.iter(|| reply_state_machine_ascii_optimized(input)),
        );
    }
}

pub fn reply_for_yell_non_ascii(c: &mut Criterion) {
    let mut group = c.benchmark_group("Reply: \"Whoa, chill out!\" (yell + non-ascii)");
    let inputs = vec![
        (1, String::from("A")),
        (10, String::from("HELLO THERE")), // ~10
        (
            100,
            format!("{}", BASIC_LOREM_IPSUM_NON_ASCII_100.to_uppercase()),
        ),
        (
            1000,
            format!("{}", BASIC_LOREM_IPSUM_NON_ASCII_1000.to_uppercase()),
        ),
        (
            10000,
            format!("{}", BASIC_LOREM_IPSUM_NON_ASCII_10000.to_uppercase()),
        ),
    ];
    for (i, input) in inputs {
        group.bench_with_input(BenchmarkId::new("Array", i), &input, |b, input| {
            b.iter(|| reply_array(input))
        });
        group.bench_with_input(BenchmarkId::new("If Chain", i), &input, |b, input| {
            b.iter(|| reply_if_chain(input))
        });
        group.bench_with_input(BenchmarkId::new("Match", i), &input, |b, input| {
            b.iter(|| reply_match(input))
        });
        group.bench_with_input(BenchmarkId::new("State Machine", i), &input, |b, input| {
            b.iter(|| reply_state_machine(input))
        });
        group.bench_with_input(
            BenchmarkId::new("State Machine (ascii opt.)", i),
            &input,
            |b, input| b.iter(|| reply_state_machine_ascii_optimized(input)),
        );
    }
}

pub fn reply_for_yell_question(c: &mut Criterion) {
    let mut group =
        c.benchmark_group("Reply: \"Calm down, I know what I'm doing!\" (yell question)");
    let inputs = vec![
        (1, String::from("A?")),
        (10, String::from("HELLO THERE?")), // ~10
        (100, format!("{}?", BASIC_LOREM_IPSUM_100.to_uppercase())),
        (1000, format!("{}?", BASIC_LOREM_IPSUM_1000.to_uppercase())),
        (
            10000,
            format!("{}?", BASIC_LOREM_IPSUM_10000.to_uppercase()),
        ),
    ];
    for (i, input) in inputs {
        group.bench_with_input(BenchmarkId::new("Array", i), &input, |b, input| {
            b.iter(|| reply_array(input))
        });
        group.bench_with_input(BenchmarkId::new("If Chain", i), &input, |b, input| {
            b.iter(|| reply_if_chain(input))
        });
        group.bench_with_input(BenchmarkId::new("Match", i), &input, |b, input| {
            b.iter(|| reply_match(input))
        });
        group.bench_with_input(BenchmarkId::new("State Machine", i), &input, |b, input| {
            b.iter(|| reply_state_machine(input))
        });
        group.bench_with_input(
            BenchmarkId::new("State Machine (ascii opt.)", i),
            &input,
            |b, input| b.iter(|| reply_state_machine_ascii_optimized(input)),
        );
    }
}

pub fn reply_for_yell_question_non_ascii(c: &mut Criterion) {
    let mut group = c.benchmark_group(
        "Reply: \"Calm down, I know what I'm doing!\" (yell question + non-ascii)",
    );
    let inputs = vec![
        (1, String::from("A?")),
        (10, String::from("HELLO THERE?")), // ~10
        (
            100,
            format!("{}?", BASIC_LOREM_IPSUM_NON_ASCII_100.to_uppercase()),
        ),
        (
            1000,
            format!("{}?", BASIC_LOREM_IPSUM_NON_ASCII_1000.to_uppercase()),
        ),
        (
            10000,
            format!("{}?", BASIC_LOREM_IPSUM_NON_ASCII_10000.to_uppercase()),
        ),
    ];
    for (i, input) in inputs {
        group.bench_with_input(BenchmarkId::new("Array", i), &input, |b, input| {
            b.iter(|| reply_array(input))
        });
        group.bench_with_input(BenchmarkId::new("If Chain", i), &input, |b, input| {
            b.iter(|| reply_if_chain(input))
        });
        group.bench_with_input(BenchmarkId::new("Match", i), &input, |b, input| {
            b.iter(|| reply_match(input))
        });
        group.bench_with_input(BenchmarkId::new("State Machine", i), &input, |b, input| {
            b.iter(|| reply_state_machine(input))
        });
        group.bench_with_input(
            BenchmarkId::new("State Machine (ascii opt.)", i),
            &input,
            |b, input| b.iter(|| reply_state_machine_ascii_optimized(input)),
        );
    }
}

pub fn reply_for_silence(c: &mut Criterion) {
    let mut group = c.benchmark_group("Reply: \"Fine.Be that way!\" (empty array or whitespace)");
    let inputs = vec![
        "".to_string(),
        " ".to_string(),
        " ".repeat(10),
        " ".repeat(100),
        " ".repeat(1000),
        " ".repeat(10000),
    ];
    for i in inputs {
        group.bench_with_input(BenchmarkId::new("Array", i.len()), &i, |b, i| {
            b.iter(|| reply_array(i))
        });
        group.bench_with_input(BenchmarkId::new("If Chain", i.len()), &i, |b, i| {
            b.iter(|| reply_if_chain(i))
        });
        group.bench_with_input(BenchmarkId::new("Match", i.len()), &i, |b, i| {
            b.iter(|| reply_match(i))
        });
        group.bench_with_input(BenchmarkId::new("State Machine", i.len()), &i, |b, i| {
            b.iter(|| reply_state_machine(i))
        });
        group.bench_with_input(
            BenchmarkId::new("State Machine (ascii opt)", i.len()),
            &i,
            |b, i| b.iter(|| reply_state_machine_ascii_optimized(i)),
        );
    }
}

pub fn reply_for_whatever(c: &mut Criterion) {
    let mut group = c.benchmark_group("Reply: \"Whatever.\" (any other)");
    let inputs = vec![
        (1, String::from("a")),
        (10, String::from("Hello there")), // ~10
        (100, format!("{BASIC_LOREM_IPSUM_100}")),
        (1000, format!("{BASIC_LOREM_IPSUM_1000}")),
        (10000, format!("{BASIC_LOREM_IPSUM_10000}")),
    ];
    for (i, input) in inputs {
        group.bench_with_input(BenchmarkId::new("Array", i), &input, |b, input| {
            b.iter(|| reply_array(input))
        });
        group.bench_with_input(BenchmarkId::new("If Chain", i), &input, |b, input| {
            b.iter(|| reply_if_chain(input))
        });
        group.bench_with_input(BenchmarkId::new("Match", i), &input, |b, input| {
            b.iter(|| reply_match(input))
        });
        group.bench_with_input(BenchmarkId::new("State Machine", i), &input, |b, input| {
            b.iter(|| reply_state_machine(input))
        });
        group.bench_with_input(
            BenchmarkId::new("State Machine (ascii opt.)", i),
            &input,
            |b, input| b.iter(|| reply_state_machine_ascii_optimized(input)),
        );
    }
}

pub fn reply_for_whatever_non_ascii(c: &mut Criterion) {
    let mut group = c.benchmark_group("Reply: \"Whatever.\" (any other + non-ascii)");
    let inputs = vec![
        (1, String::from("a")),
        (10, String::from("Hello there")), // ~10
        (100, format!("{BASIC_LOREM_IPSUM_NON_ASCII_100}")),
        (1000, format!("{BASIC_LOREM_IPSUM_NON_ASCII_1000}")),
        (10000, format!("{BASIC_LOREM_IPSUM_NON_ASCII_10000}")),
    ];
    for (i, input) in inputs {
        group.bench_with_input(BenchmarkId::new("Array", i), &input, |b, input| {
            b.iter(|| reply_array(input))
        });
        group.bench_with_input(BenchmarkId::new("If Chain", i), &input, |b, input| {
            b.iter(|| reply_if_chain(input))
        });
        group.bench_with_input(BenchmarkId::new("Match", i), &input, |b, input| {
            b.iter(|| reply_match(input))
        });
        group.bench_with_input(BenchmarkId::new("State Machine", i), &input, |b, input| {
            b.iter(|| reply_state_machine(input))
        });
        group.bench_with_input(
            BenchmarkId::new("State Machine (ascii opt.)", i),
            &input,
            |b, input| b.iter(|| reply_state_machine_ascii_optimized(input)),
        );
    }
}

criterion_group!(
    benches,
    reply_for_normal_question,
    reply_for_normal_question_non_ascii,
    reply_for_yell,
    reply_for_yell_non_ascii,
    reply_for_yell_question,
    reply_for_yell_question_non_ascii,
    reply_for_silence,
    reply_for_whatever,
    reply_for_whatever_non_ascii
);
criterion_main!(benches);
