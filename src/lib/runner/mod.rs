pub mod environment;
mod expression;
mod statement;

use self::environment::Environment;
use super::ast::{Program, Value};

enum RetErr {
    Return(Value),
    Error(String),
}

trait Evaluable {
    fn evaluate(&self, env: &mut Environment) -> Result<Value, String>;
}

trait Executable {
    fn execute(&self, env: &mut Environment) -> Result<(), RetErr>;
}

impl Program {
    pub fn run(&self, env: &mut Environment) -> Result<(), String> {
        use self::RetErr::*;

        for statement in &self.statements {
            match statement.execute(env) {
                Err(Return(_)) => return Err(format!("Cannot have return outside a function")),
                Err(Error(err)) => return Err(err),
                Ok(()) => (),
            };
        }

        Ok(())
    }
}
