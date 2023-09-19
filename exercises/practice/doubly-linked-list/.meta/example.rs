// this module adds some functionality based on the required implementations here
// like: `LinkedList::pop_back`, `Clone` and `IntoIterator` for `LinkedList<T>`
// You are free to use anything in it, but it's primarily for the test framework.
mod pre_implemented;

use std::ptr::NonNull;

type NodePtr<T> = NonNull<Node<T>>;
type OptNodePtr<T> = Option<NodePtr<T>>;

// Invariants we need to uphold
// L1. Every stored NodePtr is valid.
// L2. The nodes form a chain
//     L2.1 If Node B is `next` of A, then A is `prev` of B (see Node struct members)
//     L2.2 No cycles
// L3. `front` points to first element, `back` to last.
// (L4.) `len` should match number of elements, but unsafe code can't depend on it.
pub struct LinkedList<T> {
    back: OptNodePtr<T>,
    front: OptNodePtr<T>,
    len: usize,
    // The PhantomData signals dropck that we actually own `T`
    // I'm only aware of one case where this actually matters, which is when
    // using the dropck_eyepatch feature in Drop. We aren't using that here, so this is likely
    // unnecessary.
    // It can't hurt however and I'm not entirely certain that dropck_eyepatch is the only
    // case where it matters
    marker: std::marker::PhantomData<Box<T>>,
}

// We don't have any shared mutable state nor any thread dependent data.
// Trait bounds for Send/Sync are the same as for Box<T>
unsafe impl<T: Send> Send for LinkedList<T> {}
unsafe impl<T: Sync> Sync for LinkedList<T> {}

// Invariants we need to uphold
// C1. Never hand out more than one mutable reference at a time.
//     If we did, we couldn't guarantee non-aliasing mutable references
//     without additional book keeping as a cursor can move forwards and backwards
pub struct Cursor<'a, T> {
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
        unsafe {
            // new_unchecked() is trivially safe as we have just allocated a new Box
            NonNull::new_unchecked(Box::into_raw(Box::new(Self {
                element,
                prev: None,
                next: None,
            })))
        }
    }

    // `left` and `right` must point to adjacent nodes
    unsafe fn link(mut left: NodePtr<T>, mut right: NodePtr<T>) {
        left.as_mut().next = Some(right);
        right.as_mut().prev = Some(left);
    }
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            back: None,
            front: None,
            len: 0,
            marker: std::marker::PhantomData,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn cursor_front(&mut self) -> Cursor<'_, T> {
        Cursor {
            node: self.front,
            list: self,
        }
    }

    pub fn cursor_back(&mut self) -> Cursor<'_, T> {
        Cursor {
            node: self.back,
            list: self,
        }
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next_node: self.front,
            marker: std::marker::PhantomData,
        }
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut cursor = self.cursor_front();
        while cursor.take().is_some() {}
    }
}

pub struct Iter<'a, T> {
    next_node: OptNodePtr<T>,
    marker: std::marker::PhantomData<&'a LinkedList<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let node_ptr = self.next_node?;
        unsafe {
            // Dereferencing node.as_ptr() is safe due to invariant L1: all NodePtrs are valid.
            // Extending the lifetime to 'a is safe because the list is borrowed for 'a via Iter<'a, T>
            let current_node = &*node_ptr.as_ptr();
            self.next_node = current_node.next;
            Some(&current_node.element)
        }
    }
}

impl<T> Cursor<'_, T> {
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        unsafe {
            // This is safe, because all NodePtrs are valid (L1) and the list and cursor
            // are mutably borrowed for as long as the reference is alive.
            self.node.map(|node| &mut (*node.as_ptr()).element)
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Option<&mut T> {
        // safe as node.next is a valid potential pointer
        unsafe { self._step(|node| node.next) }
    }

    pub fn prev(&mut self) -> Option<&mut T> {
        // safe as node.prev is a valid potential pointer
        unsafe { self._step(|node| node.prev) }
    }

    // Moves location to the node returned by `get_next`, if it exists and returns
    // a mutable reference to its element.
    // `get_next` must return None or a pointer owned by the linked list
    unsafe fn _step(&mut self, get_next: impl Fn(&Node<T>) -> OptNodePtr<T>) -> Option<&mut T> {
        // safe due to L1: All NodePtrs are valid
        let new_pos = get_next(self.node?.as_ref())?;
        self.node = Some(new_pos);
        // returning a mutable reference is safe for the same reason peek_mut() is safe
        Some(&mut (*new_pos.as_ptr()).element)
    }

    pub fn take(&mut self) -> Option<T> {
        unsafe {
            // First remove all links to current node and close the hole, then deallocate and
            // return the element within.
            // All ptr accesses are safe because of L1: all NodePtrs are valid.
            let mut node = self.node?;
            let &mut Node { prev, next, .. } = node.as_mut();

            self.node = next.or(prev);
            // if the first and/or last element is taken, update the respective pointer(s) (L3)
            match next {
                Some(mut next) => next.as_mut().prev = prev,
                None => self.list.back = prev,
            };
            match prev {
                Some(mut prev) => prev.as_mut().next = next,
                None => self.list.front = next,
            }

            self.list.len -= 1;
            Some(Box::from_raw(node.as_ptr()).element)
        }
    }

    pub fn insert_after(&mut self, element: T) {
        unsafe {
            self._insert(
                element,
                // cursor_node <-> Option<next>
                // => cursor_node <-> new_element <-> Option<next>
                // This maintains the L2 invariant.
                // If `cursor_node` == `back`, `list.back` needs to be updated (L3).
                |list| &mut list.back,
                |mut cursor_node, new_node| {
                    if let Some(next) = cursor_node.as_mut().next {
                        Node::link(new_node, next);
                    }
                    Node::link(cursor_node, new_node);
                },
            );
        }
    }

    pub fn insert_before(&mut self, element: T) {
        unsafe {
            self._insert(
                element,
                // safe for the same reason as insert_after()
                // but in the other direction
                |list| &mut list.front,
                |mut cursor_node, new_node| {
                    if let Some(prev) = cursor_node.as_mut().prev {
                        Node::link(prev, new_node);
                    }
                    Node::link(new_node, cursor_node);
                },
            );
        }
    }

    // If list is empty, insert_after and insert_before are the same. Insert element directly.
    // Otherwise, let `link_new_node` callback do it and if the cursor is at the front or back node
    // given by `end_node`, update it to the new node.
    // Caller is responsible for choosing callbacks such that LinkedList invariants will hold.
    unsafe fn _insert(
        &mut self,
        element: T,
        // Should return reference to `front` or `back` member of LinkedList
        // that could have been replaced by `insert_element`.
        end_node: impl Fn(&mut LinkedList<T>) -> &mut OptNodePtr<T>,
        link_new_node: impl Fn(NodePtr<T>, NodePtr<T>),
    ) {
        // allocate new Node first.
        // if allocation panics, list is still in valid state.
        let new_node = Node::new_linkless(element);

        let cursor_node = match self.node {
            Some(node) => node,
            None => {
                // list empty
                self.node = Some(new_node);
                self.list.back = self.node;
                self.list.front = self.node;
                self.list.len += 1;
                return;
            }
        };
        link_new_node(cursor_node, new_node);
        let end_node = end_node(self.list);
        if *end_node == Some(cursor_node) {
            *end_node = Some(new_node);
        }
        self.list.len += 1;
    }
}
