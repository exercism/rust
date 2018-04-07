extern crate circular_buffer;
use circular_buffer::{CircularBuffer, Error};

#[test]
fn error_on_read_empty_buffer() {
    let mut buffer = CircularBuffer::<char>::new(1);
    assert_eq!(Err(Error::EmptyBuffer), buffer.read());
}

#[test]
#[ignore]
fn write_and_read_back_item() {
    let mut buffer = CircularBuffer::new(1);
    // Normally, using `unwrap` like this would be pretty bad practice:
    // the call panics on error, which is bad behavior on the part of an
    // executable or library.
    //
    // It's less of a problem within a test, for two reasons:
    //  1. We control the entire test environment, and so should be able to
    //     predict exactly when errors will and will not occur.
    //  2. A panic here simply fails the test instead of aborting in the middle
    //     of unrelated code.
    //
    // Finally, it underscores the fact that this isn't actually the behavior
    // under test at the moment: when we want to test some behavior, we use assert.
    // `unwrap()`, in this context, means that we're just trying to get to the point.
    buffer.write('1').unwrap();
    assert_eq!(Ok('1'), buffer.read());
    assert_eq!(Err(Error::EmptyBuffer), buffer.read());
}

#[test]
#[ignore]
fn write_and_read_back_multiple_items() {
    let mut buffer = CircularBuffer::new(2);
    buffer.write('1').unwrap();
    buffer.write('2').unwrap();
    assert_eq!(Ok('1'), buffer.read());
    assert_eq!(Ok('2'), buffer.read());
    assert_eq!(Err(Error::EmptyBuffer), buffer.read());
}

#[test]
#[ignore]
fn alternate_write_and_read() {
    let mut buffer = CircularBuffer::new(2);
    buffer.write('1').unwrap();
    assert_eq!(Ok('1'), buffer.read());
    buffer.write('2').unwrap();
    assert_eq!(Ok('2'), buffer.read());
}

#[test]
#[ignore]
fn clear_buffer() {
    let mut buffer = CircularBuffer::new(3);
    buffer.write('1').unwrap();
    buffer.write('2').unwrap();
    buffer.write('3').unwrap();
    buffer.clear();
    assert_eq!(Err(Error::EmptyBuffer), buffer.read());
    buffer.write('1').unwrap();
    buffer.write('2').unwrap();
    assert_eq!(Ok('1'), buffer.read());
    buffer.write('3').unwrap();
    assert_eq!(Ok('2'), buffer.read());
}

#[test]
#[ignore]
fn full_buffer_error() {
    let mut buffer = CircularBuffer::new(2);
    buffer.write('1').unwrap();
    buffer.write('2').unwrap();
    assert_eq!(Err(Error::FullBuffer), buffer.write('3'));
}

#[test]
#[ignore]
fn overwrite_item_in_non_full_buffer() {
    let mut buffer = CircularBuffer::new(2);
    buffer.write('1').unwrap();
    buffer.overwrite('2');
    assert_eq!(Ok('1'), buffer.read());
    assert_eq!(Ok('2'), buffer.read());
    assert_eq!(Err(Error::EmptyBuffer), buffer.read());
}

#[test]
#[ignore]
fn overwrite_item_in_full_buffer() {
    let mut buffer = CircularBuffer::new(2);
    buffer.write('1').unwrap();
    buffer.write('2').unwrap();
    buffer.overwrite('A');
    assert_eq!(Ok('2'), buffer.read());
    assert_eq!(Ok('A'), buffer.read());
}

#[test]
#[ignore]
fn integer_buffer() {
    let mut buffer = CircularBuffer::new(2);
    buffer.write(1).unwrap();
    buffer.write(2).unwrap();
    assert_eq!(Ok(1), buffer.read());
    buffer.write(-1).unwrap();
    assert_eq!(Ok(2), buffer.read());
    assert_eq!(Ok(-1), buffer.read());
    assert_eq!(Err(Error::EmptyBuffer), buffer.read());
}

#[test]
#[ignore]
fn string_buffer() {
    let mut buffer = CircularBuffer::new(2);
    buffer.write("".to_string()).unwrap();
    buffer.write("Testing".to_string()).unwrap();
    assert_eq!(0, buffer.read().unwrap().len());
    assert_eq!(Ok("Testing".to_string()), buffer.read());
}
