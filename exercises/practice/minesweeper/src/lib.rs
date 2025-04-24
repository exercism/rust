pub fn annotate(minefield: &[&str]) -> Vec<String> {
    minefield
    .iter()
    .enumerate()
    .map(|(i, row)| (0..row.len()).map(|j|  trans(i, j, minefield)).collect() )
    .collect()
}

fn trans(i: usize, j: usize, minefield: &[&str]) -> char {
    if minefield[i].chars().nth(j) == Some('*') {
        '*'
    }
    else {
        let count_around = (i.saturating_sub(1)..=i+1) // i+1 can go over range, but is filted out later one with the filter_map
        .flat_map(|k| (j.saturating_sub(1)..=j+1).map(move |l| (k, l)) )
        .filter(|&(k, l)| (k, l) != (i, j))
        .filter_map(|(k, l)| minefield.get(k).and_then(|s| s.chars().nth(l)))
        .filter(|c| *c == '*')
        .count();

        match count_around {
            0 => ' ',
            x @ 0..=9 => char::from_digit(x.try_into().unwrap(), 10).unwrap(),
            _ => unreachable!()
        }
    }
}