#[derive(Debug, PartialEq)]
pub enum DataType {
    I64,
}

#[derive(Debug, PartialEq)]
pub enum Value {
    I64(i64),
}

#[derive(Debug, PartialEq)]
pub enum OP {
    Add,
    Substract,
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Constant(Value),
    Operation(OP, Vec<Expression>),
    Empty,
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    Declaration(String, DataType),
    Assignment(String, Expression),
    Return(Expression),
}

#[derive(Debug, PartialEq)]
pub struct Function(pub String, pub DataType, pub Vec<Statement>);
