use crate::token::Token;

pub struct Lexer {
    input: Vec<char>,
    position: usize,
}
impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect(),
            position: 0,
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.saltar_espacios();

        if self.position >= self.input.len() {
            return Token::FinDeArchivo;
        }

        let char_actual = self.input[self.position];

        let token = match char_actual {
            ':' => Token::DosPuntos,
            ';' => Token::PuntoYComa,
            '+' => Token::Suma,
            '-' => {
                if self.position + 1 < self.input.len() && self.input[self.position + 1] == '>' {
                    self.position += 1; // Saltamos el '>'
                    Token::Flecha
                } else {
                    Token::Resta
                }
            }
            '*' => Token::Multiplicacion,
            '/' => Token::Division,
            '<' => Token::MenorQue,
            '>' => Token::MayorQue,
            '(' => Token::ParentesisAbre,
            ')' => Token::ParentesisCierra,
            '{' => Token::LlaveAbre,
            '}' => Token::LlaveCierra,
            '=' => {
                // Miramos si el siguiente carácter también es '='
                if self.position + 1 < self.input.len() && self.input[self.position + 1] == '=' {
                    self.position += 1; // Saltamos el segundo '='
                    Token::Igualdad
                } else {
                    Token::Asignacion
                }
            }
            '"' => return self.leer_cadena(),
            ',' => Token::Coma,             // <-- NUEVO
            '[' => Token::CorcheteAbre,     // <-- NUEVO
            ']' => Token::CorcheteCierra,   // <-- NUEVO
            'a'..='z' | 'A'..='Z' | '_' => return self.leer_identificador_o_palabra_clave(),
            '0'..='9' => return self.leer_numero(),
            _ => Token::Ilegal,
        };

        self.position += 1;
        token

    }

    // Lee texto entre comillas
    // Lee texto entre comillas soportando secuencias de escape como \n y \"
    fn leer_cadena(&mut self) -> Token {
        self.position += 1; // Saltamos la primera comilla '"'
        let mut cadena = String::new();

        while self.position < self.input.len() && self.input[self.position] != '"' {
            // Si detectamos una barra invertida '\', miramos la siguiente letra
            if self.input[self.position] == '\\' && self.position + 1 < self.input.len() {
                self.position += 1; // Saltamos la barra '\'
                match self.input[self.position] {
                    'n' => cadena.push('\n'), // Salto de línea
                    'r' => cadena.push('\r'), // Retorno de carro
                    't' => cadena.push('\t'), // Tabulación
                    '"' => cadena.push('"'),  // Comilla escapada
                    '\\' => cadena.push('\\'), // Barra invertida
                    _ => cadena.push(self.input[self.position]),
                }
            } else {
                cadena.push(self.input[self.position]);
            }
            self.position += 1;
        }

        if self.position < self.input.len() {
            self.position += 1; // Saltamos la comilla final '"'
        }

        Token::Cadena(cadena)
    }

    // Salta espacios en blanco y también IGNORA los comentarios //
    fn saltar_espacios(&mut self) {
        while self.position < self.input.len() {
            let c = self.input[self.position];

            // 1. Si es un espacio o salto de línea, lo saltamos
            if c == ' ' || c == '\t' || c == '\n' || c == '\r' {
                self.position += 1;
            }
            // 2. Si es una barra '/', miramos si la siguiente también es '/'
            else if c == '/' && self.position + 1 < self.input.len() && self.input[self.position + 1] == '/' {
                // ¡Es un comentario! Avanzamos hasta encontrar el final de la línea (\n)
                while self.position < self.input.len() && self.input[self.position] != '\n' {
                    self.position += 1;
                }
            }
            // 3. Si es código real, nos detenemos y dejamos que next_token haga su trabajo
            else {
                break;
            }
        }
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
            "true" => Token::True,   // <-- NUEVO
            "false" => Token::False, // <-- NUEVO
            "if" => Token::If,       // <-- NUEVO
            "else" => Token::Else,
            "while" => Token::While,
            "fn" => Token::Fn,
            "return" => Token::Return,
            "int" | "float" | "str" | "bool" | "Arreglo" | "void" => Token::Tipo(palabra),
            _ => Token::Identificador(palabra),
        }
    }

    fn leer_numero(&mut self) -> Token {
        let inicio = self.position;
        while self.position < self.input.len() && self.input[self.position].is_ascii_digit() {
            self.position += 1;
        }

        let numero_str: String = self.input[inicio..self.position].iter().collect();
        Token::Entero(numero_str.parse::<i64>().unwrap())

    }

}