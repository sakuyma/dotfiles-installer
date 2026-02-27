use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Ident(String),
    String(String),
    ListStart,
    ListEnd,
    Comma,
    Equals,
    Include,
    Comment,
    Newline,
    Eof,
}

pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
    pub line: usize,
    pub col: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            chars: input.chars().peekable(),
            line: 1,
            col: 1,
        }
    }

    fn next_char(&mut self) -> Option<char> {
        let ch = self.chars.next();
        if let Some(c) = ch {
            if c == '\n' {
                self.line += 1;
                self.col = 1;
            } else {
                self.col += 1;
            }
        }
        ch
    }

    fn peek_char(&mut self) -> Option<&char> {
        self.chars.peek()
    }

    fn skip_whitespace(&mut self) {
        while let Some(&ch) = self.peek_char() {
            if ch.is_whitespace() && ch != '\n' {
                self.next_char();
            } else {
                break;
            }
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        match self.peek_char() {
            None => Token::Eof,
            Some('\n') => {
                self.next_char();
                Token::Newline
            }
            Some('/') => {
                if let Some('/') = self.peek_next() {
                    self.next_char();
                    self.next_char();

                    while let Some(&ch) = self.peek_char() {
                        if ch == '\n' {
                            break;
                        }
                        self.next_char();
                    }
                    Token::Comment
                } else {
                    panic!("Unexpected character '/' at {}:{}", self.line, self.col);
                }
            }
            Some('[') => {
                self.next_char();
                Token::ListStart
            }
            Some(']') => {
                self.next_char();
                Token::ListEnd
            }
            Some(',') => {
                self.next_char();
                Token::Comma
            }
            Some('=') => {
                self.next_char();
                Token::Equals
            }
            Some('"') => {
                self.next_char(); // открывающая кавычка
                let mut s = String::new();
                while let Some(&ch) = self.peek_char() {
                    if ch == '"' {
                        self.next_char();
                        break;
                    }
                    s.push(ch);
                    self.next_char();
                }
                Token::String(s)
            }
            Some(ch) if ch.is_alphabetic() || *ch == '_' || *ch == '.' => {
                let mut ident = String::new();
                while let Some(&ch) = self.peek_char() {
                    if ch.is_alphanumeric() || ch == '_' || ch == '.' {
                        ident.push(ch);
                        self.next_char();
                    } else {
                        break;
                    }
                }
                if ident == "include" {
                    Token::Include
                } else {
                    Token::Ident(ident)
                }
            }
            _ => {
                panic!("Unexpected character at {}:{}", self.line, self.col);
            }
        }
    }
}
