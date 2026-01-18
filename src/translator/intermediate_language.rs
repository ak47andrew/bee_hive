use crate::translator::memory_manager::MemoryManager;
use crate::translator::tokenizer::{BasicType, Expr};
use crate::translator::func_call;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub enum IR {
    SET_POINTER {index: i16},
    LOAD_IMMEDIATE_STRING {value: String},
    LOAD_IMMEDIATE_INTEGER {value: u8},
    OUTPUT {value_type: BasicType},
    STORE_VARIABLE {cell: i16},  // Cell to which one you want to store a variable, pointer at a value
    LOAD_VARIABLE {cell: i16},  // Cell to which load variable, pointer at a value
    INPUT {cell: i16},
    WAIT_FOR_INPUT,
}

pub fn evaluate(statement: &Expr, memory_manager: &mut MemoryManager) -> Result<Vec<IR>, String> {
    match statement {
        Expr::Integer { value } => {
            Ok(memory_manager.load_immediate_integer(*value))
        }
        Expr::String { value } => {
            if !value.is_ascii() {
                return Err(format!("Invalid string `{}`; Only ASCII characters are supported", value));
            }

            Ok(memory_manager.load_immediate_string(value))
        }
        Expr::VariableName { name } => {
            let (cell, var_type) = match memory_manager.get_var(name) {
                Some(var) => (var.cell, var.var_type),
                None => return Err(format!("Variable {} not found", name)),
            };

            Ok(memory_manager.load_variable(cell, var_type))
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

            match var_type {
                BasicType::Integer => {},
                _ => {
                    return Err(format!("Invalid assignment value: {:?}", value.clone()));
                }
            }

            l.extend(memory_manager.store_variable(name, var_type));
            Ok(l)
        }
    }
}


// // // // // // // // // // // // // Helper Functions // // // // // // // // // // // // // // //

pub fn get_stack_free_index(memory_manager: &MemoryManager) -> i16 {
    stack_to_global(memory_manager.get_len_stack() as i16 + 1)
}

pub fn get_stack_last_index(memory_manager: &MemoryManager) -> i16 {
    let g = memory_manager.get_len_stack() as i16;
    if g == 0 {
        eprintln!("[!] Stack is empty, this can lead to UB");
    }

    stack_to_global(g)
}

pub fn get_heap_free_index(memory_manager: &MemoryManager) -> i16 {
    heap_to_global(memory_manager.get_len_heap() as i16 + 1)
}

pub fn heap_to_global(index: i16) -> i16 {
    index + 5
}

pub fn stack_to_global(index: i16) -> i16 {
    -index
}
