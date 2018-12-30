use super::tokens::TokenType::*;
use super::tokens::{Token, TokenType};
use itertools::{multipeek, MultiPeek};
use std::fmt::Display;
use std::mem;
use std::str::Chars;

pub struct Scanner<'a> {
    source: MultiPeek<Chars<'a>>,
    start_line: usize,
    end_line: usize,
    start_char: usize,
    end_char: usize,
    errors: Vec<String>,
    literal: String,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Scanner {
            source: multipeek(source.chars()),
            start_line: 1,
            end_line: 1,
            start_char: 0,
            end_char: 0,
            errors: vec![],
            literal: String::new(),
        }
    }

    pub fn tokenize(&mut self) -> (Vec<Token>, &[String]) {
        let tokens = self.collect();
        (tokens, &self.errors)
    }

    fn next_token(&mut self) -> Option<Token> {
        self.begin_token();

        match self.source.next() {
            Some('(') => self.token(LEFT_PAREN),
            Some(')') => self.token(RIGHT_PAREN),
            Some('{') => self.token(LEFT_BRACE),
            Some('}') => self.token(RIGHT_BRACE),
            Some('.') => self.token(DOT),
            Some(',') => self.token(COMMA),
            Some('-') => self.token(HYPHEN),
            Some('+') => self.token(PLUS),
            Some(';') => self.token(SEMICOLON),
            Some('*') => self.token(ASTERICS),
            Some('!') => self.match_equal(BANG, BANG_EQUAL),
            Some('=') => self.match_equal(EQUAL, EQUAL_EQUAL),
            Some('<') => self.match_equal(LESS, LESS_EQUAL),
            Some('>') => self.match_equal(GREATER, GREATER_EQUAL),
            Some('/') => self.scan_comment(),
            Some(' ') | Some('\r') | Some('\t') => self.next_token(),
            Some('"') => self.scan_string(),
            Some(ch @ '0'...'9') => self.scan_number(ch),
            Some(ch) if is_alpha(ch) => self.scan_identifier(ch),
            Some('\n') => {
                self.newline();
                self.next_token()
            }
            Some(ch) => {
                self.push_error(
                    self.start_line,
                    self.start_char,
                    format!("Unknown token: '{}'", ch),
                );
                self.next_token()
            }
            None => None,
        }
    }

    fn token(&self, token_type: TokenType) -> Option<Token> {
        Some(Token::new(token_type, self.start_line, self.start_char))
    }

    fn match_equal(&mut self, failure: TokenType, success: TokenType) -> Option<Token> {
        let token_type = if Some(true) == match_next!(self.source, '=') {
            self.end_char += 1;
            success
        } else {
            failure
        };

        self.token(token_type)
    }

    fn scan_comment(&mut self) -> Option<Token> {
        if Some(true) == match_next!(self.source, '/') {
            self.end_char += 1;
            while Some(false) == match_next!(self.source, '\n') {
                self.source.next();
            }
            self.newline();
            self.next_token()
        } else {
            self.token(SLASH)
        }
    }

    fn scan_string(&mut self) -> Option<Token> {
        while Some(false) == check_next!(self.source, '"') {
            if let Some(ch) = self.source.next() {
                self.put_char(ch);
                if ch == '\n' {
                    self.newline();
                }
            }
        }

        match self.source.next() {
            None => {
                self.push_error(self.start_line, self.start_char, "Unterminated string.");
                None
            }
            Some(_) => {
                self.end_char += 1;
                let token_type = STRING(self.get_literal());
                self.token(token_type)
            }
        }
    }

    fn scan_number(&mut self, first_digit: char) -> Option<Token> {
        self.literal.push(first_digit);

        while check_func!(self.source, is_digit) {
            if let Some(digit) = self.source.next() {
                self.put_char(digit);
            }
        }
        self.source.reset_peek();

        if Some(true) == check_next!(self.source, '.') && check_func!(self.source, is_digit) {
            if let Some(ch) = self.source.next() {
                self.put_char(ch);
            }
            while check_func!(self.source, is_digit) {
                if let Some(digit) = self.source.next() {
                    self.put_char(digit)
                }
            }
        }

        let token_type = NUMBER(self.get_literal().parse().unwrap());
        self.token(token_type)
    }

    fn scan_identifier(&mut self, ch: char) -> Option<Token> {
        self.literal.push(ch);

        while check_func!(self.source, is_alpha, is_digit) {
            if let Some(ch) = self.source.next() {
                self.put_char(ch);
            }
        }

        let token_type = map_token(self.get_literal());
        self.token(token_type)
    }

    fn begin_token(&mut self) {
        self.start_line = self.end_line;
        self.start_char = self.end_char + 1;
        self.end_char = self.start_char;
    }

    fn newline(&mut self) {
        self.end_line += 1;
        self.end_char = 0;
    }

    fn put_char(&mut self, ch: char) {
        self.literal.push(ch);
        self.end_char += 1;
    }

    fn get_literal(&mut self) -> String {
        mem::replace(&mut self.literal, String::new())
    }

    fn push_error<T: Display>(&mut self, line: usize, offset: usize, message: T) {
        self.errors
            .push(format!("L{}:{} {}", line, offset, message));
    }
}

impl<'a> Iterator for Scanner<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

fn is_alpha(ch: char) -> bool {
    ('a' <= ch && ch <= 'z') || ('A' <= ch && ch <= 'Z') || ch == '_'
}

fn is_digit(ch: char) -> bool {
    '0' <= ch && ch <= '9'
}

fn map_token(string: String) -> TokenType {
    match string.as_str() {
        "and" => AND,
        "class" => CLASS,
        "else" => ELSE,
        "false" => FALSE,
        "for" => FOR,
        "fun" => FUN,
        "if" => IF,
        "nil" => NIL,
        "or" => OR,
        "print" => PRINT,
        "return" => RETURN,
        "super" => SUPER,
        "this" => THIS,
        "true" => TRUE,
        "var" => VAR,
        "while" => WHILE,
        _ => IDENTIFIER(string),
    }
}
