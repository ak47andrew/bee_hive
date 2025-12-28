use crate::translator::memory_manager::MemoryManager;
use crate::translator::tokenizer::{BasicType, Expr};
use crate::translator::func_call;

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum IR {
    MOVE_TO_STACK_FREE,
    MOVE_TO_STACK_LAST,
    MOVE_TO_HEAP_FREE,
    MOVE_TO_HEAP_CELL {num: usize},
    LOAD_IMMEDIATE_STRING {value: String},
    LOAD_IMMEDIATE_INTEGER {value: u8},
    OUTPUT_ALL,
    STORE_VARIABLE {name: String},
    LOAD_VARIABLE {name: String},
}

pub fn evaluate(statement: &Expr, memory_manager: &mut MemoryManager) -> Result<Vec<IR>, String> {
    match statement {
        Expr::Integer { value } => {
            memory_manager.push_int();
            Ok(vec![
                IR::MOVE_TO_STACK_FREE,
                IR::LOAD_IMMEDIATE_INTEGER {value: *value}
            ])
        }
        Expr::String { value } => {
            memory_manager.push_chars(value.len());
            Ok(vec![
                IR::MOVE_TO_STACK_FREE,
                IR::LOAD_IMMEDIATE_STRING {value: value.chars().rev().collect::<String>()}
            ])
        }
        Expr::VariableName { name } => {
            let (_, var_type) = match memory_manager.get_var(name.clone()) {
                Some(var) => (var.cell, var.var_type),
                None => return Err(format!("Variable {} not found", name)),
            };

            memory_manager.push(var_type);
            Ok(vec![
                IR::LOAD_VARIABLE { name: name.clone() }
            ])
        }
        Expr::FunctionCall { name, args } => {
            func_call::validate_args(name, args, memory_manager)?;

            let mut ir: Vec<IR> = Vec::new();

            for arg in args {
                ir.append(&mut evaluate(arg, memory_manager)?);
            }

            ir.append(&mut func_call::translate_function_call(name, memory_manager)?);

            Ok(ir)
        }
        Expr::Variable { name, value } => {
            let mut l = evaluate(value, memory_manager)?;
            let var_type = value.get_type(memory_manager)?;
            memory_manager.set_var(name.clone(), var_type);
            l.append(
                &mut vec![
                    IR::MOVE_TO_STACK_LAST,
                    IR::STORE_VARIABLE { name: name.to_string() }
                ]
            );
            Ok(l)
        }
    }
}
