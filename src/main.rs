use std::env;
use std::process;

mod token;
mod lexer;
mod ast;
mod parser;
mod evaluator;
mod types;


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Error: Faltan argumentos.");
        println!("Uso: kura <archivo.kr>");
        process::exit(1);
    }

    let archivo = &args[1];
    let codigo_fuente = std::fs::read_to_string(archivo).unwrap_or_else(|_| {
        println!("Error: No se pudo abrir el archivo '{}'", archivo);
        std::process::exit(1);
    });

    let lexer = lexer::Lexer::new(&codigo_fuente);
    let mut parser = parser::Parser::new(lexer, &codigo_fuente);

    let programa = parser.parse_programa();

    let entorno = evaluator::Entorno::new();
    evaluator::evaluar_programa(programa, entorno);
}