use std::collections::HashMap;

pub struct Command<F: Fn(&Self, &Vec<&str>)> {
    name: String,
    func: F,
    min_args: u8,
    max_args: u8,
    usage: String,
}

impl<F: Fn(&Self, &Vec<&str>)> Command<F> {
    pub fn new(name: &str, func: F, min_args: u8, max_args: u8, usage: &str) -> Self {
        Command {
            name: name.to_string(),
            func,
            min_args,
            max_args,
            usage: usage.to_string(),
        }
    }

    pub fn print_usage(&self) {
        output::print(&format!("{} {}\n", self.name, self.usage));
    }

    pub fn delete_command(&self, commands_handler: &CommandsHandler, command_name: &str) -> bool {
        for (i, command) in commands_handler.commands.iter().enumerate() {
            if command.name == command_name {
                commands_handler.remove(i);
                return true;
            }
        }

        false
    }

    pub fn run(&mut self, args: &[String]) {
        self.func(self, args);
    }
}

pub struct CommandsHandler {
    pub commands: Vec<Command>
}

impl CommandsHandler {
    pub fn new() -> Self {
        CommandsHandler{
            commands: vec!()
        }
    }

    pub fn get_command(&self, name: &str, print_error: bool) -> Option<Box<&Command>> {
        for command in self.commands.iter() {
            if command.name == name {
                return Some(Box::from(command));
            }
        }

        if print_error {
            Output::print(&format!("unknown command \"{}\"\n", name));
        }

        None
    }
}

pub fn init_base_commands(variables: Box<HashMap<String, String>>, commands_handler: &CommandsHandler) {
    let help = |args: &[String]| {
        if args.len() == 1 {
            // Print usage for a specific command
            if let Some(command) = commands_handler.get_command(&args[0], true) {
                command.print_usage();
            }
            return;
        }

        // Print usage for all commands
        for command in commands_handler.commands.iter() {
            command.print_usage();
        }
    };

    let echo = |args: &[String]| {
        let message = args.join(" ");
        Output::print(&format!("{}\n", message));
    };

    let alias = |args: &[String]| {
        if args.len() == 1 {
            variables.remove(&args[0]);
            return;
        }

        if Command::get_command(&args[0], false).is_some() {
            Output::print("varName is a command name, therefore this variable can not be created\n");
            return;
        }

        let whitespace_regex = regex::Regex::new(r"\S+").unwrap();
        if !whitespace_regex.is_match(&args[0]) {
            Output::print("variable name can not have whitespace.\n");
            return;
        }

        variables.insert(args[0].clone(), args[1].clone());
    };

    let get_variables = |_args: &[String]| {
        let mut output = String::new();
        let mut count = 0;

        for (key, value) in variables.iter() {
            output += &format!("{} = \"{}\"\n", key, value);
            count += 1;
        }

        let mut out = format!("amount of variables: {}\n", count);
        if !output.is_empty() {
            output.pop(); // Remove trailing newline
            out += &format!("{}\n", output);
        }

        Output::print(&out);
    };

    let variable = |args: &[String]| {
        let key = &args[0];
        if let Some(value) = variables.get(key) {
            Output::print(&format!("{} = \"{}\"\n", key, value));
        } else {
            Output::print(&format!("variable \"{}\" does not exist\n", key));
        }
    };

    let increment_var: dyn Fn(&[String]) = |args: &[String]| {
        let variable = &args[0];
        let min_value = args[1].parse::<f64>().unwrap();
        let max_value = args[2].parse::<f64>().unwrap();
        let delta = args[3].parse::<f64>().unwrap();

        if min_value > max_value {
            Output::print("minValue is higher than maxValue");
            return;
        }

        if let Some(value) = variables.get_mut(variable) {
            let mut variable_value = value.parse::<f64>().unwrap();
            variable_value += delta;
            if variable_value > max_value {
                variable_value = min_value;
            } else if variable_value < min_value {
                variable_value = max_value;
            }
            *value = variable_value.to_string();
        } else {
            Output::print(&format!("unknown variable \"{}\"\n", variable));
        }
    };

    // Add commands
    commands.push(Command::new("help", Box::from(help), 0, 1, "<command?> - shows a list of commands usages or the usage of a specific command"));
    commands.push(Command::new("echo", echo, 1, 1, "<message> - echoes a message to the console"));
    commands.push(Command::new("alias", alias, 1, 2, "<var> <commands?> - creates/deletes variables"));
    commands.push(Command::new("variables", variables, 0, 0, "- list of variables"));
    commands.push(Command::new("variable", variable, 1, 1, "- shows variable value"));
    commands.push(Command::new("incrementvar", 4, incrementvar, 4, "<var> <minValue> <maxValue> <delta> - increments the value of a variable"));
}