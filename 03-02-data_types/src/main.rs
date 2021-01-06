// https://doc.rust-lang.org/stable/book/ch03-02-data-types.html
#![allow(unused)]

fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");
}
