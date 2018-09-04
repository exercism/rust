use std::ptr::NonNull;

type NodePtr<T> = NonNull<Node<T>>;
type OptNodePtr<T> = Option<NodePtr<T>>;

pub struct LinkedList<T> {
    head: OptNodePtr<T>,
    tail: OptNodePtr<T>,
    len: usize,
    marker: std::marker::PhantomData<Box<T>>, // for dropck
}

pub struct Cursor<'a, T: 'a> {
    list: &'a mut LinkedList<T>,
    node: OptNodePtr<T>,
}

struct Node<T> {
    element: T,
    next: OptNodePtr<T>,
    prev: OptNodePtr<T>,
}

impl<T> Node<T> {
    fn new_linkless(element: T) -> NodePtr<T> {
        Self::allocate(element, None, None)
    }

    fn allocate(element: T, prev: OptNodePtr<T>, next: OptNodePtr<T>) -> NodePtr<T> {
        unsafe {
            NonNull::new_unchecked(Box::into_raw(Box::new(Self {
                element,
                next,
                prev,
            })))
        }
    }
}

trait NodePtrHelper<T> {
    fn get_next(&mut self) -> &mut OptNodePtr<T>;
    fn get_prev(&mut self) -> &mut OptNodePtr<T>;
    fn link(left: NodePtr<T>, right: NodePtr<T>);
    fn insert_between(self, prev: NodePtr<T>, next: NodePtr<T>) -> NodePtr<T>;
    fn insert_new_after(self, element: T) -> NodePtr<T>;
    fn insert_new_before(self, element: T) -> NodePtr<T>;
    fn unlink_next(&mut self) -> OptNodePtr<T>;
    fn unlink_prev(&mut self) -> OptNodePtr<T>;
    fn into_inner(self) -> T;
}

impl<T> NodePtrHelper<T> for NodePtr<T> {
    fn get_next(&mut self)-> &mut OptNodePtr<T> {
        unsafe { &mut self.as_mut().next }
    }

    fn get_prev(&mut self)-> &mut OptNodePtr<T> {
        unsafe { &mut self.as_mut().prev }
    }

    fn link(mut left: NodePtr<T>, mut right: NodePtr<T>) {
        *left.get_next() = Some(right);
        *right.get_prev() = Some(left);
    }

    fn insert_between(self, prev: NodePtr<T>, next: NodePtr<T>) -> Self {
        Self::link(prev, self);
        Self::link(self, next);
        self
    }

    fn insert_new_after(mut self, element: T) -> Self {
        if let Some(next) = *self.get_next() {
            Node::new_linkless(element)
                .insert_between(self, next)
        } else {
            let new_node = Node::new_linkless(element);
            NodePtr::link(self, new_node);
            new_node
        }
    }

    fn insert_new_before(mut self, element: T) -> Self {
        if let Some(prev) = *self.get_prev() {
            Node::new_linkless(element)
                .insert_between(prev, self)
        } else {
            let new_node = Node::new_linkless(element);
            NodePtr::link(new_node, self);
            new_node
        }
    }

    // returns next of self, if it exists
    fn unlink_next(&mut self) -> OptNodePtr<T> {
        self.get_next().map(|mut next| {
            *next.get_prev() = None;
            //self.as_mut().next = None;
            next
        })    }

    // returns prev of self, if it exists
    fn unlink_prev(&mut self) -> OptNodePtr<T> {
        self.get_prev().map(|mut prev| {
            *prev.get_next() = None;
            //self.as_mut().prev = None;
            prev
        })
    }

    // must be unlinked from all others
    fn into_inner(self) -> T {
        unsafe {
            Box::from_raw(self.as_ptr()).element
        }
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            head: None,
            tail: None,
            len: 0,
            marker: std::marker::PhantomData,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn push_back(&mut self, element: T) {
        self.cursor_head().insert_after(element);
    }

    pub fn push_front(&mut self, element: T) {
        self.cursor_tail().insert_before(element);
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.cursor_head().take()
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.cursor_tail().take()
    }

    pub fn cursor_tail(&mut self) -> Cursor<T> {
        Cursor {
            node: self.tail,
            list: self,
        }
    }

    pub fn cursor_head(&mut self) -> Cursor<T> {
        Cursor {
            node: self.head,
            list: self,
        }
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            next_node: self.tail,
            marker: std::marker::PhantomData,
        }
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop_back() {}
    }
}

impl<T: Clone> Clone for LinkedList<T> {
    fn clone(&self) -> Self {
        let mut new_list = LinkedList::new();
        for element in self.iter().cloned() {
            new_list.push_back(element);
        }
        new_list
    }
}

pub struct Iter<'a, T: 'a> {
    next_node: OptNodePtr<T>,
    marker: std::marker::PhantomData<&'a LinkedList<T>>,
}

impl<'a, T: 'a> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let node_ptr = self.next_node?;
        unsafe {
            let current_node = &*node_ptr.as_ptr();
            self.next_node = current_node.next;
            Some(&current_node.element)
        }
    }
}

impl<'a, T: 'a> Cursor<'a, T> {
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        unsafe {
            self.node.map(|node| &mut (*node.as_ptr()).element)
        }
    }

    // FIXME: I'm pretty certain this lifetime is UNSOUND
    //        hacked only to get Iter working, temporarily
    pub fn next(&mut self) -> Option<&mut T> {
        self._step(|node| node.next)
    }

    pub fn prev(&mut self) -> Option<&mut T> {
        self._step(|node| node.prev)
    }

    fn _step(
        &mut self,
        get_next: impl Fn(&mut Node<T>) -> OptNodePtr<T>
    ) -> Option<&mut T> {
        unsafe {
            if let Some(new_pos) = get_next(self.node.as_mut()?.as_mut()) {
                self.node = Some(new_pos);
                Some(&mut (*new_pos.as_ptr()).element)
            } else {
                None
            }
        }
    }

    pub fn take(&mut self) -> Option<T> {
        let mut node = self.node?;
        let next = node.unlink_next();
        let prev = node.unlink_prev();

        match (prev, next) {
            (Some(prev), Some(next)) => NodePtr::link(prev, next),
            (Some(_), None) => self.list.head = prev,
            (None, Some(_)) => self.list.tail = next,
            _ => {
                self.list.head = None;
                self.list.tail = None;
            },
        };
        self.list.len -= 1;
        Some(node.into_inner())
    }

    pub fn insert_after(&mut self, element: T) {
        self._insert(element, |list, cursor_node, element| {
            let new_node = cursor_node.insert_new_after(element);
            if list.head == Some(cursor_node) {
                list.head = Some(new_node);
            }
        });
    }

    pub fn insert_before(&mut self, element: T) {
        self._insert(element, |list, cursor_node, element| {
            let new_node = cursor_node.insert_new_before(element);
            if list.tail == Some(cursor_node) {
                list.tail = Some(new_node);
            }
        });
    }

    // put into list, if empty, else do whatever callback says
    fn _insert(&mut self, element: T, callback: impl Fn(&mut LinkedList<T>, NodePtr<T>, T)) {
        let cursor_node = match self.node {
            Some(node) => node,
            None => { // list empty
                self.node = Some(Node::new_linkless(element));
                self.list.head = self.node;
                self.list.tail = self.node;
                self.list.len += 1;
                return
            }
        };
        callback(&mut self.list, cursor_node, element);
        self.list.len += 1;
    }

    fn _insert_list_on_empty_list(&mut self, other: &mut LinkedList<T>) {
        debug_assert!(self.list.len == 0);
        std::mem::swap(self.list, other);
        self.node = self.list.tail;
    }

    fn _insert_list(
        &mut self,
        other: &mut LinkedList<T>,
        callback: impl Fn(NodePtr<T>, NodePtr<T>, NodePtr<T>)
    ) {
        unsafe {
            let cursor_node = match self.node {
                Some(node) => node,
                None => {
                    self._insert_list_on_empty_list(other);
                    return;
                }
            };

            if let LinkedList { head: Some(head), tail: Some(tail), len, .. } = *other {
                debug_assert!(head.as_ref().next.is_none());
                debug_assert!(tail.as_ref().prev.is_none());
                std::ptr::write(other, LinkedList::new()); // empty without dropping

                // link up on the right or left side
                callback(cursor_node, head, tail);
                self.list.len += len; // may overflow
            }
        }
    }

    pub fn insert_list_after(&mut self, other: &mut LinkedList<T>) {
        unsafe {
            self._insert_list(other, |mut cursor_node, head, tail| {
                if let Some(next) = cursor_node.as_mut().next {
                    NodePtr::link(head, next);
                }
                NodePtr::link(cursor_node, tail);
            });
        }
    }

    pub fn insert_list_before(&mut self, other: &mut LinkedList<T>) {
        unsafe {
            self._insert_list(other, |mut cursor_node, head, tail| {
                if let Some(prev) = cursor_node.as_mut().prev {
                    NodePtr::link(prev, tail);
                }
                NodePtr::link(head, cursor_node);
            });
        }
    }
}
