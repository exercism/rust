extern crate linked_list;
use linked_list::*;

fn to_raw_ptr<T>(opt: Option<&T>) -> *const T {
    opt.map_or(std::ptr::null(), |r| r as *const T)
}

#[test]
fn empty_list() {
    let list: LinkedList<i32> = LinkedList::new();
    assert_eq!(list.len(), 0);
}

#[test]
#[ignore]
fn single_element() {
    let mut list: LinkedList<i32> = LinkedList::new();
    list.push_back(5);

    assert_eq!(list.len(), 1);
    assert_eq!(
        to_raw_ptr(list.front()),
        to_raw_ptr(list.back())
    );
    assert_eq!(list.pop_front(), Some(5));
}

#[test]
#[ignore]
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
#[ignore]
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
#[ignore]
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
#[ignore]
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
#[ignore]
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
    let list = std::iter::repeat_with(|| Counter(&counter))
        .take(N)
        .collect::<LinkedList<_>>();

    assert_eq!(list.len(), N);
    drop(list);
    assert_eq!(counter.get(), 0);
}


#[test]
#[ignore]
fn clone_is_equal() {
    let list = (0..10).collect::<LinkedList<_>>();
    let list2 = list.clone();

    assert!(
        list.iter().eq(list2.iter())
    );
}

#[test]
#[ignore]
fn insert_middle() {
    let mut list = LinkedList::new();
    for i in 0..10 {
        list.push_back(i);
    }

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

    assert!(
        expected.eq( list.iter().cloned() )
    );
}

#[test]
#[ignore]
fn back_front_changes_on_push_back() {
    let mut backs = vec![];
    let mut fronts = vec![];
    let mut list = LinkedList::new();

    for i in 0..10 {
        list.push_back(i);
        backs.push( to_raw_ptr(list.back()) );
        fronts.push( to_raw_ptr(list.front()) );
    }
    fronts.dedup();

    assert_eq!(fronts.len(), 1);
    assert_eq!(fronts[0], backs[0]);

    backs.sort();
    backs.dedup();

    assert_eq!(backs.len(), 10);
}

#[test]
#[ignore]
fn back_front_changes_on_push_front() {
    let mut backs = vec![];
    let mut fronts = vec![];
    let mut list = LinkedList::new();

    for i in 0..10 {
        list.push_front(i);
        backs.push( to_raw_ptr(list.back()) );
        fronts.push( to_raw_ptr(list.front()) );
    }
    backs.dedup();

    assert_eq!(backs.len(), 1);
    assert_eq!(backs[0], fronts[0]);

    fronts.sort();
    fronts.dedup();

    assert_eq!(fronts.len(), 10);
}

#[cfg(feature = "advanced")]
#[test]
#[ignore]
fn linked_list_is_send_sync() {
    trait AssertSend: Send {}
    trait AssertSync: Sync {}

    impl<T: Send> AssertSend for LinkedList<T> {}
    impl<T: Sync> AssertSync for LinkedList<T> {}
}

#[cfg(feature = "advanced")]
#[allow(dead_code)]
#[test]
#[ignore]
fn is_covariant() {
    fn a<'a>(x: LinkedList<&'static str>) -> LinkedList<&'a str> {
        x
    }

    fn a_iter<'a>(i: Iter<'static, &'static str>) -> Iter<'a, &'a str> {
        i
    }
}

#[test]
#[ignore]
fn is_generic() {
    struct Foo;
    LinkedList::<Foo>::new();
}
