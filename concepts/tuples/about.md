# Tuples

Tuples are a lightweight way to group a bounded set of heterogenous data together. A tuple doesn't have
a particular name; naming a data structure turns it into a `struct`. A tuple's fields don't have names;
they are accessed by means of destructuring or by position.

## Syntax

Tuples are a list of fields surrounded by parentheses:

- `()` is the `unit type`, which is a [special case](https://doc.rust-lang.org/std/primitive.unit.html)
- **(** _expression_ **,)** is a single-element tuple expression
- **(** _Type_ **,)** is a single-element tuple type
- **(** _expression_ **,** ... **)** is a general tuple expression
- **(** _Type_ **,** ... **)** is a general tuple type

### Creation

Tuples are always created with a tuple expression:

```rust
// pointless but legal
let unit = ();
// single element
let single_element = ("note the comma",);
// two element
let two_element = (123, "elements can be of differing types");
```

Tuples can have an arbitrary number of elements.

### Access by destructuring

It is possible to access the elements of a tuple by destruturing. This just means assigning variable
names to the individual elements of the tuple, consuming it.

```rust
let (elem1, _elem2) = two_element;
assert_eq!(elem1, 123);
```

### Access by position

It is also possible to access the elements of a tuple by numeric positional index. Indexing, as always,
begins at 0.

```rust
let notation = single_element.0;
assert_eq!(notation, "note the comma");
```

## Tuple Structs

You will also be asked to work with tuple structs. Like normal structs, these are named types; unlike
normal structs, they have anonymous fields. Their syntax is very similar to normal tuple syntax. It is
legal to use both destructuring and positional access.

```rust
struct TupleStruct(u8, i32);
let my_tuple_struct = TupleStruct(123, -321);
let borrowed_neg = &my_tuple_struct.1;
let TupleStruct(borrowed_byte, _) = &my_tuple_struct;
assert_eq!(borrowed_neg, &-321);
assert_eq!(borrowed_byte, &123);
```

### Field Visibility

All fields of anonymous tuples are always public. However, fields of tuple structs have individual
visibility which defaults to private, just like fields of standard structs. You can make the fields
public with the `pub` modifier, just as in a standard struct.

```rust
// fails due to private fields
mod tuple { pub struct TupleStruct(u8, i32); }
fn main() { let _my_tuple_struct = tuple::TupleStruct(123, -321); }
```

```rust
// succeeds: fields are public
mod tuple { pub struct TupleStruct(pub u8, pub i32); }
fn main() { let _my_tuple_struct = tuple::TupleStruct(123, -321); }
```
