#[derive(Default)]
pub struct PascalsTriangle {
    row_count: u32,
    rows: Vec<Vec<u32>>
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let mut res = 
        Self { 
            row_count,
            ..Default::default()
        };

        for i in 1..=res.row_count as usize {
            println!("i: {}", i);

            let mut row:Vec<u32> = Vec::new();
            row.push(1);

            for j in 1..i-1 {
                row.push(res.rows[i-2][j] + res.rows[i-2][j-1]);
            }

            if i > 1 {
                row.push(1);
            }

            res.rows.push(row.clone());

            println!("{:?}", row);

        }

        res
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.rows.clone()
    }
}
