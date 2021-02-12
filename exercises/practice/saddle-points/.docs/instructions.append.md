# Instructions append

## Rust Indices Start At 0

By convention, ordered sequences of values in Rust have their contents numbered
("indexed") starting from 0. This applies regardless of what the rest of the
exercise description in this README says, such as references to indices that
start at 1, so you will have to subtract 1 to translate those index numbers
to Rust index numbers.

## Efficiency Notice

This exercise uses a _vector of vectors_ to store the content of matrices. While
this exercise is designed to help students understand basic concepts about
vectors, such as indexing, and that nested data types are legal, _vector of
vectors_ is a suboptimal choice for high-performance matrix algebra and any
similar efficient processing of larger amounts of data.

The detailed explanation of this inefficiency is beyond the scope of this
exercise and this learning track in general. This aspect is known as
[cache locality](https://stackoverflow.com/questions/12065774/why-does-cache-locality-matter-for-array-performance)
and you can find a good introduction to it by clicking that link if you'd like
to learn more about details of a modern computer architecture.
