use crate::token::Token;

// Un programa completo en Kura es solo una lista de declaraciones
#[derive(Debug)]
pub struct Programa {
    pub declaraciones: Vec<Declaracion>,
}

// Las declaraciones son líneas de código que "hacen algo" pero no devuelven un valor (ej: let mut x = 5;)
#[derive(Debug)]
pub enum Declaracion {
    Let {
        es_mut: bool,
        nombre: String,
        tipo: String,
        valor: Expresion,
    },
    Print {
        valor: Expresion,
    },
    Reasignacion {        // <-- ¡NUEVO!
        nombre: String,
        valor: Expresion,
    },
    If {                                      // <-- NUEVO BLOQUE
        condicion: Expresion,
        consecuencia: Vec<Declaracion>,
        alternativa: Option<Vec<Declaracion>> // El else es opcional
    },
}

// Las expresiones son cosas que producen un valor (ej: 5, "hola", x + 2)

#[derive(Debug, Clone)] // Añadimos Clone para facilitar el manejo
pub enum Expresion {
    Entero(i64),
    Identificador(String),
    Booleano(bool), // <-- NUEVO
    Operacion {     // <-- NUEVO (ej: 10 + 5)
        izquierda: Box<Expresion>, // Box es necesario en Rust para estructuras recursivas
        operador: Token,
        derecha: Box<Expresion>,
    }
}