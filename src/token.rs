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
    Export,  // 🚀 NUEVO: export keyword
    From,
    Break,
    And,    // && <-- NUEVO
    Or,
    As,     // 🚀 NUEVO: import X as Y
    New,    // 🚀 NUEVO: allocate memory
    Null,   // 🚀 NUEVO: null pointer

    Identificador(String),
    Tipo(String),
    Entero(i64),
    Flotante(f64),  // 🚀 NUEVO: Token para flotantes
    Cadena(String),

    // Símbolos
    DosPuntos,
    Asignacion,
    PuntoYComa,
    Ampersand,          // 🚀 NUEVO: & (reference operator)

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