// https://doc.rust-lang.org/stable/book/ch03-01-variables-and-mutability.html

fn main() {
    const MAX_POINTS: u32 = 100_000;

    let x = 5;
    let x = x + MAX_POINTS;
    let x = x + 2;

    println!("X = {}", x);
}
