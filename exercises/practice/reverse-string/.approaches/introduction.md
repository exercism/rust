# Reverse String in Rust
Reverse String can be solved in different ways, too many to describe them all in detail. Most approaches can be classified in two ways:

They either do things "manually" or they use the standard library.
They either build a new reversed string, or they modify the parameter.
If you are new to programming, you might want to start with "manual" approaches, writing loops and appending/assigning characters explicitly. That gives you an idea of how the computer manipulates entities and how the algorithms work.

If you want to learn best practices and "modern Rust", you might want to try using the standard library. That leads to concise, expressive, readable, idiomatic, and efficient code.

There are also some rather unusual approaches that are not necessarily the best (simple, easy to read, idiomatic, and reasonably efficient) but solve the task in an interesting and "fun" way (for some definition of "fun").

## Approach: Iterative

    pub fn reverse(input: &str) -> String {
        let mut result = String::new();
        for c in input.chars().rev() {
            result.push(c);
        }
        result
    }
This approach iterates over the characters of the original string in reverse order and appends each character to the result string. It's a straightforward solution that does everything manually.

## Approach: Using Iterators and Collections

    pub fn reverse(input: &str) -> String {
        input.chars().rev().collect()
    }
This approach leverages Rust's iterator and collection capabilities to efficiently reverse a string. It is concise, expressive, idiomatic to Rust, and offers efficient performance for reversing strings. It is also the most submitted solution on Exercism for this Rust exercise.

## Approach: Recursive

    pub fn recursive_helper(input: &str, output: &mut String) {
        if let Some((idx, c)) = input.char_indices().next_back() {
            output.push(c);
            recursive_helper(&input[..idx], output);
        }
    }
    
    pub fn reverse(input: &str) -> String {
        let mut result = String::new();
        recursive_helper(input, &mut result);
        result
    }
This approach builds the reversed string by using linear recursion. It creates an empty string and passes that as an "output parameter" to the recursive helper function. That helper function removes the last character from the input, appends it to the output, and calls itself recursively.

## Approach: Over-Engineered

    pub fn reverse(input: &str) -> String {
        let mut stack: Vec<char> = Vec::new();
        for c in input.chars() {
            stack.push(c);
        }
        let mut reversed = String::new();
        while let Some(c) = stack.pop() {
            reversed.push(c);
        }
        reversed
    }
This approach essentially simulates the process of reversing a string using a stack: characters are pushed onto the stack in the order they appear in the input string, and then popped off the stack one by one to construct the reversed string.

That is clearly over-engineered, in most contexts it is just not necessary. But it can be interesting to analyze because it uses some interesting features.

## Bonus Task - Graphemes 

    use unicode_segmentation::UnicodeSegmentation;
    
    pub fn reverse(input: &str) -> String {
        input.graphemes(true).rev().collect()
    }
    
This approach leverages the UnicodeSegmentation crate to reverse a string by its grapheme clusters, ensuring proper handling of Unicode characters. It is similar to the `Using Iterators and Collections` approach but utilizes an external crate for proper Unicode handling.
Make sure to add `unicode_segmentation` to your `Dependencies` in `Cargo.toml`.

