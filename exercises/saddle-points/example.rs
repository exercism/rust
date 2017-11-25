
pub fn find_saddle_points(input: Vec<Vec<u64>>) -> Vec<(u64, u64)>{
    let mut saddle_points: Vec<(u64, u64)> = Vec::new();

    let row_index = input.len();
    let column_index = input[0].len();

    for i in 0..row_index {
        for j in 0..column_index {
            let row = &input[i];
            let column = input.iter().map(|x| x[j]).collect::<Vec<u64>>();

            let max = row.iter().max().unwrap();
            let min = column.iter().min().unwrap();
            let value = input[i][j];

            if value >= *max && value <= *min {
                saddle_points.push((i as u64, j as u64));
            }
        }
    }
    return saddle_points;
}