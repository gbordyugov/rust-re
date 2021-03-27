#[derive(Debug)]
enum State {
    Match,
    Char { c: char, next: Box<State> },
    Split { first: Box<State>, second: Box<State> },
}

fn main() {
    println!("Hello, world!");
}
