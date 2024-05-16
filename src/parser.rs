use crate::command::{CommandsFuncs, CommandsHandler};
use crate::lexer::Lexer;
use crate::output;
use crate::token::{Token, TokenType};

pub struct Parser {
    lexer: Box<Lexer>,
    current_token: Token,
    commands_handler: Box<CommandsHandler>,
    commands_funcs: CommandsFuncs
}

static ALIAS_MAX_CALLS: usize = 50000;

impl Parser{
    pub fn new(lexer: Box<Lexer>, commands_handler: Box<CommandsHandler>, commands_funcs: CommandsFuncs) -> Self {
        Parser {
            lexer,
            current_token: Token::new(TokenType::NOTHING, "".to_string()),
            commands_handler,
            commands_funcs
        }
    }

    pub fn parse(&mut self) {
        while self.current_token.token_type() != &TokenType::EOF {
            let variable_value = self.get_variable_from_current_token_value();

            if !variable_value.is_empty() {
                self.handle_alias_lexer(&variable_value);
            } else if self.current_token.token_type() == &TokenType::COMMAND {
                self.handle_command_token();
            } else if self.current_token.token_type() == &TokenType::STRING {
                output::print_unknown_command(self.current_token.value());
                self.advance_until(&[TokenType::EOS]);
            }

            self.advance();
        }
    }

    fn get_variable_from_current_token_value(&self) -> String {
        if let Some(value) = self.commands_handler.variables.get(self.current_token.value()) {
            value.clone()
        } else {
            "".to_string()
        }
    }

    fn advance(&mut self) {
        self.current_token = self.lexer.next_token(&self.commands_handler);
    }

    fn advance_until(&mut self, token_types: &[TokenType]) {
        self.advance(); // always skip the first one

        // checks if EOF is reached because if not, it would run forever
        while !token_types.contains(&self.current_token.token_type()) && self.current_token.token_type() != &TokenType::EOF {
            self.advance();
        }
    }

    fn handle_command_token(&mut self) {
        let command_opt = self.commands_handler.get(self.current_token.value(), true);

        if command_opt.is_none() {
            return;
        }

        let command = command_opt.unwrap();

        let command_name = command.name.clone();
        let command_usage = command.usage.clone();
        let command_max_args: u8 = command.max_args;
        let command_min_args: u8 = command.min_args;

        self.advance(); // skips the command token

        let mut arguments = self.get_arguments();
        
        // make it include whitespaces in that case
        if command_max_args == 1 && !arguments.is_empty() {
            let mut str = String::new();
            for arg in &arguments {
                str.push_str(format!("{arg} ").as_str());
            }
            arguments.clear();
            arguments.push(str.trim().to_string());
        
        } else {
            arguments.clear();
        }

        // checks if arguments size is within the allowed
        if arguments.len() > command_max_args as usize || arguments.len() < command_min_args as usize {
            output::print_command_usage(command_name, command_usage);
            if !arguments.is_empty() {
                output::print(&format!("arguments size must be within range [{}, {}], but size is {}\n", command_min_args, command_max_args, arguments.len()));
            }
            return;
        }
        
        self.commands_funcs.call(&mut self.commands_handler, command, arguments);
    }

    fn handle_alias_lexer(&mut self, input: &str) {
        let mut temp_lexers: Vec<Box<Lexer>> = vec![self.lexer.clone()];

        self.lexer = Box::new(Lexer::new(input.to_string()));
        self.advance();

        while self.current_token.token_type() != &TokenType::EOF {
            let variable = self.get_variable_from_current_token_value();

            if !variable.is_empty() {
                temp_lexers.push(self.lexer.clone());
                
                self.lexer = Box::new(*temp_lexers.last().unwrap().clone());
            
            } else if self.current_token.token_type() == &TokenType::COMMAND {
                self.handle_command_token();
            
            } else if self.current_token.token_type() == &TokenType::STRING {
                output::print_unknown_command(self.current_token.value());
                self.advance_until(&[TokenType::EOS]);
            }

            self.advance();
            
            if temp_lexers.len() == ALIAS_MAX_CALLS {
                break;
            }

            while self.current_token.token_type() == &TokenType::EOF && temp_lexers.len() > 1 {
                self.lexer = temp_lexers.pop().unwrap();
                self.advance();
            }
        }

        self.lexer = temp_lexers[0].clone();
        self.advance();
    }

    fn get_arguments(&mut self) -> Vec<String> {
        let mut arguments = Vec::new();

        while self.current_token.token_type() != &TokenType::EOF && self.current_token.token_type() != &TokenType::EOS {
            
            if self.current_token.token_type() == &TokenType::STRING || self.current_token.token_type() == &TokenType::COMMAND {
                arguments.push(self.current_token.value().to_string());
            
            } else if self.current_token.token_type() == &TokenType::VARIABLE {
            
                if let Some(value) = self.commands_handler.variables.get(&self.current_token.value()[1..]) {
                    arguments.push(value.clone());
            
                } else {
                    arguments.push(self.current_token.value().to_string());
                }
            }

            self.advance();
        }

        arguments
    }
}