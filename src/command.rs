use std::collections::HashMap;

use crate::output;

pub trait CommandCall {
    fn call(&self, commands_handler: &mut CommandsHandler, command: Box<Command>, args: &[String]);
}

#[derive(Clone)]
pub struct Command {
    pub name: String,
    pub func_idx: usize,
    pub min_args: u8,
    pub max_args: u8,
    pub usage: String,
}

impl Command {
    pub fn new(name: &str, func_idx: usize, min_args: u8, max_args: u8, usage: &str) -> Self {
        Command {
            name: name.to_string(),
            func_idx,
            min_args,
            max_args,
            usage: usage.to_string(),
        }
    }
}

pub struct CommandsFuncs {
    commands_funcs: Vec<Box<dyn CommandCall>>
}

impl CommandsFuncs {
    pub fn new() -> Self {
        CommandsFuncs {
            commands_funcs: vec!()
        }
    }

    pub fn call(&mut self, commands_handler: &mut CommandsHandler, command: Box<Command>, arguments: Vec<String>) {
        self.commands_funcs[command.func_idx].call(commands_handler, command, &arguments)
    }

    pub fn push(&mut self, command_func: impl CommandCall + 'static) {
        self.commands_funcs.push(Box::new(command_func));
    }

    pub fn get(&self) -> Vec<Box<dyn CommandCall>> {
        let mut v = vec!();
        for i in self.commands_funcs {
            v.push(i);
        }

        v
    }
}

#[derive(Clone)]
pub struct CommandsHandler {
    pub commands: Vec<Command>,
    pub variables: HashMap<String, String>
}

impl CommandsHandler {
    pub fn new() -> Self {
        CommandsHandler{
            commands: vec!(),
            variables: HashMap::new()
        }
    }

    /// Deletes a command
    pub fn delete(&mut self, command_name: &str) -> bool {
        for (i, command) in self.commands.iter().enumerate() {
            if command.name == command_name {
                self.commands.remove(i);
                return true;
            }
        }

        false
    }

    /// Gets a command
    pub fn get<S: ToString>(&mut self, name: S, print_error: bool) -> Option<Box<Command>> {
        for command in self.commands.iter() {
            if command.name == name.to_string() {
                return Some(Box::new(command.clone()));
            }
        }

        if print_error {
            output::print(&format!("unknown command \"{}\"\n", name.to_string()));
        }

        None
    }

    /// Finds if a command is inside commands vector
    pub fn contains(&self, this_command: &Command) -> bool {
        for command in self.commands.iter() {
            if command.name == this_command.name {
                return true;
            }
        }

        false
    }

    /// Registers a new commands
    pub fn register(&mut self, command: Command) {
        if self.contains(&command) {
            return;
        }

        self.commands.push(command);
    }
}

pub fn init_base_commands(commands_handler: &mut CommandsHandler, commands_funcs: &mut CommandsFuncs) {
    struct HelpFunc;
    impl CommandCall for HelpFunc {
        fn call(&self, commands_handler: &mut CommandsHandler, _command: Box<Command>, args: &[String]) {
            if args.len() == 1 {
                // Print usage for a specific command
                if let Some(command) = commands_handler.get(&args[0], true) {
                    output::print_command_usage(command.name.clone(), command.usage.clone());
                }
                return;
            }
    
            // Print usage for all commands
            for command in commands_handler.commands.iter() {
                output::print_command_usage(command.name.clone(), command.usage.clone());
            }
        }
    }

    struct EchoFunc;
    impl CommandCall for EchoFunc {
        fn call(&self, _commands_handler: &mut CommandsHandler, _command: Box<Command>, args: &[String]) {
            let message = args.join(" ");
            output::println(&format!("{}", message));
        }
    }

    struct AliasFunc;
    impl CommandCall for AliasFunc {
        fn call(&self, commands_handler: &mut CommandsHandler, _command: Box<Command>, args: &[String]) {
            if args.len() == 1 {
                commands_handler.variables.remove(&args[0]);
                return;
            }
    
            if commands_handler.get(&args[0], false).is_some() {
                output::print("varName is a command name, therefore this variable can not be created\n");
                return;
            }
    
            if args[0].chars().map(|c| {c.is_whitespace() || c.is_ascii_whitespace()}).count() > 0 {
                output::print("variable name can not have whitespace.\n");
                return;
            }
    
            commands_handler.variables.insert(args[0].clone(), args[1].clone());
        }
    }

    struct GetVariablesFunc;
    impl CommandCall for GetVariablesFunc {
        fn call(&self, commands_handler: &mut CommandsHandler, _command: Box<Command>, _args: &[String]) {
            let mut output = String::new();
            let mut count = 0;

            for (key, value) in commands_handler.variables.iter() {
                output += &format!("{} = \"{}\"\n", key, value);
                count += 1;
            }

            let mut out = format!("amount of variables: {}\n", count);
            if !output.is_empty() {
                output.pop(); // Remove trailing newline
                out += &format!("{}\n", output);
            }

            output::print(&out);
        }
    }

    struct VariableFunc;
    impl CommandCall for VariableFunc {
        fn call(&self, commands_handler: &mut CommandsHandler, _command: Box<Command>, args: &[String]) {
            let key = &args[0];
            
            if let Some(value) = commands_handler.variables.get(key) {
                output::print(&format!("{} = \"{}\"\n", key, value));
            } else {
                output::print(&format!("variable \"{}\" does not exist\n", key));
            }
        }
    }

    struct IncrementVarFunc;
    impl CommandCall for IncrementVarFunc {
        fn call(&self, commands_handler: &mut CommandsHandler, _command: Box<Command>, args: &[String]) {
            let variable = &args[0];
            let min_value = args[1].parse::<f64>().unwrap();
            let max_value = args[2].parse::<f64>().unwrap();
            let delta = args[3].parse::<f64>().unwrap();

            if min_value > max_value {
                output::print("minValue is higher than maxValue");
                return;
            }

            if let Some(value) = commands_handler.variables.get_mut(variable) {
                let mut variable_value = value.parse::<f64>().unwrap();
                variable_value += delta;
                if variable_value > max_value {
                    variable_value = min_value;
                } else if variable_value < min_value {
                    variable_value = max_value;
                }
                *value = variable_value.to_string();
            } else {
                output::print(&format!("unknown variable \"{}\"\n", variable));
            }
        }
    }

    // Add commands
    commands_handler.register(Command::new("help", 0, 0, 1, "<command?> - shows a list of commands usages or the usage of a specific command"));
    commands_funcs.push(HelpFunc);
    commands_handler.register(Command::new("echo", 1, 1, 1, "<message> - echoes a message to the console"));
    commands_funcs.push(EchoFunc);
    commands_handler.register(Command::new("alias", 2, 1, 2, "<var> <commands?> - creates/deletes variables"));
    commands_funcs.push(AliasFunc);
    commands_handler.register(Command::new("variables", 3, 0, 0, "- list of variables"));
    commands_funcs.push(GetVariablesFunc);
    commands_handler.register(Command::new("variable", 4, 1, 1, "- shows variable value"));
    commands_funcs.push(VariableFunc);
    commands_handler.register(Command::new("incrementvar", 5, 4, 4, "<var> <minValue> <maxValue> <delta> - increments the value of a variable"));
    commands_funcs.push(IncrementVarFunc);
}