# Simple Linked List

Write a simple linked list implementation that uses Elements and a List.

The linked list is a fundamental data structure in computer science,
often used in the implementation of other data structures. They're
pervasive in functional programming languages, such as Clojure, Erlang,
or Haskell, but far less common in imperative languages such as Ruby or
Python.

The simplest kind of linked list is a singly linked list. Each element in the
list contains data and a "next" field pointing to the next element in the list
of elements.

This variant of linked lists is often used to represent sequences or
push-down stacks (also called a LIFO stack; Last In, First Out).

As a first take, lets create a singly linked list to contain the range (1..10),
and provide functions to reverse a linked list and convert to and from arrays.

When implementing this in a language with built-in linked lists,
implement your own abstract data type.

## Implementation Hints

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

### Why `Option<Box<Node<T>>>` and not just `Option<Node<T>>`?
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


## Rust Installation

Refer to the [exercism help page][help-page] for Rust installation and learning
resources.

## Writing the Code

Execute the tests with:

```bash
$ cargo test
```

All but the first test have been ignored.  After you get the first test to
pass, remove the ignore flag (`#[ignore]`) from the next test and get the tests
to pass again.  The test file is located in the `tests` directory.   You can
also remove the ignore flag from all the tests to get them to run all at once
if you wish.

Make sure to read the [Modules](https://doc.rust-lang.org/book/second-edition/ch07-00-modules.html) chapter if you
haven't already, it will help you with organizing your files.

## Feedback, Issues, Pull Requests

The [exercism/rust](https://github.com/exercism/rust) repository on GitHub is the home for all of the Rust exercises. If you have feedback about an exercise, or want to help implement new exercises, head over there and create an issue. Members of the [rust track team](https://github.com/orgs/exercism/teams/rust) are happy to help!

If you want to know more about Exercism, take a look at the [contribution guide](https://github.com/exercism/docs/blob/master/contributing-to-language-tracks/README.md).

[help-page]: http://exercism.io/languages/rust
[modules]: https://doc.rust-lang.org/book/second-edition/ch07-00-modules.html
[cargo]: https://doc.rust-lang.org/book/second-edition/ch14-00-more-about-cargo.html

## Source

Inspired by 'Data Structures and Algorithms with Object-Oriented Design Patterns in Ruby', singly linked-lists. [http://www.brpreiss.com/books/opus8/html/page96.html#SECTION004300000000000000000](http://www.brpreiss.com/books/opus8/html/page96.html#SECTION004300000000000000000)

## Submitting Incomplete Solutions
It's possible to submit an incomplete solution so you can see how others have completed the exercise.
