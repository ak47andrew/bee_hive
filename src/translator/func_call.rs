use std::collections::HashMap;
use once_cell::sync::Lazy;
use crate::translator::intermediate_language::{get_stack_free_index, get_stack_last_index, IR};
use crate::translator::memory_manager::MemoryManager;
use crate::translator::tokenizer::{BasicType, Expr};

type Function = fn(&mut MemoryManager) -> Result<Vec<IR>, String>;

#[derive(Clone)]
pub struct FunctionSymbol {
    pub args: Vec<BasicType>,
    pub return_type: Option<BasicType>,
    pub func: Function,
}

static FUNCTION_SYMBOL_TABLE: Lazy<HashMap<&str, FunctionSymbol>> = Lazy::new(||
    HashMap::from([
        ("print", FunctionSymbol {args: vec![BasicType::Any], return_type: None, func: print_fn})
    ])
);

pub fn get_function_symbol(name: &str) -> Option<FunctionSymbol> {
    FUNCTION_SYMBOL_TABLE.get(name).cloned()
}

pub fn translate_function_call(name: &String, memory_manager: &mut MemoryManager) -> Result<Vec<IR>, String> {
    let fs;
    match get_function_symbol(name) {
        Some(symbol) => { fs = symbol.func; },
        None => {return Err(format!("Function '{}' not found", name)) }
    }
    fs(memory_manager)
}

pub fn compare_types(expected: &Vec<BasicType>, got: &Vec<BasicType>) -> bool {
    if expected.len() != got.len() { return false; }
    for ind in 0..expected.len() {
        if got[ind] == BasicType::Void { return false; }
        if expected[ind] == BasicType::Any { continue; }
        if expected[ind] != got[ind] { return false; }
    }

    return true;
}

pub fn validate_args(name: &String, args: &[Expr], memory_manager: &MemoryManager) -> Result<(), String> {
    let expected = &FUNCTION_SYMBOL_TABLE.get(name.as_str()).ok_or_else(|| format!("Function '{}' not found", name))?.args;
    let got: Vec<BasicType> = args
        .iter()
        .map(|arg| arg.get_type(memory_manager))
        .collect::<Result<Vec<_>, _>>()?;

    if compare_types(expected, &got) {
        Ok(())
    } else {
        Err(format!("Invalid values passed to the function '{}'", name))
    }
}


// // // // // // // // // // // // Internal Functions // // // // // // // // // // // // // // //
pub fn print_fn(memory_manager: &mut MemoryManager) -> Result<Vec<IR>, String> {
    // FIXME: STILL FIX MEMORY MANAGEMENT SYNC BC I HATE MY LIFE
    fn output_char(s: char, memory_manager: &mut MemoryManager) -> Vec<IR> {
        vec![
            memory_manager.load_immediate_string(&s.to_string()),
            memory_manager.output()
        ]
            .iter()
            .flatten()
            .map(|x| x.clone())
            .collect::<Vec<IR>>()
    }

    if memory_manager.get_len_stack() == 0 {
        return Ok(
            output_char('\n', memory_manager)
        )
    }

    let mut output: Vec<IR> = Vec::new();

    while memory_manager.get_len_stack() > 0 {
        output.extend(memory_manager.output());
        // if memory_manager.get_len_stack() != 0 {
        //     output.extend(output_char(' ', memory_manager));
        // }
        // ! Maybe readd it later with proper args? IDK
    }

    output.extend(output_char('\n', memory_manager));

    Ok(output)
}
