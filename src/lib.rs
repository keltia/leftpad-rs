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

        struct Testcase<'a> {
            s: &'a str,
            n: usize,
            want: &'a str,
        }

        struct Testcase1<'a> {
            s: &'a str,
            n: usize,
            want: Result<String,&'a str>,
        }

        #[test]
        fn test_pad() {
            let testdata: [Testcase; 4] = [
                Testcase { s: "foo", n: 2, want: "foo" },
                Testcase { s: "foo", n: 3, want: "foo" },
                Testcase { s: "foo", n: 4, want: " foo" },
                Testcase { s: "foo", n: 5, want: "  foo" },
            ];

            for t in testdata {
                assert_eq!(pad(t.s, t.n), t.want);
            }
        }

        #[test]
        fn test_nopad() {
            assert_ne!(pad("foo", 6), "foobar")
        }

        #[test]
        fn test_pad_char() {
            let testdata: [Testcase1; 4] = [
                Testcase1 { s: "foo", n: 2, want: Ok("foo".to_string()) },
                Testcase1 { s: "foo", n: 3, want: Ok("foo".to_string()) },
                Testcase1 { s: "foo", n: 4, want: Ok("Xfoo".to_string()) },
                Testcase1 { s: "foo", n: 5, want: Ok("XXfoo".to_string()) },
            ];

            for t in testdata {
                assert_eq!(pad_char(t.s, t.n, 'X'), t.want);
            }
        }

        #[test]
        fn test_nopad_char() {
            assert_ne!(pad_char("foo", 6, 'X'), Ok("foobar".to_string()))
        }
    }
}

pub use crate::leftpad_rs::*;
