use std::rc::Rc;

#[derive(Debug)]
enum State {
    Match,
    Char { c: char, next: StateP },
}

type StateP = Rc<State>;

fn print_state(s: StateP) {
    match *s {
        State::Match => print!("Match"),
        State::Char { c, next } => {
            print!("{}", c);
            print_state(next);
        },
        // State::Split { first, second } => {
        //     print_state(*first);
        //     print_state(*second);
        // },
    }
}

fn main() {
    let m = State::Match;
    let mp = Rc::new(m);
    let c = State::Char { c: 'c', next: Rc::clone(&mp) };
    let cp = Rc::new(c);
    // let s = State::Split { first: Rc::clone(&mp), second: Rc::clone(&cp) };
    // let sp = Rc::new(s);

    print_state(mp);
    print_state(cp);
    // print_state(*sp);
}
