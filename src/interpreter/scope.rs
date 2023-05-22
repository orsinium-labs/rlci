use std::collections::HashMap;

use crate::interpreter::Value;

pub trait Scope {
    fn get(&self, name: &str) -> Option<&Value>;
}

pub struct GlobalScope {
    values: HashMap<String, Value>,
}

impl Scope for GlobalScope {
    fn get(&self, name: &str) -> Option<&Value> {
        self.values.get(name)
    }
}

impl GlobalScope {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub fn set(&mut self, name: &str, val: Value) {
        self.values.insert(name.to_string(), val);
    }
}

pub struct LocalScope<'p, 'v> {
    parent: Option<&'p LocalScope<'p, 'v>>,
    name: String,
    value: &'v Value,
}

impl<'p: 'v, 'v> Scope for LocalScope<'p, 'v> {
    fn get(&self, name: &str) -> Option<&'v Value> {
        if self.name == name {
            return Some(self.value);
        }
        match self.parent {
            Some(parent) => parent.get(name),
            None => None,
        }
    }
}

impl<'p, 'v> LocalScope<'p, 'v> {
    pub fn new(name: String, value: &'v Value) -> Self {
        Self {
            parent: None,
            name,
            value,
        }
    }

    pub fn set(&'p self, name: &str, val: &'v Value) -> LocalScope {
        LocalScope {
            parent: Some(self),
            name: name.to_string(),
            value: val,
        }
    }
}
