# Leftpad

Rust package to left pad a string with a character.

Inspired by the "left-pad" NPM package (and the fiasco that happened after its removal).

### Build status

Branch: master — [![master|Build Status](https://travis-ci.org/keltia/leftpad.svg?branch=master)](http://travis-ci.org/keltia/leftpad)

### Install

```
cargo install leftpad-rs 
```

### Usage

[![Docs.rs](https://docs.rs/leftpad-rs?status.svg)](https://docs.rs/leftpad-rs)

Add the following to your `Cargo.toml`:
``` toml
[dependencies]
leftpad-rs = "0.1.0"
```

``` rust
use leftpad_rs::Leftpad;

let e = "foobar";

println!("{}", leftpad(e, 5));
```
