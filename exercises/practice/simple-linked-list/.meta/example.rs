use std::iter::FromIterator;

pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
    len: usize,
}

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList { head: None, len: 0 }
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn push(&mut self, element: T) {
        let node = Box::new(Node::new(element, self.head.take()));
        self.head = Some(node);
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.len {
            0 => None,
            _ => {
                self.len -= 1;
                self.head.take().map(|node| {
                    let node = *node;
                    self.head = node.next;
                    node.data
                })
            }
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.data)
    }

    #[must_use]
    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut rev_list = SimpleLinkedList::new();
        let mut vec: Vec<_> = self.into();
        for t in vec.drain(..).rev() {
            rev_list.push(t);
        }
        rev_list
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut sll = SimpleLinkedList::new();
        for t in iter {
            sll.push(t);
        }
        sll
    }
}

impl<T> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(mut linked_list: SimpleLinkedList<T>) -> Vec<T> {
        let mut vec: Vec<T> = Vec::with_capacity(linked_list.len());
        while let Some(data) = linked_list.pop() {
            vec.push(data);
        }
        vec.reverse();
        vec
    }
}

impl<T> Node<T> {
    pub fn new(element: T, next: Option<Box<Node<T>>>) -> Self {
        Node {
            data: element,
            next,
        }
    }
}
