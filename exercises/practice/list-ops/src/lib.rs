/// Yields each item of a and then each item of b
pub fn append<T, I>(_a: I, _b: I) -> impl Iterator<Item = T>
where
    I: Iterator<Item = T>,
{
    // this empty iterator silences a compiler complaint that
    // () doesn't implement Iterator
    std::iter::from_fn(|| todo!())
}

/// Combines all items in all nested iterators inside into one flattened iterator
pub fn concat<I, NI, T>(_nested_iter: I) -> impl Iterator<Item = T>
where
    NI: Iterator<Item = T>,
    I: Iterator<Item = NI>,
{
    // this empty iterator silences a compiler complaint that
    // () doesn't implement Iterator
    std::iter::from_fn(|| todo!())
}

/// Returns an iterator of all items in iter for which `predicate(item)` is true
pub fn filter<I, T, F>(_iter: I, _predicate: F) -> impl Iterator<Item = T>
where
    I: Iterator<Item = T>,
    F: Fn(&T) -> bool,
{
    // this empty iterator silences a compiler complaint that
    // () doesn't implement Iterator
    std::iter::from_fn(|| todo!())
}

pub fn length<I: Iterator<Item = T>, T>(_iter: I) -> usize {
    todo!("return the total number of items within iter")
}

/// Returns an iterator of the results of applying `function(item)` on all iter items
pub fn map<I, F, T, U>(_iter: I, _function: F) -> impl Iterator<Item = U>
where
    I: Iterator<Item = T>,
    F: Fn(T) -> U,
{
    // this empty iterator silences a compiler complaint that
    // () doesn't implement Iterator
    std::iter::from_fn(|| todo!())
}

pub fn foldl<I, F, T, U>(mut _iter: I, _initial: U, _function: F) -> U
where
    I: Iterator<Item = T>,
    F: Fn(U, T) -> U,
{
    todo!("starting with initial, fold (reduce) each iter item into the accumulator from the left")
}

pub fn foldr<I, F, T, U>(mut _iter: I, _initial: U, _function: F) -> U
where
    I: DoubleEndedIterator<Item = T>,
    F: Fn(U, T) -> U,
{
    todo!("starting with initial, fold (reduce) each iter item into the accumulator from the right")
}

/// Returns an iterator with all the original items, but in reverse order
pub fn reverse<I: DoubleEndedIterator<Item = T>, T>(_iter: I) -> impl Iterator<Item = T> {
    // this empty iterator silences a compiler complaint that
    // () doesn't implement Iterator
    std::iter::from_fn(|| todo!())
}
