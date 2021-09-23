pub mod leftpad_rs {
    pub fn pad(s: &str, n: usize) -> String {
        format!("{:>width$}", s, width = n)
    }

    pub fn pad_char(s: &str, n: usize, c: char) -> Result<String, &str> {
        let l = s.len();

        if n <= 0 {
            return Err("invalid size");
        }
        if n <= l {
            return Ok(s.to_string());
        }
        let f = c.to_string().repeat(n - l);
        Ok(format!("{}{}", f, s))
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        use rstest::rstest;

        #[rstest]
        #[case(2,"foo")]
        #[case(3,"foo")]
        #[case(4," foo")]
        #[case(5,"  foo")]
        fn test_pad(#[case] n: usize, #[case] want: &str) {
            assert_eq!(want, pad("foo", n));
        }

        #[test]
        fn test_nopad() {
            assert_ne!(pad("foo", 6), "foobar")
        }

        #[rstest]
        #[case(2,"foo")]
        #[case(3,"foo")]
        #[case(4,"Xfoo")]
        #[case(5,"XXfoo")]
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
    }
}

pub use crate::leftpad_rs::*;
