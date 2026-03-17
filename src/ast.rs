use crate::token::Token;

#[derive(Debug)]
pub struct Programa {
    pub declaraciones: Vec<Declaracion>,
}


#[derive(Debug)]
pub enum Declaracion {
    Let {
        es_mut: bool,
        nombre: String,
        tipo: String,
        valor: Expresion,
    },
}

#[derive(Debug)]
pub enum Expresion {
    Entero(i64),
    Identificador(String),
}