#[derive(Debug, PartialEq, Clone)]
#[allow(dead_code, unused)]
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
    For,    // <-- NUEVO
    In,     // <-- NUEVO
    Fn,      // <-- NUEVO
    Enum,   // <-- NUEVO
    Struct, // <-- NUEVO
    Match,  // <-- NUEVO
    Return,
    Import,
    From,
    Break,
    And,    // && <-- NUEVO
    Or,

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
    Modulo,         // %  <-- NUEVO
    Potencia,       // ** <-- NUEVO
    Igualdad,       // ==  <-- NUEVO
    Diferente,      // != <-- NUEVO
    MenorQue,       // <   <-- NUEVO
    MayorQue,       // >   <-- NUEVO
    MenorIgual,     // <= <-- NUEVO
    MayorIgual,     // >= <-- NUEVO
    AsignacionCompuesta, // += <-- NUEVO
    Flecha,         // ->
    FlechaGrande,   // => <-- NUEVO

    // Agrupación
    ParentesisAbre,
    ParentesisCierra,
    LlaveAbre,
    LlaveCierra,
    CorcheteAbre,   // [
    CorcheteCierra, // ]
    Coma,   
    Punto,// ,

    Ilegal,
    FinDeArchivo,
}