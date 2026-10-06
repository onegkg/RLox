pub mod token;

use std::{
    error::Error,
    fmt::{Display, Formatter},
};

use token::Token;

use crate::scanner::token::TokenType;

type Result<T> = std::result::Result<T, InternalLexError>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LexError {
    pub line: usize,
    pub loc: String,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum InternalLexError {
    Error(LexError),
    SkipToken,
    EOF,
}

impl InternalLexError {
    fn error(line: usize, message: String) -> Self {
        Self::Error(LexError {
            line,
            loc: String::from(""),
            message,
        })
    }
}

impl Display for InternalLexError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EOF => write!(f, "<EOF>"),
            Self::SkipToken => write!(f, "<SkipToken>"),
            Self::Error(LexError { line, loc, message }) => {
                write!(f, "[line {}] Error{}: {}", line, loc, message)
            }
        }
    }
}
impl Error for InternalLexError {}

pub struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    errors: Vec<LexError>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(input: String) -> Self {
        Self {
            source: input.chars().collect(),
            tokens: Vec::new(),
            errors: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> (Vec<Token>, Vec<LexError>) {
        loop {
            // if ok, push to tokens, if error, push to errors. if EOF, break
            match self.scan_token() {
                Ok(t) => {
                    self.start = self.current;
                    self.tokens.push(t);
                }
                Err(InternalLexError::Error(LexError { line, message, loc })) => {
                    self.start = self.current;
                    self.errors.push(LexError { line, loc, message });
                }
                Err(InternalLexError::SkipToken) => continue,
                Err(InternalLexError::EOF) => break,
            }
        }
        self.tokens
            .push(Token::new(TokenType::EOF, String::new(), self.line));
        (self.tokens.clone(), self.errors.clone())
    }

    /// currently unused
    fn is_at_end(&self) -> bool {
        return self.current >= self.source.len();
    }

    fn scan_token(&mut self) -> Result<Token> {
        let c = self.advance()?;
        match c {
            '(' => self.make_token(TokenType::LeftParen),
            ')' => self.make_token(TokenType::RightParen),
            '{' => self.make_token(TokenType::LeftBrace),
            '}' => self.make_token(TokenType::RightBrace),
            ',' => self.make_token(TokenType::Comma),
            '.' => self.make_token(TokenType::Dot),
            '-' => self.make_token(TokenType::Minus),
            '+' => self.make_token(TokenType::Plus),
            ';' => self.make_token(TokenType::Semicolon),
            '*' => self.make_token(TokenType::Star),
            '!' => {
                let t = if self.consume_if('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.make_token(t)
            }
            '=' => {
                let t = if self.consume_if('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.make_token(t)
            }
            '<' => {
                let t = if self.consume_if('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.make_token(t)
            }
            '>' => {
                let t = if self.consume_if('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.make_token(t)
            }
            // skip comments or emit a literal '/'
            '/' => {
                if self.consume_if('/') {
                    while let Some(c) = self.peek()
                        && c != '\n'
                    {
                        self.advance();
                    }
                    Err(InternalLexError::SkipToken)
                } else {
                    self.make_token(TokenType::Slash)
                }
            }
            // skip whitespace
            ' ' => Err(InternalLexError::SkipToken),
            '\r' => Err(InternalLexError::SkipToken),
            '\t' => Err(InternalLexError::SkipToken),
            '\n' => {
                self.line += 1;
                Err(InternalLexError::SkipToken)
            }
            // literal
            '"' => self.consume_string_literal(),
            _ => Err(InternalLexError::error(
                self.line,
                format!("Unexpected character {c}"),
            )),
        }
    }

    fn consume_string_literal(&mut self) -> Result<Token> {
        todo!()
    }

    fn consume_if(&mut self, expected: char) -> bool {
        let Some(&c) = self.source.get(self.current) else {
            return false;
        };
        if c != expected {
            return false;
        }

        self.current += 1;
        true
    }

    fn advance(&mut self) -> Result<char> {
        let temp = self
            .source
            .get(self.current)
            .copied()
            .ok_or(InternalLexError::EOF)?;
        self.current += 1;
        Ok(temp)
    }

    fn make_token(&self, kind: TokenType) -> Result<Token> {
        let text = &self.source[self.start..self.current];
        Ok(Token::new(kind, text.into_iter().collect(), self.line))
    }

    fn peek(&self) -> Option<char> {
        self.source.get(self.current).copied()
    }
}
