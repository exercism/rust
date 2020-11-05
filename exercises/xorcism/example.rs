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

/// For composability, it is important that `munge` returns an iterator compatible with its input.
///
/// However, `impl Trait` syntax can specify only a single non-auto trait.
/// Therefore, we define this output trait with generic implementations on all compatible types,
/// and return that instead.
pub trait MungeOutput: Iterator<Item = u8> + ExactSizeIterator {}
impl<T> MungeOutput for T where T: Iterator<Item = u8> + ExactSizeIterator {}

/// WARNING: This could be construed as abusing the type system
///
/// We need to be able to assert that a particular iterator has
/// an exact size. We need this because `iter::Zip` implements
/// `ExactSizeIterator` only if both its inputs implement `ExactSizeIterator`, evne though
/// it will always terminate on the shorter of them.
///
/// We wouldn't need this type if negative trait bounds were possible, but it looks like
/// even in a post-chalk world, those are not likely to be added to the language, as they
/// could make new trait implementations into a breaking change.
///
/// Essentially, this type is used to assert that an infinite iterator has an exact size,
/// of "the biggest number".
struct AssertExactSizeIterator<I>(I);

impl<I> Iterator for AssertExactSizeIterator<I>
where
    I: Iterator,
{
    type Item = <I as Iterator>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        // oops, all 1s
        const SIZE: usize = !0;
        (SIZE, Some(SIZE))
    }
}

impl<I> ExactSizeIterator for AssertExactSizeIterator<I> where I: Iterator {}

impl<'a> Xorcism<'a> {
    /// Create a new Xorcism munger from a key
    pub fn new<Key>(key: &'a Key) -> Xorcism<'a>
    where
        Key: AsRef<[u8]> + ?Sized,
    {
        let key = key.as_ref();
        Xorcism { key, pos: 0 }
    }

    /// Increase the stored pos by the specified amount, returning the old value.
    fn incr_pos(&mut self, by: usize) -> usize {
        let old_pos = self.pos;
        self.pos += by;
        old_pos
    }

    /// Produce the key iterator, offset by `pos`.
    fn key<'b>(&mut self, pos: usize) -> impl 'b + MungeOutput
    where
        'a: 'b,
    {
        AssertExactSizeIterator(self.key.iter().copied().cycle().skip(pos))
    }

    /// XOR each byte of the input buffer with a byte from the key.
    ///
    /// Note that this is stateful: repeated calls are likely to produce different results,
    /// even with identical inputs.
    pub fn munge_in_place(&mut self, data: &mut [u8]) {
        let pos = self.incr_pos(data.len());
        for (d, k) in data.iter_mut().zip(self.key(pos)) {
            *d ^= k;
        }
    }

    /// XOR each byte of the data with a byte from the key.
    ///
    /// Note that this is stateful: repeated calls are likely to produce different results,
    /// even with identical inputs.
    pub fn munge<'b, Data, B>(&mut self, data: Data) -> impl 'b + MungeOutput
    where
        'a: 'b,
        Data: IntoIterator<Item = B>,
        <Data as IntoIterator>::IntoIter: 'b + ExactSizeIterator,
        B: Borrow<u8>,
    {
        let data = data.into_iter();
        let pos = self.incr_pos(data.len());
        data.zip(self.key(pos)).map(|(d, k)| d.borrow() ^ k)
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

impl<'a, W> Write for Writer<'a, W>
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

impl<'a, R> Read for Reader<'a, R>
where
    R: Read,
{
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let bytes_read = self.reader.read(buf)?;
        self.xorcism.munge_in_place(&mut buf[..bytes_read]);
        Ok(bytes_read)
    }
}
