use crate::token::Token;

// Un programa completo en Kura es solo una lista de declaraciones
#[derive(Debug)]
pub struct Programa {
    pub declaraciones: Vec<Declaracion>,
}

// Las declaraciones son líneas de código que "hacen algo" pero no devuelven un valor (ej: let mut x = 5;)
#[derive(Debug,Clone)]
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
    While {                 // <-- ¡NUEVO BLOQUE MÁGICO!
        condicion: Expresion,
        cuerpo: Vec<Declaracion>,
    },
    Funcion {                   // <-- NUEVO: Para crear 'fn atacar() { ... }'
        nombre: String,
        parametros: Vec<String>,
        cuerpo: Vec<Declaracion>,
    },
    Return {                    // <-- NUEVO: Para 'return 100;'
        valor: Expresion,
    },
    LlamadaSuelta {             // <-- NUEVO: Para llamar una función sola: 'atacar(orco);'
        nombre: String,
        argumentos: Vec<Expresion>,
    },
}

// Las expresiones son cosas que producen un valor (ej: 5, "hola", x + 2)

#[derive(Debug, Clone)] // Añadimos Clone para facilitar el manejo
pub enum Expresion {
    Entero(i64),
    Identificador(String),
    Booleano(bool),
    Cadena(String),
    Arreglo(Vec<Expresion>), // <-- NUEVO: Para [1, 2, 3]
    Indice {                 // <-- NUEVO: Para lista[0]
        estructura: Box<Expresion>,
        indice: Box<Expresion>
    },

    Operacion {     // <-- NUEVO (ej: 10 + 5)
        izquierda: Box<Expresion>, // Box es necesario en Rust para estructuras recursivas
        operador: Token,
        derecha: Box<Expresion>,
    },
    Llamada {                   // <-- NUEVO
        nombre: String,
        argumentos: Vec<Expresion>,
    },

}