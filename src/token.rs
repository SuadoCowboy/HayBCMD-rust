use std::fmt;

#[derive(PartialEq, Eq)]
pub enum TokenType {
    NOTHING = 0,
    VARIABLE,
    STRING,
    COMMAND,
    EOF,
    EOS,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            TokenType::NOTHING => "NOTHING",
            TokenType::VARIABLE => "VARIABLE",
            TokenType::STRING => "STRING",
            TokenType::COMMAND => "COMMAND",
            TokenType::EOF => "EOF",
            TokenType::EOS => "EOS",
        };
        write!(f, "{}", s)
    }
}

#[derive(PartialEq, Eq)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
}

impl Token {
    pub fn new(token_type: TokenType, value: String) -> Self {
        Token {
            token_type,
            value,
        }
    }

    pub fn token_type(&self) -> &TokenType {
        &self.token_type
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn to_string(&self) -> String {
        format!("Token({}, \"{}\")", self.token_type, self.value)
    }
}