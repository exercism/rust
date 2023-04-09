# Hints

## General

- Checkout the [defining structures][def-struct] chapter in the Rust book.

## 1. Buy a brand-new remote controlled car

- Use [associated functions][assoc-fn] to define the `buy()` function and return
  and instance of `RemoteControlCar`.
- The return type of `buy()` can be `Self`.
- Use struct fields to keep track of battery percentage and distance drive.

## 2. Display the distance driven

- Keep track of distance driven in a struct field.
- Use an `impl` block to define the method.
- Use `&self` as the first parameter of the method.
- Use dot notation to access struct fields.

## 3. Display the battery percentage

- Keep track of batter percentage in a struct field.
- Initialize the field to a specific value to correspond to the initial batter
  charge.

## 4. Update the number of meters driven when driving

- Using `&mut self` as the first method parameter allows changing field values.
- Update the field representing the the distance driven.

## 5. Update the battery percentage when driving

- Update the field representing the battery percentage driven.

## 6. Prevent driving when the battery is drained

- Add a conditional to only update the distance and battery if the battery is not already drained.
- Add a conditional to display the empty battery message if the battery is drained.

[def-struct]:
    https://doc.rust-lang.org/book/ch05-01-defining-structs.html?highlight=struct#defining-and-instantiating-structs
[assoc-fn]:
    https://doc.rust-lang.org/book/ch05-03-method-syntax.html#associated-functions
