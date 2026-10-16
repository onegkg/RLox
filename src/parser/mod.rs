//! Updated grammar for the parser:
//! expression      -> equality
//! equality        -> comparison ( ( "!=" | "==" ) comparison )* ;
//! comparison      -> term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
//! term            -> factor ( ( "-" | "+" ) factor )* ;
//! factor          -> unary ( ( "/" | "*" ) unary)* ;
//! unary           -> ( "!" | "-" ) unary
//!                  | primary ;
//! primary         -> NUMBER | STRING | "true" | "false" | "nil"
//!                  | "(" expression ")" ;

use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
};

use crate::{
    parser::ast::{Expr, LoxLiteral},
    scanner::token::{Token, TokenType},
};

pub mod ast;

#[derive(Debug, PartialEq, Clone)]
pub struct ParseError {
    pub token: Option<Token>,
    pub msg: String,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?} {}", self.token, self.msg)
    }
}

impl Error for ParseError {}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    errors: Vec<ParseError>,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
            errors: Vec::new(),
        }
    }

    pub fn parse() -> Option<Expr> {
        todo!()
    }

    fn expression(&mut self) -> Option<Expr> {
        self.equality()
    }

    fn equality(&mut self) -> Option<Expr> {
        let mut expr = self.comparison()?;

        while self.consume_if_is(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous();
            let right = Box::from(self.comparison()?);
            expr = Expr::Binary {
                left: Box::from(expr),
                operator,
                right,
            };
        }

        Some(expr)
    }

    fn previous(&self) -> Token {
        self.tokens
            .get(self.current - 1)
            .expect("current - 1 should never be out of range")
            .clone()
    }

    fn comparison(&mut self) -> Option<Expr> {
        let mut expr = self.term()?;

        while self.consume_if_is(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous();
            let right = Box::from(self.term()?);
            expr = Expr::Binary {
                left: Box::from(expr),
                operator,
                right,
            };
        }

        Some(expr)
    }

    fn term(&mut self) -> Option<Expr> {
        let mut expr = self.factor()?;

        while self.consume_if_is(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous();
            let right = Box::from(self.factor()?);
            expr = Expr::Binary {
                left: Box::from(expr),
                operator,
                right,
            }
        }

        Some(expr)
    }

    fn factor(&mut self) -> Option<Expr> {
        let mut expr = self.unary()?;
        while self.consume_if_is(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous();
            let right = Box::from(self.unary()?);
            expr = Expr::Binary {
                left: Box::from(expr),
                operator,
                right,
            }
        }

        Some(expr)
    }

    fn unary(&mut self) -> Option<Expr> {
        if self.consume_if_is(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = Box::from(self.unary()?);
            return Some(Expr::Unary { operator, right });
        }

        self.primary()
    }

    fn primary(&mut self) -> Option<Expr> {
        let t = self.advance()?;
        match t.kind {
            TokenType::False => Some(Expr::Literal(LoxLiteral::False)),
            TokenType::True => Some(Expr::Literal(LoxLiteral::True)),
            TokenType::Nil => Some(Expr::Literal(LoxLiteral::Nil)),
            TokenType::Number(n) => Some(Expr::Literal(LoxLiteral::Float(n))),
            TokenType::String(s) => Some(Expr::Literal(LoxLiteral::String(s))),
            TokenType::LeftParen => {
                let expr = self.expression()?;
                self.expect_next_token(TokenType::RightParen, "Expected ')' after expression.")?;
                Some(Expr::Grouping {
                    expression: Box::from(expr),
                })
            }
            _ => {
                self.error(Some(t), "Invalid token for primary");
                None
            }
        }
    }

    fn error(&mut self, token: Option<Token>, msg: &str) {
        self.errors.push(ParseError {
            token,
            msg: String::from(msg),
        });
    }

    fn expect_next_token(&mut self, kind: TokenType, msg: &str) -> Option<()> {
        if let Some(t) = self.peek()
            && t.kind == kind
        {
            let _ = self.advance();
            Some(())
        } else {
            self.error(self.peek(), msg);
            None
        }
    }

    fn consume_if_is(&mut self, kinds: &[TokenType]) -> bool {
        for kind in kinds {
            if let Some(t) = self.peek()
                && t.kind == *kind
            {
                let _ = self.advance();
                return true;
            }
        }
        false
    }

    // Equivalent to peek(); current++
    fn advance(&mut self) -> Option<Token> {
        self.current += 1;
        self.tokens.get(self.current - 1).map(|t| t.clone())
    }

    fn peek(&self) -> Option<Token> {
        self.tokens.get(self.current).map(|t| t.clone())
    }

    fn synchronize(&mut self) {
        let _ = self.advance();
        while let Some(t) = self.peek() {
            if self.previous().kind == TokenType::Semicolon {
                return;
            }

            match t.kind {
                TokenType::Class => return,
                TokenType::Fun => return,
                TokenType::Var => return,
                TokenType::For => return,
                TokenType::If => return,
                TokenType::While => return,
                TokenType::Print => return,
                TokenType::Return => return,
                _ => {}
            }

            let _ = self.advance();
        }
    }
}
