#[derive(Debug)]
enum State {
    Match,
    Char { c: char, next: Box<State> },
    Split { first: Box<State>, second: Box<State> },
}

fn print_state(s: State) {
    match s {
        State::Match => print!("Match"),
        State::Char { c, next } => {
            print!("{}", c);
            print_state(*next);
        },
        State::Split { first, second} => {
            print_state(*first);
            print_state(*second);
        },
    }
}

fn main() {
    let m = State::Match;
    let c = State::Char { c: 'c', next: Box::new(State::Match) };
    let s = State::Split { first: Box::new(State::Match), second: Box::new(State::Match) } ;

    print_state(m);
    print_state(c);
    print_state(s);
}
