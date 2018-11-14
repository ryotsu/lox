pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug)]
pub enum Statement {
    Expression(Expression),
    Print(Expression),
    Declaration(Declaration),
    Function(Function),
    Block(Block),
    If(If),
    While(While),
}

#[derive(Debug)]
pub struct Declaration {
    pub name: String,
    pub value: Option<Expression>,
}

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub args: Vec<String>,
    pub body: Box<Statement>,
}

#[derive(Debug)]
pub struct Block {
    pub statements: Vec<Statement>,
}

#[derive(Debug)]
pub struct If {
    pub cond: Expression,
    pub success: Box<Statement>,
    pub failure: Option<Box<Statement>>,
}

#[derive(Debug)]
pub struct While {
    pub cond: Expression,
    pub body: Box<Statement>,
}

#[derive(Debug)]
pub enum Expression {
    Literal(Literal),
    Unary(Unary),
    Binary(Binary),
    Logical(Logical),
    Grouping(Box<Expression>),
    Assignment(Assignment),
    Call(Call),
}

#[derive(Debug)]
pub enum Literal {
    Variable(String),
    Primary(Primary),
}

#[derive(Debug)]
pub enum Primary {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
}

#[derive(Debug)]
pub struct Unary {
    pub op: UnaryOp,
    pub expression: Box<Expression>,
}

#[derive(Debug)]
pub struct Binary {
    pub op: BinaryOp,
    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

#[derive(Debug)]
pub struct Logical {
    pub op: LogicalOp,
    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

#[derive(Debug)]
pub struct Assignment {
    pub variable: String,
    pub value: Box<Expression>,
}

#[derive(Debug)]
pub enum UnaryOp {
    Not,
    Minus,
}

#[derive(Debug)]
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

#[derive(Debug)]
pub enum LogicalOp {
    And,
    Or,
}

#[derive(Debug)]
pub struct Call {
    pub callee: Box<Expression>,
    pub arguments: Vec<Expression>,
}
