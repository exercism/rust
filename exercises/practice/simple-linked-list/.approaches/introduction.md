# Introduction

There is more than one way to solve Simple Linked List.
One general approach is to keep track of the length as nodes are pushed and popped.
Another approach is to calculate the length every time it is asked for.

## General guidance

One thing to keep in mind is to not mutate the list when it is not necessary.
For instance, if you find yourself using `mut self` for  `rev()` or `into()`, that is an indication that the list is being mutated when it is not necessary.

## Approach: Keep track of length

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

For more information, check the [keep track of length appproach][approach-keep-track-of-length].

## Approach: Do not keep track of length

```rust
use std::iter::FromIterator;

type Link<T> = Option<Box<Node<T>>>;

pub struct SimpleLinkedList<T> {
    head: Link<T>,
}
struct Node<T> {
    data: T,
    next: Link<T>,
}

impl<T> Node<T> {
    fn new(data: T, next: Option<Box<Node<T>>>) -> Self {
        Self { data, next }
    }
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self { head: None }
    }
    
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }
    
    pub fn len(&self) -> usize {
        let mut current_node = &self.head;
        let mut size = 0;
        while let Some(x) = current_node {
            size += 1;
            current_node = &x.next;
        }
        size
    }
    
    pub fn push(&mut self, element: T) {
        let node = Box::new(Node::new(element, self.head.take()));
        self.head = Some(node);
    }
    
    pub fn pop(&mut self) -> Option<T> {
        if self.head.is_some() {
            let head_node = self.head.take().unwrap();
            self.head = head_node.next;
            Some(head_node.data)
        } else {
            None
        }
    }
    
    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|head| &(head.data))
    }
    
    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut list = SimpleLinkedList::new();
        let mut cur_node = self.head;
        while let Some(node) = cur_node {
            list.push(node.data);
            cur_node = node.next;
        }
        list
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = SimpleLinkedList::new();
        for item in iter {
            list.push(item);
        }
        list
    }
}

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
        let mut the_vec: Vec<T> = vec![];
        let mut cur_node = self.rev().head;
        while let Some(node) = cur_node {
            the_vec.push(node.data);
            cur_node = node.next;
        }
        the_vec
    }
}
```

For more information, check the [do not keep track of length appproach][approach-do-not-keep-track-of-length].

## Which approach to use?

Which to use is pretty much a matter of personal preference.

[approach-keep-track-of-length]: https://exercism.org/tracks/rust/exercises/simple-linked-list/approaches/keep-track-of-length
[approach-do-not-keep-track-of-length]: https://exercism.org/tracks/rust/exercises/simple-linked-list/approaches/do-not-keep-track-of-length
