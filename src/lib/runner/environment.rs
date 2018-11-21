use crate::ast::Value;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug)]
pub struct Environment {
    scope: Rc<RefCell<Scope>>,
}

#[derive(Debug)]
struct Scope {
    data: HashMap<String, Rc<Value>>,
    parent: Option<Rc<RefCell<Scope>>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            scope: Rc::new(RefCell::new(Scope::from(None))),
        }
    }

    pub fn append(&self) -> Self {
        Environment {
            scope: Rc::new(RefCell::new(Scope::from(Some(self.scope.clone())))),
        }
    }

    pub fn declare(&mut self, key: String, value: Rc<Value>) {
        self.scope.borrow_mut().declare(key, value)
    }

    pub fn get(&self, key: &str) -> Option<Rc<Value>> {
        self.scope.borrow().get(key)
    }

    pub fn assign(&mut self, key: String, value: Rc<Value>) -> Result<Rc<Value>, String> {
        self.scope
            .borrow_mut()
            .assign(key.clone(), value)
            .ok_or(format!("Variable '{}' not declared", key))
    }
}

impl Clone for Environment {
    fn clone(&self) -> Self {
        Environment {
            scope: self.scope.clone(),
        }
    }
}

impl Scope {
    fn from(parent: Option<Rc<RefCell<Scope>>>) -> Self {
        Scope {
            data: HashMap::new(),
            parent: parent,
        }
    }

    fn declare(&mut self, key: String, value: Rc<Value>) {
        self.data.insert(key, value);
    }

    fn get(&self, key: &str) -> Option<Rc<Value>> {
        self.data.get(key).cloned().or_else(|| {
            self.parent
                .as_ref()
                .and_then(|scope| scope.borrow().get(key))
        })
    }

    fn assign(&mut self, key: String, value: Rc<Value>) -> Option<Rc<Value>> {
        if self.data.contains_key(&key) {
            self.data.insert(key, value)
        } else {
            self.parent
                .as_mut()
                .and_then(|scope| scope.borrow_mut().assign(key, value))
        }
    }
}
