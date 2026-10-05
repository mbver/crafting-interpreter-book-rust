use std::fmt;
use std::iter::Peekable;
use std::str::Chars;

pub struct Scanner<'a> {
    reader: CharReader<'a>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

struct CharReader<'a> {
    source: &'a str,
    chars: Peekable<Chars<'a>>,
}

pub enum ScanError {
    UnexpectedCharacter { line: usize, character: char },
}
impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            reader: CharReader {
                source: source,
                chars: source.chars().peekable(),
            },
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 0,
        }
    }
    pub fn scan_tokens(&mut self) -> Result<(), ScanError> {
        while self.scan_token()? {
            self.start = self.current;
        }

        self.tokens.push(Token {
            token_type: TokenType::EOF,
            lexeme: "".to_string(),
            literal: Literal::Null,
            line: self.line,
        });
        Ok(())
    }

    fn advance(&mut self) -> Option<char> {
        let c = self.reader.chars.next()?;
        self.current += c.len_utf8();
        Some(c)
    }
    fn match_char(&mut self, expected: char) -> bool {
        if self.reader.chars.peek() == Some(&expected) {
            self.advance();
            return true;
        }
        false
    }
    fn match_and_add(&mut self, expected: char, matched: TokenType, unmatched: TokenType) {
        let token_type = if self.match_char(expected) {
            matched
        } else {
            unmatched
        };
        self.add_simple_token(token_type);
    }
    fn add_token(&mut self, token_type: TokenType, literal: Literal) {
        let lexeme = self.reader.source[self.start..self.current].to_string();
        self.tokens
            .push(Token::new(token_type, lexeme, literal, self.line));
    }
    fn add_simple_token(&mut self, token_type: TokenType) {
        self.add_token(token_type, Literal::Null);
    }
    // return false at EOF
    pub fn scan_token(&mut self) -> Result<bool, ScanError> {
        let Some(ch) = self.advance() else {
            return Ok(false);
        };
        match ch {
            '(' => self.add_simple_token(TokenType::LeftParen),
            ')' => self.add_simple_token(TokenType::RightParen),
            '{' => self.add_simple_token(TokenType::LeftBrace),
            '}' => self.add_simple_token(TokenType::RightBrace),
            ',' => self.add_simple_token(TokenType::Comma),
            '.' => self.add_simple_token(TokenType::Dot),
            '-' => self.add_simple_token(TokenType::Minus),
            '+' => self.add_simple_token(TokenType::Plus),
            ';' => self.add_simple_token(TokenType::SemiColon),
            '*' => self.add_simple_token(TokenType::Star),
            '!' => self.match_and_add('=', TokenType::BangEqual, TokenType::Equal),
            '=' => self.match_and_add('=', TokenType::EqualEqual, TokenType::Equal),
            '<' => self.match_and_add('=', TokenType::LessEqual, TokenType::Less),
            '>' => self.match_and_add('=', TokenType::GreaterEqual, TokenType::Greater),
            '/' => {
                if self.match_char('/') {
                    while let Some(&ch) = self.reader.chars.peek()
                        && ch != '\n'
                    {
                        self.advance();
                    }
                } else {
                    self.add_simple_token(TokenType::Slash);
                }
            }
            ch => {
                return Err(ScanError::UnexpectedCharacter {
                    line: self.line,
                    character: ch,
                });
            }
        };
        Ok(true) // continue
    }
}

#[derive(Debug)]
pub enum Literal {
    Number(f64),
    String(String),
    Boolean(bool),
    Null,
}

#[derive(Debug)]
pub enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    SemiColon,
    Slash,
    Star,
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Identifier,
    String,
    Number,
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    EOF,
}

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Literal,
    line: usize,
}

impl Token {
    fn new(token_type: TokenType, lexeme: String, literal: Literal, line: usize) -> Self {
        Self {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?} {} {:?}",
            self.token_type, self.lexeme, self.literal
        )
    }
}
