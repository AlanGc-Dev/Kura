use std::collections::HashMap;
use crate::ast::{Programa, Declaracion, Expresion};
use crate::token::Token;

// 1. Definimos qué tipos de datos pueden existir en la memoria de Kura cuando se ejecuta
#[derive(Debug, Clone)]
pub enum ObjetoKura {
    Entero(i64),
    Booleano(bool), // <-- NUEVO
    Nulo,
}

// 2. Esta es la "Memoria RAM" de tu lenguaje
pub struct Entorno {
    // Un diccionario que asocia el nombre de la variable (ej: "vida") con su valor (ej: 100)
    pub variables: HashMap<String, ObjetoKura>,
}

impl Entorno {
    pub fn new() -> Self {
        Entorno {
            variables: HashMap::new(),
        }
    }

    // Guarda una variable en la memoria
    pub fn guardar(&mut self, nombre: String, valor: ObjetoKura) {
        self.variables.insert(nombre, valor);
    }

    // Busca una variable en la memoria
    pub fn obtener(&self, nombre: &str) -> Option<ObjetoKura> {
        self.variables.get(nombre).cloned()
    }
}

// 3. El motor principal que recorre el árbol y ejecuta el código
pub fn evaluar_programa(programa: Programa, entorno: &mut Entorno) {
    for declaracion in programa.declaraciones {
        evaluar_declaracion(declaracion, entorno);
    }
}

fn evaluar_declaracion(declaracion: Declaracion, entorno: &mut Entorno) {
    match declaracion {
        Declaracion::Let { nombre, valor, .. } => {
            let valor_evaluado = evaluar_expresion(valor, entorno);
            entorno.guardar(nombre, valor_evaluado);
        }
        Declaracion::Print { valor } => {
            let valor_evaluado = evaluar_expresion(valor, entorno);
            match valor_evaluado {
                ObjetoKura::Entero(n) => println!("{}", n),
                ObjetoKura::Nulo => println!("null (variable no encontrada)"),
            }
        }
        // <-- ¡NUEVO BLOQUE REASIGNACION! -->
        Declaracion::Reasignacion { nombre, valor } => {
            // Evaluamos el nuevo valor
            let valor_evaluado = evaluar_expresion(valor, entorno);

            // Verificamos si la variable ya existe en la memoria antes de cambiarla
            if entorno.variables.contains_key(&nombre) {
                entorno.guardar(nombre, valor_evaluado); // Sobrescribe el valor anterior
            } else {
                // Control de errores al estilo Rust/C#
                println!("Error de Ejecución: La variable '{}' no ha sido declarada con 'let'.", nombre);
            }
        }
    }
}
fn evaluar_expresion(expresion: Expresion, entorno: &Entorno) -> ObjetoKura {
    match expresion {
        Expresion::Entero(n) => ObjetoKura::Entero(n),
        Expresion::Booleano(b) => ObjetoKura::Booleano(b),
        Expresion::Identificador(nombre) => {
            entorno.obtener(&nombre).unwrap_or(ObjetoKura::Nulo)
        }
        Expresion::Operacion { izquierda, operador, derecha } => {
            let izq_val = evaluar_expresion(*izquierda, entorno);
            let der_val = evaluar_expresion(*derecha, entorno);

            // Si ambos lados son números, hacemos matemáticas
            if let (ObjetoKura::Entero(i), ObjetoKura::Entero(d)) = (izq_val, der_val) {
                match operador {
                    Token::Suma => ObjetoKura::Entero(i + d),
                    Token::Resta => ObjetoKura::Entero(i - d),
                    Token::Multiplicacion => ObjetoKura::Entero(i * d),
                    Token::Division => ObjetoKura::Entero(i / d),
                    Token::Igualdad => ObjetoKura::Booleano(i == d),
                    Token::MenorQue => ObjetoKura::Booleano(i < d),
                    Token::MayorQue => ObjetoKura::Booleano(i > d),
                    _ => ObjetoKura::Nulo,
                }
            } else {
                ObjetoKura::Nulo // Manejo de errores básico
            }
        }
    }
}