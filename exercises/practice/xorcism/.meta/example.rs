use std::borrow::Borrow;
use std::io::{self, Read, Write};

/// A munger which XORs a key with some data
///
/// This is a low-level structure; more often, you'll want to use [`Writer`], [`Reader`], or [`munge`].
//
// You might wonder: why implement this manually, instead of just storing `key: Cycle<Iter<'a, u8>>,`?
//
// If we do it like that, the lifetimes get kind of crazy. In particular, in `fn munge`, we want to do
// `data.zip(self.key.by_ref())`, and that `by_ref()` thing really confuses the lifetime inferencer.
// It ended up being simpler to just handle the key indexing manually than to figure out the correct
// incantation to get that lifetime style to work.
#[derive(Clone)]
pub struct Xorcism<'a> {
    key: &'a [u8],
    pos: usize,
}

/// Ideally this would be a method, but we run into lifetime
/// issues with that. For more information, see:
/// https://github.com/rust-lang/rust/issues/80518
fn next_key_byte(key: &[u8], pos: &mut usize) -> u8 {
    let b = key[*pos];
    *pos += 1;
    if *pos >= key.len() {
        *pos = 0;
    }
    b
}

impl<'a> Xorcism<'a> {
    /// Create a new Xorcism munger from a key
    pub fn new<Key>(key: &'a Key) -> Xorcism<'a>
    where
        Key: AsRef<[u8]> + ?Sized,
    {
        let key = key.as_ref();
        Xorcism { key, pos: 0 }
    }

    /// XOR each byte of the input buffer with a byte from the key.
    ///
    /// Note that this is stateful: repeated calls are likely to produce different results,
    /// even with identical inputs.
    pub fn munge_in_place(&mut self, data: &mut [u8]) {
        for d in data.iter_mut() {
            *d ^= next_key_byte(self.key, &mut self.pos);
        }
    }

    /// XOR each byte of the data with a byte from the key.
    ///
    /// Note that this is stateful: repeated calls are likely to produce different results,
    /// even with identical inputs.
    pub fn munge<'b, Data, B>(&'b mut self, data: Data) -> impl 'b + Iterator<Item = u8>
    where
        Data: IntoIterator<Item = B>,
        <Data as IntoIterator>::IntoIter: 'b,
        B: Borrow<u8>,
    {
        let key = self.key;
        let pos = &mut self.pos;
        data.into_iter()
            .map(move |d| d.borrow() ^ next_key_byte(key, pos))
    }

    /// Convert this into a [`Writer`]
    pub fn writer<W>(self, writer: W) -> Writer<'a, W>
    where
        W: Write,
    {
        Writer {
            xorcism: self,
            writer,
        }
    }

    /// Convert this into a [`Reader`]
    pub fn reader<R>(self, reader: R) -> Reader<'a, R>
    where
        R: Read,
    {
        Reader {
            xorcism: self,
            reader,
        }
    }
}

/// This implements `Write` and performs xor munging on the data stream.
#[derive(Clone)]
pub struct Writer<'a, W> {
    xorcism: Xorcism<'a>,
    writer: W,
}

impl<'a, W> Writer<'a, W>
where
    W: Write,
{
    pub fn new<Key>(key: &'a Key, writer: W) -> Writer<'a, W>
    where
        Key: AsRef<[u8]> + ?Sized,
    {
        Writer {
            xorcism: Xorcism::new(key),
            writer,
        }
    }
}

impl<W> Write for Writer<'_, W>
where
    W: Write,
{
    /// This implementation will block until the underlying writer
    /// has written the entire input buffer.
    fn write(&mut self, data: &[u8]) -> io::Result<usize> {
        let munged: Vec<_> = self.xorcism.munge(data).collect();
        self.writer.write_all(&munged)?;
        Ok(data.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        self.writer.flush()
    }
}

/// This implements `Read` and performs xor munging on the data stream.
#[derive(Clone)]
pub struct Reader<'a, R> {
    xorcism: Xorcism<'a>,
    reader: R,
}

impl<'a, R> Reader<'a, R>
where
    R: Read,
{
    pub fn new<Key>(key: &'a Key, reader: R) -> Reader<'a, R>
    where
        Key: AsRef<[u8]> + ?Sized,
    {
        Reader {
            xorcism: Xorcism::new(key),
            reader,
        }
    }
}

impl<R> Read for Reader<'_, R>
where
    R: Read,
{
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let bytes_read = self.reader.read(buf)?;
        self.xorcism.munge_in_place(&mut buf[..bytes_read]);
        Ok(bytes_read)
    }
}
