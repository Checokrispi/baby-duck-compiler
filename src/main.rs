use baby_duck::grammar::ProgramParser;
use baby_duck::lexer::Lexer;
use baby_duck::semantics::SemanticCube;

fn main() {
    //Deberia parsear
    let source_code = "program s; var s:bool; main {b=10+12;} end";
    let lexer = Lexer::new(source_code);
    let parser = ProgramParser::new();
    let _ast = match parser.parse(lexer) {
        Ok(tree) => tree,
        Err(e) => {
            eprintln!(" x Sintax Error: {:?} ", e);
            std::process::exit(1);
        }
    };
    println!("valid Sintax");
   
    let cube = SemanticCube::new();
    SemanticCube::validate_program(&_ast, &cube); 
}