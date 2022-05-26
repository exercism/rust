# Instructions

Add the mine counts to a completed Minesweeper board.

Minesweeper is a popular game where the user has to find the mines using
numeric hints that indicate how many mines are directly adjacent
(horizontally, vertically, diagonally) to a square.

In this exercise you have to create some code that counts the number of
mines adjacent to a given empty square and replaces that square with the
count.

The board is a rectangle composed of blank space (' ') characters. A mine
is represented by an asterisk ('\*') character.

If a given space has no adjacent mines at all, leave that square blank.

## Examples

For example you may receive a 5 x 4 board like this (empty spaces are
represented here with the '·' character for display on screen):

```
·*·*·
··*··
··*··
·····
```

And your code will transform it into this:

```
1*3*1
13*31
·2*2·
·111·
```

## Performance Hint

All the inputs and outputs are in ASCII. Rust `String`s and `&str` are utf8,
so while one might expect "Hello".chars() to be simple, it actually has to
check each char to see if it's 1, 2, 3 or 4 `u8`s long. If we know a `&str`
is ASCII then we can call `.as_bytes()` and refer to the underlying data via a `&[u8]` slice.
Iterating over a u8 slice of ASCII is much quicker as there are no codepoints
involved - every ASCII char is one u8 long.

Can you complete the challenge without cloning the input?
