use simple_linked_list::*;

mod count {
    #[test]
    fn empty_list_has_length_of_zero() {
        let mut list = SimpleLinkedList::new();
        // {"operation":"count","expected":0}
    }
    #[test]
    #[ignore]
    fn singleton_list_has_length_of_one() {
        let mut list = SimpleLinkedList::new();
        list.push(1);
        // {"operation":"count","expected":1}
    }
    #[test]
    #[ignore]
    fn non_empty_list_has_correct_length() {
        let mut list = SimpleLinkedList::new();
        list.push(1);
        list.push(2);
        list.push(3);
        // {"operation":"count","expected":3}
    }
}

mod pop {
    #[test]
    #[ignore]
    fn pop_from_empty_list_is_an_error() {
        let mut list = SimpleLinkedList::new();
        // {"operation":"pop","expected":{"error":"list is empty"}}
    }
    #[test]
    #[ignore]
    fn can_pop_from_singleton_list() {
        let mut list = SimpleLinkedList::new();
        list.push(1);
        // {"operation":"pop","expected":1}
    }
    #[test]
    #[ignore]
    fn can_pop_from_non_empty_list() {
        let mut list = SimpleLinkedList::new();
        list.push(1);
        list.push(2);
        // {"operation":"pop","expected":2}
    }
    #[test]
    #[ignore]
    fn can_pop_multiple_items() {
        let mut list = SimpleLinkedList::new();
        list.push(1);
        list.push(2);
        // {"operation":"pop","expected":2}
        // {"operation":"pop","expected":1}
    }
    #[test]
    #[ignore]
    fn pop_updates_the_count() {
        let mut list = SimpleLinkedList::new();
        list.push(1);
        list.push(2);
        // {"operation":"count","expected":2}
        // {"operation":"pop","expected":2}
        // {"operation":"count","expected":1}
        // {"operation":"pop","expected":1}
        // {"operation":"count","expected":0}
    }
}

mod push {
    #[test]
    #[ignore]
    fn can_push_to_an_empty_list() {
        let mut list = SimpleLinkedList::new();
        // {"operation":"push","value":1}
    }
    #[test]
    #[ignore]
    fn can_push_to_a_non_empty_list() {
        let mut list = SimpleLinkedList::new();
        list.push(1);
        list.push(2);
        // {"operation":"push","value":3}
    }
    #[test]
    #[ignore]
    fn push_updates_count() {
        let mut list = SimpleLinkedList::new();
        list.push(1);
        list.push(2);
        // {"operation":"push","value":3}
        // {"operation":"count","expected":3}
    }
    #[test]
    #[ignore]
    fn push_and_pop() {
        let mut list = SimpleLinkedList::new();
        // {"operation":"push","value":1}
        // {"operation":"push","value":2}
        // {"operation":"pop","expected":2}
        // {"operation":"push","value":3}
        // {"operation":"count","expected":2}
        // {"operation":"pop","expected":3}
        // {"operation":"pop","expected":1}
        // {"operation":"count","expected":0}
    }
}

mod peek {
    #[test]
    #[ignore]
    fn peek_on_empty_list_is_an_error() {
        let mut list = SimpleLinkedList::new();
        // {"operation":"peek","expected":{"error":"list is empty"}}
    }
    #[test]
    #[ignore]
    fn can_peek_on_singleton_list() {
        let mut list = SimpleLinkedList::new();
        list.push(1);
        // {"operation":"peek","expected":1}
    }
    #[test]
    #[ignore]
    fn can_peek_on_non_empty_list() {
        let mut list = SimpleLinkedList::new();
        list.push(1);
        list.push(2);
        // {"operation":"peek","expected":2}
    }
    #[test]
    #[ignore]
    fn peek_does_not_change_the_count() {
        let mut list = SimpleLinkedList::new();
        list.push(1);
        list.push(2);
        // {"operation":"peek","expected":2}
        // {"operation":"count","expected":2}
    }
    #[test]
    #[ignore]
    fn can_peek_after_a_pop_and_push() {
        let mut list = SimpleLinkedList::new();
        // {"operation":"push","value":1}
        // {"operation":"push","value":2}
        // {"operation":"peek","expected":2}
        // {"operation":"pop","expected":2}
        // {"operation":"peek","expected":1}
        // {"operation":"push","value":3}
        // {"operation":"peek","expected":3}
    }
}

mod to_list_l_i_f_o {
    #[test]
    #[ignore]
    fn empty_linked_list_to_list_is_empty() {
        let mut list = SimpleLinkedList::new();
        // {"operation":"toList","expected":[]}
    }
    #[test]
    #[ignore]
    fn to_list_with_multiple_values() {
        let mut list = SimpleLinkedList::new();
        list.push(1);
        list.push(2);
        list.push(3);
        // {"operation":"toList","expected":[3,2,1]}
    }
    #[test]
    #[ignore]
    fn to_list_after_a_pop() {
        let mut list = SimpleLinkedList::new();
        // {"operation":"push","value":1}
        // {"operation":"push","value":2}
        // {"operation":"push","value":3}
        // {"operation":"pop","expected":3}
        // {"operation":"push","value":4}
        // {"operation":"toList","expected":[4,2,1]}
    }
}

mod to_list_f_i_f_o {
    #[test]
    #[ignore]
    fn empty_linked_list_to_list_is_empty() {
        let mut list = SimpleLinkedList::new();
        // {"operation":"toList","expected":[]}
    }
    #[test]
    #[ignore]
    fn to_list_with_multiple_values() {
        let mut list = SimpleLinkedList::new();
        list.push(1);
        list.push(2);
        list.push(3);
        // {"operation":"toList","expected":[1,2,3]}
    }
    #[test]
    #[ignore]
    fn to_list_after_a_pop() {
        let mut list = SimpleLinkedList::new();
        // {"operation":"push","value":1}
        // {"operation":"push","value":2}
        // {"operation":"push","value":3}
        // {"operation":"pop","expected":3}
        // {"operation":"push","value":4}
        // {"operation":"toList","expected":[1,2,4]}
    }
}

mod reverse {
    #[test]
    #[ignore]
    fn reversed_empty_list_has_same_values() {
        let mut list = SimpleLinkedList::new();
        // {"operation":"reverse"}
        // {"operation":"toList","expected":[]}
    }
    #[test]
    #[ignore]
    fn reversed_singleton_list_is_same_list() {
        let mut list = SimpleLinkedList::new();
        list.push(1);
        // {"operation":"reverse"}
        // {"operation":"toList","expected":[1]}
    }
    #[test]
    #[ignore]
    fn reversed_non_empty_list_is_reversed() {
        let mut list = SimpleLinkedList::new();
        list.push(1);
        list.push(2);
        list.push(3);
        // {"operation":"reverse"}
        // {"operation":"count","expected":3}
        // {"operation":"pop","expected":1}
        // {"operation":"pop","expected":2}
        // {"operation":"pop","expected":3}
    }
    #[test]
    #[ignore]
    fn double_reverse() {
        let mut list = SimpleLinkedList::new();
        list.push(1);
        list.push(2);
        list.push(3);
        // {"operation":"reverse"}
        // {"operation":"reverse"}
        // {"operation":"pop","expected":3}
        // {"operation":"pop","expected":2}
        // {"operation":"pop","expected":1}
    }
}
