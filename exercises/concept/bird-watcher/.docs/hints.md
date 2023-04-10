# Hints

## General

- The bird count per day is an array that contains exactly 7 integers.

## 1. Check what the counts were last week

- Define an array that contains last week's log.
- Return the array as a result.

## 2. Check how many birds visited today

- Remember that the counts are ordered by day from oldest to most recent, with the last element representing today.
- Accessing the last element can be done by using its (fixed) index (remember to start counting from zero).

## 3. Increment today's count

- Set the element representing today's count to today's count plus 1.

## 4. Check if there was a day with no visiting birds

- Use if statements for performing comparisions with array elements.
- Use [`for in`][for-in] construct to loop over an array.

## 5. Calculate the number of visiting birds for the first x number of days

- A variable can be used to hold the count for the number of visiting birds.
- The array can be looped over using [`for in`][for-in] construct.
- The variable can be updated inside the loop.
- Use `break` statement to stop a loop.

## 6. Calculate the number of busy days

- A variable can be used to hold the number of busy days.
- A variable can be used to hold the current index while looping over an array.
- The variable can be updated inside the loop.
- A [conditional statement][if-statement] can be used inside the loop.

[if-statement]:
    https://doc.rust-lang.org/rust-by-example/flow_control/if_else.html
[for-in]:
    https://doc.rust-lang.org/rust-by-example/flow_control/for.html#for-and-range
