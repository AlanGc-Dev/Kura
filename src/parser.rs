use crate::lexer::Lexer;
use crate::token::Token;
use crate::ast::{Programa, Declaracion, Expresion};

pub struct Parser {
    lexer: Lexer,
    token_actual: Token,
    token_siguiente: Token,
}

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

    fn avanzar(&mut self) {
        self.token_actual = self.token_siguiente.clone();
        self.token_siguiente = self.lexer.next_token();
    }

    fn esperar_token(&mut self, token_esperado: Token) -> bool {
        if self.token_actual == token_esperado {
            self.avanzar();
            true
        } else {
            false
        }
    }

    pub fn parse_programa(&mut self) -> Programa {
        let mut declaraciones = Vec::new();
        while self.token_actual != Token::FinDeArchivo {
            if let Some(declaracion) = self.parse_declaracion() {
                declaraciones.push(declaracion);
            }
        }
        Programa { declaraciones }
    }

    fn parse_declaracion(&mut self) -> Option<Declaracion> {
        match self.token_actual {
            Token::Let => self.parse_declaracion_let(),
            Token::Print => self.parse_print(),
            Token::If => self.parse_if(),
            Token::While => self.parse_while(),
            Token::Fn => self.parse_funcion(),
            Token::Break => self.parse_break(),
            Token::Import => self.parse_declaracion_import(),// <-- LEEMOS FUNCIONES
            Token::Return => self.parse_return(),    // <-- LEEMOS RETORNOS
            Token::Identificador(_) => {
                // Miramos el siguiente token para saber si es reasignacion o llamada
                if self.token_siguiente == Token::Asignacion {
                    self.parse_reasignacion()
                } else if self.token_siguiente == Token::ParentesisAbre {
                    self.parse_llamada_suelta()
                } else {
                    println!("Error Parser [Instruccion]: Variable suelta o palabra no reconocida: {:?} {:?}", self.token_actual, self.token_siguiente);
                    self.avanzar();
                    None
                }

            }
            _ => {
                println!("Error Parser [General]: No se reconocio la instruccion. Ignorando token: {:?}", self.token_actual);
                self.avanzar();
                None
            }
        }
    }

    fn parse_break(&mut self) -> Option<Declaracion> {
        self.avanzar(); // pasamos 'break'
        if self.token_actual == Token::PuntoYComa { self.avanzar(); }
        Some(Declaracion::Break)
    }

    fn parse_declaracion_import(&mut self) -> Option<Declaracion> {
        self.avanzar(); // Saltamos la palabra 'import'

        // 1. Esperamos abrir llaves '{'
        if !self.esperar_token(Token::LlaveAbre) {
            println!("Error: Faltaba '{{' despues de import");
            return None;
        }

        let mut elementos = Vec::new();

        // 2. Leemos los nombres de lo que vamos a importar hasta encontrar '}'
        while self.token_actual != Token::LlaveCierra && self.token_actual != Token::Ilegal {
            match &self.token_actual {
                Token::Identificador(nombre) => {
                    elementos.push(nombre.clone());
                    self.avanzar();
                },
                Token::Coma => {
                    self.avanzar(); // Si hay una coma (,), la ignoramos y seguimos
                },
                _ => {
                    println!("Error en import: Se esperaba el nombre de una funcion o variable.");
                    return None;
                }
            }
        }

        // 3. Esperamos cerrar llaves '}'
        if !self.esperar_token(Token::LlaveCierra) { return None; }

        // 4. Esperamos la palabra 'from'
        if !self.esperar_token(Token::From) {
            println!("Error: Faltaba 'from' en el import");
            return None;
        }

        // 5. Esperamos la ruta del archivo ("archivo.kr")
        let archivo = match &self.token_actual {
            Token::Cadena(ruta) => {
                let r = ruta.clone();
                self.avanzar();
                r
            },
            _ => {
                println!("Error en import: Faltan las comillas en la ruta del archivo.");
                return None;
            }
        };

        // 6. Esperamos el punto y coma ';'
        if !self.esperar_token(Token::PuntoYComa) { return None; }

        // ¡Si todo salió bien, devolvemos el Nodo creado!
        Some(Declaracion::Importar { elementos, archivo })
    }
    fn parse_declaracion_let(&mut self) -> Option<Declaracion> {
        self.avanzar(); // Pasamos 'let'
        let mut es_mut = false;
        if self.token_actual == Token::Mut {
            es_mut = true;
            self.avanzar();
        }

        let nombre = match &self.token_actual {
            Token::Identificador(n) => n.clone(),
            _ => {
                println!("Error Parser [Let]: Esperaba nombre de variable, encontre: {:?}", self.token_actual);
                return None;
            }
        };
        self.avanzar();

        let mut tipo = "inferido".to_string();
        if self.token_actual == Token::DosPuntos {
            self.avanzar();
            tipo = match &self.token_actual {
                Token::Tipo(t) => t.clone(),
                Token::Identificador(t) => t.clone(), // Por si usan un tipo no registrado
                _ => {
                    println!("Error Parser [Let]: Tipo invalido para '{}', encontre: {:?}", nombre, self.token_actual);
                    return None;
                }
            };
            self.avanzar();
        }

        if self.token_actual != Token::Asignacion {
            println!("Error Parser [Let]: Esperaba '=' despues de '{}', encontre: {:?}", nombre, self.token_actual);
            return None;
        }
        self.avanzar();


        let valor = match self.parse_expresion() {
            Some(expr) => expr,
            None => {
                println!("Error Parser [Let]: No se reconocio el valor asignado a '{}'. Token atascado en: {:?}", nombre, self.token_actual);
                return None;
            }
        };

        if self.token_actual == Token::PuntoYComa { self.avanzar(); }
        Some(Declaracion::Let { es_mut, nombre, tipo, valor })

    }

    fn parse_print(&mut self) -> Option<Declaracion> {
        self.avanzar();
        if self.token_actual != Token::ParentesisAbre { return None; }
        self.avanzar();
        let valor = self.parse_expresion()?;
        if self.token_actual != Token::ParentesisCierra { return None; }
        self.avanzar();
        if self.token_actual == Token::PuntoYComa { self.avanzar(); }
        Some(Declaracion::Print { valor })
    }

    fn parse_reasignacion(&mut self) -> Option<Declaracion> {
        let nombre = match &self.token_actual {
            Token::Identificador(n) => n.clone(),
            _ => return None,
        };
        self.avanzar();
        if self.token_actual != Token::Asignacion { return None; }
        self.avanzar();
        let valor = self.parse_expresion()?;
        if self.token_actual == Token::PuntoYComa { self.avanzar(); }
        Some(Declaracion::Reasignacion { nombre, valor })
    }

    fn parse_if(&mut self) -> Option<Declaracion> {
        self.avanzar(); // Pasamos el 'if'
        let condicion = self.parse_expresion()?;

        if self.token_actual != Token::LlaveAbre { return None; }
        self.avanzar(); // Pasamos '{'

        let mut consecuencia = Vec::new();
        while self.token_actual != Token::LlaveCierra && self.token_actual != Token::FinDeArchivo {
            if let Some(decl) = self.parse_declaracion() { consecuencia.push(decl); }
        }
        self.avanzar(); // Pasamos '}'

        let mut alternativa = None;
        if self.token_actual == Token::Else {
            self.avanzar(); // Pasamos el 'else'

            // --- MAGIA AQUI: SOPORTE PARA 'else if' ---
            if self.token_actual == Token::If {
                // Si encontramos un 'if', llamamos a parse_if recursivamente
                if let Some(decl_if) = self.parse_if() {
                    // Guardamos todo ese nuevo bloque 'if' como nuestra alternativa
                    alternativa = Some(vec![decl_if]);
                }
            }
            // ------------------------------------------
            // Soporte para el 'else' tradicional con llaves { }
            else if self.token_actual == Token::LlaveAbre {
                self.avanzar(); // Pasamos '{'
                let mut bloque_else = Vec::new();
                while self.token_actual != Token::LlaveCierra && self.token_actual != Token::FinDeArchivo {
                    if let Some(decl) = self.parse_declaracion() { bloque_else.push(decl); }
                }
                self.avanzar(); // Pasamos '}'
                alternativa = Some(bloque_else);
            }
        }

        Some(Declaracion::If { condicion, consecuencia, alternativa })
    }

    fn parse_while(&mut self) -> Option<Declaracion> {
        self.avanzar();
        let condicion = self.parse_expresion()?;
        if self.token_actual != Token::LlaveAbre { return None; }
        self.avanzar();
        let mut cuerpo = Vec::new();
        while self.token_actual != Token::LlaveCierra && self.token_actual != Token::FinDeArchivo {
            if let Some(decl) = self.parse_declaracion() { cuerpo.push(decl); }
        }
        self.avanzar();
        Some(Declaracion::While { condicion, cuerpo })
    }

    // --- NUEVO: PARSEAR FUNCIONES ---
    fn parse_funcion(&mut self) -> Option<Declaracion> {
        self.avanzar(); // pasamos 'fn'
        let nombre = match &self.token_actual {
            Token::Identificador(n) => n.clone(),
            _ => return None,
        };
        self.avanzar();
        if self.token_actual != Token::ParentesisAbre { return None; }
        self.avanzar();

        let mut parametros = Vec::new();
        if let Token::Identificador(p) = &self.token_actual {
            parametros.push(p.clone());
            self.avanzar();
            if self.token_actual == Token::DosPuntos {
                self.avanzar(); self.avanzar(); // saltar tipo
            }
        }
        while self.token_actual == Token::Coma {
            self.avanzar();
            if let Token::Identificador(p) = &self.token_actual {
                parametros.push(p.clone());
                self.avanzar();
                if self.token_actual == Token::DosPuntos {
                    self.avanzar(); self.avanzar(); // saltar tipo
                }
            }
        }
        if self.token_actual == Token::ParentesisCierra { self.avanzar(); }

        if self.token_actual == Token::Flecha {
            self.avanzar(); self.avanzar(); // saltar tipo de retorno
        }

        if self.token_actual != Token::LlaveAbre { return None; }
        self.avanzar();

        let mut cuerpo = Vec::new();
        while self.token_actual != Token::LlaveCierra && self.token_actual != Token::FinDeArchivo {
            if let Some(decl) = self.parse_declaracion() { cuerpo.push(decl); }
        }
        self.avanzar(); // pasamos '}'

        Some(Declaracion::Funcion { nombre, parametros, cuerpo })
    }

    fn parse_return(&mut self) -> Option<Declaracion> {
        self.avanzar(); // pasamos 'return'
        let valor = match self.parse_expresion() {
            Some(v) => v,
            None => {
                println!("Error Parser [Return]: Expresion invalida despues de 'return'. Token atascado en: {:?}", self.token_actual);
                return None;
            }
        };
        if self.token_actual == Token::PuntoYComa { self.avanzar(); }
        Some(Declaracion::Return { valor })
    }

    fn parse_llamada_suelta(&mut self) -> Option<Declaracion> {
        let nombre = match &self.token_actual {
            Token::Identificador(n) => n.clone(),
            _ => return None,
        };
        self.avanzar(); // pasamos nombre
        self.avanzar(); // pasamos '('

        let mut argumentos = Vec::new();
        if self.token_actual != Token::ParentesisCierra {
            if let Some(arg) = self.parse_expresion() { argumentos.push(arg); }
            while self.token_actual == Token::Coma {
                self.avanzar();
                if let Some(arg) = self.parse_expresion() { argumentos.push(arg); }
            }
        }
        if self.token_actual == Token::ParentesisCierra { self.avanzar(); }
        if self.token_actual == Token::PuntoYComa { self.avanzar(); }

        Some(Declaracion::LlamadaSuelta { nombre, argumentos })
    }

    fn parse_arreglo(&mut self) -> Option<Expresion> {
        self.avanzar();
        let mut elementos = Vec::new();
        if self.token_actual == Token::CorcheteCierra {
            self.avanzar();
            return Some(Expresion::Arreglo(elementos));
        }
        if let Some(expr) = self.parse_expresion() { elementos.push(expr); }
        while self.token_actual == Token::Coma {
            self.avanzar();
            if let Some(expr) = self.parse_expresion() { elementos.push(expr); }
        }
        if self.token_actual != Token::CorcheteCierra { return None; }
        self.avanzar();
        Some(Expresion::Arreglo(elementos))
    }

    fn parse_expresion(&mut self) -> Option<Expresion> {
        let mut izquierda = match &self.token_actual {
            Token::Entero(n) => Expresion::Entero(*n),
            Token::Identificador(nom) => Expresion::Identificador(nom.clone()),
            Token::True => Expresion::Booleano(true),
            Token::False => Expresion::Booleano(false),
            Token::LlaveAbre => return self.parse_diccionario(),
            Token::Cadena(texto) => Expresion::Cadena(texto.clone()),
            Token::CorcheteAbre => return self.parse_arreglo(),
            _ => return None,
        };
        self.avanzar();

        // <-- NUEVO: Reconocer arreglos [ ] o llamadas a función ( )
        while self.token_actual == Token::CorcheteAbre || self.token_actual == Token::ParentesisAbre {
            if self.token_actual == Token::CorcheteAbre {
                self.avanzar();
                let indice = self.parse_expresion()?;
                if self.token_actual != Token::CorcheteCierra { return None; }
                self.avanzar();
                izquierda = Expresion::Indice { estructura: Box::new(izquierda), indice: Box::new(indice) };
            } else if self.token_actual == Token::ParentesisAbre {
                self.avanzar(); // Pasamos '('
                let mut argumentos = Vec::new();
                if self.token_actual != Token::ParentesisCierra {
                    if let Some(arg) = self.parse_expresion() { argumentos.push(arg); }
                    while self.token_actual == Token::Coma {
                        self.avanzar();
                        if let Some(arg) = self.parse_expresion() { argumentos.push(arg); }
                    }
                }
                if self.token_actual == Token::ParentesisCierra { self.avanzar(); }

                if let Expresion::Identificador(nombre) = izquierda {
                    izquierda = Expresion::Llamada { nombre, argumentos };
                } else {
                    return None;
                }
            }
        }

        match self.token_actual {
            Token::Suma | Token::Resta | Token::Multiplicacion | Token::Division |
            Token::Igualdad | Token::MenorQue | Token::MayorQue |
            Token::And | Token::Or => {
                let operador = self.token_actual.clone();
                self.avanzar();
                let derecha = self.parse_expresion()?;
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
    fn parse_diccionario(&mut self) -> Option<Expresion> {
        self.avanzar(); // pasamos '{'
        let mut pares = Vec::new();

        while self.token_actual != Token::LlaveCierra && self.token_actual != Token::FinDeArchivo {
            let clave = match &self.token_actual {
                Token::Cadena(c) => c.clone(),
                Token::Identificador(i) => i.clone(),
                _ => {
                    println!("Error Parser [Diccionario]: Esperaba 'clave', encontre: {:?}", self.token_actual);
                    return None;
                }
            };
            self.avanzar();

            if self.token_actual != Token::DosPuntos {
                println!("Error Parser [Diccionario]: Esperaba ':' despues de la clave '{}', encontre: {:?}", clave, self.token_actual);
                return None;
            }
            self.avanzar(); // pasamos ':'

            let valor = match self.parse_expresion() {
                Some(v) => v,
                None => {
                    println!("Error Parser [Diccionario]: Valor invalido para la clave '{}'", clave);
                    return None;
                }
            };
            pares.push((clave, valor));

            if self.token_actual == Token::Coma {
                self.avanzar(); // saltar coma opcional
            }
        }

        if self.token_actual != Token::LlaveCierra {
            println!("Error Parser [Diccionario]: Esperaba '}}' al final del diccionario, encontre: {:?}", self.token_actual);
            return None;
        }
        self.avanzar(); // pasamos '}'
        Some(Expresion::Diccionario(pares))
    }
}