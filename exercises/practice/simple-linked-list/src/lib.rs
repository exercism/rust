use std::iter::successors;

struct Node<T> where T: Clone{
    value: T,
    next: Option<Box<Node<T>>>
}

impl <T: Clone> Clone for Node<T> {
    fn clone(&self) -> Node<T> {
        Self {
            value: self.value.clone(),
            next: self.next.as_ref().map(|next_node| Box::new((**next_node).clone()))
        }
    }
}

pub struct SimpleLinkedList<T: std::clone::Clone> {
    // Delete this field
    // dummy is needed to avoid unused parameter error during compilation
    head: Option<Box<Node<T>>>,
    len: usize
}

impl<T: std::clone::Clone> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self {
            len: 0, 
            head: None
        }
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for SimpleLinkedList)
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn push(&mut self, _element: T) {
        let node = Node::<T>{
            value: _element,
            next: self.head.take()
        };
        self.head = Some(Box::new(node));
        self.len+=1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if let Some(ret) = self.head.take() {
            self.head = ret.next;
            self.len -= 1;
            Some(ret.value)
        }
        else {
            None
        }
    }

    pub fn peek(&self) -> Option<&T> {
        if let Some(ret) = &self.head {
            Some(&ret.value)
        }
        else {
            None
        }
    }

    #[must_use]
    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut ret = Self::new();

        successors(self.head, |node| node.next.clone())
        .for_each(|node| {
            ret.push(node.value);
        });

        ret     
    }
}

impl<T: std::clone::Clone> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        let mut ret: Self = Self::new();

        _iter.into_iter().for_each(|el| {
            ret.push(el);
        });

        ret
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.
//
// Please note that the "front" of the linked list should correspond to the "back"
// of the vector as far as the tests are concerned.

impl<T: std::clone::Clone> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(mut _linked_list: SimpleLinkedList<T>) -> Vec<T> {
        let mut ret:Vec<T> = Vec::new();

        while let Some(val) = _linked_list.pop() {
            ret.push(val);
        }

        ret.reverse();
        ret
    }
}
