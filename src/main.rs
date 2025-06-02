use baby_duck::grammar::ProgramParser;
use baby_duck::lexer::Lexer;
use baby_duck::semantics::SemanticCube;

fn main() {
    //Deberia parsear
    let source_code = "program s; main {b=10+10;} end";
    let lexer = Lexer::new(source_code);
    let parser = ProgramParser::new();
    let _ast = match parser.parse(lexer) {
        Ok(tree) => tree,
        Err(e) => {
            eprintln!("xxxxxxxxxxxxxxxxxxxxx Sintax Error: {:?} xxxxxxxxxxxxxxxxxxxxxxxxxxxx", e);
            std::process::exit(1);
        }
    };
    println!("");
    println!("----------------baby duck parsed correclty-----------------");
    println!("");
    println!("{_ast:?}");
   
    let cube = SemanticCube::new();
    SemanticCube::validate_program(&_ast, &cube); 
}