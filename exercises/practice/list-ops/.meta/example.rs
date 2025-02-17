pub fn append<T, I>(a: I, b: I) -> impl Iterator<Item = T>
where
    I: Iterator<Item = T>,
{
    struct Append<T, I: Iterator<Item = T>> {
        a: I,
        b: I,
    }

    impl<T, I: Iterator<Item = T>> Iterator for Append<T, I> {
        type Item = T;

        fn next(&mut self) -> Option<Self::Item> {
            self.a.next().or_else(|| self.b.next())
        }
    }

    Append { a, b }
}

pub fn concat<I, NI, T>(list: I) -> impl Iterator<Item = T>
where
    NI: Iterator<Item = T>,
    I: Iterator<Item = NI>,
{
    struct Concat<I: Iterator<Item = NI>, NI: Iterator<Item = T>, T> {
        nested_list: I,
        cur: Option<NI>,
    }

    impl<I: Iterator<Item = NI>, NI: Iterator<Item = T>, T> Iterator for Concat<I, NI, T> {
        type Item = T;

        fn next(&mut self) -> Option<Self::Item> {
            if let Some(nested_iterator) = self.cur.as_mut() {
                if let Some(val) = nested_iterator.next() {
                    return Some(val);
                }
            }

            if let Some(next_nested) = self.nested_list.next() {
                self.cur = Some(next_nested);
                self.next()
            } else {
                None
            }
        }
    }

    Concat {
        nested_list: list,
        cur: None,
    }
}

pub fn filter<I, T, F>(list: I, predicate: F) -> impl Iterator<Item = T>
where
    I: Iterator<Item = T>,
    F: Fn(&T) -> bool,
{
    struct Filter<I: Iterator<Item = T>, T, F: Fn(&T) -> bool> {
        list: I,
        predicate: F,
    }

    impl<I, T, F> Iterator for Filter<I, T, F>
    where
        I: Iterator<Item = T>,
        F: Fn(&T) -> bool,
    {
        type Item = T;

        fn next(&mut self) -> Option<Self::Item> {
            self.list.find(|val| (self.predicate)(val))
        }
    }

    Filter { list, predicate }
}

pub fn length<I: Iterator<Item = T>, T>(list: I) -> usize {
    let mut len = 0;

    for _ in list {
        len += 1;
    }

    len
}

pub fn map<I, F, T, U>(list: I, function: F) -> impl Iterator<Item = U>
where
    I: Iterator<Item = T>,
    F: Fn(T) -> U,
{
    struct Map<I: Iterator<Item = T>, F: Fn(T) -> U, T, U> {
        list: I,
        function: F,
    }

    impl<I: Iterator<Item = T>, F: Fn(T) -> U, T, U> Iterator for Map<I, F, T, U> {
        type Item = U;

        fn next(&mut self) -> Option<Self::Item> {
            self.list.next().map(&self.function)
        }
    }

    Map { list, function }
}

pub fn foldl<I, F, T, U>(list: I, initial: U, function: F) -> U
where
    I: Iterator<Item = T>,
    F: Fn(U, T) -> U,
{
    let mut result = initial;

    for item in list {
        result = (function)(result, item)
    }

    result
}

pub fn foldr<I, F, T, U>(mut list: I, initial: U, function: F) -> U
where
    I: DoubleEndedIterator<Item = T>,
    F: Fn(U, T) -> U,
{
    let mut result = initial;

    while let Some(item) = list.next_back() {
        result = (function)(result, item)
    }

    result
}

pub fn reverse<I: DoubleEndedIterator<Item = T>, T>(list: I) -> impl Iterator<Item = T> {
    struct Reverse<I: DoubleEndedIterator<Item = T>, T> {
        list: I,
    }

    impl<I: DoubleEndedIterator<Item = T>, T> Iterator for Reverse<I, T> {
        type Item = T;

        fn next(&mut self) -> Option<Self::Item> {
            self.list.next_back()
        }
    }

    Reverse { list }
}
