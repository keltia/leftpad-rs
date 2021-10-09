# Leftpad

[![CircleCI](https://circleci.com/gh/keltia/leftpad-rs/tree/main.svg?style=shield)](https://circleci.com/gh/keltia/leftpad-rs/tree/main)
[![dependency status](https://deps.rs/repo/github/keltia/leftpad-rs/status.svg)](https://deps.rs/repo/github/keltia/leftpad-rs)
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
use leftpad_rs::{pad,pad_char};

fn main() {
    let s = "foo";

    println!("{}", pad(s, 5));
    println!("{:?}", pad_char(s, 5, 'b'));
}
```
## crates.io
You can use this package in your project by adding the following
to your `Cargo.toml`:

``` toml
[dependencies]
leftpad-rs = "1.0.1"
```
then you can use it in your own crates.

