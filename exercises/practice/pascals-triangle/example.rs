pub struct PascalsTriangle {
    row_count: u32,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle { row_count }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        (0..self.row_count).map(PascalsTriangle::row).collect()
    }

    pub fn row(number: u32) -> Vec<u32> {
        let mut r = vec![1];

        for p in 1..=number {
            if let Some(&last) = r.last() {
                r.push((last * (number + 1 - p)) / p)
            }
        }
        r
    }
}
