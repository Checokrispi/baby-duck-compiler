use lalrpop_util::lalrpop_mod;
lalrpop_mod!(pub grammar);

mod ast;
mod token;

fn main() {
    println!("Compilador babyduck");

    let program = r#"
        program myProg;
        var x, y : int;
        void myFunc(a : int, b : int) [
            var z : int;
            z = 5;
        ];
        main {
            myFunc(5 + 3, 2);
        }
        end
    "#;
    let parser = grammar::ProgramParser::new();
    let result = parser.parse(program);
    if result.is_ok() {
        println!("======== Programa leido y parseado correctamente: {:?}", result.unwrap());
    } else {
        println!("xxxxxxx Error al parsear: {:?}", result);
    }
}
