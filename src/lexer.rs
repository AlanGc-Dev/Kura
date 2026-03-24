#![allow(dead_code, unused)]

use crate::token::Token;

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    pub linea: usize,
    pub columna: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect(),
            position: 0,
            linea: 1,
            columna: 1,
        }
    }

    fn avanzar_char(&mut self) {
        if self.position < self.input.len() {
            if self.input[self.position] == '\n' {
                self.linea += 1;
                self.columna = 1;
            } else {
                self.columna += 1;
            }
            self.position += 1;
        }
    }



    pub fn next_token(&mut self) -> Token {
        self.saltar_espacios();

        let _l_token = self.linea;
        let _c_token = self.columna;

        if self.position >= self.input.len() {
            return Token::FinDeArchivo;
        }

        let char_actual = self.input[self.position];

        let token = match char_actual {
            ':' => Token::DosPuntos,
            ';' => Token::PuntoYComa,
            '.' => Token::Punto, // <-- Agregado para rutas como "lexer.kr"
            ',' => Token::Coma,
            '+' => {
                if self.position + 1 < self.input.len() && self.input[self.position + 1] == '=' {
                    self.position += 1;
                    Token::AsignacionCompuesta
                } else {
                    Token::Suma
                }
            }
            '*' => {
                if self.position + 1 < self.input.len() && self.input[self.position + 1] == '*' {
                    self.position += 1;
                    Token::Potencia
                } else {
                    Token::Multiplicacion
                }
            }
            '/' => Token::Division,
            '%' => Token::Modulo,
            '<' => {
                if self.position + 1 < self.input.len() && self.input[self.position + 1] == '=' {
                    self.position += 1;
                    Token::MenorIgual
                } else {
                    Token::MenorQue
                }
            }
            '>' => {
                if self.position + 1 < self.input.len() && self.input[self.position + 1] == '=' {
                    self.position += 1;
                    Token::MayorIgual
                } else {
                    Token::MayorQue
                }
            }
            '(' => Token::ParentesisAbre,
            ')' => Token::ParentesisCierra,
            '{' => Token::LlaveAbre,
            '}' => Token::LlaveCierra,
            '[' => Token::CorcheteAbre,
            ']' => Token::CorcheteCierra,
            '-' => {
                if self.position + 1 < self.input.len() && self.input[self.position + 1] == '>' {
                    self.position += 1; // Saltamos el '>'
                    Token::Flecha
                } else {
                    Token::Resta
                }
            }
            '&' => {
                if self.position + 1 < self.input.len() && self.input[self.position + 1] == '&' {
                    self.position += 1;
                    Token::And
                } else {
                    Token::Ampersand  // 🚀 NUEVO: single &
                }
            }
            '=' => {
                if self.position + 1 < self.input.len() && self.input[self.position + 1] == '=' {
                    self.position += 1; // Saltamos el segundo '='
                    Token::Igualdad
                } else if self.position + 1 < self.input.len() && self.input[self.position + 1] == '>' {
                    self.position += 1; // Saltamos el '>'
                    Token::FlechaGrande
                } else {
                    Token::Asignacion
                }
            }
            '&' => {
                if self.position + 1 < self.input.len() && self.input[self.position + 1] == '&' {
                    self.position += 1;
                    Token::And
                } else {
                    Token::Ilegal
                }
            }
            '|' => {
                if self.position + 1 < self.input.len() && self.input[self.position + 1] == '|' {
                    self.position += 1;
                    Token::Or
                } else {
                    Token::Ilegal
                }
            }
            '!' => {
                if self.position + 1 < self.input.len() && self.input[self.position + 1] == '=' {
                    self.position += 1;
                    Token::Diferente
                } else {
                    Token::Ilegal
                }
            }
            '"' => return self.leer_cadena(),
            'a'..='z' | 'A'..='Z' | '_' => return self.leer_identificador_o_palabra_clave(),
            '0'..='9' => return self.leer_numero(),
            _ => Token::Ilegal,
        };

        self.avanzar_char();
        token
    }

    fn saltar_espacios(&mut self) {
        while self.position < self.input.len() {
            let c = self.input[self.position];

            // 1. Manejo de espacios y SALTOS DE LÍNEA
            if c.is_whitespace() {
                if c == '\n' {
                    self.linea += 1;     // 🚀 ¡Vital! Subimos de línea
                    self.columna = 1;    // Reiniciamos columna
                } else {
                    self.columna += 1;
                }
                self.position += 1;
            }
            // 2. Manejo de comentarios //
            else if c == '/' && self.position + 1 < self.input.len() && self.input[self.position + 1] == '/' {
                // Saltamos todo hasta el final de la línea
                while self.position < self.input.len() && self.input[self.position] != '\n' {
                    self.position += 1;
                    // No hace falta actualizar columna aquí porque la línea va a morir
                }
                // NOTA: No consumimos el '\n' aquí, dejamos que el bloque de arriba 
                // lo maneje en la siguiente vuelta para que incremente self.linea.
            }
            else {
                break;
            }
        }
    }

    fn leer_cadena(&mut self) -> Token {
        self.position += 1; // Saltamos '"'
        let mut cadena = String::new();

        while self.position < self.input.len() && self.input[self.position] != '"' {
            if self.input[self.position] == '\\' && self.position + 1 < self.input.len() {
                self.position += 1;
                match self.input[self.position] {
                    'n' => cadena.push('\n'),
                    'r' => cadena.push('\r'),
                    't' => cadena.push('\t'),
                    '"' => cadena.push('"'),
                    '\\' => cadena.push('\\'),
                    _ => cadena.push(self.input[self.position]),
                }
            } else {
                cadena.push(self.input[self.position]);
            }
            self.position += 1;
        }

        if self.position < self.input.len() {
            self.position += 1; // Saltamos '"' final
        }
        Token::Cadena(cadena)
    }

    fn leer_identificador_o_palabra_clave(&mut self) -> Token {
        let inicio = self.position;
        while self.position < self.input.len() && (self.input[self.position].is_alphanumeric() || self.input[self.position] == '_') {
            self.position += 1;
        }

        let palabra: String = self.input[inicio..self.position].iter().collect();

        match palabra.as_str() {
            "let" => Token::Let,
            "mut" => Token::Mut,
            "print" => Token::Print,
            "true" => Token::True,
            "false" => Token::False,
            "if" => Token::If,
            "else" => Token::Else,
            "while" => Token::While,
            "for" => Token::For,
            "in" => Token::In,
            "fn" | "fun" => Token::Fn,
            "enum" => Token::Enum,
            "struct" => Token::Struct,
            "match" => Token::Match,
            "return" => Token::Return,
            "import" => Token::Import,
            "export" => Token::Export,  // 🚀 NUEVO
            "from" => Token::From,
            "as" => Token::As,          // 🚀 NUEVO
            "new" => Token::New,        // 🚀 NUEVO: allocate memory
            "null" => Token::Null,      // 🚀 NUEVO: null pointer
            "break" => Token::Break,
            "int" | "float" | "str" | "bool" | "Arreglo" | "void" => Token::Tipo(palabra),
            _ => Token::Identificador(palabra),
        }
    }

    fn leer_numero(&mut self) -> Token {
        let inicio = self.position;
        
        // 🚀 NUEVO: Reconocer números flotantes
        while self.position < self.input.len() && self.input[self.position].is_ascii_digit() {
            self.position += 1;
        }
        
        // Verificar si hay un punto decimal
        if self.position < self.input.len() && self.input[self.position] == '.' && 
           self.position + 1 < self.input.len() && self.input[self.position + 1].is_ascii_digit() {
            self.position += 1; // Saltamos el punto
            while self.position < self.input.len() && self.input[self.position].is_ascii_digit() {
                self.position += 1;
            }
            
            let numero_str: String = self.input[inicio..self.position].iter().collect();
            return Token::Flotante(numero_str.parse::<f64>().unwrap_or(0.0));
        }
        
        let numero_str: String = self.input[inicio..self.position].iter().collect();
        Token::Entero(numero_str.parse::<i64>().unwrap_or(0))
    }
}