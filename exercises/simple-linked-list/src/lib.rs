use std::iter::FromIterator;

pub struct SimpleLinkedList<T> {
    // Delete this field
    // dummy is needed to avoid unused parameter error during compilation
    dummy: ::std::marker::PhantomData<T>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        unimplemented!()
    }

    pub fn len(&self) -> usize {
        unimplemented!()
    }

    pub fn push(&mut self, _element: T) {
        unimplemented!()
    }

    pub fn pop(&mut self) -> Option<T> {
        unimplemented!()
    }

    pub fn peek(&self) -> Option<&T> {
        unimplemented!()
    }

    pub fn rev(self) -> SimpleLinkedList<T> {
        unimplemented!()
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        unimplemented!()
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
        unimplemented!()
    }
}
