use super::runner::environment::Environment;
use std::fmt;
use std::rc::Rc;

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Expression(Expression),
    Print(Expression),
    Declaration(Declaration),
    Function(Function),
    Block(Block),
    Conditional(Conditional),
    Iteration(Iteration),
    Return(Expression),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Declaration {
    pub name: Rc<String>,
    pub value: Option<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub name: Rc<String>,
    pub params: Vec<Rc<String>>,
    pub body: Box<Statement>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Conditional {
    pub cond: Expression,
    pub success: Box<Statement>,
    pub failure: Option<Box<Statement>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Iteration {
    pub cond: Expression,
    pub body: Box<Statement>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Literal(Literal),
    Unary(Unary),
    Binary(Binary),
    Logical(Logical),
    Grouping(Box<Expression>),
    Assignment(Assignment),
    Call(Call),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Variable(Rc<String>),
    Primary(Primary),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Primary {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Unary {
    pub op: UnaryOp,
    pub expression: Box<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Binary {
    pub op: BinaryOp,
    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Logical {
    pub op: LogicalOp,
    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Assignment {
    pub variable: Rc<String>,
    pub value: Box<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Not,
    Minus,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
    EqualEqual,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Plus,
    Minus,
    Multiply,
    Divide,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LogicalOp {
    And,
    Or,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Call {
    pub callee: Box<Expression>,
    pub arguments: Vec<Expression>,
}

#[derive(Debug, Clone)]
pub enum Value {
    Literal(Primary),
    Function(Function, Environment),
}

impl PartialEq for Value {
    fn eq(&self, other: &Value) -> bool {
        use self::Value::*;

        match (self, other) {
            (Literal(s), Literal(o)) => s == o,
            (Function(s, _), Function(o, _)) => s == o,
            _ => false,
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Literal(literal) => write!(f, "{}", literal),
            Value::Function(func, _) => write!(f, "<function {}>", func.name),
        }
    }
}

impl fmt::Display for Primary {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Primary::Number(number) => write!(f, "{}", number),
            Primary::String(string) => write!(f, "{}", string),
            Primary::Boolean(boolean) => write!(f, "{}", boolean),
            Primary::Nil => write!(f, "nil"),
        }
    }
}

impl Value {
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Literal(literal) => match literal {
                Primary::Boolean(false) | Primary::Nil => false,
                _ => true,
            },
            _ => true,
        }
    }
}
