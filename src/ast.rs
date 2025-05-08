#[derive(Clone, Debug, PartialEq)]
pub struct Program {
    pub id: String,
    pub vars: Vec<(String, String)>, 
    pub body: Vec<Statement>,
    pub funcs: Vec<Function>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Function {
    id: String,
    params: Vec<(String, String)>,
    vars: Vec<(String, String)>,
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
    Boolean(bool), 
    BinaryOperation {
        lhs: Box<Expression>,
        operator: Operator,
        rhs: Box<Expression>,
    },
    String(String),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Lt, 
    Gt, 
    Ne, 
}