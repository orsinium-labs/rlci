use std::collections::HashMap;

use crate::interpreter::Value;

/// The global scope holds all global names defined in the current session.
///
/// This is a quite straightforward wrapper around a hash map.
/// The scope owns all values inside of it.
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

    /// Get the value of the given name from the scope if available.
    pub fn get(&self, name: &str) -> Option<&Value> {
        self.values.get(name)
    }

    /// Save a value in the scope udner the given name.
    ///
    /// It takes the ownership of the value, so for the code to keep using the value
    /// the pointer to that value should be returned.
    pub fn set(&mut self, name: &str, val: Value) -> &Value {
        self.values.insert(name.to_string(), val);
        self.get(name).unwrap()
    }
}
