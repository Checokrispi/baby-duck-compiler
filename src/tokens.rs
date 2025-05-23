use logos::Logos;
use std::fmt; 
use std::num::ParseIntError;

#[derive(Default, Debug, Clone, PartialEq)]
pub enum LexicalError {
    InvalidInteger(ParseIntError),
    #[default]
    InvalidToken,
}
impl From<ParseIntError> for LexicalError {
    fn from(err: ParseIntError) -> Self {
        LexicalError::InvalidInteger(err)
    }
}
#[derive(Logos, Clone, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+", skip r"#.*\n?", error = LexicalError)]
pub enum Token {
    #[token("program")]
    KeywordProgram,
    #[token("main")]
    KeywordMain,
    #[token("print")]
    KeywordPrint,
    #[token("if")]
    KeywordIf,
    #[token("else")]
    KeywordElse,
    #[token("while")]
    KeywordWhile,
    #[token("do")]
    KeywordDo,
    #[token("end")]
    KeywordEnd,
    #[token("int")]
    KeywordInt,
    #[token("cte_int")]
    CteInt,
    #[token("float")]
    KeywordFloat,
    #[token("cte_float")]
    CteFloat,
    #[token("var")]
    KeywordVar,
    #[regex("[\"][_0-9a-zA-Z]*[\"]")]
    CteString,
    #[token("void")]
    KeywordVoid,

    #[regex("[_a-zA-Z][_0-9a-zA-Z]*", |lex| lex.slice().to_string())]
    Identifier(String),
    #[regex("[0-9]+[.][1-9][0-9]*", |lex| lex.slice().parse())]
    Float(f64),
    #[regex("[1-9][0-9]*", |lex| lex.slice().parse())]
    Integer(i64),
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("[")]
    LBraket,
    #[token("]")]
    RBraket,
    #[token("=")]
    AssignSimbol,
    #[token("!=")]
    NotEqual,
    #[token(">")]
    BigerThan,
    #[token("<")]
    SmallerThan,
    #[token(";")]
    Semicolon,
    #[token(":")]
    TwoPoints,
    #[token(",")]
    Comma,

    #[token("+")]
    OperatorAdd,
    #[token("-")]
    OperatorSub,
    #[token("*")]
    OperatorMul,
    #[token("/")]
    OperatorDiv,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}