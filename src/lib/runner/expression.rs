use super::environment::Environment;
use super::{Evaluable, Executable, RetErr};
use crate::ast::Expression::*;
use crate::ast::{
    Assignment, Binary, Call, Expression, Function, Literal, Logical, Primary, Unary, Value,
};
use std::rc::Rc;

impl Evaluable for Expression {
    fn evaluate(&self, env: &mut Environment) -> Result<Rc<Value>, String> {
        match self {
            Literal(literal) => literal.evaluate(env),
            Unary(unary) => unary.evaluate(env),
            Binary(binary) => binary.evaluate(env),
            Logical(logical) => logical.evaluate(env),
            Grouping(expression) => expression.evaluate(env),
            Assignment(assignment) => assignment.evaluate(env),
            Call(call) => call.evaluate(env),
        }
    }
}

impl Evaluable for Literal {
    fn evaluate(&self, env: &mut Environment) -> Result<Rc<Value>, String> {
        match self {
            Literal::Variable(name) => env
                .get(name)
                .ok_or(format!("{} not defined", name)),
            Literal::Primary(primary) => Ok(Rc::new(Value::Literal(primary.clone()))),
        }
    }
}

impl Evaluable for Unary {
    fn evaluate(&self, env: &mut Environment) -> Result<Rc<Value>, String> {
        use crate::ast::{Primary::*, UnaryOp::*};

        let value = self.expression.evaluate(env)?;

        let result = match (&self.op, &*value) {
            (Not, Value::Literal(Boolean(ref val))) => Boolean(!val),
            (Not, Value::Literal(Nil)) => Boolean(true),
            (Not, _) => Boolean(false),
            (Minus, Value::Literal(Number(ref number))) => Number(-number),
            (Minus, _) => return Err(format!("Can't apply unary operator '-' to {}", value)),
        };

        Ok(Rc::new(Value::Literal(result)))
    }
}

impl Evaluable for Binary {
    fn evaluate(&self, env: &mut Environment) -> Result<Rc<Value>, String> {
        use self::Value::Literal;
        use crate::ast::{BinaryOp::*, Primary::*};

        let left = self.left.evaluate(env)?;
        let right = self.right.evaluate(env)?;

        let result = match (&*left, &self.op, &*right) {
            (l, EqualEqual, r) => Boolean(l == r),
            (l, NotEqual, r) => Boolean(l != r),
            (Literal(Number(l)), Less, Literal(Number(r))) => Boolean(l < r),
            (Literal(Number(l)), LessEqual, Literal(Number(r))) => Boolean(l <= r),
            (Literal(Number(l)), Greater, Literal(Number(r))) => Boolean(l > r),
            (Literal(Number(l)), GreaterEqual, Literal(Number(r))) => Boolean(l >= r),
            (Literal(Number(l)), Divide, Literal(Number(r))) => Number(l / r),
            (Literal(Number(l)), Minus, Literal(Number(r))) => Number(l - r),
            (Literal(Number(l)), Multiply, Literal(Number(r))) => Number(l * r),
            (Literal(Number(l)), Plus, Literal(Number(r))) => Number(l + r),
            (Literal(String(l)), Plus, Literal(String(r))) => {
                let mut s = l.clone();
                s.push_str(&r);
                String(s)
            }
            (l, op, r) => {
                return Err(format!(
                    "'{:?}' operator is not defined for {} and {}",
                    op, l, r
                ))
            }
        };

        Ok(Rc::new(Literal(result)))
    }
}

impl Evaluable for Logical {
    fn evaluate(&self, env: &mut Environment) -> Result<Rc<Value>, String> {
        use crate::ast::LogicalOp;

        let left = self.left.evaluate(env)?;

        let value = match (&self.op, left.is_truthy()) {
            (LogicalOp::And, false) => left,
            (LogicalOp::Or, true) => left,
            _ => self.right.evaluate(env)?,
        };

        Ok(value)
    }
}

impl Evaluable for Assignment {
    fn evaluate(&self, env: &mut Environment) -> Result<Rc<Value>, String> {
        let value = self.value.evaluate(env)?;
        env.assign(self.variable.to_string(), value)
    }
}

impl Evaluable for Call {
    fn evaluate(&self, env: &mut Environment) -> Result<Rc<Value>, String> {
        let result = self.callee.evaluate(env)?;

        let (func, func_env) = match *result {
            Value::Function(ref func, ref func_env) => (func, func_env),
            ref value => return Err(format!("{} is not callable", value)),
        };

        let mut func_env = func_env.append();

        for (key, val) in func.params.iter().zip(&self.arguments) {
            func_env.declare(key.clone(), val.evaluate(env)?);
        }

        func.evaluate(&mut func_env)
    }
}

impl Evaluable for Function {
    fn evaluate(&self, env: &mut Environment) -> Result<Rc<Value>, String> {
        let value = match self.body.execute(env) {
            Err(RetErr::Return(val)) => val,
            Err(RetErr::Error(err)) => return Err(err),
            _ => Rc::new(Value::Literal(Primary::Nil)),
        };

        Ok(value)
    }
}
