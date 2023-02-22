# Do not keep track of length

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

This approach starts be defining a `Link` type which will be used for the `head` field of `SimpleLinkedList` and the `next` field of `Node`.
Defining `Link<T>` as an `Option<Box<Node<T>>>` in one place helps to keep the code [DRY][dry].

The `is_empty()` method is implemented by returning the result of the [is_none()][is-none] method on the `head`.

The `len()` method is implemented by iterating all of the nodes while mutating a counter variable.
When the iteration is done, the value of the counter variable is returned.

[dry]: https://en.wikipedia.org/wiki/Don%27t_repeat_yourself
[is-none]: https://doc.rust-lang.org/std/option/enum.Option.html#method.is_none
