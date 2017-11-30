pub fn find_saddle_points(input: &[&[u64]]) -> Vec<(u64, u64)>{
    let mut saddle_points: Vec<(u64, u64)> = Vec::new();

    let width = input.len();
    let height = input[0].len();

    for i in 0..width {
        for j in 0..height {

            let column = input.iter().map(|x| x[j]).collect::<Vec<u64>>();
            let row = &input[i];

            let max = row.iter().max().unwrap();
            let min = column.iter().min().unwrap();

            let value = input[i][j];

            if value >= *max && value <= *min {
                saddle_points.push((i as u64, j as u64));
            }
        }
    }
    saddle_points
}
