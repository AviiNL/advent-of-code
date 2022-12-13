use super::ast::{ArrayLiteral, Expression};
use super::lexer::{tokenize, TokenType};
use super::{ast, parser_ext::*, Error};
use std::collections::VecDeque;

pub struct Parser {
    pub tokens: VecDeque<TokenType>,
}

impl Parser {
    pub fn produce_ast(input: &str) -> Result<ast::Program, Error> {
        let tokens = tokenize(input)?;
        let mut parser = Parser { tokens };

        let body = parser.parse_block()?;

        Ok(ast::Program { body })
    }

    fn parse_block(&mut self) -> Result<Vec<ast::Expression>, Error> {
        let mut body = Vec::new();

        while !self.is_eof() {
            let expr = self.parse()?;
            body.push(expr);
        }

        Ok(body)
    }

    fn parse(&mut self) -> Result<ast::Expression, Error> {
        let expr = self.parse_expression();

        while self.peek() == &TokenType::EOL {
            self.eat()?;
        }

        expr
    }

    fn parse_expression(&mut self) -> Result<ast::Expression, Error> {
        let expr = self.parse_pair()?;

        Ok(expr)
    }

    fn parse_pair(&mut self) -> Result<ast::Expression, Error> {
        let mut expr = self.parse_array_expr()?;

        if self.peek() == &TokenType::EOL {
            self.eat()?;
            if self.peek() != &TokenType::EOL {
                let right = self.parse_pair()?;
                expr = Expression::Pair(vec![expr, right]);
            }
        }

        Ok(expr)
    }

    fn parse_array_expr(&mut self) -> Result<ast::Expression, Error> {
        if self.peek() != &TokenType::OpenBracket {
            return self.parse_primary();
        }

        self.eat()?; // eat the open bracket

        let mut elements = Vec::new();
        while !self.is_eof() && self.peek() != &TokenType::CloseBracket {
            let expr = self.parse();
            elements.push(expr);
            if self.peek() == &TokenType::Comma {
                self.eat()?;
            }
        }
        self.expect(TokenType::CloseBracket)?;
        Ok(Expression::Array(ArrayLiteral {
            elements: elements.into_iter().map(|e| e.unwrap()).collect(),
        }))
    }

    fn parse_primary(&mut self) -> Result<ast::Expression, Error> {
        let token = self.eat()?;
        match token {
            TokenType::Number(value) => Ok(Expression::Number(ast::NumericLiteral { value })),
            _ => panic!("Unexpected token: {:?}", token),
        }
    }
}
