use std::collections::HashMap;
use crate::translator::tokenizer::BasicType;

pub struct MemoryManagerVariable {
    pub cell: usize,
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

    pub fn get_var(&self, name: String) -> Option<&MemoryManagerVariable> {
        self.variables.get(&name)
    }

    pub fn set_var(&mut self, name: String, var_type: BasicType) {
        self.variables.insert(name, MemoryManagerVariable {
            cell: self.variables.len(),
            var_type
        });
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
}
