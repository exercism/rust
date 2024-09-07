# Hints

## General

- [Tuples][tuples]: shows how to define and use tuples.

## 1. Analyze a phone number

- Make sure the tuple has values at the right place. 
- Tuples are passed as a [return value][tuples-return].

- Use `.split_at()` method on a string to split it and get a tuple of its parts.
    ```rust
    let str = "Per Martin-Löf";

    let (first, last) = str.split_at(3);

    first // => "Per"
    last // => " Martin-Löf"
    ```

## 2. Detect if a phone number is fake prefix code (555)

- You can extract the value of a field with the same sort of dot syntax as you employ with `struct`s or `class`s.

[tuples]: https://doc.rust-lang.org/book/ch03-02-data-types.html#the-tuple-type
[tuples-return]: https://docs.microsoft.com/en-us/dotnet/csharp/tuples#tuples-as-method-return-values
