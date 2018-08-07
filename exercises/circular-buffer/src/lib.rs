use std::fmt::Debug;
use std::marker::PhantomData;

pub struct CircularBuffer<T: Debug> {
    // This field is here to make the template compile and not to
    // complain about unused type parameter 'T'. Once you start
    // solving the exercise, delete this field and the 'std::marker::PhantomData'
    // import.
    field: PhantomData<T>,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T: Debug> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        unimplemented!(
            "Construct a new CircularBuffer with the capacity to hold {}.",
            match capacity {
                1 => format!("1 element"),
                _ => format!("{} elements", capacity),
            }
        );
    }

    pub fn write(&mut self, element: T) -> Result<(), Error> {
        unimplemented!("Write the element '{:?}' to the CircularBuffer or return FullBuffer error if CircularBuffer is full.", element);
    }

    pub fn read(&mut self) -> Result<T, Error> {
        unimplemented!("Read the oldest element from the CircularBuffer or return EmptyBuffer error if CircularBuffer is empty.");
    }

    pub fn clear(&mut self) {
        unimplemented!("Clear the CircularBuffer.");
    }

    pub fn overwrite(&mut self, element: T) {
        unimplemented!("Write the element '{:?}' to the CircularBuffer, overwriting the existing elements if CircularBuffer is full.", element);
    }
}
