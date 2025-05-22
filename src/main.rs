use lalrpop_util::lalrpop_mod;
lalrpop_mod!(pub grammar); // Se genera un mod llamado `grammar1`

mod ast;
mod token;

fn main() {
    println!("Compilador babyduck");

    let program = "program myProg;";

    let parser = grammar1::ProgramParser::new();
    let result = parser.parse(program);
    if result.is_ok() {
        println!("======== Programa leído y parseado correctamente: {:?}", result.unwrap());
    } else {
        println!("xxxxxxx Error al parsear: {:?}", result);
    }
}
