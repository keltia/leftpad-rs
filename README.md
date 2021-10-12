# Leftpad

[![CircleCI](https://circleci.com/gh/keltia/leftpad-rs/tree/main.svg?style=shield)](https://circleci.com/gh/keltia/leftpad-rs/tree/main)
[![](https://img.shields.io/crates/v/leftpad-rs.svg)](https://crates.io/crates/leftpad-rs)
[![Docs](https://docs.rs/leftpad-rs/badge.svg)](https://docs.rs/leftpad-rs)

Rust package to left pad a string with a character.

Inspired by the "left-pad" NPM package (and the fiasco that happened after its removal).

**Supported Platforms**
* Unix (tested on FreeBSD, Linux and macOS)
* Windows
    * cmd.exe
    * Powershell

## Example
``` rust
use leftpad_rs::{pad,pad_char,pad_with};

fn main() {
    let s = "foo";

    println!("{}", pad(s, 5));
    let s1 = match pad_char(s, 6, ' ')) {
        Ok(s) => s,
        Err(e) => Err("Error: {}", e)
    };
    println!("{:?}", s1);
    
    // You can use pad_with() too now.
```
## crates.io
You can use this package in your project by adding the following
to your `Cargo.toml`:

``` toml
[dependencies]
leftpad-rs = "1.1.0"
```
then you can use it in your own crates.

