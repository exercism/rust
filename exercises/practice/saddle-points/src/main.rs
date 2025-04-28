use std::collections::HashSet;
use saddle_points::*;

fn main() {
    let matrix = vec![
        vec![3, 5, 5],
        vec![1, 8, 8],
        vec![7, 0, 6],
    ];

    // let matrix = [vec![]];

    let result = row_max_locations(&matrix);

    println!("{:?}", result);
    // Output: [[1, 2], [1, 2], [0]]

    let cols: HashSet<usize> = [0, 2].into_iter().collect();
    let result2 = column_minima(&matrix, &cols);
    println!("{:?}", result2); 

}