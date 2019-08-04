//! Everything in thes file is implemented in terms of required functionality.
//! You are free to use anything, if it suits you.
//! They are useful for the test framework, but the implementation is trivial.
//! We supply them to reduce work both for you and the mentors.
use crate::{Cursor, LinkedList};

impl<T> LinkedList<T> {
    pub fn push_back(&mut self, element: T) {
        self.cursor_back().insert_after(element);
    }

    pub fn push_front(&mut self, element: T) {
        self.cursor_front().insert_before(element);
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.cursor_back().take()
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.cursor_front().take()
    }

    pub fn front(&self) -> Option<&T> {
        self.iter().next()
    }

    pub fn back(&self) -> Option<&T> {
        self.iter().last()
    }
}

impl<T> std::iter::FromIterator<T> for LinkedList<T> {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        let mut list = Self::new();
        for elem in iter {
            list.push_back(elem);
        }
        list
    }
}

// seek methods, return false if end of list is reached prematurely
impl<T> Cursor<'_, T> {
    pub fn seek_forward(&mut self, n: usize) -> bool {
        (0..n).all(|_| self.next().is_some())
    }

    pub fn seek_backward(&mut self, n: usize) -> bool {
        (0..n).all(|_| self.prev().is_some())
    }
}

// These are tests for code that must not compile. They need to be here (or in lib.rs)
// because only doctests can use `compile_fail` without additional dependencies
// and doctests are ignored inside tests/doubly-linked-list.rs.

#[allow(unused)]
#[cfg(feature = "advanced")]
/// ```compile_fail
/// use doubly_linked_list::LinkedList;
/// trait AssertSend: Send {}
/// impl<T> AssertSend for LinkedList<T> {}
/// ```
pub struct IllegalSend;

#[allow(unused)]
#[cfg(feature = "advanced")]
/// ```compile_fail
/// use doubly_linked_list::LinkedList;
/// trait AssertSync: Sync {}
/// impl<T> AssertSync for LinkedList<T> {}
/// ```
pub struct IllegalSync;
