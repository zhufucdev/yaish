use std::io::Write;
use console::Term;

pub struct Display {
    term: Term,
}

impl From<Term> for Display {
    fn from(term: Term) -> Display {
        return Display { term };
    }
}

impl Display {
    fn show_suggestions(&mut self, suggestions: Vec<String>) {}

    fn show_completion(&mut self, completion: String) {}
}