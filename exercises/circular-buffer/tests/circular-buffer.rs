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
    assert!(buffer.write('1').is_ok());
    assert_eq!(Ok('1'), buffer.read());
    assert_eq!(Err(Error::EmptyBuffer), buffer.read());
}

#[test]
#[ignore]
fn write_and_read_back_multiple_items() {
    let mut buffer = CircularBuffer::new(2);
    assert!(buffer.write('1').is_ok());
    assert!(buffer.write('2').is_ok());
    assert_eq!(Ok('1'), buffer.read());
    assert_eq!(Ok('2'), buffer.read());
    assert_eq!(Err(Error::EmptyBuffer), buffer.read());
}

#[test]
#[ignore]
fn alternate_write_and_read() {
    let mut buffer = CircularBuffer::new(2);
    assert!(buffer.write('1').is_ok());
    assert_eq!(Ok('1'), buffer.read());
    assert!(buffer.write('2').is_ok());
    assert_eq!(Ok('2'), buffer.read());
}

#[test]
#[ignore]
fn clear_buffer() {
    let mut buffer = CircularBuffer::new(3);
    assert!(buffer.write('1').is_ok());
    assert!(buffer.write('2').is_ok());
    assert!(buffer.write('3').is_ok());
    buffer.clear();
    assert_eq!(Err(Error::EmptyBuffer), buffer.read());
    assert!(buffer.write('1').is_ok());
    assert!(buffer.write('2').is_ok());
    assert_eq!(Ok('1'), buffer.read());
    assert!(buffer.write('3').is_ok());
    assert_eq!(Ok('2'), buffer.read());
}

#[test]
#[ignore]
fn full_buffer_error() {
    let mut buffer = CircularBuffer::new(2);
    assert!(buffer.write('1').is_ok());
    assert!(buffer.write('2').is_ok());
    assert_eq!(Err(Error::FullBuffer), buffer.write('3'));
}

#[test]
#[ignore]
fn overwrite_item_in_non_full_buffer() {
    let mut buffer = CircularBuffer::new(2);
    assert!(buffer.write('1').is_ok());
    buffer.overwrite('2');
    assert_eq!(Ok('1'), buffer.read());
    assert_eq!(Ok('2'), buffer.read());
    assert_eq!(Err(Error::EmptyBuffer), buffer.read());
}

#[test]
#[ignore]
fn overwrite_item_in_full_buffer() {
    let mut buffer = CircularBuffer::new(2);
    assert!(buffer.write('1').is_ok());
    assert!(buffer.write('2').is_ok());
    buffer.overwrite('A');
    assert_eq!(Ok('2'), buffer.read());
    assert_eq!(Ok('A'), buffer.read());
}

#[test]
#[ignore]
fn integer_buffer() {
    let mut buffer = CircularBuffer::new(2);
    assert!(buffer.write(1).is_ok());
    assert!(buffer.write(2).is_ok());
    assert_eq!(Ok(1), buffer.read());
    assert!(buffer.write(-1).is_ok());
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
