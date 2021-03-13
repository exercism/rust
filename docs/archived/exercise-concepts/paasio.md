# Concepts of Paasio

PaaS I/O explores extending standard `Read` and `Write` traits to provide additional statistics via a wrapper type. A good way to learn about traits in Rust and their best features.

This exercise can be implemented using either a single or two (for `Read` and `Write`)
types, with appropriate trait bounds.

## Extracted concepts

- **Primitives**
  - `usize`
  - `u8`
- **Immutability/explicit mutability**
- **Structs**
- **Functions**
  - **Methods**
- **Slices**
- **Type aliases**
- **References**
  - Dereferencing
- **Traits**
  - Trait methods
  - `self` and mutability
  - Trait bounds
- **`impl` blocks**

## Example using a single wrapper struct

Taken from ["sophie-cyborg"](https://exercism.io/tracks/rust/exercises/paasio/solutions/a2255eb1cc73491184cf88edd982a8d0)

```rust
use std::io::{Read, Result, Write};

pub type ReadStats<T> = IoStats<T>;
pub type WriteStats<T> = IoStats<T>;

pub struct IoStats<T> {
    wrapped: T,
    bytes_through: usize,
    writes: usize,
    reads: usize,
}

impl<T> IoStats<T> {
    pub fn new(wrapped: T) -> IoStats<T> {
        IoStats {
            wrapped,
            bytes_through: 0,
            writes: 0,
            reads: 0,
        }
    }

    pub fn get_ref(&self) -> &T {
        &self.wrapped
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_through
    }
}

impl<T: Read> IoStats<T> {
    pub fn reads(&self) -> usize {
        self.reads
    }
}

impl<T: Write> IoStats<T> {
    pub fn writes(&self) -> usize {
        self.writes
    }
}

impl<T: Read> Read for IoStats<T> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.reads += 1;
        self.wrapped.read(buf).map(|bytes| {
            self.bytes_through += bytes;
            bytes
        })
    }
}

impl<T: Write> Write for IoStats<T> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.writes += 1;
        self.wrapped.write(buf).map(|bytes| {
            self.bytes_through += bytes;
            bytes
        })
    }

    fn flush(&mut self) -> Result<()> {
        self.wrapped.flush()
    }
}
```

## Example using two separate structs

Taken from ["fdumontmd"](https://exercism.io/tracks/rust/exercises/paasio/solutions/95344f0ee9ec43a0b152ca2d56c44519)

```rust
use std::io::{Read, Result, Write};

pub struct ReadStats<R> {
    wrapped: R,
    bytes_through: usize,
    reads: usize,
}

impl<R: Read> ReadStats<R> {
    pub fn new(wrapped: R) -> ReadStats<R> {
        ReadStats {
            wrapped,
            bytes_through: 0,
            reads: 0,
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.wrapped
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_through
    }

    pub fn reads(&self) -> usize {
        self.reads
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.reads += 1;
        self.wrapped.read(buf)
            .map(|bt| { self.bytes_through += bt; bt })
    }
}

pub struct WriteStats<W> {
    wrapped: W,
    bytes_through: usize,
    writes: usize,
}

impl<W: Write> WriteStats<W> {
    pub fn new(wrapped: W) -> WriteStats<W> {
        WriteStats {
            wrapped,
            bytes_through: 0,
            writes: 0,
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.wrapped
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_through
    }

    pub fn writes(&self) -> usize {
        self.writes
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.writes += 1;
        self.wrapped.write(buf)
            .map(|bt| { self.bytes_through += bt; bt })
    }

    fn flush(&mut self) -> Result<()> {
        self.wrapped.flush()
    }
}
```
