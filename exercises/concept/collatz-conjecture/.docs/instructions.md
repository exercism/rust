# Instructions

The Collatz Conjecture or 3x+1 problem can be summarized as follows:

Take any positive integer n. If n is even, divide n by 2 to get n / 2. If n is
odd, multiply n by 3 and add 1 to get 3n + 1. Repeat the process indefinitely.
The conjecture states that no matter which number you start with, you will
always reach 1 eventually.

While this conjecture has not been proven true for all numbers, it has been
demonstrated to be true for all numbers under `2^68`.

## Examples

Starting with n = 12, the steps would be as follows:

0. 12
1. 6
2. 3
3. 10
4. 5
5. 16
6. 8
7. 4
8. 2
9. 1

Resulting in 9 steps. So for input n = 12, the return value would be 9.

## 1. Compute the next value in a Collatz sequence

Write a function `collatz_next(n)` which, given an integer `n`, produces the next value in the Collatz sequence. For `n == 12`, this produces `6`.

## 2. Count the number of steps for a Collatz sequence to reach 1

Write a function `collatz_count(n)` which, given an integer `n`, produces the number of steps for the sequence to reach `1`. For `n == 12`, this produces `9`.
