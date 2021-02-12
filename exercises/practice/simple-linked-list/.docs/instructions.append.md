# Implementation Hints

Do not implement the struct `SimpleLinkedList` as a wrapper around a `Vec`. Instead, allocate nodes on the heap.  
This might be implemented as:
```
pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}
```
The `head` field points to the first element (Node) of this linked list.  
This implementation also requires a struct `Node` with the following fields:
```
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}
```
`data` contains the stored data, and `next` points to the following node (if available) or None.  

## Why `Option<Box<Node<T>>>` and not just `Option<Node<T>>`?
Try it on your own. You will get the following error.

```
| struct Node<T>
| ^^^^^^^^^^^^^^ recursive type has infinite size
...
|     next: Option<Node<T>>,
|     --------------------- recursive without indirection
 ```

 The problem is that at compile time the size of next must be known.
 Since `next` is recursive ("a node has a node has a node..."), the compiler does not know how much memory is to be allocated.
 In contrast, [Box](https://doc.rust-lang.org/std/boxed/) is a heap pointer with a defined size.
