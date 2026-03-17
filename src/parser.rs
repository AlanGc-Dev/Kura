use crate::lexer::Lexer;
use crate::token::Token;
use crate::ast::{Programa, Declaracion, Expresion};

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    token_actual: Token,
    token_siguiente: Token,
}

impl<'a> Parser<'a> {
    pub fn new(mut lexer: Lexer<'a>) -> Self {
        let actual = lexer.next_token();
        let siguiente = lexer.next_token();

        Parser {
            lexer,
            token_actual: actual,
            token_siguiente: siguiente,
        }
    }

    // Avanza al siguiente token
    fn avanzar(&mut self) {
        // Clonamos para evitar problemas de referencias en Rust
        self.token_actual = self.token_siguiente.clone();
        self.token_siguiente = self.lexer.next_token();
    }

    // Procesa todo el archivo
    pub fn parse_programa(&mut self) -> Programa {
        let mut declaraciones = Vec::new();

        while self.token_actual != Token::FinDeArchivo {
            if let Some(declaracion) = self.parse_declaracion() {
                declaraciones.push(declaracion);
            }
            self.avanzar();
        }

        Programa { declaraciones }
    }

    // Decide qué tipo de declaración estamos leyendo
    fn parse_declaracion(&mut self) -> Option<Declaracion> {
        match self.token_actual {
            Token::Let => self.parse_declaracion_let(),
            _ => None, // Por ahora ignoramos lo que no sea 'let'
        }
    }

    // Entiende la sintaxis: let [mut] nombre: tipo = valor;
    fn parse_declaracion_let(&mut self) -> Option<Declaracion> {
        self.avanzar(); // Pasamos el 'let'

        let mut es_mut = false;
        if self.token_actual == Token::Mut {
            es_mut = true;
            self.avanzar(); // Pasamos el 'mut'
        }

        // 1. Obtenemos el nombre de la variable
        let nombre = match &self.token_actual {
            Token::Identificador(nombre) => nombre.clone(),
            _ => return None, // Error: se esperaba un nombre
        };
        self.avanzar();

        // 2. Esperamos los dos puntos ':'
        if self.token_actual != Token::DosPuntos { return None; }
        self.avanzar();

        // 3. Obtenemos el tipo (int, float, etc.)
        let tipo = match &self.token_actual {
            Token::Tipo(t) => t.clone(),
            _ => return None, // Error: se esperaba un tipo
        };
        self.avanzar();

        // 4. Esperamos el igual '='
        if self.token_actual != Token::Asignacion { return None; }
        self.avanzar();

        // 5. Obtenemos el valor (por ahora solo enteros simples)
        let valor = match &self.token_actual {
            Token::Entero(n) => Expresion::Entero(*n),
            _ => return None,
        };
        self.avanzar();

        // 6. Esperamos el punto y coma ';' (opcional según cómo diseñemos el final)
        if self.token_actual == Token::PuntoYComa {
            self.avanzar();
        }

        Some(Declaracion::Let {
            es_mut,
            nombre,
            tipo,
            valor,
        })
    }
}