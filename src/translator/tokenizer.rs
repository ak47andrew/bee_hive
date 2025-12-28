use std::fmt::Debug;
use fancy_regex::Regex;
use once_cell::sync::Lazy;
use crate::translator::func_call::get_function_symbol;
use crate::translator::memory_manager::MemoryManager;

#[derive(Copy, Clone, PartialEq)]
pub enum BasicType {
    Integer,
    Char,
    Any,
    Void
}

#[derive(Debug, Clone)]
pub enum Expr {
    Integer { value: u8 },
    String { value: String },
    VariableName { name: String },
    FunctionCall { name: String, args: Vec<Expr> },
    Variable { name: String, value: Box<Expr> },
}

impl Expr {
    pub fn get_type(&self, memory_manager: &MemoryManager) -> Result<BasicType, String> {
        match self {
            Expr::Integer { .. } => { Ok(BasicType::Integer) }
            Expr::String { .. } => { Ok(BasicType::Char) }
            Expr::VariableName { name } => {
                match memory_manager.get_var(name.clone()) {
                    None => { Err(format!("Variable '{}' not found", name)) }
                    Some(value) => { Ok(value.var_type) }
                }
            }
            Expr::FunctionCall { name, .. } => {
                match get_function_symbol(name.as_str()) {
                    None => { Err(format!("Function '{}' not found", name)) }
                    Some(fs) => {
                        match fs.return_type {
                            None => { Ok(BasicType::Void) }
                            Some(ty) => { Ok(ty) }
                        }
                    }
                }
            }
            Expr::Variable { .. } => {
                Err("Are you trying to... assign a variable as an argument? What in hell do you think you're doing???".to_string())
            }
        }
    }
}


pub static INTEGER_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(\d+)$").unwrap());
pub static STRING_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r#"^"([^"]*)"$"#).unwrap());
pub static VARIABLE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^var\s+(.+)\s*=\s*(.+)$").unwrap());
pub static FUNC_CALL_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\s*([\w_]+)\s*\(\s*(.*)\s*\)\s*$").unwrap());

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

const KEYWORDS: &[&str] = &[
    "if", "else", "while", "for", "fn", "return", "let", "var",
];

pub fn is_valid_identifier(s: &str) -> bool {
    if KEYWORDS.contains(&s) {
        return false;
    }

    let mut chars = s.chars();

    let first = match chars.next() {
        Some(c) => c,
        None => return false,
    };

    if !first.is_ascii_alphabetic() && first != '_' {
        return false;
    }

    chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}


pub fn tokenize(statement: String) -> Result<Expr, String> {
    if let Ok(Some(captures)) = INTEGER_REGEX.captures(&statement) {
        return match captures.get(1).unwrap().as_str().parse::<u8>()
        {
            Ok(value) => Ok(Expr::Integer { value }),
            Err(_) => Err(format!("Integer overflow: {}; currently integers are only supported in the 0-255 range", statement))
        };
    }
    if let Ok(Some(captures)) = STRING_REGEX.captures(&statement) {
        return Ok(Expr::String {value: captures.get(1).unwrap().as_str().to_string().replace("\\n", "\n")});
    }
    if let Ok(Some(captures)) = VARIABLE_REGEX.captures(&statement) {
        let name = captures.get(1).unwrap().as_str().strip_suffix(' ').unwrap();
        if !is_valid_identifier(name) {
            return Err(format!("variable name {} should be a valid identifier", name));
        }

        let var_value = tokenize(captures.get(2).unwrap().as_str().to_string())?;
        return match var_value {
            Expr::Integer { .. } => Ok(Expr::Variable {
                name: name.to_string(),
                value: Box::new(var_value)
            }),
            _ => Err("Currently only numbers are supported for storing in variables".to_string()),
        }
    }
    if let Ok(Some(captures)) = FUNC_CALL_REGEX.captures(&statement) {
        let name = captures.get(1).unwrap().as_str().to_string();
        if !is_valid_identifier(name.as_str()) {
            return Err(format!("function name {} should be a valid identifier", name));
        }

        let parsed_args = split_args(captures.get(2).unwrap().as_str().to_string());
        let mut args = vec![];
        for arg in parsed_args {
            match tokenize(arg) {
                Ok(value) => args.push(value),
                Err(err) => return Err(err),
            }
        }

        return Ok(Expr::FunctionCall {name, args});
    }

    if !is_valid_identifier(&statement) {
        Err(format!("Can't parse statement: {}", statement))
    } else {
        Ok(Expr::VariableName {name: statement})
    }
}
