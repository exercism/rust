extern crate circular_buffer;

#[allow(unused_must_use)]
mod tests {

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
        buffer.write('1');
        assert_eq!('1', buffer.read().unwrap());
        assert_eq!(Err(Error::EmptyBuffer), buffer.read());
    }

    #[test]
    #[ignore]
    fn write_and_read_back_multiple_items() {
        let mut buffer = CircularBuffer::new(2);
        buffer.write('1');
        buffer.write('2');
        assert_eq!('1', buffer.read().unwrap());
        assert_eq!('2', buffer.read().unwrap());
        assert_eq!(Err(Error::EmptyBuffer), buffer.read());
    }

    #[test]
    #[ignore]
    fn alternate_write_and_read() {
        let mut buffer = CircularBuffer::new(2);
        buffer.write('1');
        assert_eq!('1', buffer.read().unwrap());
        buffer.write('2');
        assert_eq!('2', buffer.read().unwrap());
    }

    #[test]
    #[ignore]
    fn clear_buffer() {
        let mut buffer = CircularBuffer::new(3);
        buffer.write('1');
        buffer.write('2');
        buffer.write('3');
        buffer.clear();
        assert_eq!(Err(Error::EmptyBuffer), buffer.read());
        buffer.write('1');
        buffer.write('2');
        assert_eq!('1', buffer.read().unwrap());
        buffer.write('3');
        assert_eq!('2', buffer.read().unwrap());
    }

    #[test]
    #[ignore]
    fn full_buffer_error() {
        let mut buffer = CircularBuffer::new(2);
        buffer.write('1');
        buffer.write('2');
        assert_eq!(Err(Error::FullBuffer), buffer.write('3'));
    }
    
    #[test]
    #[ignore]
    fn overwrite_item_in_non_full_buffer() {
        let mut buffer = CircularBuffer::new(2);
        buffer.write('1');
        buffer.overwrite('2');
        assert_eq!('1', buffer.read().unwrap());
        assert_eq!('2', buffer.read().unwrap());
        assert_eq!(Err(Error::EmptyBuffer), buffer.read());
    }

    #[test]
    #[ignore]
    fn overwrite_item_in_full_buffer() {
        let mut buffer = CircularBuffer::new(2);
        buffer.write('1');
        buffer.write('2');
        buffer.overwrite('A');
        assert_eq!('2', buffer.read().unwrap());
        assert_eq!('A', buffer.read().unwrap());
    }

    #[test]
    #[ignore]
    fn integer_buffer() {
        let mut buffer = CircularBuffer::new(2);
        buffer.write(1);
        buffer.write(2);
        assert_eq!(1,buffer.read().unwrap());
        buffer.write(-1);
        assert_eq!(2,buffer.read().unwrap());
        assert_eq!(-1,buffer.read().unwrap());
        assert_eq!(Err(Error::EmptyBuffer), buffer.read());
    }
    
    #[test]
    #[ignore]
    fn string_buffer() {
        let mut buffer = CircularBuffer::new(2);
        buffer.write("".to_string());
        buffer.write("Testing".to_string());
        assert_eq!(0,buffer.read().unwrap().len());
        assert_eq!("Testing",buffer.read().unwrap());
    }
}
