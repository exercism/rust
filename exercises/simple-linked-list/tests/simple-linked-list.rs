use simple_linked_list::SimpleLinkedList;

#[test]
fn test_new_list_is_empty() {
    let list: SimpleLinkedList<u32> = SimpleLinkedList::new();
    assert_eq!(0, list.len(), "list's length must be 0");
}

#[test]
#[ignore]
fn test_push_increments_length() {
    let mut list: SimpleLinkedList<u32> = SimpleLinkedList::new();
    list.push(1);
    assert_eq!(1, list.len(), "list's length must be 1");
    list.push(2);
    assert_eq!(2, list.len(), "list's length must be 2");
}

#[test]
#[ignore]
fn test_pop_decrements_length() {
    let mut list: SimpleLinkedList<u32> = SimpleLinkedList::new();
    list.push(1);
    list.push(2);
    list.pop();
    assert_eq!(1, list.len(), "list's length must be 1");
    list.pop();
    assert_eq!(0, list.len(), "list's length must be 0");
}

#[test]
#[ignore]
fn test_pop_returns_last_added_element() {
    let mut list: SimpleLinkedList<u32> = SimpleLinkedList::new();
    list.push(1);
    list.push(2);
    assert_eq!(Some(2), list.pop(), "Element must be 2");
    assert_eq!(Some(1), list.pop(), "Element must be 1");
    assert_eq!(None, list.pop(), "No element should be contained in list");
}

#[test]
#[ignore]
fn test_peek_returns_head_element() {
    let mut list: SimpleLinkedList<u32> = SimpleLinkedList::new();
    assert_eq!(None, list.peek(), "No element should be contained in list");
    list.push(2);
    assert_eq!(Some(&2), list.peek(), "Element must be 2");
    assert_eq!(Some(&2), list.peek(), "Element must be still 2");
}

#[test]
#[ignore]
fn test_from_slice() {
    let array = ["1", "2", "3", "4"];
    let mut list = SimpleLinkedList::from(array.as_ref());
    assert_eq!(Some("4"), list.pop());
    assert_eq!(Some("3"), list.pop());
    assert_eq!(Some("2"), list.pop());
    assert_eq!(Some("1"), list.pop());
}

#[test]
#[ignore]
fn test_reverse() {
    let mut list: SimpleLinkedList<u32> = SimpleLinkedList::new();
    list.push(1);
    list.push(2);
    list.push(3);
    let mut rev_list = list.rev();
    assert_eq!(Some(1), rev_list.pop());
    assert_eq!(Some(2), rev_list.pop());
    assert_eq!(Some(3), rev_list.pop());
    assert_eq!(None, rev_list.pop());
}

#[test]
#[ignore]
fn test_into_vector() {
    let mut v = Vec::new();
    let mut s = SimpleLinkedList::new();
    for i in 1..4 {
        v.push(i);
        s.push(i);
    }
    let s_as_vec: Vec<i32> = s.into();
    assert_eq!(v, s_as_vec);
}
