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
            '=' => Token::Asignacion,
            ';' => Token::PuntoYComa,
            '+' => Token::Suma,
            '-' => Token::Resta,
            '*' => Token::Multiplicacion,
            '/' => Token::Division,
            '(' => Token::ParentesisAbre,
            ')' => Token::ParentesisCierra,
            '{' => Token::LlaveAbre,
            '}' => Token::LlaveCierra,
            'a'..='z' | 'A'..='Z' | '_' => return self.leer_identificador_o_palabra_clave(),
            '0'..='9' => return self.leer_numero(),
            _ => Token::Ilegal,
        };

        self.position += 1;
        token

    }

    fn saltar_espacios(&mut self) {
        while self.position < self.input.len() && self.input[self.position].is_whitespace() {
            self.position += 1;
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
            "int" | "float" | "str" | "bool" => Token::Tipo(palabra),
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