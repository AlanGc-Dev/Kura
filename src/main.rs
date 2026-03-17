mod token;
mod lexer;
mod ast;
mod parser;

use lexer::Lexer;
use parser::Parser;

fn main() {
    let codigo_kura = "
        let mut vida: int = 100;
        let daño: int = 15;
    ";

    println!("Compilando Kura...\n");

    // 1. El Lexer lee el texto
    let lexer = Lexer::new(codigo_kura);

    // 2. El Parser arma el árbol lógico
    let mut parser = Parser::new(lexer);
    let programa = parser.parse_programa();

    // 3. ¡Vemos la estructura que entiende el compilador!
    println!("{:#?}", programa);
}