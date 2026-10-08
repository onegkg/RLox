pub mod token;

use std::{
    collections::HashMap,
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
    fn error(line: usize, message: &str) -> Self {
        Self::Error(LexError {
            line,
            loc: String::from(""),
            message: String::from(message),
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
    keywords: HashMap<&'static str, TokenType>,
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
            keywords: Self::default_keywords_map(),
        }
    }

    fn default_keywords_map() -> HashMap<&'static str, TokenType> {
        let mut m = HashMap::new();
        m.insert("and", TokenType::And);
        m.insert("class", TokenType::Class);
        m.insert("else", TokenType::Else);
        m.insert("false", TokenType::False);
        m.insert("for", TokenType::For);
        m.insert("fun", TokenType::Fun);
        m.insert("if", TokenType::If);
        m.insert("nil", TokenType::Nil);
        m.insert("or", TokenType::Or);
        m.insert("print", TokenType::Print);
        m.insert("return", TokenType::Return);
        m.insert("super", TokenType::Super);
        m.insert("this", TokenType::This);
        m.insert("true", TokenType::True);
        m.insert("var", TokenType::Var);
        m.insert("while", TokenType::While);
        m
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
                Err(InternalLexError::SkipToken) => {
                    self.start = self.current;
                    continue;
                }
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
                        let _ = self.advance();
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
            d if d.is_ascii_digit() => self.consume_number_literal(),
            c if c.is_ascii_alphabetic() || c == '_' => self.consume_identifier(),
            _ => Err(InternalLexError::error(
                self.line,
                &format!("Unexpected character {c}"),
            )),
        }
    }

    fn consume_identifier(&mut self) -> Result<Token> {
        while self
            .peek()
            .is_some_and(|c| c.is_ascii_alphanumeric() || c == '_')
        {
            let _ = self.advance();
        }

        let text: String = self.source[self.start..self.current].iter().collect();
        let kind = self
            .keywords
            .get(&*text)
            .unwrap_or(&TokenType::Identifier)
            .clone();

        self.make_token(kind)
    }

    fn consume_number_literal(&mut self) -> Result<Token> {
        // consume until next current is not a digit
        while let Some(c) = self.peek()
            && c.is_ascii_digit()
        {
            let _ = self.advance();
        }

        if self.peek() == Some('.') && self.peek_next().is_some_and(|d| d.is_ascii_digit()) {
            // consume the `.`
            let _ = self.advance();
        }

        while let Some(c) = self.peek()
            && c.is_ascii_digit()
        {
            let _ = self.advance();
        }

        self.make_token(TokenType::Number({
            let test = self.source[self.start..self.current]
                .iter()
                .collect::<String>();

            test.trim().parse()
                .expect(
                    // TODO: Fix the pseudo-regex
                    &format!("We can only parse literals in the form d+(.d+)?, which are all valid floats. got {test}"),
                    )
        }))
    }

    fn consume_string_literal(&mut self) -> Result<Token> {
        while let Some(c) = self.peek()
            && c != '"'
        {
            if c == '\n' {
                self.line += 1;
            }
            let _ = self.advance();
        }

        if self.is_at_end() {
            return Err(InternalLexError::error(
                self.line,
                "Unterminated string literal.",
            ));
        }

        // consume the closing "
        let _ = self.advance();

        let value = &self.source[(self.start + 1)..(self.current - 1)];
        self.make_token(TokenType::String(value.iter().collect()))
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
        // let start_idx = text
        //     .iter()
        //     .position(|c| !c.is_whitespace())
        //     .unwrap_or(text.len());
        // let end_idx = text
        //     .iter()
        //     .rposition(|c| !c.is_whitespace())
        //     .map_or(start_idx, |i| i + 1);
        //
        // let lexeme: String = text[start_idx..end_idx].iter().collect();
        Ok(Token::new(kind, text.iter().collect(), self.line))
    }

    fn peek(&self) -> Option<char> {
        self.source.get(self.current).copied()
    }

    fn peek_next(&self) -> Option<char> {
        self.source.get(self.current + 1).copied()
    }
}
