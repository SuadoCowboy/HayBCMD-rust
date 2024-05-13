#[cfg(test)]
mod tests {
    #[test]
    fn token() {
        let token = haybcmd::token::Token{
            token_type: haybcmd::token::TokenType::STRING,
            value: String::from("Hello, World!")
        };

        assert_ne!(token.token_type.to_string(), haybcmd::token::TokenType::NOTHING.to_string());
        assert_eq!(token.token_type.to_string(), haybcmd::token::TokenType::STRING.to_string());
    }
}
