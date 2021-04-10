# About

To understand [`Box`][box] it is helpful to understand what the [stack and heap][stack and heap] are.

The stack is an area of memory that works like a last-in-first-out (LIFO) buffer. It is very fast to push data onto the stack and it is fast to pop it off.
The data can include variables or function addresses, but the data must be of a known fixed size at compilation time.

If the data is of an unknown size at compilation time or has a size that may change, then it must be allocated on the heap. The allocation process must find
space in the heap big enough to store the data, write the data to the heap, and push the memory address of that data onto the stack, all of which takes
significantly more time than simply putting data on the stack. Retrieving data from the heap is a bit slower because we need to go through the address on the
stack and follow that to the location of the data on the heap.

`Box` is a smart pointer which allocates data on the heap. The address is on the stack, pointing to the data on the heap. One [reason][reasons] to use `Box`
is to put data on the heap when you have a type whose size can’t be known at compile time and you want to use a value of that type in a context that requires
an exact size. One practical usage of that is when you have multiple structs which you want to handle in the same way via a trait which they all implement, as
exampled below.

Let's say we have two structs. Each struct is of something that can be annoying. We can have each struct implement an Annoyance trait like so

```rust
struct WerewolfOfLondon;
struct Rooster;

trait Annoyance {
    fn annoy(&self) -> &'static str;
}

impl Annoyance for WerewolfOfLondon {
    fn annoy(&self) -> &'static str {
        "ahhh-OOOOOOO!"
    }
}

impl Annoyance for Rooster {
    fn annoy(&self) -> &'static str {
        "errr-errr-errr-errr-ERRRRRRRRR!"
    }
}
```

Now let's say we want a function that will give us an Annoyance based on the time of day that's passed into it. We can first try this, which we discover won't compile

```rust
enum TimeOfDay {
    Midnight,
    Morning,
}

//return something that implements Annoyance
fn annoy_me(time_of_day: TimeOfDay) -> impl Annoyance {
    match time_of_day {
        TimeOfDay::Midnight => WerewolfOfLondon,
        TimeOfDay::Morning => Rooster,
    }
}

// `match` arms have incompatible types
// expected struct `WerewolfOfLondon`, found struct `Rooster`
// this is found to be of type `WerewolfOfLondon`
// `match` arms have incompatible types
// you could change the return type to be a boxed trait object
// if you change the return type to expect trait objects, box the returned expressions

```

The compiler, in all of its helpfulness, tells us exactly what to do, which is to return a boxed trait object. If you don't know what a trait object is,
a simple way to describe it is a way of returning a specific object via [dynamic dispatch][dynamic dispatch] from a function that could return any object
implementing a specified trait. Since we need dynamic dispatch, we can try just putting a `dyn` in front of the trait name and see if we get away with it.

```rust
fn annoy_me(time_of_day: TimeOfDay) -> dyn Annoyance {
    match time_of_day {
        TimeOfDay::Midnight => WerewolfOfLondon,
        TimeOfDay::Morning => Rooster,
    }
}

// the size for values of type `dyn Annoyance` cannot be known at compilation time
// the trait `Sized` is not implemented for `dyn Annoyance`
// all local variables must have a statically known size
// unsized locals are gated as an unstable feature
```

One of the errors is that the size for `dyn Annoyance` cannot be known at compile time. This is because `annoy_me` does not know what specific trait object
it will return in advance of when it is called. Structs implementing Annoyance could have different fields and be different sizes. And we are reminded that
one of the uses for `Box` is for a type whose size can’t be known at compile time. So now we try using a `Box`ed return value, and we `Box`the values returned
from the match expression, like so

```rust
fn annoy_me(time_of_day: TimeOfDay) -> Box<dyn Annoyance> {
    match time_of_day {
        TimeOfDay::Midnight => Box::new(WerewolfOfLondon),
        TimeOfDay::Morning => Box::new(Rooster),
    }
}
```

The signature for `Box` is `Box<T>` where `T` stands for the specific type to be boxed which is placed within the angle brackets, as in `Box<dyn Annoyance>` above.

We can consume the boxed trait object like so

```rust
fn be_annoying(annoying_one: &dyn Annoyance) {
    println!("{:?}", annoying_one.annoy());
}

pub fn main() {
    let my_annoyance = annoy_me(TimeOfDay::Midnight);
    be_annoying(&*my_annoyance);
    let my_annoyance = annoy_me(TimeOfDay::Morning);
    be_annoying(&*my_annoyance);
}
```

which prints

```rust
"ahhh-OOOOOOO!"
"errr-errr-errr-errr-ERRRRRRRRR!"
```

Since `my_annoyance` is of type `Box<dyn Annoyance>` it must be dereferenced from the `Box` using the deref operator `*`. It is then referenced as a
`dyn Annoyance` for the `be_annoying` function to borrow it.

Another way to implement it would be to remove the requirement that the parameter be sized. In that case, compilation would create a separate function
for each variant of Annoyance so it could be statically dispatched (a process called [monomorphization][monomorphization]), like so

```rust
fn be_annoying<A: Annoyance + ?Sized>(annoying_one: &A) {
    println!("{:?}", annoying_one.annoy());
}
```

Yet another way to implement it would be for the `be_annoying` function to accept a `Box<dyn Annoyance>` and use [deref coercion][deref coercion] to avoid
having to dereference the `Box` manually. This would transfer ownership of the Annoyance trait object to the `be_annoying` function. It would go out of
scope at the end of the function and no longer be available.

```rust
fn be_annoying(annoying_one: Box<dyn Annoyance>) {
    println!("{:?}", annoying_one.annoy());
}

pub fn main() {
    let my_annoyance = annoy_me(TimeOfDay::Midnight);
    be_annoying(my_annoyance);
    let my_annoyance = annoy_me(TimeOfDay::Morning);
    be_annoying(my_annoyance);
}
```

You can learn more about how to use `Box` in [its chapter][using box] in The Rust Programming Language book.

[box]: https://doc.rust-lang.org/std/boxed/index.html
[stack and heap]: https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#the-stack-and-the-heap
[reasons]: https://doc.rust-lang.org/book/ch15-01-box.html#using-boxt-to-point-to-data-on-the-heap
[dynamic dispatch]: https://doc.rust-lang.org/book/ch17-02-trait-objects.html#trait-objects-perform-dynamic-dispatch
[monomorphization]: https://doc.rust-lang.org/book/ch10-01-syntax.html#performance-of-code-using-generics
[deref coercion]: https://doc.rust-lang.org/book/ch15-02-deref.html?highlight=auto-deref#implicit-deref-coercions-with-functions-and-methods
[using box]: https://doc.rust-lang.org/nightly/book/ch15-01-box.html
