use console::Term;
use crate::interactive::display::Display;

mod config;
mod interactive;


fn main() {
    let term = Term::stdout();
    if !term.is_term() {
        println!("Liar! This is not a terminal!")
    }

    let mut display = Display::from(term);
    display.main_loop();
}
