use crate::error::{Error, ErrorKind, Result};
use std::{iter::Peekable, str::CharIndices};

#[derive(Clone, Copy)]
#[repr(i8)]
pub enum Token {
    EOF,
    Quoted,
    Value,
    Field,
    Separator,
    Operator,
    Illegal,
}

impl From<u8> for Token {
    fn from(item: u8) -> Self {
        use Token::*;
        match item {
            0 => return EOF,
            1 => return Quoted,
            2 => return Value,
            3 => return Field,
            4 => return Separator,
            5 => return Operator,
            _ => return Illegal
        }
    }
}

impl From<char> for Token {
    fn from(item: char) -> Self {
        use Token::*;
        match item as u8 {
            0 => return EOF,
            1 => return Quoted,
            2 => return Value,
            3 => return Field,
            4 => return Separator,
            5 => return Operator,
            _ => return Illegal,
        }
    }
}

impl Into<char> for Token {
    fn into(self) -> char {
        self as u8 as char
    }
}

impl Token {
    fn as_str<'a>(self) -> &'static str {
        use Token::*;
        match self {
            EOF => return "EOF",
            Quoted => return "Quoted",
            Value => return "Value",
            Field => return "Field",
            Separator => return "Separator",
            Operator => return "Operator",
            Illegal => return "Illegal",
        }
    }

    fn to_string(self) -> String {
        return "token".to_owned() + self.as_str();
    }
}

#[derive(Clone)]
struct Scanner<'a> {
    input: &'a str,
    iter: Peekable<CharIndices<'a>>,
    pos: usize,
    ppos: usize,
    value: bool,
    err: &'a str,
}

impl<'a> Scanner<'a> {
    fn from_string<'b>(input: &'b str) -> Scanner<'b> {
        Scanner {
            input,
            iter: input.char_indices().peekable(),
            pos: 0,
            ppos: 0,
            value: false,
            err: "",
        }
    }

    fn next<'b>(&'b mut self) -> char {
        let (pos, ch) = match self.iter.next() {
            Some(c) => c,
            None => return Token::EOF.into(),
        };
        self.pos += pos;

        ch
    }

    fn peek(&mut self) -> char {
        match self.iter.peek() {
            Some((_, c)) => c.clone(),
            None => Token::EOF.into(),
        }
    }

    fn scan(&mut self) -> (usize, Token, &'a str) {
        let mut pos = self.pos;

        // skip all whitespackes
        while self.peek().is_whitespace() {
            self.next();
            pos = self.pos;
        }

        let ch = self.next();
        pos = self.pos;

        if is_quote_token(ch) {
            if !self.scan_quoted(ch) {
                // this shouldn't error out. pos and ppos will not exceed the
                // total length
                let slice = self.input.get(self.pos..self.ppos).unwrap();
                return (pos, Token::Illegal, slice);
            } else if is_separator_token(ch) {
                self.value = false;
                let slice = self.input.get(self.pos..self.ppos).unwrap();
                return (pos, Token::Separator, slice);
            } else if is_operator_token(ch) {
                self.scan_operator();
                self.value = true;
                let slice = self.input.get(self.pos..self.ppos).unwrap();
                return (pos, Token::Operator, slice);
            } else if is_field_token(ch) {
                self.scan_field();
                let slice = self.input.get(self.pos..self.ppos).unwrap();
                return (pos, Token::Field, slice);
            }
        }

        (self.pos, Token::from(ch), "")
    }

    fn scan_field(&mut self) {
        loop {
            let ch = self.peek();
            if !is_field_token(ch) {
                break;
            }
            self.next();
        }
    }

    fn scan_operator(&mut self) {
        loop {
            let ch = self.peek();
            match ch {
                '=' | '!' | '~' => self.next(),
                _ => return,
            };
        }
    }

    fn scan_value(mut self) {
        loop {
            let ch = self.peek();
            if !is_value_token(ch) {
                break;
            }

            self.next();
        }
    }

    fn scan_quoted(&mut self, quote: char) -> bool {
        let mut illegal = false;
        let mut ch = self.next(); // read character after quote

        while ch != quote {
            if ch == '\n' || ch as u8 <= 0 {
                self.err = "quoted literal not terminated";
                return false;
            };
            if ch == '\\' {
                let mut legal = false;
                (ch, legal) = self.scan_escape(quote);
                if !illegal {
                    illegal = true
                }
            } else {
                ch = self.next();
            }
        }
        !illegal
    }

    fn scan_escape(&mut self, quote: char) -> (char, bool) {
        let mut ch = self.next();
        match ch {
            'a' | 'b' | 'f' | 'n' | 'r' | 't' | 'v' | '\\' => (self.next(), true),
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' => self.scan_digits(ch, 8, 3),
            'x' => self.scan_digits(ch, 16, 2),
            'u' => {
                ch = self.next();
                self.scan_digits(ch, 16, 4)
            }
            'U' => {
                ch = self.next();
                self.scan_digits(ch, 16, 8)
            }
            quote if quote == ch => (self.next(), true),
            _ => {
                self.err = "illegal escape sequence";
                (self.next(), true)
            }
        }
    }

    fn scan_digits<'b>(&'b mut self, ch: char, base: u64, n: u64) -> (char, bool) {
        let mut chi = ch;
        let mut ni = n;
        while ni > 0 && digit_val(ch) < base {
            chi = self.next();
            ni = ni - 1;
        }

        if ni > 0 {
            self.err = "illegal numeric escape sequence";
            return (chi, false);
        }
        return (chi, true);
    }
}

fn digit_val(ch: char) -> u64 {
    if '0' <= ch && ch <= '9' {
        return ch as u64 - '0' as u64;
    } else if 'a' <= ch && ch <= 'f' {
        return ch as u64 - ('a' as u64 + 10);
    } else if 'A' <= ch && ch <= 'F' {
        return ch as u64 - ('A' as u64 + 10);
    };
    16 // larger than any legal digit val
}

fn is_field_token(ch: char) -> bool {
    ch == '_' || is_alpha_token(ch) || is_digit_token(ch)
}

fn is_alpha_token(ch: char) -> bool {
    ch >= 'A' && ch <= 'Z' || ch >= 'a' && ch <= 'z'
}

fn is_digit_token(ch: char) -> bool {
    ch >= '0' && ch <= '9'
}

fn is_operator_token(ch: char) -> bool {
    match ch {
        '=' | '!' | '~' => return true,
        _ => {} // ignore (default false)
    }
    false
}

fn is_quote_token(ch: char) -> bool {
    match ch {
        '/' | '|' | '"' => return true,
        _ => {} // ignore (default false)
    }
    false
}

fn is_separator_token(ch: char) -> bool {
    match ch {
        ',' | '.' => return true,
        _ => {} // ignore (default false)
    }
    false
}

fn is_value_token(ch: char) -> bool {
    ch != ','
        && !ch.is_ascii_whitespace()
        && (ch.is_ascii_alphanumeric() || ch.is_ascii_graphic() || ch.is_ascii_punctuation())
}
