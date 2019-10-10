use std::iter::FromIterator;

pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
    len: usize,
}

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn unpack(self) -> (T, Option<Box<Node<T>>>) {
        (self.data, self.next)
    }
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList { head: None, len: 0 }
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

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(mut self) -> Vec<T> {
        let mut vec = Vec::new();
        while let Some(data) = self.pop() {
            vec.push(data);
        }
        vec.reverse();
        vec
    }
}

pub struct IntoIter<T> (Option<Box<Node<T>>>);

impl<T> IntoIterator for SimpleLinkedList<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self.head)
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let mut out = None;
        if self.0.is_some() {
            let node_pointer = self.0.take().unwrap();
            let (item, next) = node_pointer.unpack();
            out = Some(item);
            if next.is_some() {
                self.0.replace(next.unwrap());
            }
        }
        out
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
