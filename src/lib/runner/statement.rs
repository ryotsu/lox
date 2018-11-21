use super::environment::Environment;
use super::{Evaluable, Executable, RetErr};
use crate::ast::Statement::*;
use crate::ast::{Block, Conditional, Declaration, Function, Iteration, Primary, Statement, Value};
use crate::print_js;

impl Executable for Statement {
    fn execute(&self, env: &mut Environment) -> Result<(), RetErr> {
        match self {
            Expression(expression) => {
                expression.evaluate(env).map_err(|err| RetErr::Error(err))?;
                Ok(())
            }
            Print(expression) => {
                let result = expression.evaluate(env).map_err(|err| RetErr::Error(err))?;
                Ok(print_js(&result.to_string()))
            }
            Declaration(declaration) => declaration.execute(env),
            Function(function) => function.execute(env),
            Block(block) => block.execute(env),
            Conditional(conditional) => conditional.execute(env),
            Iteration(iteration) => iteration.execute(env),
            Return(expression) => {
                let value = expression.evaluate(env).map_err(|err| RetErr::Error(err))?;
                Err(RetErr::Return(value))
            }
        }
    }
}

impl Executable for Declaration {
    fn execute(&self, env: &mut Environment) -> Result<(), RetErr> {
        let value = match &self.value {
            Some(expression) => expression.evaluate(env).map_err(|err| RetErr::Error(err))?,
            None => Value::Literal(Primary::Nil),
        };

        env.declare(self.name.clone(), value);
        Ok(())
    }
}

impl Executable for Function {
    fn execute(&self, env: &mut Environment) -> Result<(), RetErr> {
        env.declare(self.name.clone(), Value::Function((*self).clone()));
        Ok(())
    }
}

impl Executable for Block {
    fn execute(&self, env: &mut Environment) -> Result<(), RetErr> {
        let mut block_env = env.append();
        for statement in &self.statements {
            statement.execute(&mut block_env)?;
        }
        Ok(())
    }
}

impl Executable for Conditional {
    fn execute(&self, env: &mut Environment) -> Result<(), RetErr> {
        if self
            .cond
            .evaluate(env)
            .map_err(|err| RetErr::Error(err))?
            .is_truthy()
        {
            self.success.execute(env)?;
        } else if let Some(failure) = &self.failure {
            failure.execute(env)?;
        }

        Ok(())
    }
}

impl Executable for Iteration {
    fn execute(&self, env: &mut Environment) -> Result<(), RetErr> {
        while self
            .cond
            .evaluate(env)
            .map_err(|err| RetErr::Error(err))?
            .is_truthy()
        {
            self.body.execute(env)?;
        }

        Ok(())
    }
}
