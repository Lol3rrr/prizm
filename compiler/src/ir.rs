#[derive(Debug, PartialEq, Clone)]
pub enum DataType {
    Void,
    I32,
    U32,
    I16,
    U16,
    Ptr(Box<DataType>),
}

#[derive(Debug, PartialEq)]
pub enum Value {
    I32(i32),
    U32(u32),
    Short(i16),
    UShort(u16),
}

#[derive(Debug, PartialEq)]
pub enum OP {
    Add,
    Substract,
    Multiply,
    Divide,
}

#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    LessThan,
}

#[derive(Debug, PartialEq)]
pub struct Condition {
    pub left: Expression,
    pub right: Expression,
    pub comparison: Comparison,
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Constant(Value),
    Variable(String),
    Reference(String),
    Dereference(Box<Expression>),
    Operation(OP, Vec<Expression>),
    Call(String, Vec<Expression>),
    Empty,
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    Declaration(String, DataType),
    Assignment(String, Expression),
    DerefAssignment(Expression, Expression),
    Return(Expression),
    SingleExpression(Expression),
    WhileLoop(Condition, Vec<Statement>),
    If(Condition, Vec<Statement>),
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
        println!();
    }
}

impl Statement {
    pub fn pretty_print(&self, padding_length: u8) {
        let padding = gen_padding(padding_length);

        match self {
            Self::WhileLoop(
                Condition {
                    left,
                    right,
                    comparison,
                },
                inner,
            ) => {
                println!(
                    "{}While ({:?} {:?} {:?}):",
                    padding, left, comparison, right
                );
                for tmp in inner.iter() {
                    tmp.pretty_print(padding_length + 2);
                }
            }
            _ => {
                println!("{}{:?}", padding, self);
            }
        };
    }
}
