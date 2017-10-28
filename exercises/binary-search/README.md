# Binary Search

Implement a binary search algorithm.

Searching a sorted collection is a common task. A dictionary is a sorted
list of word definitions. Given a word, one can find its definition. A
telephone book is a sorted list of people's names, addresses, and
telephone numbers. Knowing someone's name allows one to quickly find
their telephone number and address.

If the list to be searched contains more than a few items (a dozen, say)
a binary search will require far fewer comparisons than a linear search,
but it imposes the requirement that the list be sorted.

In computer science, a binary search or half-interval search algorithm
finds the position of a specified input value (the search "key") within
an array sorted by key value.

In each step, the algorithm compares the search key value with the key
value of the middle element of the array.

If the keys match, then a matching element has been found and its index,
or position, is returned.

Otherwise, if the search key is less than the middle element's key, then
the algorithm repeats its action on the sub-array to the left of the
middle element or, if the search key is greater, on the sub-array to the
right.

If the remaining array to be searched is empty, then the key cannot be
found in the array and a special "not found" indication is returned.

A binary search halves the number of items to check with each iteration,
so locating an item (or determining its absence) takes logarithmic time.
A binary search is a dichotomic divide and conquer search algorithm.

## Restrictions

Rust provides in its standard library already a
[binary search function](https://doc.rust-lang.org/std/primitive.slice.html#method.binary_search).
For this exercise you should not use this function but just other basic tools instead.

## Hints

[Slices](https://doc.rust-lang.org/book/second-edition/ch04-03-slices.html) have additionally to
the normal element access via indexing (slice[index]) many useful functions like
[split_at](https://doc.rust-lang.org/std/primitive.slice.html#method.split_at) or [getting
subslices](https://doc.rust-lang.org/std/primitive.slice.html#method.get) (slice[start..end]).

You can solve this exercise by just using boring old element access via indexing, but maybe the
other provided functions can make your code cleaner and safer.

## For bonus points

Did you get the tests passing and the code clean? If you want to, there
are some additional things you could try.

- Currently your find function will probably only work for slices of numbers,
  but the Rust type system is flexible enough to create a find function which
  works on all slices which contains elements which can be ordered.
- Additionally this find function can work not only on slices, but at the
  same time also on a Vec or an Array.

You can find tests (commented out) for these bonus tasks in the test file.

Then please share your thoughts in a comment on the submission. Did this
experiment make the code better? Worse? Did you learn anything from it?

### Hints for Bonus Points

- To get your function working with all kind of elements which can be ordered,
  have a look at the [Ord Trait](https://doc.rust-lang.org/std/cmp/trait.Ord.html).
- To get your function working directly on Vec and Array, you can use the
  [AsRef Trait](https://doc.rust-lang.org/std/convert/trait.AsRef.html)


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

Wikipedia [http://en.wikipedia.org/wiki/Binary_search_algorithm](http://en.wikipedia.org/wiki/Binary_search_algorithm)

## Submitting Incomplete Solutions
It's possible to submit an incomplete solution so you can see how others have completed the exercise.
