use std::collections::{HashMap, HashSet};

pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let row_maxs_locations = row_max_locations(input);
    println!("{:?}", row_maxs_locations);

    // Only search mins for columns with a row max. 
    let cols_to_consider: HashSet<usize> = row_maxs_locations
    .iter()
    .map(|(_, second)| *second)
    .collect();
    println!("{:?}", cols_to_consider);
    
    // Compute minima for relevant columns
    let cols_min= column_minima(input, &cols_to_consider);
    println!("{:?}", cols_min);

    // Only return row max corresponding column mins
    row_maxs_locations
    .iter()
    .filter_map(|(i, j)| {
        if let Some(val) = input
        .get(*i)
        .and_then(|row| row.get(*j)) {
            if val == cols_min.get(j).unwrap() {
                Some((*i, *j))
            }
            else {
                None
            }
        }
        else {
            None
        }
    })
    .collect()

}


pub fn row_max_locations(matrix: &[Vec<u64>]) -> Vec<(usize, usize)> {
    matrix
        .iter()
        .enumerate()
        .map(|(i, row)| {
            if let Some(row_max) = row.iter().copied().max() {
                row
                .iter()
                .enumerate()
                .filter_map(|(j, &val)| {
                    if val == row_max {
                        Some((i, j))
                    }
                    else {
                        None
                    }
                })
                .collect() // each row generate a [(a1, b1), (a2, b2)]
            }
            else {
                vec![]
            }
        }) 
        .flatten()
        .collect()
}

pub fn column_minima(matrix: &[Vec<u64>], cols: &HashSet<usize>) -> HashMap<usize, u64> {
    let mut minima: HashMap<usize, u64> = HashMap::new();
    
    if matrix.is_empty() {
        return minima; 
    }

    for &i in cols {
        minima.insert(i, u64::MAX) ;

        for row in matrix {
            let val = row.get(i).unwrap();

            minima
            .entry(i)
            .and_modify(|v|
                if *v > *val {
                    *v = *val; 
                }
            );
        }
    }
    minima
}