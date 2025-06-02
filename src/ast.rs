// Program
#[derive(Clone, Debug, PartialEq)]
pub struct Program {
    pub id: String,
    pub vars: Option<Vec<Vars>>, 
    pub funcs: Option<Vec<Func>>,
    pub body: Vec<Statement>,
}

// Vars 
#[derive(Clone, Debug, PartialEq)]
pub struct Vars {
    pub id: Vec<String>,
    pub var_type: Box<Expression>,
}

// Function
#[derive(Clone, Debug, PartialEq)]
pub struct Func {
    pub id: String,
    pub param: Vec<Param>,
    pub vars: Option<Vec<Vars>>,
    pub body: Vec<Statement>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Param {
    pub id: String,
    pub param_type: Box<Expression>,
}

// Expression 
#[derive(Clone, Debug, PartialEq)]
pub enum Expression {
    Integer(i64),
    Float(f64),
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
    GreaterThan,
    LessThan,
}

// Statement
#[derive(Clone, Debug, PartialEq)]
pub enum Statement {
    Assign(Assign),
    Condition(Condition),
    Cycle(Cycle),
    FCall(FCall),
    Print(Print),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Assign {
    pub id: String,
    pub value: Box<Expression>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct Condition {
    pub condition: Box<Expression>,
    pub body: Option<Box<Statement>>,
    pub elsebody: Option<Box<Statement>>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct Cycle {
    pub wcondition: Box<Expression>,
    pub wbody: Vec<Statement>
}
#[derive(Clone, Debug, PartialEq)]
pub struct FCall {
    pub id: String,
    pub expression: Vec<Box<Expression>>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct Print {
    pub value: Expression,
}