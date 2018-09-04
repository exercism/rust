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
    assert_eq!(list.len(), 0);
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
    assert_eq!(list.len(), 0);
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
        for _ in 0..4 {
            cursor.next();
        }
        //cursor.seek_forward(4);
        cursor.insert_list_after(&mut list2);
    }

    assert_eq!(list.len(), 20);
    assert_eq!(list2.len(), 0);

    let expected = (0..5).chain(0..10).chain(5..10);

    for (exp, &actual) in expected.zip(list.iter()) {
        assert_eq!(exp, actual);
    }
}

#[test]
fn head_tail_changes_on_push_back() {
    let mut heads = vec![];
    let mut tails = vec![];
    let mut list = LinkedList::new();

    for i in 0..10 {
        list.push_back(i);
        heads.push(list.cursor_head().peek_mut().map_or(std::ptr::null(), |r| r as *const i32));
        tails.push(list.cursor_tail().peek_mut().map_or(std::ptr::null(), |r| r as *const i32));
    }
    tails.sort();
    tails.dedup();

    assert_eq!(tails.len(), 1);
    assert_eq!(tails[0], heads[0]);

    heads.sort();
    heads.dedup();

    assert_eq!(heads.len(), 10);
}

#[test]
fn head_tail_changes_on_push_front() {
    let mut heads = vec![];
    let mut tails = vec![];
    let mut list = LinkedList::new();

    for i in 0..10 {
        list.push_front(i);
        heads.push(list.cursor_head().peek_mut().map_or(std::ptr::null(), |r| r as *const i32));
        tails.push(list.cursor_tail().peek_mut().map_or(std::ptr::null(), |r| r as *const i32));
    }
    heads.sort();
    heads.dedup();

    assert_eq!(heads.len(), 1);
    assert_eq!(heads[0], tails[0]);

    tails.sort();
    tails.dedup();

    assert_eq!(tails.len(), 10);
}

#[test]
fn linked_list_is_send_sync() {
    trait AssertSend: Send {}
    trait AssertSync: Sync {}

    impl<T: Send> AssertSend for LinkedList<T> {}
    impl<T: Sync> AssertSync for LinkedList<T> {}
}

#[allow(dead_code)]
#[test]
fn is_covariant() {
    fn a<'a>(x: LinkedList<&'static str>) -> LinkedList<&'a str> {
        x
    }
}

#[test]
fn is_generic() {
    struct Foo;
    LinkedList::<Foo>::new();
}
