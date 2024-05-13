use std::sync::Mutex;

pub mod output;
pub mod token;
pub mod lexer;
pub mod parser;
pub mod command;

static OUTPUT: Mutex<output::Output> = Mutex::new(output::Output{pipe: None});

pub fn set_output_pipe(pipe: std::io::Stdout) {
    OUTPUT.lock().unwrap().pipe = Some(pipe);
}

pub fn init(pipe: std::io::Stdout) {
    OUTPUT.lock().unwrap().pipe = Some(pipe);
}

pub fn parse(str: String) -> String {
    //let parser_var = parser::new();
    //parser_var.parse(str)
    String::from("")
}