//--------------------------------- AST para program -------------------------------------------------------
#[derive(Clone, Debug, PartialEq)]
pub struct Program {
    pub id: String,
    pub vars: Vec<(String, String)>, 
    pub body: Vec<Statement>,
    pub funcs: Vec<Function>,
}

//--------------------------------- AST para function -------------------------------------------------------
//representa un tipo de dato (int, float, etc.)
#[derive(Debug)]
enum Type {
    Int,
    Float,
}

//parámetro: id : tipo
#[derive(Debug)]
struct Param {
    name: String,
    param_type: Type,
}

//lista de parámetros
type ParamList = Vec<Param>;

//variable 
#[derive(Debug)]
struct VarDecl {
    name: Vec<String>,
    var_type: Type,
}

//lista de variables
type VarList = Vec<VarDecl>;

//cuerpo de la función (puede ser una lista de sentencias, aquí solo un placeholder)
#[derive(Debug)]
struct Body {
    //puedes definir aquí statements, instrucciones, etc.
    content: Vec<Statement>,
}

// Función
#[derive(Debug)]
struct Func {
    name: String,           // id
    params: ParamList,      // <ID_LOOP>
    vars: Option<VarList>,  // <VARS_DES>, puede ser None
    body: Body,             // <Body>
}
//-------------------------------------------------------------------------------------------------------------

//-------------------------------------------------- AST para statement ---------------------------------------
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
    FCall {
        id: String,
        args: Vec<Expression>, 
    },
}

//------------------------------------------------ AST para Expression -------------------------------------------
#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Integer(i64),
    Float(f64),
    Variable(String)
    Binary {
        left: Box<Expression>,
        operator: BinaryOperator,
        right: Box<Expression>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperator {
    Add,
    Sub,
    Mul,
    Div,
    GreaterThan,
    LessThan,
    NotEqual,
}