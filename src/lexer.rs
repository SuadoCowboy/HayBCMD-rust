use crate::token::{Token, TokenType};
use crate::command;

#[derive(Clone)]
pub struct Lexer {
    input: String,
    position: usize,
    last_token: Token,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Lexer {
            input,
            position: 0,
            last_token: Token::new(TokenType::NOTHING, "".to_string())
        }
    }

    pub fn next_token(&mut self, commands_handler: &command::CommandsHandler) -> Token {
        if self.position >= self.input.len() {
            self.last_token = Token::new(TokenType::EOF, "".to_string());
            return self.last_token.clone();
        }

        let current_char = self.input.chars().nth(self.position).unwrap();
        while current_char.is_whitespace() {
            self.position += 1;

            if self.position >= self.input.len() {
                self.last_token = Token::new(TokenType::EOF, "".to_string());
                return self.last_token.clone();
            }
        }

        if self.input.chars().nth(self.position).unwrap() == ';' {
            self.position += 1;
            self.last_token = Token::new(TokenType::EOS, ";".to_string());
            return self.last_token.clone();
        }

        self.last_token = self.parse_token(commands_handler);
        self.last_token.clone()
    }

    fn is_variable(&self, identifier: &str) -> bool {
        identifier.starts_with('$')
    }

    fn is_command(&self, command_name: &str, commands_handler: &command::CommandsHandler) -> bool {
        commands_handler.commands.iter().any(|command| command.name == command_name)
    }

    fn parse_token(&mut self, commands_handler: &command::CommandsHandler) -> Token {
        if self.input.chars().nth(self.position).unwrap() == '"' {
            return self.parse_string();
        }

        let mut token_value = String::new();
        while self.position < self.input.len()
            && !self.input.chars().nth(self.position).unwrap().is_whitespace()
            && self.input.chars().nth(self.position).unwrap() != ';'
        {
            token_value.push(self.input.chars().nth(self.position).unwrap());
            self.position += 1;
        }

        if self.is_command(&token_value, commands_handler) && (self.last_token.token_type() == &TokenType::NOTHING || self.last_token.token_type() != &TokenType::COMMAND) {
            Token::new(TokenType::COMMAND, token_value)
        } else if self.is_variable(&token_value) {
            Token::new(TokenType::VARIABLE, token_value)
        } else {
            Token::new(TokenType::STRING, token_value)
        }
    }

    fn parse_string(&mut self) -> Token {
        let mut token_value = String::new();

        self.position += 1; // Skip the first double quote
        while self.position < self.input.len() && self.input.chars().nth(self.position).unwrap() != '"' {
            if self.input.chars().nth(self.position).unwrap() == '\\' && self.position + 1 < self.input.len() && self.input.chars().nth(self.position + 1).unwrap() == '"' {
                self.position += 1; // Skip the backslash
            }

            token_value.push(self.input.chars().nth(self.position).unwrap());
            self.position += 1;
        }

        self.position += 1; // Skip the last double quote

        Token::new(TokenType::STRING, token_value)
    }
}