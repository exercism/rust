/// Yields each item of a and then each item of b
pub fn append<I, J>(_a: I, _b: J) -> impl Iterator<Item = I::Item>
where
    I: Iterator,
    J: Iterator<Item = I::Item>,
{
    let mut res: Vec<I::Item> = _a.collect();
    for el in _b {
        res.push(el);
    }
    res.into_iter()
}

/// Combines all items in all nested iterators inside into one flattened iterator
pub fn concat<I>(_nested_iter: I) -> impl Iterator<Item = <I::Item as Iterator>::Item>
where
    I: Iterator,
    I::Item: Iterator,
{
    let mut res: Vec<<I::Item as Iterator>::Item> = Vec::new();
    for it_el in _nested_iter {
        for el in it_el {
            res.push(el);
        }
    }

    res.into_iter()
}

/// Returns an iterator of all items in iter for which `predicate(item)` is true
pub fn filter<I, F>(_iter: I, _predicate: F) -> impl Iterator<Item = I::Item>
where
    I: Iterator,
    F: Fn(&I::Item) -> bool,
{
    let mut res: Vec<I::Item> = Vec::new();

    for el in _iter {
        if _predicate(&el) {
            res.push(el);
        }
    }

    res.into_iter()
}

pub fn length<I: Iterator>(_iter: I) -> usize {
    let mut res: usize = 0;

    for _ in _iter {
        res += 1;
    }

    res
}

/// Returns an iterator of the results of applying `function(item)` on all iter items
pub fn map<I, F, U>(_iter: I, _function: F) -> impl Iterator<Item = U>
where
    I: Iterator,
    F: Fn(I::Item) -> U,
{
    let mut res: Vec<U> = Vec::new();

    for el in _iter {
        res.push(_function(el));
    }

    res.into_iter()
}

pub fn foldl<I, F, U>(mut _iter: I, _initial: U, _function: F) -> U
where
    I: Iterator,
    F: Fn(U, I::Item) -> U,
{
    let mut res: U = _initial;

    for el in _iter {
        res = _function(res, el);
    }

    res
}

pub fn foldr<I, F, U>(mut _iter: I, _initial: U, _function: F) -> U
where
    I: DoubleEndedIterator,
    F: Fn(U, I::Item) -> U,
{
    let mut res: U = _initial;

    while let Some(el) = _iter.next_back() {
        res = _function(res, el);
    }

    res
}

/// Returns an iterator with all the original items, but in reverse order
pub fn reverse<I: DoubleEndedIterator>(_iter: I) -> impl Iterator<Item = I::Item> {
    // this empty iterator silences a compiler complaint that
    // () doesn't implement Iterator
    // let mut res: Vec<I::Item> = Vec::new();
    let mut res: Vec<I::Item> = Vec::new();
    let mut _iter =_iter;

    while let Some(el) = _iter.next_back() {
        res.push(el); 
    }

    res.into_iter()
}
