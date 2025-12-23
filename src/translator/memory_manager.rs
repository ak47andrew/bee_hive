use std::collections::HashMap;
use crate::translator::tokenizer::{Type, Expr};

struct MemoryMangerVariable {
    pub cell: u64,
    pub var_type: Type,
}

// #[derive(Copy, Clone)]
pub struct MemoryManager {
    variables: HashMap<String, MemoryMangerVariable>,
}

impl MemoryManager {
    pub fn get_variable(&self, name: String) -> Option<&MemoryMangerVariable> {
        return self.variables.get(&name);
    }
}