pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let (width, height) = match (input.len(), input[0].len()) {
        (0, _) | (_, 0) => return Vec::new(),
        (n, k) => (n, k),
    };

    let mut saddle_points = Vec::new();

    for i in 0..width {
        let row_max = input[i].iter().max().unwrap();

        for j in 0..height {
            let column_min = input.iter().map(|x| x[j]).min().unwrap();

            let value = input[i][j];

            if value == *row_max && value == column_min {
                saddle_points.push((i, j));
            }
        }
    }

    saddle_points
}
