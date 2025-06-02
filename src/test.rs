#[cfg(test)]
mod tests {
use crate::grammar::ProgramParser;
use crate::lexer::Lexer;

    #[test]
    fn test1() {
        let source_code = "program s; var b:int; void f(r:int) [var d:float; {a=10;}] main {b=10;} end";
        let _lexer = Lexer::new(source_code);
        let _parser = ProgramParser::new();
        let _ast = _parser.parse(_lexer).unwrap_or_else(|e| {panic!("Sintax Error: {:?}", e);});
    }

    #[test]
    fn test2() {
        let source_code = "program s; main {b=10;} end";
        let _lexer = Lexer::new(source_code);
        let _parser = ProgramParser::new();
        let _ast = _parser.parse(_lexer).unwrap_or_else(|e| {panic!("Sintax Error: {:?}", e);});
    }

    #[test]
    fn test3() {
        let source_code = "program s; main {} end";
        let _lexer = Lexer::new(source_code);
        let _parser = ProgramParser::new();
        let _ast = _parser.parse(_lexer).unwrap_or_else(|e| {panic!("Sintax Error: {:?}", e);});
    }

    #[test]
    fn test4() {
        let source_code = "program s; var b:int; main {} end";
        let _lexer = Lexer::new(source_code);
        let _parser = ProgramParser::new();
        let _ast = _parser.parse(_lexer).unwrap_or_else(|e| {panic!("Sintax Error: {:?}", e);});
    }
}
