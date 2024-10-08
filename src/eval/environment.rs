use std::collections::HashMap;

use crate::{
    ast::types::{Value, ValueType},
    eval::runtime_error::RuntimeError,
};

pub struct Environment {
    global: HashMap<String, ValueType>,
    stack: Vec<HashMap<String, ValueType>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            stack: Vec::new(),
            global: HashMap::new(),
        }
    }

    pub fn push_env(&mut self) {
        self.stack.push(HashMap::new());
    }

    pub fn pop_env(&mut self) {
        self.stack.pop();
    }

    pub fn define(&mut self, identifier: String, value: ValueType) {
        let map = self.stack.first_mut().unwrap_or(&mut self.global);
        map.insert(identifier, value);
    }

    pub fn assign(&mut self, ident: String, value: Value) -> super::Result<()> {
        for env in self.stack.iter_mut().rev() {
            if env.contains_key(&ident) {
                env.insert(ident, value.value);
                return Ok(());
            }
        }

        if self.global.contains_key(&ident) {
            self.global.insert(ident, value.value);
            Ok(())
        } else {
            Err(RuntimeError::UnboundName(value.location, ident))
        }
    }

    pub fn get(&self, identifier: &str) -> Option<&ValueType> {
        for map in &self.stack {
            if let Some(value) = map.get(identifier) {
                return Some(value);
            }
        }
        self.global.get(identifier)
    }
}
