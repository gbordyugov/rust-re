#[derive(Debug)]
enum State<'a> {
    Match,
    Char { c: char, next: &'a State<'a> },
    Split { first: &'a State<'a>, second: &'a State<'a> },
}

fn print_state<'a>(s: &'a State) {
    match s {
        State::Match => print!("Match"),
        State::Char { c, next } => {
            print!("{}", c);
            print_state(&next);
        },
        State::Split { first, second } => {
            print_state(&first);
            print_state(&second);
        },
    }
}

fn main() {
    let m = State::Match;
    let c = State::Char { c: 'c', next: &m };
    let s = State::Split { first: &m, second: &c} ;

    print_state(&m);
    print_state(&c);
    print_state(&s);
}
