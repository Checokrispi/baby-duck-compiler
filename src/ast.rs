#[derive(Clone, Debug, PartialEq)]
pub struct Program {
    pub id: String,
    pub vars: Vec<(String, String)>, 
    pub body: Vec<Statement>,
    pub funcs: Vec<()>,
}

pub struct Function {
    name: String,
    params: Vec<(String, String)>,
    body: Vec<Statement>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Statement {
    Variable {
        id: String,
        value: Box<Expression>,
    },
    Print {
        value: Box<Expression>,
    },
    Condition {
        condition: Box<Expression>,
        then_branch: Vec<Statement>,
        else_branch: Vec<Statement>, 
    },
    Cycle {
        condition: Box<Expression>,
        body: Vec<Statement>,
    },
    F_Call {
        id: String,
        args: Vec<Expression>, 
    },
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expression {
    Integer(i64),
    Variable(String),
    BinaryOperation {
        lhs: Box<Expression>,
        operator: Operator,
        rhs: Box<Expression>,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub enum Operator {
    Add,
    Sub,    
    Mul,
    Div,
    #[cfg(feature = "bit")]
    Shl,
}