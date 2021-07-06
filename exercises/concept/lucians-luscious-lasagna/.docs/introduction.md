# Introduction

TODO: rewrite for Rust

In F#, assigning a value to a name is referred to as a _binding_. Bindings are immutable, which makes them similar to constants in other languages. As F# is a statically-typed language, each binding has a type at compile-time.

Bindings are defined using the `let` keyword. Specifying a binding's type is optional for most bindings, as F#'s _type inference_ can usually infer the type based on their value. A binding looks like this:

```fsharp
// Automatically inferred type
let fingers = 10
```

Functions are also regular bindings, but with one or more parameters. A function automatically returns its last expression. Type inference also works for most functions, by analyzing what values the function is called with and what value the function returns.

```fsharp
// Automatically inferred types for parameters and return type
let add x y = x + y
```

Invoking a function is done by specifying its name and passing arguments for each of the function's parameters.

```fsharp
let five = add 2 3
```

If a binding's type cannot be inferred, the compiler will report an error. To fix this, add an explicit type annotation to the binding.

```fsharp
// Explicit type annotation
let fingers: int = 10

// Explicit type annotation (also for parameters)
let add (x: int) (y: int): int = x + y
```

Bindings in F# can only be used _after_ they have been defined. Using a binding before it has been defined results in a compile error.

```fsharp
// Compile error as the add binding has not yet been defined
// let seven = add 3 + 4

let add x y = x + y
```

F# uses significant whitespace to define scope. This means that scope is defined by indenting the code with spaces, relative to the line declaring the binding. The default convention is to use four spaces for indentation:

```fsharp
let toes =
    let left = 5
    let right = 5
    left + right

let multiplyPlusTwo x y =
    let product = x * y
    product + 2

// Trying to access the left, right or product bindings
// here would result in a compile error
```

F# bindings (which include functions) are usually organized in modules. A module groups related functionality and is defined using the `module` keyword. The module definition must precede its bindings:

```fsharp
module Calculator

let pi = 3.14

let add x y = x + y
```

F# supports two types of comments. Single line comments are preceded by `//` and multiline comments are inserted between `(*` and `*)`.
