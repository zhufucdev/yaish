use std::io::stdout;
use crossterm::tty::IsTty;
use crate::interactive::display::Display;

mod config;
mod interactive;


fn main() {
    let out = stdout();
    if !out.is_tty() {
        println!("Liar! This is not a terminal!");
        return;
    }

    let mut display = Display::from(out);
    display.main_loop();
}
