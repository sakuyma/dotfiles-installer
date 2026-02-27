use super::lexer::{Lexer, Token};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    String(String),
    List(Vec<String>),
}

pub struct Parser {
    includes_processed: HashSet<PathBuf>,
}

impl Parser {
    pub fn new() -> Self {
        Parser {
            includes_processed: HashSet::new(),
        }
    }

    /// Parse a configuration file and return a map of keys to values
    pub fn parse_file(&mut self, path: &Path) -> Result<HashMap<String, Value>, String> {
        let canonical = path
            .canonicalize()
            .map_err(|e| format!("Cannot canonicalize path {}: {}", path.display(), e))?;

        if self.includes_processed.contains(&canonical) {
            return Err(format!("Circular include detected: {}", path.display()));
        }
        self.includes_processed.insert(canonical.clone());

        let content = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read {}: {}", path.display(), e))?;

        let mut lexer = Lexer::new(&content);
        let mut assignments = HashMap::new();
        let mut includes = Vec::new();

        loop {
            match lexer.next_token() {
                Token::Eof => break,
                Token::Newline | Token::Comment => continue,
                Token::Include => {
                    // include = [ ... ]
                    if lexer.next_token() != Token::Equals {
                        return Err(format!(
                            "Expected '=' after include at {}:{}",
                            lexer.line, lexer.col
                        ));
                    }
                    let value = self.parse_value(&mut lexer)?;
                    match value {
                        Value::List(paths) => includes.extend(paths),
                        _ => {
                            return Err(format!(
                                "Include must be a list at {}:{}",
                                lexer.line, lexer.col
                            ));
                        }
                    }
                }
                Token::Ident(key) => {
                    if lexer.next_token() != Token::Equals {
                        return Err(format!(
                            "Expected '=' after key '{}' at {}:{}",
                            key, lexer.line, lexer.col
                        ));
                    }
                    let value = self.parse_value(&mut lexer)?;
                    assignments.insert(key, value);
                }
                _ => return Err(format!("Unexpected token at {}:{}", lexer.line, lexer.col)),
            }
        }

        // Process includes recursively
        for include_path in includes {
            let base_dir = path.parent().unwrap_or(Path::new("."));
            let full_path = base_dir.join(include_path);
            let sub_config = self.parse_file(&full_path)?;

            // Merge sub-config, with main file taking precedence
            for (k, v) in sub_config {
                assignments.entry(k).or_insert(v);
            }
        }

        Ok(assignments)
    }

    /// Parse a value (string or list) from the lexer
    fn parse_value(&self, lexer: &mut Lexer) -> Result<Value, String> {
        match lexer.next_token() {
            Token::String(s) => Ok(Value::String(s)),
            Token::ListStart => {
                let mut items = Vec::new();
                let mut last_was_comma = false;

                loop {
                    let token = lexer.next_token();

                    match token {
                        Token::String(s) => {
                            items.push(s);
                            last_was_comma = false;
                        }
                        Token::Comma => {
                            if last_was_comma {
                                return Err(format!(
                                    "Double comma in list at {}:{}",
                                    lexer.line, lexer.col
                                ));
                            }
                            last_was_comma = true;
                        }
                        Token::ListEnd => {
                            // Allow trailing comma (just ignore it)
                            break;
                        }
                        Token::Newline | Token::Comment => continue,
                        _ => {
                            return Err(format!(
                                "Unexpected token in list at {}:{} (got {:?})",
                                lexer.line, lexer.col, token
                            ));
                        }
                    }
                }
                Ok(Value::List(items))
            }
            _ => Err(format!("Expected value at {}:{}", lexer.line, lexer.col)),
        }
    }
}
