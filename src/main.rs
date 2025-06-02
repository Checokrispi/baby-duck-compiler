use baby_duck::grammar::ProgramParser;
use baby_duck::lexer::Lexer;

fn main() {
    //Deberia parsear
    let source_code = "program s; var b:int; void f(r:int) [var d:float; {a=10;}] main {b=10;} end";
    let lexer = Lexer::new(source_code);
    let parser = ProgramParser::new();
    let _ast = match parser.parse(lexer) {
        Ok(tree) => tree,
        Err(e) => {
            // Usamos "{:?}" para imprimir el error en formato Debug,
            // porque `LexicalError` no implementa Display.
            eprintln!("xxxxxxxxxxxxxxxxxxxxx Sintax Error: {:?} xxxxxxxxxxxxxxxxxxxxxxxxxxxx", e);
            std::process::exit(1);
        }
    };
    println!("");
    println!("----------------baby duck parsed correclty-----------------");
    println!("");

    //Deberia parsear
    let source_code = "program s; void f(r:int) [var d:float; {a=10;}] main {b=10;} end";
    let lexer = Lexer::new(source_code);
    let parser = ProgramParser::new();
    let _ast = match parser.parse(lexer) {
        Ok(tree) => tree,
        Err(e) => {
            // Usamos "{:?}" para imprimir el error en formato Debug,
            // porque `LexicalError` no implementa Display.
            eprintln!("xxxxxxxxxxxxxxxxxxxxx Sintax Error: {:?} xxxxxxxxxxxxxxxxxxxxxxxxxxxx", e);
            std::process::exit(1);
        }
    };
    println!("");
    println!("----------------baby duck parsed correclty-----------------");
    println!("");

    //Deberia parsear
    let source_code = "program s; main {b=10;} end";
    let lexer = Lexer::new(source_code);
    let parser = ProgramParser::new();
    let _ast = match parser.parse(lexer) {
        Ok(tree) => tree,
        Err(e) => {
            // Usamos "{:?}" para imprimir el error en formato Debug,
            // porque `LexicalError` no implementa Display.
            eprintln!("xxxxxxxxxxxxxxxxxxxxx Sintax Error: {:?} xxxxxxxxxxxxxxxxxxxxxxxxxxxx", e);
            std::process::exit(1);
        }
    };
    println!("");
    println!("----------------baby duck parsed correclty-----------------");
    println!("");

    //Deberia parsear
    let source_code = "program s; main {} end";
    let lexer = Lexer::new(source_code);
    let parser = ProgramParser::new();
    let _ast = match parser.parse(lexer) {
        Ok(tree) => tree,
        Err(e) => {
            // Usamos "{:?}" para imprimir el error en formato Debug,
            // porque `LexicalError` no implementa Display.
            eprintln!("xxxxxxxxxxxxxxxxxxxxx Sintax Error: {:?} xxxxxxxxxxxxxxxxxxxxxxxxxxxx", e);
            std::process::exit(1);
        }
    };
    println!("");
    println!("----------------baby duck parsed correclty-----------------");
    println!("");
}
