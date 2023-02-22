# Keep track of length

```rust
use std::iter::FromIterator;

type Link<T> = Option<Box<Node<T>>>;

pub struct SimpleLinkedList<T> {
    head: Link<T>,
    len: usize,
}
struct Node<T> {
    data: T,
    next: Link<T>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self { head: None, len: 0 }
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn push(&mut self, _element: T) {
        let new_node = Box::new(Node {
            data: _element,
            next: self.head.take(),
        });

        self.head = Some(new_node);
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }
        self.len -= 1;
        self.head.take().map(|node| {
            self.head = node.next;
            node.data
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.data)
    }

    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut list = Self::new();
        if self.len == 0 {
            return list;
        }
        let mut cur_node = self.head;
        while let Some(node) = cur_node {
            list.push(node.data);
            cur_node = node.next;
        }
        list
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        let mut list = Self::new();
        for val in _iter {
            list.push(val);
        }
        list
    }
}

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
        let mut the_vec: Vec<T> = vec![];
        if self.len == 0 {
            return the_vec;
        }
        let mut cur_node = self.rev().head;
        while let Some(node) = cur_node {
            the_vec.push(node.data);
            cur_node = node.next;
        }
        the_vec
    }
}
```

This approach starts by defining a `Link` type which will be used for the `head` field of `SimpleLinkedList` and the `next` field of `Node`.
Defining `Link<T>` as an `Option<Box<Node<T>>>` in one place helps to keep the code [DRY][dry].

A `len` field is defined for `SimpleLinkedList`.
The `len` field is updated as items are pushed and popped.

The `is_empty()` method is implemented by returning if `len` is `0`.

The `len()` method is implemented by returning the value of the `len` field.

[dry]: https://en.wikipedia.org/wiki/Don%27t_repeat_yourself
