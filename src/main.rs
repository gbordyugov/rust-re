use std::rc::Rc;

#[derive(Debug)]
enum State {
    Match,
    Char { c: char, next: Rc<State> },
    Split { first: Rc<State>, second: Rc<State> },
}

#[derive(Debug)]
enum List<T> {
    Nil,
    Cons { car: T, cdr: Rc<List<T>> },
}

/*
fn tmp (s: Rc<State>) {
    match *s {
        State::Match => print!("Match"),
        State::Char { c, next } => {
            print!("{}", c);
            print_state(&*next);
        },
        State::Split { first, second } => {
            print_state(&*first);
            print_state(&*second);
        },
    }
}
*/

fn print_state(s: &State) {
    match s {
        State::Match => print!("Match"),
        State::Char { c, next } => {
            print!("{}", c);
            print_state(&*next);
        },
        State::Split { first, second } => {
            print_state(&*first);
            print_state(&*second);
        },
    }
}

fn main() {
    let m = State::Match;
    let mp = Rc::new(m);
    let c = State::Char { c: 'c', next: Rc::clone(&mp) };
    let cp = Rc::new(c);

    print_state(&mp);
    print_state(&cp);
}
