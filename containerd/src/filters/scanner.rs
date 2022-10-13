use std::{iter::Peekable, str::CharIndices};

pub enum Token {
    EOF,
    Quoted,
    Value,
    Field,
    Separator,
    Operator,
    Illegal,
}

impl Token {
    fn as_str<'a>(self) -> &'a str {
        match self {
            Token::EOF => return "EOF",
            Token::Quoted => return "Quoted",
            Token::Value => return "Value",
            Token::Field => return "Field",
            Token::Separator => return "Separator",
            Token::Operator => return "Operator",
            Token::Illegal => return "Illegal",
        }
    }

    fn to_string(self) -> String {
        return "token".to_owned() + self.as_str();
    }
}

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

    fn next(&mut self) -> Option<char> {
        if self.pos >= self.input.chars().count() {
            return None;
        };
        self.pos = self.ppos;
        // we are keeping track of the next
        // so unwrap assumed to be safe.
        let (s, c) = self.iter.next().unwrap();
        self.ppos += s;
        Some(c)
    }

    fn peek(&mut self) -> Option<char> {
        let pos = self.pos;
        let ppos = self.ppos;

        let ch = self.next();

        self.pos = pos;
        self.ppos = ppos;
        ch
    }

    fn scan<'b>(&mut self) -> (usize, Token, &'b str) {
        let pos: usize = self.pos;

        let ch = match self.next() {
            Some(c) => c,
            None => return (self.pos, Token::EOF, ""),
        };

        if is_quote_token(ch) {
            match self.scan_quoted(ch) {
                Some(b) => {
                    if !b {
                        match self.input.get(self.pos..self.ppos) {
                            Some(c) => todo!(),//return (pos, Token::Illegal, c),
                            None => {}
                        }
                    }
                }
                None => {}
            }
            todo!()
        }

        (self.pos, Token::EOF, "")
    }

    fn scan_field(mut self) -> Option<()> {
        loop {
            match self.peek() {
                Some(c) => {
                    if !is_field_token(c.clone()) {
                        break;
                    }
                }
                _ => {
                    self.err = "iterater next returned None";
                    return None;
                }
            };
            match self.next() {
                Some(_) => {}
                _ => {
                    self.err = "iterater next returned None";
                    return None;
                }
            };
        }
        Some(())
    }

    fn scan_operator(mut self) -> Option<()> {
        loop {
            match self.peek() {
                Some(c) => {
                    match c.clone() {
                        '=' | '!' | '~' => self.next().unwrap(),
                        _ => {
                            self.err = "iterater next returned None";
                            return None;
                        }
                    };
                }
                _ => {
                    self.err = "iterater next returned None";
                    return None;
                }
            };
        }
    }

    fn scan_value(mut self) -> Option<()> {
        loop {
            match self.peek() {
                Some(c) => {
                    if !is_value_token(c.clone()) {
                        break;
                    }
                }
                _ => {
                    self.err = "iterater next returned None";
                    return None;
                }
            };

            match self.next() {
                Some(_) => {}
                _ => {
                    self.err = "iterater next returned None";
                    return None;
                }
            }
        }
        Some(())
    }

    fn scan_quoted(&mut self, quote: char) -> Option<bool> {
        let mut illegal = false;
        let mut ch = match self.next() {
            Some(c) => c,
            _ => {
                self.err = "iterater next returned None";
                return None;
            }
        };
        while ch != quote {
            if ch == '\n' || ch < '0' {
                self.err = "quoted literal not terminated";
                return None;
            };
            if ch == '\\' {
                let mut legal = false;
                (ch, legal) = match self.scan_escape(quote) {
                    Some(c) => c,
                    None => return None,
                };
                if !illegal {
                    illegal = true
                }
            } else {
                ch = match self.next() {
                    Some(c) => c,
                    _ => return None,
                };
            }
        }
        Some(!illegal)
    }

    fn scan_escape(&mut self, quote: char) -> Option<(char, bool)> {
        let mut legal: bool = false;
        let mut ch = match self.next() {
            Some(c) => c,
            _ => {
                self.err = "iterater next returned None";
                return None;
            }
        };

        match ch {
            'a' | 'b' | 'f' | 'n' | 'r' | 't' | 'v' | '\\' => {
                ch = match self.next() {
                    Some(c) => c,
                    _ => {
                        self.err = "iterater next returned None";
                        return None;
                    }
                };
                legal = true;
            }
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' => return self.scan_digits(ch, 8, 3),
            'x' => match self.next() {
                Some(c) => return self.scan_digits(c, 16, 2),
                None => return None,
            },
            'u' => match self.next() {
                Some(c) => return self.scan_digits(c, 16, 4),
                None => return None,
            },
            'U' => match self.next() {
                Some(c) => return self.scan_digits(c, 16, 8),
                None => return None,
            },
            quote if quote == ch => {
                ch = match self.next() {
                    Some(c) => c,
                    _ => {
                        self.err = "iterater next returned None";
                        return None;
                    }
                };
                legal = true;
            }
            _ => self.err = "illegal escape sequence",
        }
        Some((ch, legal))
    }

    fn scan_digits(&mut self, ch: char, base: u64, n: u64) -> Option<(char, bool)> {
        let mut chi = ch;
        let mut ni = n;
        while ni > 0 && digit_val(ch) < base {
            chi = match self.next() {
                Some(c) => c,
                _ => {
                    self.err = "iterater next returned None";
                    return None;
                }
            };
            ni = ni - 1;
        }

        if ni > 0 {
            self.err = "illegal numeric escape sequence";
            return Some((chi, false));
        }
        Some((chi, true))
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
