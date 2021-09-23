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
            want: String,
        }

        #[test]
        fn test_pad() {
            let testdata: [Testcase; 4] = [
                Testcase { s: "foo", n: 2, want: "foo".to_string()},
                Testcase { s: "foo", n: 3, want: "foo".to_string()},
                Testcase { s: "foo", n: 4, want: " foo".to_string()},
                Testcase { s: "foo", n: 5, want: "  foo".to_string()},
            ];

            for t in testdata {
                assert_eq!(pad(t.s, t.n), t.want);
            }
        }
    }

}

pub use crate::leftpad_rs::pad;
