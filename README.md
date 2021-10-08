# Leftpad

Rust package to left pad a string with a character.

Inspired by the "left-pad" NPM package (and the fiasco that happened after its removal).

### Build status

[![CircleCI](https://circleci.com/gh/keltia/leftpad-rs/tree/main.svg?style=shield)](https://circleci.com/gh/keltia/leftpad-rs/tree/main)

### Usage

[![Docs.rs](https://docs.rs/leftpad-rs?status.svg)](https://docs.rs/leftpad-rs)

Add the following to your `Cargo.toml`:
``` toml
[dependencies]
leftpad-rs = "0.1"
```

``` rust
use leftpad_rs::*;

fn main() {
    let s = "foo";

    println!("{}", pad(s, 5));
    println!("{:?}", pad_char(s, 5, 'b'));
}
```
