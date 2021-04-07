#[derive(Debug, PartialEq, Clone)]
pub enum DataType {
    Void,
    I32,
    U32,
    Ptr(Box<DataType>),
}

#[derive(Debug, PartialEq)]
pub enum Value {
    I32(i32),
    U32(u32),
}

#[derive(Debug, PartialEq)]
pub enum OP {
    Add,
    Substract,
}

#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Constant(Value),
    Variable(String),
    Reference(String),
    Dereference(String),
    Operation(OP, Vec<Expression>),
    Call(String, Vec<Expression>),
    Empty,
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    Declaration(String, DataType),
    Assignment(String, Expression),
    DerefAssignment(String, Expression),
    Return(Expression),
    SingleExpression(Expression),
    WhileLoop(Expression, Comparison, Expression, Vec<Statement>),
}

#[derive(Debug, PartialEq)]
pub struct Function(
    pub String,
    pub DataType,
    pub Vec<(String, DataType)>,
    pub Vec<Statement>,
);

fn gen_padding(length: u8) -> String {
    let mut result = String::new();

    for _ in 0..(length - 1) {
        result.push('-');
    }
    result.push('>');

    result
}
impl Function {
    pub fn pretty_print(&self) {
        let padding = gen_padding(2);
        let sub_padding = gen_padding(3);

        println!("Function:");
        println!("{}Name: {:?}", padding, self.0);
        println!("{}Params:", padding);
        for tmp in self.2.iter() {
            println!("{}{}: {:?}", sub_padding, tmp.0, tmp.1);
        }
        println!("{}Returns: {:?}", padding, self.1);
        println!("{}Body:", padding);
        for tmp in self.3.iter() {
            tmp.pretty_print(3);
        }
    }
}

impl Statement {
    pub fn pretty_print(&self, padding_length: u8) {
        let padding = gen_padding(padding_length);

        println!("{}{:?}", padding, self);
    }
}
