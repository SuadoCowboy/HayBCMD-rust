/// while it's just testing, it will be as simple as that,<br>
/// but when i start making a game i will change this code<br>
/// to be related with the game interface
pub fn print<S: ToString>(s: S) { 
    print!("{}", s.to_string());
}

pub fn println<S: ToString>(s: S) { 
    println!("{}", s.to_string());
}

pub fn print_unknown_command(command: &str) {
    println!("unknown command \"{}\"", command);
}

pub fn print_command_usage(name: String, usage: String) {
    println(&format!("{} {}", name, usage));
}