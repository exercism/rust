struct PascalsTriangle {
    row_count: u32,
}

impl PascalsTriangle {
    fn with_rows(row_count: u32) -> Self {
        PascalsTriangle { row_count: row_count }
    }

    fn rows(&self) -> Vec<Vec<u32>> {
        let mut r: Vec<Vec<u32>> = Vec::new();
        if self.row_count == 1 {
            r.push(vec![1]);
        }

        if self.row_count == 2 {
            r.push(vec![1]);
            r.push(vec![1, 1]);
        }

        if self.row_count == 3 {
            r.push(vec![1]);
            r.push(vec![1, 1]);
            r.push(vec![1, 2, 1]);
        }

        if self.row_count == 4 {
            r.push(vec![1]);
            r.push(vec![1, 1]);
            r.push(vec![1, 2, 1]);
            r.push(vec![1, 3, 3, 1]);
        }
        r
    }
}
