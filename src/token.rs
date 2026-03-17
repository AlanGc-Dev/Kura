#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Let,
    Mut,

    Identificador(String),
    Tipo(String),
    Entero(i64),

    PuntoYComa,

    DosPuntos,
    Asignacion,



    Suma,
    Resta,
    Multiplicacion,
    Division,

    ParentesisAbre,
    ParentesisCierra,
    LlaveAbre,
    LlaveCierra,

    Ilegal,
    FinDeArchivo,

}