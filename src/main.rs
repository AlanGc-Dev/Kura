use std::env;
use std::fs; // <-- Solo dejamos este import aquí arriba
use std::process;

mod token;
mod lexer;
mod ast;
mod parser;
mod evaluator;
mod types;

use lexer::Lexer;
use parser::Parser;
use evaluator::{Entorno, evaluar_programa};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Error: Faltan argumentos.");
        println!("Uso: kura <archivo.kr>");
        process::exit(1);
    }

    let ruta_archivo = &args[1];

    if !ruta_archivo.ends_with(".kr") {
        println!("Error: El compilador solo acepta archivos con extensión '.kr'");
        process::exit(1);
    }

    let codigo_kura = fs::read_to_string(ruta_archivo).unwrap_or_else(|err| {
        println!("Error al leer el archivo '{}': {}", ruta_archivo, err);
        process::exit(1);
    });

    // --- EL MOTOR DE KURA ---
    let lexer = Lexer::new(&codigo_kura);
    let mut parser = Parser::new(lexer);
    let programa = parser.parse_programa();

    let mut memoria = Entorno::new();
    evaluar_programa(programa, &mut memoria);
}