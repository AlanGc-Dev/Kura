mod token;
mod lexer;
mod ast;
mod parser;
mod evaluator;

use lexer::Lexer;
use parser::Parser;
use evaluator::{Entorno, evaluar_programa};

fn main() {
    // Escribimos un pequeño programa en Kura
    let codigo_kura = "
        let mut vida: int = 100;
        let danio: int = 15;
        
        vida = vida - danio;
        print(vida); // Debería imprimir 85
        
        let esta_vivo: bool = vida > 0;
        print(esta_vivo); // Debería imprimir true
        
        let es_jefe: bool = vida == 1000;
        print(es_jefe); // Debería imprimir false
    ";

    println!("Iniciando Kura Engine...\n");

    // 1. Lexer: Convierte el texto en Tokens
    let lexer = Lexer::new(codigo_kura);

    // 2. Parser: Convierte los Tokens en el Árbol Lógico (AST)
    let mut parser = Parser::new(lexer);
    let programa = parser.parse_programa();

    // 3. Evaluador: Creamos la memoria y ejecutamos el código
    let mut memoria = Entorno::new();
    evaluar_programa(programa, &mut memoria);

    // 4. ¡Comprobamos la memoria para ver si Kura hizo su trabajo!
    println!("--- ESTADO DE LA MEMORIA DE KURA ---");
    for (variable, valor) in &memoria.variables {
        println!("Variable '{}' -> {:?}", variable, valor);
    }
}