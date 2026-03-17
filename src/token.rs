#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // Palabras clave
    Let,
    Mut,
    Print,
    True,   // <-- NUEVO
    False,  // <-- NUEVO
    If,    // <-- NUEVO
    Else,
    While,

    Identificador(String),
    Tipo(String),
    Entero(i64),
    Cadena(String),

    // Símbolos
    DosPuntos,
    Asignacion,
    PuntoYComa,

    // Matemáticas y Lógica
    Suma,           // +
    Resta,          // -
    Multiplicacion, // *
    Division,       // /
    Igualdad,       // ==  <-- NUEVO
    MenorQue,       // <   <-- NUEVO
    MayorQue,       // >   <-- NUEVO

    // Agrupación
    ParentesisAbre,
    ParentesisCierra,
    LlaveAbre,
    LlaveCierra,
    CorcheteAbre,   // [
    CorcheteCierra, // ]
    Coma,           // ,

    Ilegal,
    FinDeArchivo,
}