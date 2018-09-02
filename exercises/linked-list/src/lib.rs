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

    /* TODO: allocate and link, shorter and maybe optimized
    fn new(element: T, prev: NodePtr<T>, next: NodePtr<T>) -> NodePtr<T> {
        Self::allocate(element, Some(prev), Some(next))
    }

    fn with_prev(element: T, prev: NodePtr<T>) -> NodePtr<T> {
        Self::allocate(element, Some(prev), None)
    }

    fn with_next(element: T, next: NodePtr<T>) -> NodePtr<T> {
        Self::allocate(element, None, Some(next))
    }
    */
}

trait NodePtrHelper<T> {
    fn set_next(&mut self, node: NodePtr<T>);
    fn set_prev(&mut self, node: NodePtr<T>);
    fn link(left: NodePtr<T>, right: NodePtr<T>);
    fn insert_between(self, prev: NodePtr<T>, next: NodePtr<T>) -> NodePtr<T>;
    fn insert_new_after(self, element: T) -> NodePtr<T>;
    fn insert_new_before(self, element: T) -> NodePtr<T>;
    fn unlink_next(&mut self) -> OptNodePtr<T>;
    fn unlink_prev(&mut self) -> OptNodePtr<T>;
    fn into_inner(self) -> T;
}

impl<T> NodePtrHelper<T> for NodePtr<T> {
    fn set_next(&mut self, node: NodePtr<T>) {
        unsafe {
            self.as_mut().next = Some(node);
        }
    }

    fn set_prev(&mut self, node: NodePtr<T>) {
        unsafe {
            self.as_mut().next = Some(node);
        }
    }

    fn link(mut left: NodePtr<T>, mut right: NodePtr<T>) {
        left.set_next(right);
        right.set_prev(left);
    }

    fn insert_between(self, prev: NodePtr<T>, next: NodePtr<T>) -> Self {
        Self::link(prev, self);
        Self::link(self, next);
        self
    }

    fn insert_new_after(mut self, element: T) -> Self {
        unsafe {
            if let Some(next) = self.as_mut().next {
                Node::new_linkless(element)
                    .insert_between(self, next)
            } else {
                let new_node = Node::new_linkless(element);
                NodePtr::link(self, new_node);
                new_node
            }
        }
    }

    fn insert_new_before(mut self, element: T) -> Self {
        unsafe {
            if let Some(prev) = self.as_mut().prev {
                Node::new_linkless(element)
                    .insert_between(prev, self)
            } else {
                let new_node = Node::new_linkless(element);
                NodePtr::link(new_node, self);
                new_node
            }
        }
    }

    // returns next of self, if it exists
    fn unlink_next(&mut self) -> OptNodePtr<T> {
        unsafe {
            self.as_mut().next.map(|mut next| {
                next.as_mut().prev = None;
                //self.as_mut().next = None;
                next
            })
        }
    }

    // returns prev of self, if it exists
    fn unlink_prev(&mut self) -> OptNodePtr<T> {
        unsafe {
            self.as_mut().prev.map(|mut prev| {
                prev.as_mut().next = None;
                //self.as_mut().prev = None;
                prev
            })
        }
    }

    // must be unlinked from all others
    fn into_inner(self) -> T {
        unsafe {
            debug_assert!(self.as_ref().next.is_none());
            debug_assert!(self.as_ref().prev.is_none());
            let element = std::ptr::read(&(*self.as_ptr()).element);
            Box::from_raw(self.as_ptr());
            element
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

    fn _insert_first(&mut self, element: T) {
        let new_node = Node::new_linkless(element);
        self.head = Some(new_node);
        self.tail = self.head;
    }

    pub fn push_back(&mut self, element: T) {
        // FIXME (NLL): Use &mut, not take
        if let Some(head) = self.head.take() {
            self.head = Some(head.insert_new_after(element));
        } else {
            self._insert_first(element);
        }
        self.len += 1;
    }

    pub fn push_front(&mut self, element: T) {
        // FIXME (NLL): Use &mut, not take
        if let Some(tail) = self.tail.take() {
            self.tail = Some(tail.insert_new_before(element));
        } else {
            self._insert_first(element);
        }
        self.len += 1;
    }

    pub fn pop_back(&mut self) -> Option<T> {
        if let Some(mut head) = self.head.take() {
            if let Some(prev) = head.unlink_prev() {
                self.head = Some(prev);
            } else {
                self.tail = None;
            }
            self.len -= 1;
            Some(head.into_inner())
        } else {
            None
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if let Some(mut tail) = self.tail.take() {
            if let Some(next) = tail.unlink_next() {
                self.tail = Some(next);
            } else {
                self.head = None;
            }
            self.len -= 1;
            Some(tail.into_inner())
        } else {
            None
        }
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
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop_back() {}
        /* premature optimization
           to avoid putting head back in all the time
        if let Some((_, mut tail)) = self.head_tail {
            while let Some(next) = tail.unlink_next() {
                tail.into_inner();
                tail = next;
            }
        }
        */
    }
}

impl<'a, T: 'a> Cursor<'a, T> {
    pub fn peek(&self) -> Option<&T> {
        unsafe {
            self.node.as_ref().map(|node| &node.as_ref().element)
        }
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        unsafe {
            self.node.as_mut().map(|node| &mut node.as_mut().element)
        }
    }

    pub fn next(&mut self) -> Option<&mut T> {
        unsafe {
            if let Some(next) = self.node.as_mut()?.as_mut().next {
                self.node = Some(next);
                Some(&mut (*next.as_ptr()).element)
                //self.node.as_mut().map(|node| &mut (*node.as_ptr()).element)
            } else {
                None
            }
        }
    }

    pub fn prev(&mut self) -> Option<&mut T> {
        unsafe {
            if let Some(prev) = self.node.as_mut()?.as_mut().prev {
                self.node = Some(prev);
                Some(&mut (*prev.as_ptr()).element)
            } else {
                None
            }
        }
    }

    pub fn jump_to_tail(&mut self) {
        self.node = self.list.tail;
    }

    pub fn jump_to_head(&mut self) {
        self.node = self.list.head;
    }

    pub fn take(&mut self) -> Option<T> {
        let mut node = self.node?;
        let next = node.unlink_next();
        let prev = node.unlink_prev();

        match (prev, next) {
            (Some(prev), Some(next)) => NodePtr::link(prev, next),
            (Some(_), None) => self.list.head = prev,
            (None, Some(_)) => self.list.tail = next,
            _ => {},
        };
        self.list.len -= 1;
        Some(node.into_inner())
    }

    pub fn insert_after(&mut self, element: T) {
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
        cursor_node.insert_new_after(element);
        self.list.len += 1;
    }

    pub fn insert_before(&mut self, element: T) {
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
        cursor_node.insert_new_before(element);
        self.list.len += 1;
    }

    pub fn insert_list_after(&mut self, other: &mut LinkedList<T>) {
        unsafe {
            let mut cursor_node = match self.node {
                Some(node) => node,
                None => { // own list empty,
                    debug_assert!(self.list.len == 0);
                    std::mem::swap(self.list, other);
                    self.node = self.list.tail;
                    return;
                }
            };

            if let (Some(head), Some(tail)) = (other.head, other.tail) {
                // first delete from other list, then insert into own
                // no duplicates if we're interrupted (by panic from another thread)
                // this list may still be in an inconsistent state (wrong length, leaked data)
                // but there'll be no double free
                // TODO: Is the above too paranoid? Do we have more guarantees for panics?
                other.head = None;
                other.tail = None;
                let len = other.len;
                other.len = 0;

                debug_assert!(tail.as_ref().prev.is_none());
                NodePtr::link(cursor_node, tail);

                if let Some(mut next) = cursor_node.as_mut().next {
                    debug_assert!(head.as_ref().next.is_none());
                    NodePtr::link(head, next);
                }

                // FIXME: what do on overflow?
                self.list.len += len;
            }
        }
    }
}
