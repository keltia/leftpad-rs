//! Leftpad implementation in Rust.
//!
//! This is a Rust implementation of the famous Leftpad NPM package with `pad()` and `pad_char()`.
//! Based on the Go implementation I wrote back in 2018.
//!
//! Examples:
//! ```
//!   use leftpad_rs::{pad,pad_char};
//!
//!   let r = pad("foo", 5);  // -> "  foo"
//!
//!   let s = pad_char("foo", 6, 'X');    // -> "XXXfoo"
//! ```

/// This module implements `pad` and `pad_char`.
/// Left-pads the string with spaces.
pub fn pad(s: &str, n: usize) -> String {
    format!("{:>width$}", s, width = n)
}

/// Left-pads the string with the supplied character.
pub fn pad_char(s: &str, n: usize, c: char) -> Result<String, &str> {
    let l = s.len();

    if n == 0 {
        return Err("invalid size");
    }
    if n <= l {
        return Ok(s.to_string());
    }
    let f = c.to_string().repeat(n - l);
    Ok(format!("{}{}", f, s))
}

/// Useful alias
pub fn pad_with(s: &str, n: usize, c: char) -> Result<String, &str> {
    pad_char(s, n, c)
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(2, "foo")]
    #[case(3, "foo")]
    #[case(4, " foo")]
    #[case(5, "  foo")]
    fn test_pad(#[case] n: usize, #[case] want: &str) {
        assert_eq!(want, pad("foo", n));
    }

    #[test]
    fn test_nopad() {
        assert_ne!(pad("foo", 6), "foobar")
    }

    #[rstest]
    #[case(2, "foo")]
    #[case(3, "foo")]
    #[case(4, "Xfoo")]
    #[case(5, "XXfoo")]
    fn test_pad_char(#[case] n: usize, #[case] want: &str) {
        assert_eq!(Ok(want.to_string()), pad_char("foo", n, 'X'));
    }

    #[test]
    fn test_pad_char_0() {
        assert_eq!(Err("invalid size"), pad_char("foo", 0, 'X'))
    }

    #[test]
    fn test_nopad_char() {
        assert_ne!(Ok("foobar".to_string()), pad_char("foo", 6, 'X'))
    }

    #[rstest]
    #[case(2, "foo")]
    #[case(3, "foo")]
    #[case(4, "Xfoo")]
    #[case(5, "XXfoo")]
    fn test_pad_with(#[case] n: usize, #[case] want: &str) {
        assert_eq!(Ok(want.to_string()), pad_with("foo", n, 'X'));
    }

    #[test]
    fn test_pad_with_0() {
        assert_eq!(Err("invalid size"), pad_with("foo", 0, 'X'))
    }

    #[test]
    fn test_nopad_with() {
        assert_ne!(Ok("foobar".to_string()), pad_with("foo", 6, 'X'))
    }
}
