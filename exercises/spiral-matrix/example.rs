pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let mut matrix: Vec<Vec<u32>> = vec![vec![0; size as usize]; size as usize];
    let num_concentric_squares = (f64::from(size) / 2.0).ceil() as usize;
    let mut counter: u32 = 1;
    let mut sidelen = size as usize;
    for i in 0..num_concentric_squares {
        for j in 0..sidelen {
            matrix[i][i + j] = counter;
            counter += 1;
        }

        for j in 1..sidelen {
            matrix[i + j][size as usize - 1 - i] = counter;
            counter += 1;
        }

        for j in (0..sidelen - 1).rev() {
            matrix[size as usize - 1 - i][i + j] = counter;
            counter += 1;
        }

        for j in (1..sidelen - 1).rev() {
            matrix[i + j][i] = counter;

            counter += 1;
        }
        if sidelen >= 2 {
            sidelen -= 2;
        } else {
            sidelen -= 1;
        }
    }
    matrix
}
