#[derive(Debug, PartialEq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

pub struct CircularBuffer<T> {
    /// Using Option leads to less efficient memory layout, but
    /// it allows us to avoid using `unsafe` to handle uninitialized
    /// mempory ourselves.
    data: Vec<Option<T>>,
    start: usize,
    end: usize,
}

impl<T> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        let mut data = Vec::with_capacity(capacity);
        data.resize_with(capacity, || None);
        Self {
            data,
            start: 0,
            end: 0,
        }
    }

    pub fn read(&mut self) -> Result<T, Error> {
        if self.is_empty() {
            return Err(Error::EmptyBuffer);
        }
        let v = self.data[self.start]
            .take()
            .expect("should not read 'uninitialized' memory");
        self.advance_start();
        Ok(v)
    }

    pub fn write(&mut self, byte: T) -> Result<(), Error> {
        if self.is_full() {
            return Err(Error::FullBuffer);
        }
        self.data[self.end] = Some(byte);
        self.advance_end();
        Ok(())
    }

    pub fn overwrite(&mut self, byte: T) {
        self.data[self.end] = Some(byte);
        if self.start == self.end {
            self.advance_start();
        }
        self.advance_end();
    }

    pub fn clear(&mut self) {
        self.start = 0;
        self.end = 0;

        // Clear any values in the buffer
        self.data.fill_with(|| None);
    }

    fn is_empty(&self) -> bool {
        self.start == self.end && self.data[self.start].is_none()
    }

    fn is_full(&self) -> bool {
        self.start == self.end && self.data[self.start].is_some()
    }

    fn advance_start(&mut self) {
        self.start = (self.start + 1) % self.data.len();
    }

    fn advance_end(&mut self) {
        self.end = (self.end + 1) % self.data.len();
    }
}
