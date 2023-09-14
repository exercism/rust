pub struct CircularBuffer<T> {
    // We fake using T here, so the compiler does not complain that
    // "parameter `T` is never used". Delete when no longer needed.
    phantom: std::marker::PhantomData<T>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        todo!(
            "Construct a new CircularBuffer with the capacity to hold {}.",
            match capacity {
                1 => "1 element".to_string(),
                _ => format!("{capacity} elements"),
            }
        );
    }

    pub fn write(&mut self, _element: T) -> Result<(), Error> {
        todo!("Write the passed element to the CircularBuffer or return FullBuffer error if CircularBuffer is full.");
    }

    pub fn read(&mut self) -> Result<T, Error> {
        todo!("Read the oldest element from the CircularBuffer or return EmptyBuffer error if CircularBuffer is empty.");
    }

    pub fn clear(&mut self) {
        todo!("Clear the CircularBuffer.");
    }

    pub fn overwrite(&mut self, _element: T) {
        todo!("Write the passed element to the CircularBuffer, overwriting the existing elements if CircularBuffer is full.");
    }
}
