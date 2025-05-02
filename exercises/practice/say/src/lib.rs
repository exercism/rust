use std::iter::successors;

const UP_TO_NINETEEN: [&'static str; 20] = [
    "zero", 
    "one", 
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen"];

const TENS: [&'static str; 10] = [
    "zero", // for indexing, never accessed
    "ten",
    "twenty",
    "thirty",
    "forty",
    "fifty",
    "sixty",
    "seventy",
    "eighty",
    "ninety"
];

const THOUSANDS: [&'static str; 7] = [
    "zero", // for indexing, never accessed
    "thousand",
    "million",
    "billion",
    "trillion", 
    "quadrillion", 
    "quintillion"
];

pub fn encode(n: u64) -> String {
    match n {
        0..20 => UP_TO_NINETEEN[n as usize].to_string(),
        20..=99 => {
            let nb_tens = (n / 10) as usize;
            match n % 10 {
                0 => TENS[nb_tens].to_string(),
                units => format!("{}-{}", TENS[nb_tens], UP_TO_NINETEEN[units as usize]) 
            }
        },
        100..=999 => {
            format_num(n, 100, "hundred")
        },
        _ => {
            // dealing with the thousands. 100_000 is a hundrer thousands
            // Try to do it with a for loop, but exponential increment not simple
            // Alternative to `.by_step()` for exponential growth is to use 
            // successors ! 
            let order = successors(Some(1u64), |n| n.checked_mul(1000))
            .find(|&i| i > n / 1000)
            .unwrap();
            println!("{} {}", order, order / 1000);
            format_num(n, order, THOUSANDS[((order as f64).log10() / (1000 as f64).log10()) as usize])
        }
    }
}

fn format_num(n: u64, div: u64, div_desc: &str) -> String {
    match (n / div, n % div) {
        (nb_div, 0) => format!("{} {}", UP_TO_NINETEEN[nb_div as usize].to_string(), div_desc.to_string() ),
        (nb_div, remainder) => format!("{} {} {}", encode(nb_div), div_desc.to_string(), encode(remainder))
    }
}

// Inspired by https://exercism.org/tracks/rust/exercises/say/solutions/LudwigStecher
// Last arm of the match end up been the simplest option
// Tried for loop that returns value but https://users.rust-lang.org/t/how-to-return-value-from-for-loop/79225/2
// Tried ().by_step().find() but exponential growth not possible with equivalent of by_step()
// Turns out the .zip() choice is not that complicated since to find related 
// element in THOUSANDS, we have the compute the log in base 1000 (which does not exist for (unsigned) int)
 