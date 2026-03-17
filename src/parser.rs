use crate::lexer::Lexer;
use crate::token::Token;
use crate::ast::{Programa, Declaracion, Expresion};

// 1. Le quitamos el <'a> aquí
pub struct Parser {
    lexer: Lexer,
    token_actual: Token,
    token_siguiente: Token,
}

// 2. Y le quitamos el <'a> aquí
impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
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
            // ELIMINAMOS el self.avanzar() extra que había aquí,
            // porque cada función ya avanza por su cuenta.
        }

        Programa { declaraciones }
    }

    // Decide qué tipo de declaración estamos leyendo
    // Decide qué tipo de declaración estamos leyendo
    fn parse_declaracion(&mut self) -> Option<Declaracion> {
        match self.token_actual {
            Token::Let => self.parse_declaracion_let(),
            Token::Print => self.parse_print(),
            Token::If => self.parse_if(), // <-- NUEVO
            Token::While => self.parse_while(),
            Token::Identificador(_) => self.parse_reasignacion(), // <-- ¡NUEVO!
            _ => {
                self.avanzar();
                None
            }
        }
    }

    // Entiende la sintaxis: while condicion { cuerpo }
    fn parse_while(&mut self) -> Option<Declaracion> {
        self.avanzar(); // Pasamos el 'while'

        // Leemos la condición (ej: vida > 0)
        let condicion = self.parse_expresion()?;

        // Esperamos que abra el bloque '{'
        if self.token_actual != Token::LlaveAbre { return None; }
        self.avanzar();

        // Leemos todo el cuerpo del bucle
        let mut cuerpo = Vec::new();
        while self.token_actual != Token::LlaveCierra && self.token_actual != Token::FinDeArchivo {
            if let Some(decl) = self.parse_declaracion() {
                cuerpo.push(decl);
            }
        }
        self.avanzar(); // Pasamos el '}'

        Some(Declaracion::While { condicion, cuerpo })
    }

    fn parse_if(&mut self) -> Option<Declaracion> {
        self.avanzar(); // Pasamos el 'if'

        // Leemos la condición (ej: vida > 0)
        let condicion = self.parse_expresion()?;

        // Esperamos que abra el bloque '{'
        if self.token_actual != Token::LlaveAbre { return None; }
        self.avanzar();

        // Leemos todo lo que hay dentro del if hasta encontrar '}'
        let mut consecuencia = Vec::new();
        while self.token_actual != Token::LlaveCierra && self.token_actual != Token::FinDeArchivo {
            if let Some(decl) = self.parse_declaracion() {
                consecuencia.push(decl);
            }
        }
        self.avanzar(); // Pasamos el '}'

        // Miramos si hay un 'else'
        let mut alternativa = None;
        if self.token_actual == Token::Else {
            self.avanzar(); // Pasamos el 'else'
            if self.token_actual == Token::LlaveAbre {
                self.avanzar();
                let mut bloque_else = Vec::new();
                while self.token_actual != Token::LlaveCierra && self.token_actual != Token::FinDeArchivo {
                    if let Some(decl) = self.parse_declaracion() {
                        bloque_else.push(decl);
                    }
                }
                self.avanzar();
                alternativa = Some(bloque_else);
            }
        }

        Some(Declaracion::If { condicion, consecuencia, alternativa })
    }

    // Entiende la sintaxis: variable = nuevo_valor;
    // Entiende la sintaxis: variable = nuevo_valor;
    fn parse_reasignacion(&mut self) -> Option<Declaracion> {
        let nombre = match &self.token_actual {
            Token::Identificador(n) => n.clone(),
            _ => return None,
        };
        self.avanzar();

        if self.token_actual != Token::Asignacion { return None; }
        self.avanzar();

        // Leemos la expresión matemática completa
        let valor = self.parse_expresion()?;

        if self.token_actual == Token::PuntoYComa {
            self.avanzar();
        }

        Some(Declaracion::Reasignacion { nombre, valor })
    }

    // Entiende la sintaxis: print(variable);
    fn parse_print(&mut self) -> Option<Declaracion> {
        self.avanzar(); // Pasamos el 'print'

        // Esperamos '('
        if self.token_actual != Token::ParentesisAbre { return None; }
        self.avanzar();

        // Leemos lo que está adentro (un número o una variable)
        // Leemos lo que está adentro (puede ser una operación matemática también)
        let valor = self.parse_expresion()?;

        // Esperamos ')'
        if self.token_actual != Token::ParentesisCierra { return None; }
        self.avanzar();

        // Esperamos el ';' opcional al final
        if self.token_actual == Token::PuntoYComa {
            self.avanzar();
        }

        Some(Declaracion::Print { valor })
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
        let valor = self.parse_expresion()?;

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

    // Entiende [1, 2, "hola"]
    fn parse_arreglo(&mut self) -> Option<Expresion> {
        self.avanzar(); // Pasamos el '['
        let mut elementos = Vec::new();

        // Si el arreglo está vacío: []
        if self.token_actual == Token::CorcheteCierra {
            self.avanzar();
            return Some(Expresion::Arreglo(elementos));
        }

        // Leemos el primer elemento
        if let Some(expr) = self.parse_expresion() {
            elementos.push(expr);
        }

        // Mientras haya comas, seguimos leyendo elementos
        while self.token_actual == Token::Coma {
            self.avanzar(); // Pasamos la ','
            if let Some(expr) = self.parse_expresion() {
                elementos.push(expr);
            }
        }

        if self.token_actual != Token::CorcheteCierra { return None; }
        self.avanzar(); // Pasamos el ']'

        Some(Expresion::Arreglo(elementos))
    }

    // Lee un valor, y si ve un operador matemático, lee el siguiente valor
    // Lee valores, arreglos, accesos a índices y operaciones matemáticas
    fn parse_expresion(&mut self) -> Option<Expresion> {
        // 1. Leemos el lado izquierdo (puede ser un número, variable, o un arreglo nuevo)
        let mut izquierda = match &self.token_actual {
            Token::Entero(n) => Expresion::Entero(*n),
            Token::Identificador(nom) => Expresion::Identificador(nom.clone()),
            Token::True => Expresion::Booleano(true),
            Token::False => Expresion::Booleano(false),
            Token::Cadena(texto) => Expresion::Cadena(texto.clone()),
            Token::CorcheteAbre => return self.parse_arreglo(), // <-- SI ES UN ARREGLO, LO CREAMOS Y SALIMOS
            _ => return None,
        };
        self.avanzar();

        // <-- NUEVO: Verificamos si después de la variable hay un '[' (ej: horda[0])
        while self.token_actual == Token::CorcheteAbre {
            self.avanzar(); // Pasamos '['
            let indice = self.parse_expresion()?;
            if self.token_actual != Token::CorcheteCierra { return None; }
            self.avanzar(); // Pasamos ']'

            izquierda = Expresion::Indice {
                estructura: Box::new(izquierda),
                indice: Box::new(indice),
            };
        }

        // 2. Miramos si hay un operador matemático (+, -, ==, etc)
        match self.token_actual {
            Token::Suma | Token::Resta | Token::Multiplicacion | Token::Division | Token::Igualdad | Token::MenorQue | Token::MayorQue => {
                let operador = self.token_actual.clone();
                self.avanzar(); // Pasamos el operador

                let derecha = self.parse_expresion()?; // <-- Usamos recursividad para el lado derecho

                return Some(Expresion::Operacion {
                    izquierda: Box::new(izquierda),
                    operador,
                    derecha: Box::new(derecha),
                });
            }
            _ => {}
        }

        Some(izquierda)
    }

}
