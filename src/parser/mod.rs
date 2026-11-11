//! Updated grammar for the parser:
//! program         -> declaration* EOF ;
//! declaration     -> varDecl
//!                  | statement
//!                  ;
//! varDecl         -> "var" IDENTIFIER ( "=" expression )? ";" ;
//! statement       -> exprStmt
//!                  | printStmt
//!                  ;
//! exprStmt        -> expression ";" ;
//! printStmt       -> "print" expression ";" ;
//! expression      -> equality
//! equality        -> comparison ( ( "!=" | "==" ) comparison )* ;
//! comparison      -> term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
//! term            -> factor ( ( "-" | "+" ) factor )* ;
//! factor          -> unary ( ( "/" | "*" ) unary)* ;
//! unary           -> ( "!" | "-" ) unary
//!                  | primary
//!                  ;
//! primary         -> NUMBER
//!                  | STRING
//!                  | IDENTIFIER
//!                  | "true"
//!                  | "false"
//!                  | "nil"
//!                  | "(" expression ")"
//!                  ;

use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

use crate::{
    ast::{Expr, LoxLiteral, Program, Stmt},
    scanner::token::{Token, TokenType},
};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct ParseError {
    pub token: Option<Token>,
    pub msg: String,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} {}", self.token, self.msg)
    }
}

#[derive(Debug, PartialEq, Default, Clone)]
pub struct ParseErrors(pub Vec<ParseError>);

impl ParseErrors {
    fn new() -> Self {
        Self(Vec::new())
    }
}

impl Display for ParseErrors {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for err in &self.0 {
            write!(f, "{err}")?;
        }
        Ok(())
    }
}

impl Error for ParseErrors {}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    errors: ParseErrors,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
            errors: ParseErrors::new(),
        }
    }

    pub fn parse(&mut self) -> Result<Program, ParseErrors> {
        let mut stmts: Vec<Stmt> = Vec::new();
        while !self.is_at_end() {
            if let Some(stmt) = self.declaration() {
                stmts.push(stmt);
            }
        }
        if self.errors.0.len() != 0 {
            Err(std::mem::take(&mut self.errors))
        } else {
            Ok(Program::from(stmts))
        }
    }

    fn declaration(&mut self) -> Option<Stmt> {
        if self.consume_if_is(&[TokenType::Var]) {
            return match self.var_declaration() {
                Some(stmt) => Some(stmt),
                None => {
                    self.synchronize();
                    None
                }
            };
        }
        match self.statement() {
            Some(stmt) => Some(stmt),
            None => {
                self.synchronize();
                None
            }
        }
    }

    fn var_declaration(&mut self) -> Option<Stmt> {
        let name = self.expect_next_token(TokenType::Identifier, "Expected variable name.")?;

        let mut initializer = None;
        if self.consume_if_is(&[TokenType::Equal]) {
            initializer = self.expression();
        }

        self.expect_next_token(
            TokenType::Semicolon,
            "Expected `;` after variable declaration.",
        )?;
        Some(Stmt::Var { name, initializer })
    }

    fn statement(&mut self) -> Option<Stmt> {
        if self.consume_if_is(&[TokenType::Print]) {
            self.print_statement()
        } else {
            self.expression_statement()
        }
    }

    fn print_statement(&mut self) -> Option<Stmt> {
        let value = self.expression()?;
        let _ = self.expect_next_token(TokenType::Semicolon, "Expected `;` after value.")?;
        Some(Stmt::Print(value))
    }

    fn expression_statement(&mut self) -> Option<Stmt> {
        let value = self.expression()?;
        let _ = self.expect_next_token(TokenType::Semicolon, "Expected `;` after expression.")?;
        Some(Stmt::Expression(value))
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
                token: operator.clone(),
                operator,
                right,
            };
        }

        Some(expr)
    }

    /// Returns the `Token` immediately before the current `Token`. Panics if that token is somehow out of range.
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
                token: operator.clone(),
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
                token: operator.clone(),
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
                token: operator.clone(),
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
            return Some(Expr::Unary {
                token: operator.clone(),
                operator,
                right,
            });
        }

        self.primary()
    }

    fn primary(&mut self) -> Option<Expr> {
        let t = self.advance()?;
        match t.kind {
            TokenType::False => Some(Expr::Literal {
                literal: LoxLiteral::False,
                token: t,
            }),
            TokenType::True => Some(Expr::Literal {
                literal: LoxLiteral::True,
                token: t,
            }),
            TokenType::Nil => Some(Expr::Literal {
                literal: LoxLiteral::Nil,
                token: t,
            }),
            TokenType::Number(n) => Some(Expr::Literal {
                literal: LoxLiteral::Float(n),
                token: t,
            }),
            TokenType::String(ref s) => Some(Expr::Literal {
                literal: LoxLiteral::String(s.clone()),
                token: t,
            }),
            TokenType::LeftParen => {
                let expr = self.expression()?;
                self.expect_next_token(TokenType::RightParen, "Expected `)` after expression.")?;
                Some(Expr::Grouping {
                    expression: Box::from(expr),
                    token: t,
                })
            }
            TokenType::Identifier => Some(Expr::Variable {
                name: self.previous(),
            }),
            TokenType::EOF => None,
            _ => {
                self.error(Some(t), "Expected expression");
                None
            }
        }
    }

    fn error(&mut self, token: Option<Token>, msg: &str) {
        self.errors.0.push(ParseError {
            token,
            msg: String::from(msg),
        });
    }

    /// Used to assert that the next token is of type `kind` in combination with the `?` operator. If the next token is of type `kind` it is returned, otherwise, an error with the message `msg` is recorded and `None` is returned, short circuiting the caller.
    fn expect_next_token(&mut self, kind: TokenType, msg: &str) -> Option<Token> {
        if let Some(t) = self.peek()
            && t.kind == kind
        {
            self.advance()
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

    /// Consumes the current `Token` and returns it. Returns `None` if the end of file has been reached.
    fn advance(&mut self) -> Option<Token> {
        self.current += 1;
        self.tokens.get(self.current - 1).map(|t| t.clone())
    }

    fn peek(&self) -> Option<Token> {
        self.tokens.get(self.current).map(|t| t.clone())
    }

    /// Exits panic mode by advancing the parser until the end of the statement or until a new declaration.
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

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }
}
