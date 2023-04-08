# Introduction

## String Formatting

Rust allows composing strings using _format strings_. Format strings are strings
that have "placeholders" in them with optional formatting parameters.

The format strings are written as `"Hello {}!"` where `{}` is a "placeholder".
Using format strings and _interpolating_ them requires a macro called `format!()`.


### The `format!()` macro

`format!()` macro takes a _format string_ comprising staic text and "placeholders" (known in the
documentation as _Positional/Named parameters), and a variable number of arguments. The
return value resolves each "placeholder" using the corresponding argument and
combines the resolved values with the static text.

```rust
format!("Hello, my name is {0} and I am {1} years old.", "John", 20);
// => "Hello, my name is John and I am 20 years old.
```

### Positional/Named Parameters

The text in braces is known as a _Positional/Named parameter_.

A Positional/Named parameter has the followign syntax:
```txt
'{' [ argument ] [ ':' format_spec ] [ ws ] * '}'
        ^                 ^                 ^
        |                 |                 |
Argument index or name    |                 |
Formating specification   |                 |
Further formating specification             |
```

Here's an example that breaks down each part:
```txt
format!("Hello Worl{0:!<4}", 'd'); //=> "Hello World!!!"
//                  ^ ^^^     ^
//                  | |||     |
// Argument index   | |||     |
// Pad character      |||     |
// Pad alignment       ||     |
// Pad width            |     |
// Zero'th argument           |
```

#### It comprises of 2 parts. 

 1. The first defines the argument you want the
"placeholder" to take from the ones being passed. 
This can be a positional index
or a named argument. In the example above, `'d'` was the 0th argument being
passed after the _format string_ to `format!()` macro. 
Hence the argument index being 0.  
 1. We then have a separator `:` after which we can define one or more
_formatting parameters_. In our example, we used a padding character `'!'` with
padding alignment of right and width of 4. This means after inserting the 0th
argument `'d'` pad it with `'!'` till length becomes 4.


### Escaping
The literal characters `{` and `}` may be included in a string by preceding them with the same character. For example, the `{` character is escaped with `{{` and the `}` character is escaped with `}}`.

```rust
format!("Hello {{}}"), "Hello {}";
format!("{{ Hello"), "{ Hello";
```

## Multiline Strings

Rust allows multi-line strings. They do not require any special syntax.

```rust
const MY_STRING: &str = "
See no wretched
quotes everywhere
";
```
