# Introduction

Rust provides a user defininable data type called Enum. Enums give you a way of
saying a value is one of a possible set of values. The possible values of an
enum are referred to as _variants_.

## Enums

### Declaration 
In Rust the keyword `enum` is used to define enums. The syntax is as follows:
```rust
enum MyEnum {
    FirstValue,
    SecondValue,
}
```

### Use as type
Like any other data type, enums can be used as types of a variable, function
parameter and return value from a function.

```rust
let my_enum_value: MyEnum;

fn my_function(input: MyEnum) -> MyEnum {}
```

### Access enum variants
- One can use the double-colon syntax `::` to access an enum value like so:
    ```rust
    let my_enum_value: MyEnum = MyEnum::FirstValue;
    let my_second_enum_value = MyEnum::SecondValue;
    ```

- Sometimes one may desire to omit typing `MyEmum::` everywhere. In this case a
 `use` declaration can be used such that manual scoping is not needed.
    ```rust
    use MyEnum::*;

    let my_enum_value = FirstValue;
    // We neither set the type of the variable not needed to use MyEnum:: prefix
    ```

### Assign a value to variants
Enum variants can have values or sometimes referred to as _descriminators_.
- To assign a value to an enum variant, normal assignment can be used:
    ```rust
    enum MyEnum {
        A = 5,
        B = 6,
    }

    let my_enum_value = MyEnum::A;
    ```

- To access the assigned value, casting can be used:
    ```rust
    enum MyEnum {
        A = 5,
        B = 6,
    }

    let my_enum_value = MyEnum::A;

    let my_int = my_enum_value as i32;
    let my_other_int = MyEnum::B as i32;
    ```

- By default, values are assigned based on their index starting from 0:
    ```rust
    enum MyEnum {
        A, // => 0
        B, // => 1
    }
    ```
