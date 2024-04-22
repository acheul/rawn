# Rawn

[![Crates.io](https://img.shields.io/crates/v/rawn)](https://crates.io/crates/rawn)
[![docs.rs](https://img.shields.io/docsrs/rawn?color=blue&label=docs.rs)](https://docs.rs/rawn)


Helper of raw pointers handling.

## BoxRaw
Trait `BoxRaw` is default implemented for tuples of mut raw pointers.

Struct `BoxRaws` is new type struct wrapping `BoxRaw` implemented type.

### Use
```rust
use rawn::{BoxRaw, BoxRaws};

// declare a raw pointer
let x: *mut u8 = Box::into_raw(Box::new(0u8));
// destruct it with `clean()` method of trait `BoxRaw`
x.clean();

// below code would work, but it's accessing dangling pointer.
// which is already cleaned out.
// rust's miri test would not pass this case.
{
  unsafe {
    let _ = *x;
  }
}

// BoxRaw is implemented for tuples of mut raw pointers.
// Available tuple size is from 1 to 12.
let a = Box::into_raw(Box::new(String::from("raw")));
let b = Box::into_raw(Box::new(12.));
let c: Box<Vec<u8>> = Box::new(vec![0, 1, 2]);
let c: *mut Vec<u8> = Box::into_raw(c);

let x = (a, b, c);
x.clean();

// `BoxRaws` is new type struct wrapping a tuple of raw pointers.
// It implements BoxRaw too.
let a = Box::into_raw(Box::new(0u8));
let b = Box::into_raw(Box::new(1u8));
let raws = BoxRaws::new((a, b));
raws.clean();
```

```rust
// BoxRaw is only for raw pointers which are declared using `Box::into_raw()`.
// Thus, Using coerced mutable references would make errors.
// This code will panic:
let mut c: Vec<u8> = vec![0, 1, 2];
let c: *mut Vec<u8> = &mut c;
c.clean();
```