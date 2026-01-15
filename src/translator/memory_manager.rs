use std::collections::HashMap;
use crate::translator::intermediate_language::{get_heap_free_index, get_stack_free_index, get_stack_last_index, heap_to_global, IR};
use crate::translator::tokenizer::BasicType;

#[derive(Eq, PartialEq)]
pub struct MemoryManagerVariable {
    pub cell: i16,
    pub var_type: BasicType
}

// #[derive(Copy, Clone)]
pub struct MemoryManager {
    variables: HashMap<String, MemoryManagerVariable>,
    stack: Vec<BasicType>
}

impl MemoryManager {
    pub fn new() -> MemoryManager {
        MemoryManager {
            variables: HashMap::new(),
            stack: Vec::new()
        }
    }

    pub fn get_var(&self, name: &String) -> Option<&MemoryManagerVariable> {
        self.variables.get(name)
    }

    pub fn set_var(&mut self, name: String, var_type: BasicType) -> i16 {
        let cell = get_heap_free_index(self);
        self.variables.insert(name, MemoryManagerVariable {
            cell,
            var_type
        });

        cell
    }

    pub fn push(&mut self, var_type: BasicType) {
        self.stack.push(var_type);
    }

    pub fn push_int(&mut self) {
        self.stack.push(BasicType::Integer);
    }

    pub fn push_chars(&mut self, length: usize) {
        self.stack.extend(std::iter::repeat(BasicType::Char).take(length));
    }

    pub fn pop(&mut self) -> Option<BasicType> {
        self.stack.pop()
    }

    pub fn peek(&self) -> Option<&BasicType> {
        self.stack.last()
    }
    
    pub fn get_len_stack(&self) -> usize {
        self.stack.len()
    }
    
    pub fn get_len_heap(&self) -> usize {
        self.variables.len()
    }

    // === IR gen functions ===
    pub fn load_immediate_string(&mut self, value: &String) -> Vec<IR> {
        assert!(value.is_ascii(), "Non-ASCII string passed to memory manager");

        let output = vec![
            IR::SET_POINTER {index: get_stack_free_index(&self)},
            IR::LOAD_IMMEDIATE_STRING {value: value.chars().rev().collect()} // reversing bc it's going on the stack
        ];

        self.push_chars(value.len());

        output
    }

    pub fn load_immediate_integer(&mut self, value: u8) -> Vec<IR> {
        assert!(true, "No check needed here - u8 is our safeguard");

        let output = vec![
            IR::SET_POINTER {index: get_stack_free_index(&self)},
            IR::LOAD_IMMEDIATE_INTEGER {value}
        ];
        self.push_int();

        output
    }

    pub fn output(&mut self) -> Vec<IR> {
        let o = vec![
            IR::SET_POINTER {index: get_stack_last_index(&self)},
            IR::OUTPUT {value_type: *self.peek().unwrap_or(&BasicType::Void)},
        ];
        self.pop();

        o
    }

    pub fn load_variable(&mut self, cell: i16, var_type: BasicType) -> Vec<IR> {
        assert!(self.variables.values().any(|v| v.cell == cell && v.var_type == var_type),
                "No variable found");

        let output = vec![
            IR::SET_POINTER {index: heap_to_global(cell)},
            IR::LOAD_VARIABLE {cell: get_stack_free_index(self)}
        ];
        self.push(var_type);

        output
    }

    pub fn store_variable(&mut self, name: &String, var_type: BasicType) -> Vec<IR> {
        let cell = self.set_var(name.clone(), var_type);
        vec![
            IR::SET_POINTER {index: get_stack_last_index(&self)},
            IR::STORE_VARIABLE { cell }
        ]
    }
}
