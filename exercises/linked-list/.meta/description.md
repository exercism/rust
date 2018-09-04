# Doubly Linked List using `unsafe {}`

Write a doubly linked list and a cursor to traverse it, using unsafe Rust.

The doubly linked list is a fundamental data structure in computer science,
often used in the implementation of other data structures. They're
pervasive in functional programming languages, such as Clojure, Erlang,
or Haskell, but far less common in imperative languages such as Ruby or
Python.

Each node in a doubly linked list contains data and pointers to the next
and previous node, if they exist.

New nodes can be efficiently added at any point in the list, if one already has
a reference to the position. Likewise, all elements
from another list can be inserted at any point in constant time.

In Rust, linked lists are very rarely used, but occasionally they trip up
newcomers, when they try implementing one. Often, they find it unexpectedly
difficult to work with the yet unfamiliar borrow checker.
