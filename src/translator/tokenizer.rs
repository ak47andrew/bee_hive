use std::fmt;
use std::fmt::{Display, Formatter};
use fancy_regex::Regex;
use once_cell::sync::Lazy;

pub enum Type {
    Integer,
    String,
    Bool,
    Void,
    Function {arg_types: Vec<Type>, return_type: Box<Type>}
}

#[derive(Debug)]
pub enum Expr {
    Integer {value: u8},
    String {value: String},
    Identifier {name: String},
    FunctionCall {name: String, args: Vec<Expr>},
}

#[derive(Debug)]
pub enum TokenizerError {
    UnrecognizedToken(String),
    IntegerOverflow(String),
}

pub static INTEGER_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(\d+)$").unwrap());
pub static STRING_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r#"^"([^"]*)"$"#).unwrap());
pub static VARIABLE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^var\s+(.+)\s*=\s*(.+)$").unwrap());
pub static OBJECT_CALL_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\s*([\w_]+)\s*\(\s*(.*)\s*\)\s*$").unwrap());

fn split_args(s: String) -> Vec<String> {
    let escaped_chars = vec!['n'];

    let mut parts: Vec<String> = vec![];
    let mut current: Vec<String> = vec![];
    let mut is_string = false;
    let mut escaped = false;

    for c in s.as_str().chars() {
        if escaped {
            if escaped_chars.contains(&c) {
                current.push('\\'.to_string());
            }

            current.push(c.to_string());
            escaped = false;
        } else if c == '\\' {
            escaped = true;
        } else if c == '\'' {
            is_string = !is_string;
            current.push(c.to_string());
        } else if c == ',' && !is_string {
            parts.push(current.join(""));
            current.clear();
        } else {
            current.push(c.to_string());
        }
    }

    if !current.is_empty() {
        parts.push(current.join(""));
    }

    parts
}

pub fn tokenize(statement: String) -> Result<Expr, TokenizerError> {
    if let Ok(Some(captures)) = INTEGER_REGEX.captures(&statement) {
        return match captures.get(1).unwrap().as_str().parse::<u8>()
        {
            Ok(value) => Ok(Expr::Integer { value }),
            Err(_) => Err(TokenizerError::IntegerOverflow(statement.clone()))
        };
    }
    if let Ok(Some(captures)) = STRING_REGEX.captures(&statement) {
        return Ok(Expr::String {value: captures.get(1).unwrap().as_str().to_string().replace("\\n", "\n")});
    }
    if let Ok(Some(captures)) = VARIABLE_REGEX.captures(&statement) {
        return Err(TokenizerError::UnrecognizedToken("variables aren't ready yet, wait a bit pls".to_string()));
    }
    if let Ok(Some(captures)) = OBJECT_CALL_REGEX.captures(&statement) {
        let parsed_args = split_args(captures.get(2).unwrap().as_str().to_string());
        let mut args = vec![];
        for arg in parsed_args {
            match tokenize(arg) {
                Ok(value) => args.push(value),
                Err(err) => return Err(err),
            }
        }

        return Ok(Expr::FunctionCall {name: captures.get(1).unwrap().as_str().to_string(), args});
    }

    Err(TokenizerError::UnrecognizedToken(statement))
}
