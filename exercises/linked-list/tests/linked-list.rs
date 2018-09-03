extern crate linked_list;
use linked_list::*;

// Add your tests here
#[test]
fn empty_list() {
    let list: LinkedList<i32> = LinkedList::new();
    assert_eq!(list.len(), 0);
}

#[test]
fn single_element() {
    let mut list: LinkedList<i32> = LinkedList::new();
    list.push_back(5);
    assert_eq!(list.len(), 1);
    assert_eq!(list.pop_front(), Some(5));
}

#[test]
fn push_pop_back_multiple() {
    let mut list: LinkedList<i32> = LinkedList::new();
    for i in 0..10 {
        list.push_back(i);
    }
    assert_eq!(list.len(), 10);
    for i in (0..10).rev() {
        assert_eq!(i, list.pop_back().unwrap());
    }
}

#[test]
fn push_pop_front_multiple() {
    let mut list: LinkedList<i32> = LinkedList::new();
    for i in 0..10 {
        list.push_front(i);
    }
    assert_eq!(list.len(), 10);
    for i in (0..10).rev() {
        assert_eq!(i, list.pop_front().unwrap());
    }
}

#[test]
fn push_front_pop_back() {
    let mut list: LinkedList<i32> = LinkedList::new();
    for i in 0..10 {
        list.push_front(i);
    }
    for i in 0..10 {
        assert_eq!(i, list.pop_back().unwrap());
    }
}

#[test]
fn many_list() {
    let mut list: LinkedList<i32> = LinkedList::new();
    for num in 0..10 {
        list.push_back(num);
    }
    assert_eq!(list.len(), 10);

    for (num, &entered_num) in (0..10).zip(list.iter()) {
        assert_eq!(num, entered_num);
    }
}

// or same number of leaks as double frees
#[test]
fn no_leaks_or_double_frees() {
    use std::cell::Cell;
    struct Counter<'a>(&'a Cell<usize>);

    impl<'a> Drop for Counter<'a> {
        fn drop(&mut self) {
            let num = self.0.get();
            self.0.set(num-1);
        }
    }

    const N: usize  = 15;

    let counter = Cell::new(N);
    let mut list = LinkedList::new();
    for _ in 0..N {
        list.push_back(Counter(&counter));
    }
    assert_eq!(list.len(), N);
    drop(list);
    assert_eq!(counter.get(), 0);
}

#[allow(dead_code)]
#[test]
fn is_covariant() {
    fn a<'a>(x: LinkedList<&'static str>) -> LinkedList<&'a str> {
        x
    }
}

#[test]
fn clone_is_equal() {
    let mut list = LinkedList::new();
    for i in 0..10 {
        list.push_back(i);
    }
    let mut list2 = list.clone();

    for _ in 0..10 {
        assert_eq!(list.pop_back(), list2.pop_back());
    }
}

#[test]
fn insert_middle() {
    let mut list = LinkedList::new();
    for i in 0..10 {
        list.push_back(i);
    }
    let mut list2 = list.clone();

    {
        let mut cursor = list.cursor_tail();
        cursor.seek_forward(4);
        cursor.insert_list_after(&mut list2);
    }

    assert_eq!(list.len(), 20);
    assert_eq!(list2.len(), 0);

    let expected = (0..5).chain(0..10).chain(5..10);

    for (exp, &actual) in expected.zip(list.iter()) {
        assert_eq!(exp, actual);
    }
}
