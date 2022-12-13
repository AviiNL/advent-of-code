use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenType {
    OpenBracket,
    CloseBracket,
    Comma,
    Number(i64),
    EOL,
    EOF,
}

pub fn tokenize(input: &str) -> Result<VecDeque<TokenType>, Box<dyn std::error::Error>> {
    let mut tokens = VecDeque::new();

    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '\n' => tokens.push_back(TokenType::EOL),
            '[' => tokens.push_back(TokenType::OpenBracket),
            ']' => tokens.push_back(TokenType::CloseBracket),
            ',' => tokens.push_back(TokenType::Comma),
            '0'..='9' => {
                let mut number = String::new();
                number.push(c);
                while let Some(&c) = chars.peek() {
                    if c.is_numeric() {
                        number.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push_back(TokenType::Number(number.parse()?));
            }
            _ => {
                if c.is_whitespace() {
                    continue;
                }
                panic!("Unexpected token: {}", c)
            }
        }
    }

    tokens.push_back(TokenType::EOF);

    Ok(tokens)
}
