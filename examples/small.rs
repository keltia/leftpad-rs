use leftpad_rs::{pad, pad_char};

fn main() {
    let s = "foo";

    println!("{}", pad(s, 5));
    println!("{:?}", pad_char(s, 5, 'b'));
}
