pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let height = if !input.is_empty() && !input[0].is_empty() {
        input[0].len()
    } else {
        return Vec::new();
    };

    let mut saddle_points = Vec::new();

    let col_mins: Vec<u64> = (0..height)
        .filter_map(|j| input.iter().map(|row| row[j]).min())
        .collect();

    for (i, row) in input.iter().enumerate() {
        let row_max = *row.iter().max().unwrap();

        for j in 0..height {
            if row[j] == row_max && row[j] == col_mins[j] {
                saddle_points.push((i, j));
            }
        }
    }

    saddle_points
}
