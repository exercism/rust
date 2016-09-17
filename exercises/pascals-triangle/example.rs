pub struct PascalsTriangle {
    row_count: u32,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle { row_count: row_count }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        (0..self.row_count).map(|row| PascalsTriangle::row(row)).collect()
    }

    pub fn row(number: u32) -> Vec<u32> {
        let mut r = vec![1];

        for p in 1..(number + 1) {
            if let Some(last) = r.pop() {
                let next = last as f32 * ((number as f32 + 1f32 - p as f32) / p as f32);
                r.extend(&[last, next as u32])
            }
        }
        r
    }
}
