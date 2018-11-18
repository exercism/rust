use std::collections::HashMap;

pub fn score(word: &str) -> u16 {
    let values = dictionary();
    word.to_lowercase()
        .chars()
        .map(|c| values.get(&c))
        .fold(0, |score, v| score + v.unwrap_or(&0))
}

fn dictionary() -> HashMap<char, u16> {
    let mut values = HashMap::new();
    values.insert('a', 1);
    values.insert('b', 3);
    values.insert('c', 3);
    values.insert('d', 2);
    values.insert('e', 1);
    values.insert('f', 4);
    values.insert('g', 2);
    values.insert('h', 4);
    values.insert('i', 1);
    values.insert('j', 8);
    values.insert('k', 5);
    values.insert('l', 1);
    values.insert('m', 3);
    values.insert('n', 1);
    values.insert('o', 1);
    values.insert('p', 3);
    values.insert('q', 10);
    values.insert('r', 1);
    values.insert('s', 1);
    values.insert('t', 1);
    values.insert('u', 1);
    values.insert('v', 4);
    values.insert('w', 4);
    values.insert('x', 8);
    values.insert('y', 4);
    values.insert('z', 10);
    values
}
