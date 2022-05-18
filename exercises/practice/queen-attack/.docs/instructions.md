# Instructions

Given the position of two queens on a chess board, indicate whether or not they
are positioned so that they can attack each other.

In the game of chess, a queen can attack pieces which are on the same
row, column, or diagonal.

A chessboard can be represented by an 8 by 8 array. The rows of a chessboard are known as ranks and columns are known as files.

So if you're told the white queen is at (2, 3) and the black queen at
(5, 6), then you'd know you've got a set-up like so:

```text
_ _ _ _ _ _ _ _
_ _ _ _ _ _ _ _
_ _ _ W _ _ _ _
_ _ _ _ _ _ _ _
_ _ _ _ _ _ _ _
_ _ _ _ _ _ B _
_ _ _ _ _ _ _ _
_ _ _ _ _ _ _ _
```

You'd also be able to answer whether the queens can attack each other.
In this case, that answer would be yes, they can, because both pieces
share a diagonal.

### Examples of queens attacking:

```text
_ _ _ _ _ _ _ _     _ _ _ _ _ _ _ _     _ _ _ _ _ _ _ _
_ _ _ _ _ _ _ _     _ _ _ _ _ _ _ _     _ _ _ W _ _ _ _
_ _ _ W _ _ _ _     _ B _ _ _ W _ _     _ _ _ _ _ _ _ _
_ _ _ _ _ _ _ _     _ _ _ _ _ _ _ _     _ _ _ _ _ _ _ _
_ _ _ _ _ _ _ _     _ _ _ _ _ _ _ _     _ _ _ _ _ _ _ _
_ _ _ _ _ _ B _     _ _ _ _ _ _ _ _     _ _ _ _ _ _ _ _
_ _ _ _ _ _ _ _     _ _ _ _ _ _ _ _     _ _ _ B _ _ _ _
_ _ _ _ _ _ _ _     _ _ _ _ _ _ _ _     _ _ _ _ _ _ _ _
```

### Examples of queens not interacting:

```text

_ _ _ _ _ _ _ _     _ _ _ _ _ _ _ _     _ _ _ _ _ _ _ _
_ _ _ _ _ _ _ _     _ _ _ _ _ _ _ _     _ _ _ W _ _ _ _
_ _ _ _ _ _ _ _     _ _ _ _ _ W _ _     _ _ _ _ _ _ _ _
_ _ _ _ _ _ _ _     _ _ _ _ _ _ _ _     _ _ _ _ _ _ _ _
_ _ _ _ W _ _ _     _ _ _ _ _ _ _ _     _ _ _ _ _ _ _ _
_ _ _ _ _ _ B _     _ _ _ _ _ _ _ _     _ _ _ _ _ _ _ _
_ _ _ _ _ _ _ _     _ _ B _ _ _ _ _     _ _ _ _ _ _ B _
_ _ _ _ _ _ _ _     _ _ _ _ _ _ _ _     _ _ _ _ _ _ _ _
```

B and W stand for **Black** and **White**, the two sides competing
against each other in a game of chess. For this exercise you do not need to know which side
the queens are on.
