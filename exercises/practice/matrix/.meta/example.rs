pub struct Matrix {
    grid: Vec<Vec<u32>>,
}

impl Matrix {
    pub fn new(input: &str) -> Self {
        let grid = input
            .lines()
            .map(|l| l.split(' ').map(|n| n.parse::<u32>().unwrap()).collect())
            .collect();

        Self { grid }
    }

    pub fn row(&self, row_no: usize) -> Option<Vec<u32>> {
        self.grid.get(row_no.checked_sub(1)?).cloned()
    }

    pub fn column(&self, col_no: usize) -> Option<Vec<u32>> {
        self.grid.iter().map(|row| row.get(col_no.checked_sub(1)?).copied()).collect()
    }
}
