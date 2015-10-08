#[derive(Debug, PartialEq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

pub struct CircularBuffer {
    buffer: Vec<char>,
    size: usize,
    start: usize,
    end: usize,
}

impl CircularBuffer {
    // this circular buffer keeps an unallocated slot between the start and the end
    // when the buffer is full. 
    pub fn new(size: usize) -> CircularBuffer {
        let mut v = Vec::new();
        v.reserve(size + 1);

        // initialize the buffer
        for _ in 0..(size + 1) {
            v.push('0');
        }

        let buffer = CircularBuffer { 
                buffer: v, 
                size: size + 1, 
                start: 0, 
                end: 0 
        };

        buffer
    }

    pub fn read(&mut self) -> Result<char, Error> {
        if self.start == self.end {
            return Err(Error::EmptyBuffer);
        }

        let v = *self.buffer.get(self.start).unwrap();
        self.start = (self.start + 1) % self.size;

        Ok(v)
    }

    pub fn write(&mut self, byte: char) -> Result<(), Error> {
        if (self.end + 1) % self.size == self.start {
            return Err(Error::FullBuffer);
        } else {
            self.buffer[self.end] = byte;
            self.end = (self.end + 1) % self.size;
            return Ok(());
        }
    }

    pub fn overwrite(&mut self, byte: char) {
        self.buffer[self.end] = byte;
        self.end = (self.end + 1) % self.size;
        if self.start == self.end {
            self.start = (self.start + 1) % self.size;
        }
    }

    pub fn clear(&mut self) {
        self.start = 0;
        self.end = 0;
    }
}
