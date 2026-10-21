use sao::{Automaton, Output};

fn main() {
    let mut automaton = Automaton::default();
    loop {
        match automaton.run() {
            Ok(Output::Continue) => continue,
            Ok(Output::Exit(code)) => std::process::exit(code),
            Err(err) => std::process::exit(automaton.handle_error(err)),
        }
    }
}
