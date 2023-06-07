use console::Term;

mod config;
mod interactive;


fn main() {
    let term = Term::stdout();
    if !term.is_term() {
        println!("Liar! This is not a terminal!")
    }

    interactive::terminal::main_loop(term);
}
