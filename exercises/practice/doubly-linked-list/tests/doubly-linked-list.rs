use doubly_linked_list::*;

#[test]
fn is_generic() {
    struct Foo;
    LinkedList::<Foo>::new();
}

// ———————————————————————————————————————————————————————————
// Tests for Step 1: push / pop at front and back
// ———————————————————————————————————————————————————————————

#[test]
#[ignore]
fn basics_empty_list() {
    let list: LinkedList<i32> = LinkedList::new();
    assert_eq!(list.len(), 0);
    assert!(list.is_empty());
}

// push / pop at back ————————————————————————————————————————
#[test]
#[ignore]
fn basics_single_element_back() {
    let mut list: LinkedList<i32> = LinkedList::new();
    list.push_back(5);

    assert_eq!(list.len(), 1);
    assert!(!list.is_empty());

    assert_eq!(list.pop_back(), Some(5));

    assert_eq!(list.len(), 0);
    assert!(list.is_empty());
}

#[test]
#[ignore]
fn basics_push_pop_at_back() {
    let mut list: LinkedList<i32> = LinkedList::new();
    for i in 0..10 {
        list.push_back(i);
        assert_eq!(list.len(), i as usize + 1);
        assert!(!list.is_empty());
    }
    for i in (0..10).rev() {
        assert_eq!(list.len(), i as usize + 1);
        assert!(!list.is_empty());
        assert_eq!(i, list.pop_back().unwrap());
    }
    assert_eq!(list.len(), 0);
    assert!(list.is_empty());
}

// push / pop at front ———————————————————————————————————————
#[test]
#[ignore]
fn basics_single_element_front() {
    let mut list: LinkedList<i32> = LinkedList::new();
    list.push_front(5);

    assert_eq!(list.len(), 1);
    assert!(!list.is_empty());

    assert_eq!(list.pop_front(), Some(5));

    assert_eq!(list.len(), 0);
    assert!(list.is_empty());
}

#[test]
#[ignore]
fn basics_push_pop_at_front() {
    let mut list: LinkedList<i32> = LinkedList::new();
    for i in 0..10 {
        list.push_front(i);
        assert_eq!(list.len(), i as usize + 1);
        assert!(!list.is_empty());
    }
    for i in (0..10).rev() {
        assert_eq!(list.len(), i as usize + 1);
        assert!(!list.is_empty());
        assert_eq!(i, list.pop_front().unwrap());
    }
    assert_eq!(list.len(), 0);
    assert!(list.is_empty());
}

// push / pop at mixed sides —————————————————————————————————
#[test]
#[ignore]
fn basics_push_front_pop_back() {
    let mut list: LinkedList<i32> = LinkedList::new();
    for i in 0..10 {
        list.push_front(i);
        assert_eq!(list.len(), i as usize + 1);
        assert!(!list.is_empty());
    }
    for i in 0..10 {
        assert_eq!(list.len(), 10 - i as usize);
        assert!(!list.is_empty());
        assert_eq!(i, list.pop_back().unwrap());
    }
    assert_eq!(list.len(), 0);
    assert!(list.is_empty());
}

#[test]
#[ignore]
fn basics_push_back_pop_front() {
    let mut list: LinkedList<i32> = LinkedList::new();
    for i in 0..10 {
        list.push_back(i);
        assert_eq!(list.len(), i as usize + 1);
        assert!(!list.is_empty());
    }
    for i in 0..10 {
        assert_eq!(list.len(), 10 - i as usize);
        assert!(!list.is_empty());
        assert_eq!(i, list.pop_front().unwrap());
    }
    assert_eq!(list.len(), 0);
    assert!(list.is_empty());
}

// ———————————————————————————————————————————————————————————
// Tests for Step 2: iteration
// ———————————————————————————————————————————————————————————

#[test]
#[ignore]
fn iter() {
    let mut list: LinkedList<i32> = LinkedList::new();
    for num in 0..10 {
        list.push_back(num);
    }

    for (num, &entered_num) in (0..10).zip(list.iter()) {
        assert_eq!(num, entered_num);
    }
}

// ———————————————————————————————————————————————————————————
// Tests for Step 3: full cursor functionality
// ———————————————————————————————————————————————————————————

#[test]
#[ignore]
fn cursor_insert_before_on_empty_list() {
    // insert_after on empty list is already tested via push_back()
    let mut list = LinkedList::new();
    list.cursor_front().insert_before(0);
    assert_eq!(Some(0), list.pop_front());
}

#[test]
#[ignore]
fn cursor_insert_after_in_middle() {
    let mut list = (0..10).collect::<LinkedList<_>>();

    {
        let mut cursor = list.cursor_front();
        let didnt_run_into_end = cursor.seek_forward(4);
        assert!(didnt_run_into_end);

        for n in (0..10).rev() {
            cursor.insert_after(n);
        }
    }

    assert_eq!(list.len(), 20);

    let expected = (0..5).chain(0..10).chain(5..10);

    assert!(expected.eq(list.iter().cloned()));
}

#[test]
#[ignore]
fn cursor_insert_before_in_middle() {
    let mut list = (0..10).collect::<LinkedList<_>>();

    {
        let mut cursor = list.cursor_back();
        let didnt_run_into_end = cursor.seek_backward(4);
        assert!(didnt_run_into_end);

        for n in 0..10 {
            cursor.insert_before(n);
        }
    }

    assert_eq!(list.len(), 20);

    let expected = (0..5).chain(0..10).chain(5..10);

    assert!(expected.eq(list.iter().cloned()));
}

// "iterates" via next() and checks that it visits the right elements
#[test]
#[ignore]
fn cursor_next_and_peek() {
    let mut list = (0..10).collect::<LinkedList<_>>();
    let mut cursor = list.cursor_front();

    assert_eq!(cursor.peek_mut(), Some(&mut 0));

    for n in 1..10 {
        let next = cursor.next().cloned();
        assert_eq!(next, Some(n));
        assert_eq!(next, cursor.peek_mut().cloned());
    }
}

// "iterates" via prev() and checks that it visits the right elements
#[test]
#[ignore]
fn cursor_prev_and_peek() {
    let mut list = (0..10).collect::<LinkedList<_>>();
    let mut cursor = list.cursor_back();

    assert_eq!(cursor.peek_mut(), Some(&mut 9));

    for n in (0..9).rev() {
        let prev = cursor.prev().cloned();
        assert_eq!(prev, Some(n));
        assert_eq!(prev, cursor.peek_mut().cloned());
    }
}

// removes all elements starting from the middle
#[test]
#[ignore]
fn cursor_take() {
    let mut list = (0..10).collect::<LinkedList<_>>();
    let mut cursor = list.cursor_front();
    cursor.seek_forward(5);

    for expected in (5..10).chain((0..5).rev()) {
        assert_eq!(cursor.take(), Some(expected));
    }
}

// ———————————————————————————————————————————————————————————
// Tests for Step 4: clean-up via `Drop`
// ———————————————————————————————————————————————————————————

// The leak tests that are also for this step are separated into
// their own files so that nothing else interferes with the allocator
// whilst they run

// checks number of drops
// may pass for incorrect programs if double frees happen
// exactly as often as destructor leaks
#[test]
#[ignore]
fn drop_no_double_frees() {
    use std::cell::Cell;
    struct DropCounter<'a>(&'a Cell<usize>);

    impl Drop for DropCounter<'_> {
        fn drop(&mut self) {
            let num = self.0.get();
            self.0.set(num + 1);
        }
    }

    const N: usize = 15;

    let counter = Cell::new(0);
    let list = std::iter::repeat_with(|| DropCounter(&counter))
        .take(N)
        .collect::<LinkedList<_>>();

    assert_eq!(list.len(), N);
    drop(list);
    assert_eq!(counter.get(), N);
}

#[test]
#[ignore]
fn drop_large_list() {
    drop((0..2_000_000).collect::<LinkedList<i32>>());
}

// ———————————————————————————————————————————————————————————
// Tests for Step 5 (advanced): covariance and Send/Sync
// ———————————————————————————————————————————————————————————

// These are compile time tests. They won't compile unless your
// code passes.
// Additional tests for code that must *not* compile are in
// pre_implemented.rs for technical reasons.

#[cfg(feature = "advanced")]
#[test]
#[ignore]
fn advanced_linked_list_is_send_sync() {
    trait AssertSend: Send {}
    trait AssertSync: Sync {}

    impl<T: Send> AssertSend for LinkedList<T> {}
    impl<T: Sync> AssertSync for LinkedList<T> {}
}

#[cfg(feature = "advanced")]
#[allow(dead_code)]
#[test]
#[ignore]
fn advanced_is_covariant() {
    fn a<'a>(x: LinkedList<&'static str>) -> LinkedList<&'a str> {
        x
    }

    fn a_iter<'a>(i: Iter<'static, &'static str>) -> Iter<'a, &'a str> {
        i
    }
}
