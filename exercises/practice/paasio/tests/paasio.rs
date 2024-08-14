use std::io::{Error, ErrorKind, Read, Result, Write};

#[test]
fn create_stats() {
    let mut data: Vec<u8> = Vec::new();
    let _ = paasio::ReadStats::new(data.as_slice());
    let _ = paasio::WriteStats::new(data.as_mut_slice());
}

mod read_string {
    use paasio::*;
    use std::io::{BufReader, Read};

    const CHUNK_SIZE: usize = 2;

    static INPUT: &[u8] = b"Twas brillig, and the slithy toves/Did gyre and gimble in the wabe:/All mimsy were the borogoves,/And the mome raths outgrabe.";

    #[test]
    #[ignore]
    fn read_passthrough() {
        let data = INPUT;
        let len = |d: &[u8]| d.len();
        let size = len(data);
        let mut reader = ReadStats::new(data);

        let mut buffer = Vec::with_capacity(size);
        let qty_read = reader.read_to_end(&mut buffer);

        assert!(qty_read.is_ok());
        assert_eq!(size, qty_read.unwrap());
        assert_eq!(size, buffer.len());
        // 2: first to read all the data, second to check that
        // there wasn't any more pending data which simply didn't
        // fit into the existing buffer
        assert_eq!(2, reader.reads());
        assert_eq!(size, reader.bytes_through());
    }

    #[test]
    #[ignore]
    fn read_chunks() {
        let data = INPUT;
        let len = |d: &[u8]| d.len();
        let size = len(data);
        let mut reader = ReadStats::new(data);

        let mut buffer = [0_u8; CHUNK_SIZE];
        let mut chunks_read = 0;
        while reader
            .read(&mut buffer[..])
            .unwrap_or_else(|_| panic!("read failed at chunk {}", chunks_read + 1))
            > 0
        {
            chunks_read += 1;
        }

        assert_eq!(
            size / CHUNK_SIZE + std::cmp::min(1, size % CHUNK_SIZE),
            chunks_read
        );
        // we read once more than the number of chunks, because the final
        // read returns 0 new bytes
        assert_eq!(1 + chunks_read, reader.reads());
        assert_eq!(size, reader.bytes_through());
    }

    #[test]
    #[ignore]
    fn read_buffered_chunks() {
        let data = INPUT;
        let len = |d: &[u8]| d.len();
        let size = len(data);
        let mut reader = BufReader::new(ReadStats::new(data));

        let mut buffer = [0_u8; CHUNK_SIZE];
        let mut chunks_read = 0;
        while reader
            .read(&mut buffer[..])
            .unwrap_or_else(|_| panic!("read failed at chunk {}", chunks_read + 1))
            > 0
        {
            chunks_read += 1;
        }

        assert_eq!(
            size / CHUNK_SIZE + std::cmp::min(1, size % CHUNK_SIZE),
            chunks_read
        );
        // the BufReader should smooth out the reads, collecting into
        // a buffer and performing only two read operations:
        // the first collects everything into the buffer,
        // and the second ensures that no data remains
        assert_eq!(2, reader.get_ref().reads());
        assert_eq!(size, reader.get_ref().bytes_through());
    }
}

mod write_string {
    use paasio::*;
    use std::io::{self, BufWriter, Write};

    const CHUNK_SIZE: usize = 2;

    static INPUT: &[u8] = b"Beware the Jabberwock, my son!/The jaws that bite, the claws that catch!/Beware the Jubjub bird, and shun/The frumious Bandersnatch!";

    #[test]
    #[ignore]
    fn write_passthrough() {
        let data = INPUT;
        let len = |d: &[u8]| d.len();
        let size = len(data);
        let mut writer = WriteStats::new(Vec::with_capacity(size));
        let written = writer.write(data);
        assert!(written.is_ok());
        assert_eq!(size, written.unwrap());
        assert_eq!(size, writer.bytes_through());
        assert_eq!(1, writer.writes());
        assert_eq!(data, writer.get_ref().as_slice());
    }

    #[test]
    #[ignore]
    fn sink_oneshot() {
        let data = INPUT;
        let len = |d: &[u8]| d.len();
        let size = len(data);
        let mut writer = WriteStats::new(io::sink());
        let written = writer.write(data);
        assert!(written.is_ok());
        assert_eq!(size, written.unwrap());
        assert_eq!(size, writer.bytes_through());
        assert_eq!(1, writer.writes());
    }

    #[test]
    #[ignore]
    fn sink_windowed() {
        let data = INPUT;
        let len = |d: &[u8]| d.len();
        let size = len(data);
        let mut writer = WriteStats::new(io::sink());

        let mut chunk_count = 0;
        for chunk in data.chunks(CHUNK_SIZE) {
            chunk_count += 1;
            let written = writer.write(chunk);
            assert!(written.is_ok());
            assert_eq!(CHUNK_SIZE, written.unwrap());
        }
        assert_eq!(size, writer.bytes_through());
        assert_eq!(chunk_count, writer.writes());
    }

    #[test]
    #[ignore]
    fn sink_buffered_windowed() {
        let data = INPUT;
        let len = |d: &[u8]| d.len();
        let size = len(data);
        let mut writer = BufWriter::new(WriteStats::new(io::sink()));

        for chunk in data.chunks(CHUNK_SIZE) {
            let written = writer.write(chunk);
            assert!(written.is_ok());
            assert_eq!(CHUNK_SIZE, written.unwrap());
        }
        // at this point, nothing should have yet been passed through to
        // our writer
        assert_eq!(0, writer.get_ref().bytes_through());
        assert_eq!(0, writer.get_ref().writes());

        // after flushing, everything should pass through in one go
        assert!(writer.flush().is_ok());
        assert_eq!(size, writer.get_ref().bytes_through());
        assert_eq!(1, writer.get_ref().writes());
    }
}

mod read_byte_literal {
    use paasio::*;
    use std::io::{BufReader, Read};

    const CHUNK_SIZE: usize = 2;

    static INPUT: &[u8] = &[1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144];

    #[test]
    #[ignore]
    fn read_passthrough() {
        let data = INPUT;
        let len = |d: &[u8]| d.len();
        let size = len(data);
        let mut reader = ReadStats::new(data);

        let mut buffer = Vec::with_capacity(size);
        let qty_read = reader.read_to_end(&mut buffer);

        assert!(qty_read.is_ok());
        assert_eq!(size, qty_read.unwrap());
        assert_eq!(size, buffer.len());
        // 2: first to read all the data, second to check that
        // there wasn't any more pending data which simply didn't
        // fit into the existing buffer
        assert_eq!(2, reader.reads());
        assert_eq!(size, reader.bytes_through());
    }

    #[test]
    #[ignore]
    fn read_chunks() {
        let data = INPUT;
        let len = |d: &[u8]| d.len();
        let size = len(data);
        let mut reader = ReadStats::new(data);

        let mut buffer = [0_u8; CHUNK_SIZE];
        let mut chunks_read = 0;
        while reader
            .read(&mut buffer[..])
            .unwrap_or_else(|_| panic!("read failed at chunk {}", chunks_read + 1))
            > 0
        {
            chunks_read += 1;
        }

        assert_eq!(
            size / CHUNK_SIZE + std::cmp::min(1, size % CHUNK_SIZE),
            chunks_read
        );
        // we read once more than the number of chunks, because the final
        // read returns 0 new bytes
        assert_eq!(1 + chunks_read, reader.reads());
        assert_eq!(size, reader.bytes_through());
    }

    #[test]
    #[ignore]
    fn read_buffered_chunks() {
        let data = INPUT;
        let len = |d: &[u8]| d.len();
        let size = len(data);
        let mut reader = BufReader::new(ReadStats::new(data));

        let mut buffer = [0_u8; CHUNK_SIZE];
        let mut chunks_read = 0;
        while reader
            .read(&mut buffer[..])
            .unwrap_or_else(|_| panic!("read failed at chunk {}", chunks_read + 1))
            > 0
        {
            chunks_read += 1;
        }

        assert_eq!(
            size / CHUNK_SIZE + std::cmp::min(1, size % CHUNK_SIZE),
            chunks_read
        );
        // the BufReader should smooth out the reads, collecting into
        // a buffer and performing only two read operations:
        // the first collects everything into the buffer,
        // and the second ensures that no data remains
        assert_eq!(2, reader.get_ref().reads());
        assert_eq!(size, reader.get_ref().bytes_through());
    }
}

mod write_byte_literal {
    use paasio::*;
    use std::io::{self, BufWriter, Write};

    const CHUNK_SIZE: usize = 2;

    static INPUT: &[u8] = &[
        2_u8, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61,
    ];

    #[test]
    #[ignore]
    fn write_passthrough() {
        let data = INPUT;
        let len = |d: &[u8]| d.len();
        let size = len(data);
        let mut writer = WriteStats::new(Vec::with_capacity(size));
        let written = writer.write(data);
        assert!(written.is_ok());
        assert_eq!(size, written.unwrap());
        assert_eq!(size, writer.bytes_through());
        assert_eq!(1, writer.writes());
        assert_eq!(data, writer.get_ref().as_slice());
    }

    #[test]
    #[ignore]
    fn sink_oneshot() {
        let data = INPUT;
        let len = |d: &[u8]| d.len();
        let size = len(data);
        let mut writer = WriteStats::new(io::sink());
        let written = writer.write(data);
        assert!(written.is_ok());
        assert_eq!(size, written.unwrap());
        assert_eq!(size, writer.bytes_through());
        assert_eq!(1, writer.writes());
    }

    #[test]
    #[ignore]
    fn sink_windowed() {
        let data = INPUT;
        let len = |d: &[u8]| d.len();
        let size = len(data);
        let mut writer = WriteStats::new(io::sink());

        let mut chunk_count = 0;
        for chunk in data.chunks(CHUNK_SIZE) {
            chunk_count += 1;
            let written = writer.write(chunk);
            assert!(written.is_ok());
            assert_eq!(CHUNK_SIZE, written.unwrap());
        }
        assert_eq!(size, writer.bytes_through());
        assert_eq!(chunk_count, writer.writes());
    }

    #[test]
    #[ignore]
    fn sink_buffered_windowed() {
        let data = INPUT;
        let len = |d: &[u8]| d.len();
        let size = len(data);
        let mut writer = BufWriter::new(WriteStats::new(io::sink()));

        for chunk in data.chunks(CHUNK_SIZE) {
            let written = writer.write(chunk);
            assert!(written.is_ok());
            assert_eq!(CHUNK_SIZE, written.unwrap());
        }
        // at this point, nothing should have yet been passed through to
        // our writer
        assert_eq!(0, writer.get_ref().bytes_through());
        assert_eq!(0, writer.get_ref().writes());

        // after flushing, everything should pass through in one go
        assert!(writer.flush().is_ok());
        assert_eq!(size, writer.get_ref().bytes_through());
        assert_eq!(1, writer.get_ref().writes());
    }
}

mod read_file {
    use paasio::*;
    use std::io::{BufReader, Read};

    const CHUNK_SIZE: usize = 2;

    #[test]
    #[ignore]
    fn read_passthrough() {
        let data = std::fs::File::open("Cargo.toml").expect("Cargo.toml must be present");
        let len =
            |f: &::std::fs::File| f.metadata().expect("metadata must be present").len() as usize;
        let size = len(&data);
        let mut reader = ReadStats::new(data);

        let mut buffer = Vec::with_capacity(size);
        let qty_read = reader.read_to_end(&mut buffer);

        assert!(qty_read.is_ok());
        assert_eq!(size, qty_read.unwrap());
        assert_eq!(size, buffer.len());
        // 2: first to read all the data, second to check that
        // there wasn't any more pending data which simply didn't
        // fit into the existing buffer
        assert_eq!(2, reader.reads());
        assert_eq!(size, reader.bytes_through());
    }

    #[test]
    #[ignore]
    fn read_chunks() {
        let data = std::fs::File::open("Cargo.toml").expect("Cargo.toml must be present");
        let len =
            |f: &::std::fs::File| f.metadata().expect("metadata must be present").len() as usize;
        let size = len(&data);
        let mut reader = ReadStats::new(data);

        let mut buffer = [0_u8; CHUNK_SIZE];
        let mut chunks_read = 0;
        while reader
            .read(&mut buffer[..])
            .unwrap_or_else(|_| panic!("read failed at chunk {}", chunks_read + 1))
            > 0
        {
            chunks_read += 1;
        }

        assert_eq!(
            size / CHUNK_SIZE + std::cmp::min(1, size % CHUNK_SIZE),
            chunks_read
        );
        // we read once more than the number of chunks, because the final
        // read returns 0 new bytes
        assert_eq!(1 + chunks_read, reader.reads());
        assert_eq!(size, reader.bytes_through());
    }

    #[test]
    #[ignore]
    fn read_buffered_chunks() {
        let data = std::fs::File::open("Cargo.toml").expect("Cargo.toml must be present");
        let len =
            |f: &::std::fs::File| f.metadata().expect("metadata must be present").len() as usize;
        let size = len(&data);
        let mut reader = BufReader::new(ReadStats::new(data));

        let mut buffer = [0_u8; CHUNK_SIZE];
        let mut chunks_read = 0;
        while reader
            .read(&mut buffer[..])
            .unwrap_or_else(|_| panic!("read failed at chunk {}", chunks_read + 1))
            > 0
        {
            chunks_read += 1;
        }

        assert_eq!(
            size / CHUNK_SIZE + std::cmp::min(1, size % CHUNK_SIZE),
            chunks_read
        );
        // the BufReader should smooth out the reads, collecting into
        // a buffer and performing only two read operations:
        // the first collects everything into the buffer,
        // and the second ensures that no data remains
        assert_eq!(2, reader.get_ref().reads());
        assert_eq!(size, reader.get_ref().bytes_through());
    }
}

#[test]
#[ignore]
fn read_stats_by_ref_returns_wrapped_reader() {
    use paasio::ReadStats;

    let input =
        "Why, sometimes I've believed as many as six impossible things before breakfast".as_bytes();
    let reader = ReadStats::new(input);
    assert_eq!(reader.get_ref(), &input);
}

/// a Read type that always errors
struct ReadFails;

impl ReadFails {
    const MESSAGE: &'static str = "this reader always fails";
}

impl Read for ReadFails {
    fn read(&mut self, _buf: &mut [u8]) -> Result<usize> {
        Err(Error::other(Self::MESSAGE))
    }
}

/// a Write type that always errors
struct WriteFails;

impl WriteFails {
    const MESSAGE: &'static str = "this writer always fails";
}

impl Write for WriteFails {
    fn write(&mut self, _buf: &[u8]) -> Result<usize> {
        Err(Error::other(Self::MESSAGE))
    }

    fn flush(&mut self) -> Result<()> {
        Err(Error::other(Self::MESSAGE))
    }
}

#[test]
#[ignore]
fn read_propagates_errors() {
    use paasio::ReadStats;

    let mut reader = ReadStats::new(ReadFails);
    let mut buffer = Vec::new();

    let Err(e) = reader.read(&mut buffer) else {
        panic!("read error not propagated")
    };

    // check that the correct error was returned
    assert_eq!(e.kind(), ErrorKind::Other);
    assert_eq!(e.get_ref().unwrap().to_string(), ReadFails::MESSAGE);
}

#[test]
#[ignore]
fn write_propagates_errors() {
    use paasio::WriteStats;

    let mut writer = WriteStats::new(WriteFails);
    let buffer = "This text won't be written";

    let Err(e) = writer.write(buffer.as_bytes()) else {
        panic!("write error not propagated")
    };

    // check that the correct error is returned
    assert_eq!(e.kind(), ErrorKind::Other);
    assert_eq!(e.get_ref().unwrap().to_string(), WriteFails::MESSAGE);
}
