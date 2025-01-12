use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::model::Object;

pub struct Env {
    parent: Option<Rc<RefCell<Env>>>,
    bindings: HashMap<String, Object>,
}

impl Env {
    pub fn new() -> Self {
        Self {
            parent: None,
            bindings: HashMap::new(),
        }
    }

    pub fn new_child(parent: Rc<RefCell<Env>>) -> Self {
        Self {
            parent: Some(parent),
            bindings: HashMap::new(),
        }
    }

    pub fn get(&self, key: &str) -> Option<&Object> {
        self.bindings.get(key)
    }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut Object> {
        self.bindings.get_mut(key)
    }

    pub fn set(&mut self, key: &str, value: Object) {
        self.bindings.insert(key.to_string(), value);
    }
}
