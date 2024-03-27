pub struct Matrix {
    grid: Vec<Vec<u32>>,
}

impl Matrix {
    pub fn new(input: &str) -> Self {
        let grid = input
            .lines()
            .map(|l| {
                l.split(" ")
                    .map(|n| n.parse::<u32>().unwrap())
                    .collect()
            })
            .collect();

        Self { grid }
    }

    pub fn row(&self, row_no: usize) -> Option<Vec<u32>> {
        if row_no > self.grid.len() {
            return None;
        };

        Some(self.grid[row_no - 1].to_owned())
    }

    pub fn column(&self, col_no: usize) -> Option<Vec<u32>> {
        if col_no > self.grid[0].len() {
            return None;
        };

        Some(self.grid.iter().map(|row| row[col_no - 1]).collect())
    }
}
