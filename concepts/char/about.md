# About

Rust implements the [char][char type] type to represent a single character. A `char` literal is placed within single quotes, like `'a'`.
Each `char` is four bytes in size and represents a single [Unicode Scalar Value][unicode scalar].

However, a [`char`][character type] is not always what we think of as a letter. There are some languages, e.g. Hindi,  that use [diacritics][diacritics],
which are special symbols which modify the character they are attached to. Although the diacritic in Rust is a separate `char`, it is the diacritic and
the character it modifies that we commonly think of as a letter.

The term for a character and its diacritic is [grapheme cluster][grapheme cluster]. There are external crates that can be used to process grapheme clusters,
such as [unicode-segmentation][unicode-segmentation].

Example

```rust
pub fn main() {
    let text = "ü"; // a "u" with a diacritic
    let text_vec: Vec<char> = text.chars().collect(); // this gets the chars in "ü"
    println!("{:?}", text_vec.len()); // this prints the number of chars in "ü"
    println!("{:?}", text_vec[0]); // this prints the first char in "ü"
    println!("{:?}", text_vec[1]); // this prints the second char in "ü"
}
```

prints

```rust
2
'u'
'\u{308}'
```

'\u{308}' is another way of writing a `char` literal. `\u` indicates it is a unicode `char` with `{308}` being the unique Unicode number for that character
or diacritic.

[char type]: https://doc.rust-lang.org/std/primitive.char.html
[unicode]: http://www.unicode.org/glossary/#unicode
[character type]: https://doc.rust-lang.org/book/ch03-02-data-types.html#the-character-type
[unicode scalar]: http://www.unicode.org/glossary/#unicode_scalar_value
[diacritics]: http://www.unicode.org/glossary/#diacritic
[grapheme cluster]: https://doc.rust-lang.org/book/ch08-02-strings.html#bytes-and-scalar-values-and-grapheme-clusters-oh-my
[unicode-segmentation]: https://crates.io/crates/unicode-segmentation
