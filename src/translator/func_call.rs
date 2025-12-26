use std::collections::HashMap;
use once_cell::sync::Lazy;
use crate::translator::intermediate_language::IR;
use crate::translator::memory_manager::MemoryManager;
use crate::translator::tokenizer::{BasicType, Expr};

type Function = fn(&mut MemoryManager) -> Result<Vec<IR>, String>;

#[derive(Clone)]
struct FunctionSymbol {
    args: Vec<BasicType>,
    return_type: Option<BasicType>,
    func: Function,
}

static FUNCTION_SYMBOL_TABLE: Lazy<HashMap<&String, FunctionSymbol>> = Lazy::new(||
    HashMap::from([

    ])
);

pub fn translate_function_call(name: &String, memory_manager: &mut MemoryManager) -> Result<Vec<IR>, String> {
    let fs;
    match &FUNCTION_SYMBOL_TABLE.get(name) {
        Some(symbol) => { fs = symbol.func;},
        None => {unreachable!()}  // bc we check for it in `validate_args` tho we could just copy & paste it from there
    }
    fs(memory_manager)
}

pub fn get_type(arg: &Expr, memory_manager: &MemoryManager) -> Result<BasicType, String> {
    match arg {
        Expr::Integer { .. } => { Ok(BasicType::Integer) }
        Expr::String { .. } => { Ok(BasicType::Char) }
        Expr::VariableName { name } => {
            match memory_manager.get_var(name.clone()) {
                None => { Err(format!("Variable '{}' not found", name)) }
                Some(value) => { Ok(value.var_type) }
            }
        }
        Expr::FunctionCall { name, .. } => {
            match FUNCTION_SYMBOL_TABLE.get(name) {
                None => { Err(format!("Function '{}' not found", name)) }
                Some(fs) => {
                    match fs.return_type {
                        None => { Err(format!("Function '{}' returns nothing so it can't be taken as an argument", name)) }
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

pub fn validate_args(name: &String, args: &[Expr], memory_manager: &MemoryManager) -> Result<(), String> {
    let expected = &FUNCTION_SYMBOL_TABLE.get(name).ok_or_else(|| format!("Function '{}' not found", name))?.args;
    let got: Vec<BasicType> = args
        .iter()
        .map(|arg| get_type(arg, memory_manager))
        .collect::<Result<Vec<_>, _>>()?;

    if expected == &got {
        Ok(())
    } else {
        Err(format!("Invalid values passed to the function '{}'", name))
    }
}
