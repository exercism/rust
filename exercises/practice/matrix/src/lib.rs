pub struct Matrix {
    // Implement your Matrix struct
}

impl Matrix {
    pub fn new(input: &str) -> Self {
        todo!("Create new method to store the {input}")
    }

    pub fn row(&self, row_no: usize) -> Option<Vec<u32>> {
        todo!("Return the row at {row_no} (1-indexed) or None if the number is invalid")
    }

    pub fn column(&self, col_no: usize) -> Option<Vec<u32>> {
        todo!("Return the column at {col_no} (1-indexed) or None if the number is invalid")
    }
}
