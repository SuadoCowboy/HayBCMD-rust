use std::io::{Stdout, Write};

pub struct Output {
    pub pipe: Option<Stdout>,
}

impl Output {
    pub fn new(pipe: Stdout) -> Self {
        Output { pipe: Some(pipe) }
    }

    pub fn print(&self, str: &str) {
        self.pipe.as_ref().unwrap().lock().write_all(str.as_bytes()).unwrap();
    }

    pub fn print_unknown_command(&self, command: &str) {
        self.pipe.as_ref().unwrap().lock().write_fmt(format_args!("unknown command \"{}\"\n", command)).unwrap();
    }
}