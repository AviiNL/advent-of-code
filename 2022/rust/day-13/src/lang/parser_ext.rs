use super::lexer::TokenType;

use super::parser::Parser;

type Error = Box<dyn std::error::Error>;

pub trait ParserExt {
    fn peek(&self) -> &TokenType;
    fn eat(&mut self) -> Result<TokenType, Error>;
    fn expect(&mut self, expected: TokenType) -> Result<TokenType, Error>;
    fn is_eof(&self) -> bool;
}

impl ParserExt for Parser {
    fn peek(&self) -> &TokenType {
        &self.tokens[0]
    }

    fn eat(&mut self) -> Result<TokenType, Error> {
        let token = match self.tokens.pop_front() {
            Some(token) => token,
            None => return Err("Unexpected end of input".into()),
        };
        Ok(token)
    }

    fn expect(&mut self, expected: TokenType) -> Result<TokenType, Error> {
        let token = self.eat()?;

        if token == expected || expected == TokenType::EOF {
            Ok(token)
        } else {
            Err(format!(
                "Unexpected token. Expected: {:?}, got: {:?}",
                expected, token
            )
            .into())
        }
    }

    fn is_eof(&self) -> bool {
        self.peek() == &TokenType::EOF
    }
}
