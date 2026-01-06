use crate::translator::memory_manager::MemoryManager;
use crate::translator::tokenizer::{BasicType, Expr};
use crate::translator::func_call;

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum IR {
    SET_POINTER {index: i16},
    LOAD_IMMEDIATE_STRING {value: String},
    LOAD_IMMEDIATE_INTEGER {value: u8},
    OUTPUT,
    STORE_VARIABLE {name: String},
    LOAD_VARIABLE,
}

pub fn evaluate(statement: &Expr, memory_manager: &mut MemoryManager) -> Result<Vec<IR>, String> {
    match statement {
        Expr::Integer { value } => {
            memory_manager.push_int();
            Ok(vec![
                IR::SET_POINTER {index: get_stack_free_index(&memory_manager)},
                IR::LOAD_IMMEDIATE_INTEGER {value: *value}
            ])
        }
        Expr::String { value } => {
            for char in value.chars() {
                if !char.is_ascii() {
                    return Err(format!("Invalid string `{}`; Only ASCII characters are supported", value));
                }
            }

            memory_manager.push_chars(value.len());
            Ok(vec![
                IR::SET_POINTER {index: get_stack_free_index(&memory_manager)},
                IR::LOAD_IMMEDIATE_STRING {value: value.chars().rev().collect::<String>()}
            ])
        }
        Expr::VariableName { name } => {
            let (cell, var_type) = match memory_manager.get_var(name.clone()) {
                Some(var) => (var.cell, var.var_type),
                None => return Err(format!("Variable {} not found", name)),
            };

            memory_manager.push(var_type);
            Ok(vec![
                IR::SET_POINTER {index: heap_to_global(cell as i16)},
                IR::LOAD_VARIABLE
            ])
        }
        Expr::FunctionCall { name, args } => {
            func_call::validate_args(name, args, memory_manager)?;

            let mut ir: Vec<IR> = Vec::new();

            for arg in args {
                ir.extend(evaluate(arg, memory_manager)?);
            }

            ir.extend(func_call::translate_function_call(name, memory_manager)?);

            Ok(ir)
        }
        Expr::VariableAssignment { name, value } => {
            let mut l = evaluate(value, memory_manager)?;
            let var_type = value.get_type(memory_manager)?;
            memory_manager.set_var(name.clone(), var_type);
            l.extend(
                [
                    IR::SET_POINTER {index: get_stack_last_index(&memory_manager)},
                    IR::STORE_VARIABLE { name: name.to_string() }
                ]
            );
            Ok(l)
        }
    }
}


// // // // // // // // // // // // // Helper Functions // // // // // // // // // // // // // // //

pub fn get_stack_free_index(memory_manager: &MemoryManager) -> i16 {
    -(memory_manager.get_len_stack() as i16 + 1)
}

pub fn get_stack_last_index(memory_manager: &MemoryManager) -> i16 {
    let g = memory_manager.get_len_stack() as i16;
    if g == 0 {
        eprintln!("[!] Stack is empty, this can lead to UB");
    }

    heap_to_global(g)
}

pub fn heap_to_global(index: i16) -> i16 {
    index + 5
}

pub fn stack_to_global(index: i16) -> i16 {
    -index
}
