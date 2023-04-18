# Hints

## General

- [Vectors by Example][vectors-by-example]: basic tutorial on how to work with vectors.
- [Storing lists of values with Vectors][rust-book-vectors]: reference chapter in the Rust Book.

## 1. List finished

- Use `vec![]` to create an empty vector.
- Use type annotation to create a vector with specific type for it's elements:
  ```rust
  let result: Vec<&str> = vec![];
  //          ^^^^^^^^^
  ```

## 2. Define an existing wish list

- Use inline element declaration when you already know the elements:
  ```rust
  let v = vec!["A", "B", "C"];
  ```
- Use [`.push`][push()] method for dynamically adding elements:
  ```rust
  let mut v: Vec<&str> = vec![];
  v.push("A");
  v.push("B");
  ```

## 3. Add a new language to the wish list

- Use [`.push`][push()] method on a vector to add new elements.

## 4. Count the languages in the wish list

- Use [`.len`][len()] method on a vector to count its elements.

## 5. Check to see if a language is in the wish list

- Use [`.contains`][contains()] method on a vector to check if an element is present.

## 6. Reverse the list

- Use [`while`][while-loop] construct to loop over a vactor in reverse order:
  ```rust
  let v = vec![1, 2, 3];
  let len = v.len();
  let mut i = len - 1;

  while i >= 0 {
    let element = v[i];
    i -= 1;
  }
  ```

## 7. Check if list is exciting

- Use indexing to access an element at a specific index:
  ```rust
  let v = vec![1, 2, 3];
  let first = v[0];
  let second = v[1];
  ```

## 8.Remove Language from wish list

- Use [`.remove`][remove()] to remove an element by its index.

## 9. Check if all languages in the wish list are unique

- Use [`.sort`][sort()] to re-arrange all repetitive elements to be consecutive.
- Use [`.dedup`][dedup()] to remove all repetitive elements.
- Use [`.len`][len()] to check if the length of the vector changed. If it did,
  it had duplicate elements.

[vectors-by-example]: https://doc.rust-lang.org/book/ch08-01-vectors.html
[rust-book-vectors]: https://doc.rust-lang.org/book/ch08-01-vectors.html
[contains()]:
    https://doc.rust-lang.org/stable/std/vec/struct.Vec.html#method.contains
[push()]: https://doc.rust-lang.org/stable/std/vec/struct.Vec.html#method.push
[remove()]: https://doc.rust-lang.org/stable/std/vec/struct.Vec.html#method.remove
[len()]: https://doc.rust-lang.org/stable/std/vec/struct.Vec.html#method.len
[sort()]: https://doc.rust-lang.org/stable/std/vec/struct.Vec.html#method.sort
[dedup()]: https://doc.rust-lang.org/stable/std/vec/struct.Vec.html#method.dedup
[while-loop]: https://doc.rust-lang.org/rust-by-example/flow_control/while.html
