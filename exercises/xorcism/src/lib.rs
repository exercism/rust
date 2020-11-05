/// A munger which XORs a key with some data
#[derive(Clone)]
pub struct Xorcism<'a> {
    // This field is just to suppress compiler complaints;
    // feel free to delete it at any point.
    _phantom: std::marker::PhantomData<&'a u8>,
}

/// For composability, it is important that `munge` returns an iterator compatible with its input.
///
/// However, `impl Trait` syntax can specify only a single non-auto trait.
/// Therefore, we define this output trait with generic implementations on all compatible types,
/// and return that instead.
pub trait MungeOutput: Iterator<Item = u8> + ExactSizeIterator {}
impl<T> MungeOutput for T where T: Iterator<Item = u8> + ExactSizeIterator {}

impl<'a> Xorcism<'a> {
    /// Create a new Xorcism munger from a key
    ///
    /// Should accept anything which has a cheap conversion to a byte slice.
    pub fn new<Key>(key: &Key) -> Xorcism<'a> {
        unimplemented!()
    }

    /// XOR each byte of the input buffer with a byte from the key.
    ///
    /// Note that this is stateful: repeated calls are likely to produce different results,
    /// even with identical inputs.
    pub fn munge_in_place(&mut self, data: &mut [u8]) {
        unimplemented!()
    }

    /// XOR each byte of the data with a byte from the key.
    ///
    /// Note that this is stateful: repeated calls are likely to produce different results,
    /// even with identical inputs.
    ///
    /// Should accept anything which has a cheap conversion to a byte iterator.
    /// Shouldn't matter whether the byte iterator's values are owned or borrowed.
    pub fn munge<Data>(&mut self, data: Data) -> impl MungeOutput {
        unimplemented!();
        // this empty iterator silences a compiler complaint that
        // () doesn't implement ExactSizeIterator
        std::iter::empty()
    }
}
