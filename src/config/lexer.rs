use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Ident(String),  // key like "git.repo"
    String(String), // "value"
    ListStart,      // [
    ListEnd,        // ]
    Comma,          // ,
    Equals,         // =
    Include,        // include keyword
    Comment,        // //comment ...
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

    /// Look at the next character without consuming it (peek next)
    fn peek_next(&mut self) -> Option<char> {
        let mut chars = self.chars.clone();
        chars.next(); // skip current
        chars.peek().copied()
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
                // Check if it's a // comment
                match self.peek_next() {
                    Some('/') => {
                        // // comment
                        self.next_char(); // consume first '/'
                        self.next_char(); // consume second '/'
                        while let Some(&ch) = self.peek_char() {
                            if ch == '\n' {
                                break;
                            }
                            self.next_char();
                        }
                        Token::Comment
                    }
                    _ => {
                        // Single '/' is not allowed
                        panic!("Unexpected character '/' at {}:{}", self.line, self.col);
                    }
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
                // String literal
                self.next_char(); // skip opening quote
                let mut s = String::new();
                while let Some(&ch) = self.peek_char() {
                    if ch == '"' {
                        self.next_char(); // skip closing quote
                        break;
                    }
                    s.push(ch);
                    self.next_char();
                }
                Token::String(s)
            }
            Some(ch) if ch.is_alphabetic() || *ch == '_' || *ch == '.' => {
                // Identifier (key)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_ident() {
        let mut lexer = Lexer::new("git.repo");
        assert_eq!(lexer.next_token(), Token::Ident("git.repo".to_string()));
        assert_eq!(lexer.next_token(), Token::Eof);
    }

    #[test]
    fn test_string() {
        let mut lexer = Lexer::new("\"hello world\"");
        assert_eq!(lexer.next_token(), Token::String("hello world".to_string()));
        assert_eq!(lexer.next_token(), Token::Eof);
    }

    #[test]
    fn test_comment() {
        let mut lexer = Lexer::new("# this is a comment\nkey = \"value\"");
        assert_eq!(lexer.next_token(), Token::Comment);
        assert_eq!(lexer.next_token(), Token::Ident("key".to_string()));
    }
}
