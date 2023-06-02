use std::collections::HashMap;

use crate::interpreter::Value;

#[derive(Debug)]
pub struct GlobalScope {
    values: HashMap<String, Value>,
}

impl GlobalScope {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub fn get(&self, name: &str) -> Option<&Value> {
        self.values.get(name)
    }

    pub fn set(&mut self, name: &str, val: Value) -> &Value {
        self.values.insert(name.to_string(), val);
        self.get(name).unwrap()
    }
}
