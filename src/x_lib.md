# Thanks.

# Import slide modules for testing
```rust
#![feature(test)]
#![cfg(test)]

extern crate test;
extern crate time;
extern crate gj;
extern crate eventual;

#[macro_use]
extern crate mioco;
#[macro_use]
extern crate async_await;

pub mod slide_01_setup;
pub mod slide_02_standard_library;
pub mod slide_03_coroutines;
pub mod slide_04_futures;
pub mod slide_05_async_await;
```
