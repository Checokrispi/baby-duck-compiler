pub mod ast;
pub mod lexer;
pub mod tokens;
pub mod semantics;

use lalrpop_util::lalrpop_mod;
lalrpop_mod!(pub grammar);

#[cfg(test)]
mod test;