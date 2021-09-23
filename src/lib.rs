mod leftpad_rs {
    pub fn pad(s: &str, n: usize) -> String {
        format!("{:>width$}", s, width = n)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        struct Testcase<'a> {
            s: &'a str,
            n: usize,
            want: &'a str,
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
    }

}

pub use crate::leftpad_rs::*;
