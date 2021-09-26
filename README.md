# `partial-array` - potentially partial-filled arrays for `#![no_std]`

[![Crates.io](https://img.shields.io/crates/v/partial-array.svg)](https://crates.io/crates/partial-array)
[![Docs.rs](https://docs.rs/partial-array/badge.svg)](https://docs.rs/partial-array)

This crate provides a central new data type, similar to an [array]: the [`PartialArray<N>`].
It is equivalent to an array, but the number of entries might be anywhere from `0` to `N`.
While this has similarities to a `Vec<T>` keep in mind, that a [`PartialArray`] does not grow its memory: it always takes up the memory for the fully array (with some additional counter) and it cannot ever hold more than `N` elements.
This means that its memory is _fully static_ and _on the stack_, making it usable from `#![no_std]` crates.

```rust
// some filter function, even numbers as an easy example
let f = |x: &i32| x % 2 == 0;

let array: partial_array::PartialArray<i32, 32> = (0..).take(32).filter(f).collect();
```

## Features

This crate is rather simple, but has a few key features, that might enable this crate to be considered:

- zero dependencies
- `#![no_std]` (enabled for embedded targets without dynamic memory)
- only few `unsafe` code, can be audited easily
- open source
- permissive license

## Example usage

This new data type is most likely to be used for collecting iterators into arrays, when then length is not known, but has an upper bound, e.g.:

```rust
use partial_array::PartialArray;

/// Take the first 10 elements of an iterator, that match the condition.
///
/// This can return less than 10 elements if the iterator has fewer than 10
/// items or there are less than 10 matching elements.
fn first_10_matching<T, I, F>(iter: I, check: F) -> PartialArray<T, 10>
    where I: IntoIterator<Item = T>,
          F: FnMut(&T) -> bool,
{
    iter.into_iter().filter(check).take(10).collect()
}
```
