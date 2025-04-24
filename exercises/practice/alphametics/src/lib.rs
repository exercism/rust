use std::collections::{HashMap, HashSet};
use itertools::Itertools;

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    println!("{}", input);

    let firsts: HashSet<char> = input
    .split(&['+', '='])
    .filter_map(|s| s.trim().chars().next())
    .collect();

    println!("{:?}", firsts);

    let (letters, factors) = parse(input);

    println!("letter: {:?}, factors: {:?}", letters, factors);

    for perm in (0..=9).permutations(letters.len()) {
        if cal_perm_sum(&perm, &factors) == 0 
            && !perm.iter().enumerate().any(|(i, v)| firsts.contains(letters.get(i).unwrap()) && *v == 0) {
                return Some(HashMap::from_iter(
                    perm.iter().enumerate().map(|(i, v)| (*letters.get(i).unwrap(), *v as u8))
                ));
            }
    }
    
    None
}

fn cal_perm_sum(perm: &[i64], factors: &[i64]) -> i64 {
    perm.iter().enumerate().map(|(i, v)| *v * factors.get(i).unwrap()).sum()
}

// 26 + 26 + 766 = 1970

fn parse(input: &str) -> (Vec<char>, Vec<i64>) {
    let mut factors = HashMap::new();
    let mut pos = 0;
    let mut sign = -1;

    for c in input.chars().filter(|c| !c.is_whitespace()).rev() {
        match c {
            '=' => {
                sign = 1;
                pos = 0;
            },
            '+' => {
                pos = 0;
            },
            _ => {
                let factor = factors.entry(c).or_insert(0);
                *factor += sign * 10_i64.pow(pos);
                pos += 1;
            }
        }
    }

    factors.iter().unzip()

}