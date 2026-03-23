#![allow(dead_code)]

use crate::token::Token;
use crate::types::TipoKura;

// Un programa completo en Kura es solo una lista de declaraciones
#[derive(Debug, Clone)]
pub struct Programa {
    pub declaraciones: Vec<Declaracion>,
}

// Las declaraciones son líneas de código que "hacen algo" pero no devuelven un valor (ej: let mut x = 5;)
#[derive(Debug,Clone)]
pub enum Declaracion {
    Break,
    Let {
        es_mut: bool,
        nombre: String,
        tipo: Option<TipoKura>,
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
    For {                      // <-- NUEVO: for elemento in iterable { ... }
        variable: String,
        iterable: Expresion,
        cuerpo: Vec<Declaracion>,
    },
    Funcion {
        nombre: String,
        parametros: Vec<(String, Option<TipoKura>)>, // <-- Ahora soporta el tipo opcional
        retorno: Option<TipoKura>,                   // <-- Retorno opcional
        cuerpo: Vec<Declaracion>,
    },
    Return {                    // <-- NUEVO: Para 'return 100;'
        valor: Expresion,
    },
    LlamadaSuelta {             // <-- NUEVO: Para llamar una función sola: 'atacar(orco);'
        nombre: String,
        argumentos: Vec<Expresion>,
    },
    LlamadaMetodoSuelta {
        objeto: Box<Expresion>,
        metodo: String,
        argumentos: Vec<Expresion>,
    },
    Importar {
        elementos: Vec<String>, // Guarda ["funcion1", "variable2"]
        archivo: String,        // Guarda "archivo.kr"
    },
    Enum {                      // <-- NUEVO: Definición de enum
        nombre: String,
        variantes: Vec<VarianteEnum>,
    },
    Match {                     // <-- NUEVO: Pattern matching
        valor: Expresion,
        casos: Vec<CasoMatch>,
    },
    Struct {
        nombre: String,
        campos: Vec<(String, TipoKura)>,
        metodos: Vec<Declaracion>,
    },
    ReasignacionPropiedad {
        objeto: String,
        propiedad: String,
        valor: Expresion,
    },
}

// Estructura para variantes de enum
#[derive(Debug, Clone)]
pub struct VarianteEnum {
    pub nombre: String,
    pub campos: Vec<String>, // Nombres de campos (ej: Ok(valor), Err(error))
}

// Estructura para casos de match
#[derive(Debug, Clone)]
pub struct CasoMatch {
    pub patron: Pattern,
    pub cuerpo: Vec<Declaracion>,
}

// Tipos de patrones para pattern matching
#[derive(Debug, Clone)]
pub enum Pattern {
    Variante {
        nombre: String,
        bindings: Vec<String>, // variables que se vinculan (ej: Ok(v), Err(e))
    },
    Identificador(String),
    Comodin, // _ para "cualquier cosa"
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
    ConstructorEnum {           // <-- NUEVO: Para Ok(valor), Err(error)
        variante: String,
        valores: Vec<Expresion>,
    },
    Diccionario(Vec<(String, Expresion)>),
    InstanciaStruct {
        nombre: String,
        campos: Vec<(String, Expresion)>,
    },
    AccesoPropiedad {
        objeto: Box<Expresion>,
        propiedad: String,
    },
    LlamadaMetodo {
        objeto: Box<Expresion>,
        metodo: String,
        argumentos: Vec<Expresion>,
    },


}