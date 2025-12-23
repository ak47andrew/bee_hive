use std::collections::HashMap;
use once_cell::sync::Lazy;
use crate::translator::memory_manager::MemoryManager;
use crate::translator::tokenizer::{Arguments, Expr, Type};

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum IR {
    MOVE_TO_STACK_FREE,
    MOVE_TO_HEAP_FREE,
    MOVE_TO_HEAP_CELL {num: u64},
    LOAD_IMMEDIATE_STRING {value: String},
    LOAD_IMMEDIATE_INTEGER {value: u8},
    OUTPUT,
    STORE_VARIABLE {name: String},
    LOAD_VARIABLE {name: String},
}

pub static FUNCTIONS: Lazy<HashMap<&str, Type>> = Lazy::new(|| HashMap::from([
        ("print", Type::Function {arg_types: vec![Arguments{
            available_types: vec![Type::Integer, Type::String],
            is_any_amount: true
        }], return_type: Box::new(Type::Void)})
]));

fn compare_types(arg: &Expr, current_type: &Arguments) -> bool{
    !arg.compare_with_types(&current_type.available_types)
}

fn validate_types(statement: &Expr, memory_manager: &mut MemoryManager) -> Result<(), String> {
    match statement {
        Expr::FunctionCall {name, args} => {
            let args_types;
            match FUNCTIONS.get(name.as_str()) {
                Some(function_type) => {
                    match function_type {
                        Type::Function {arg_types, .. } => {args_types = arg_types;}
                        _ => unreachable!()
                    };
                }
                None => { return Err(format!("Function {} is not defined", name)) }
            }

            if args.len() != args_types.len() {
                return Err(format!("Function {} takes length {}, got {}",
                                   name, args_types.len(), args.len()));
            }

            if args.len() == 0 {
                return Ok(())
            }

            let current_type = &args_types[0];
            let type_index = 0;
            for index in 0..args.len() {
                let arg = &args[index];

                match arg {
                    Expr::Integer { .. } => {
                        if !compare_types(arg, current_type) {
                            return Err("Error when checking types".to_string());
                        }
                    }
                    Expr::String { .. } => {
                        if !compare_types(arg, current_type) {
                            return Err("Error when checking types".to_string());
                        }
                    }
                    Expr::Identifier { name } => {
                        let var_type = memory_manager.get_variable(name.clone());
                        
                        if var_type.is_none() {
                            return Err(format!("Variable {} not found", name));
                        }
                        
                        if !compare_types(var_type.unwrap().var_type, current_type) {}
                    }
                    Expr::FunctionCall { .. } => {}
                    Expr::Variable { .. } => {}
                }
            }

            todo!()
        }
        _ => Ok(()),
    }
}

fn evaluate(statement: &Expr, memory_manager: &mut MemoryManager) -> Result<Vec<IR>, String> {
    if !validate_types(statement) {
        return Err(format!("Type validation failed: {:?}", statement));
    }

    todo!()
}