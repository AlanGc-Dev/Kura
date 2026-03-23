use crate::lexer::Lexer;
use crate::token::Token;
use crate::ast::{Programa, Declaracion, Expresion, VarianteEnum, CasoMatch, Pattern};
use crate::types::TipoKura;

pub struct Parser {
    lexer: Lexer,
    token_actual: Token,
    token_siguiente: Token,
    pub linea_siguiente: usize, // <-- NUEVO
    pub col_siguiente: usize,
    pub linea_actual: usize,    // <-- NUEVO
    pub col_actual: usize,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let actual = lexer.next_token();
        let l_actual = lexer.linea;
        let c_actual = lexer.columna;

        let siguiente = lexer.next_token();
        let l_sig = lexer.linea;
        let c_sig = lexer.columna;

        Parser {
            lexer,
            token_actual: actual,
            linea_actual: l_actual,
            col_actual: c_actual,
            token_siguiente: siguiente,
            linea_siguiente: l_sig,
            col_siguiente: c_sig,
        }
    }

   fn avanzar(&mut self) {
        self.token_actual = self.token_siguiente.clone();
        self.linea_actual = self.linea_siguiente;
        self.col_actual = self.col_siguiente;

        let nuevo_siguiente = self.lexer.next_token();
        let l = self.lexer.linea;
        let c = self.lexer.columna;
        self.token_siguiente = nuevo_siguiente;
        self.linea_siguiente = l;
        self.col_siguiente = c;
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
            Token::For => self.parse_for(),
            Token::Fn => self.parse_funcion(),
            Token::Break => self.parse_break(),
            Token::Enum => self.parse_enum(),
            Token::Match => self.parse_match(),
            Token::Import => self.parse_declaracion_import(),// <-- LEEMOS FUNCIONES
            Token::Return => self.parse_return(),    // <-- LEEMOS RETORNOS
            Token::Identificador(_) => {
                // Miramos qué viene después del nombre
                match self.token_siguiente {
                    Token::Asignacion => self.parse_reasignacion(),
                    Token::ParentesisAbre => self.parse_llamada_suelta(),
                    // 🚀 ¡NUEVO!: Soporte para reasignar diccionarios/arreglos
                    Token::CorcheteAbre => self.parse_reasignacion_indice(),
                    _ => {
                        println!("Error Parser [Instruccion]: Variable suelta o palabra no reconocida: {:?} {:?}", self.token_actual, self.token_siguiente);
                        self.avanzar();
                        None
                    }
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

    fn parse_enum(&mut self) -> Option<Declaracion> {
        self.avanzar(); // pasamos 'enum'
        let nombre = match &self.token_actual {
            Token::Identificador(n) => n.clone(),
            _ => return None,
        };
        self.avanzar();
        if self.token_actual != Token::LlaveAbre { return None; }
        self.avanzar();

        let mut variantes = Vec::new();
        while self.token_actual != Token::LlaveCierra && self.token_actual != Token::FinDeArchivo {
            if let Token::Identificador(var_nombre) = &self.token_actual {
                let var_name = var_nombre.clone();
                self.avanzar();
                let mut campos = Vec::new();
                
                // Parsear campos: Ok(valor) o Err(error)
                if self.token_actual == Token::ParentesisAbre {
                    self.avanzar();
                    while self.token_actual != Token::ParentesisCierra {
                        if let Token::Identificador(campo) = &self.token_actual {
                            campos.push(campo.clone());
                            self.avanzar();
                            if self.token_actual == Token::Coma {
                                self.avanzar();
                            }
                        } else {
                            break;
                        }
                    }
                    if self.token_actual == Token::ParentesisCierra { self.avanzar(); }
                }
                
                variantes.push(VarianteEnum { nombre: var_name, campos });
                
                // Saltar coma si existe
                if self.token_actual == Token::Coma { self.avanzar(); }
            } else {
                self.avanzar();
            }
        }
        
        if self.token_actual == Token::LlaveCierra { self.avanzar(); }
        Some(Declaracion::Enum { nombre, variantes })
    }

    fn parse_match(&mut self) -> Option<Declaracion> {
        self.avanzar(); // pasamos 'match'
        let valor = self.parse_expresion()?;
        if self.token_actual != Token::LlaveAbre { return None; }
        self.avanzar();

        let mut casos = Vec::new();
        while self.token_actual != Token::LlaveCierra && self.token_actual != Token::FinDeArchivo {
            // Parsear patrón
            let patron = self.parse_pattern()?;
            if self.token_actual != Token::FlechaGrande { return None; }
            self.avanzar();

            // Parsear cuerpo del caso
            let mut cuerpo = Vec::new();
            loop {
                // Detectar fin del caso: coma, siguiente variante, o }
                if self.token_actual == Token::Coma || 
                   self.token_actual == Token::LlaveCierra ||
                   (matches!(&self.token_actual, Token::Identificador(_)) && 
                    self.token_siguiente != Token::Asignacion) {
                    break;
                }
                if let Some(decl) = self.parse_declaracion() {
                    cuerpo.push(decl);
                } else {
                    break;
                }
            }

            casos.push(CasoMatch { patron, cuerpo });
            
            if self.token_actual == Token::Coma { self.avanzar(); }
        }

        if self.token_actual == Token::LlaveCierra { self.avanzar(); }
        Some(Declaracion::Match { valor, casos })
    }

    fn parse_pattern(&mut self) -> Option<Pattern> {
        match &self.token_actual {
            Token::Identificador(name) => {
                let name_clone = name.clone();
                self.avanzar();
                
                // Verificar si es comodín (_)
                if name_clone == "_" {
                    return Some(Pattern::Comodin);
                }
                
                // Verificar si es variante con campos: Ok(v)
                if self.token_actual == Token::ParentesisAbre {
                    self.avanzar();
                    let mut bindings = Vec::new();
                    while self.token_actual != Token::ParentesisCierra {
                        if let Token::Identificador(var) = &self.token_actual {
                            bindings.push(var.clone());
                            self.avanzar();
                            if self.token_actual == Token::Coma { self.avanzar(); }
                        } else {
                            break;
                        }
                    }
                    if self.token_actual == Token::ParentesisCierra { self.avanzar(); }
                    Some(Pattern::Variante { nombre: name_clone, bindings })
                } else {
                    Some(Pattern::Identificador(name_clone))
                }
            }
            _ => None,
        }
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

        let mut tipo: Option<TipoKura> = None;
        if self.token_actual == Token::DosPuntos {
            self.avanzar();
            let tipo_str = match &self.token_actual {
                Token::Tipo(t) => t.clone(),
                Token::Identificador(t) => t.clone(), // Por si usan un tipo no registrado
                _ => {
                    println!("Error Parser [Let]: Tipo invalido para '{}', encontre: {:?}", nombre, self.token_actual);
                    return None;
                }
            };
            tipo = TipoKura::from_string(&tipo_str);
            if tipo.is_none() {
                println!("Error Parser [Let]: Tipo desconocido '{}'", tipo_str);
                return None;
            }
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
        let valor = self.parse_expresion()?;
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

    fn parse_for(&mut self) -> Option<Declaracion> {
        self.avanzar(); // pasamos 'for'
        let variable = match &self.token_actual {
            Token::Identificador(v) => v.clone(),
            _ => return None,
        };
        self.avanzar();
        if self.token_actual != Token::In { return None; }
        self.avanzar();
        let iterable = self.parse_expresion()?;
        if self.token_actual != Token::LlaveAbre { return None; }
        self.avanzar();
        let mut cuerpo = Vec::new();
        while self.token_actual != Token::LlaveCierra && self.token_actual != Token::FinDeArchivo {
            if let Some(decl) = self.parse_declaracion() { cuerpo.push(decl); }
        }
        self.avanzar();
        Some(Declaracion::For { variable, iterable, cuerpo })
    }

    // --- NUEVO: PARSEAR FUNCIONES (MODO SUPERVIVENCIA Y ERROR RECOVERY) ---
    fn parse_funcion(&mut self) -> Option<Declaracion> {
        self.avanzar(); // pasamos 'fn'

        let nombre = match &self.token_actual {
            Token::Identificador(n) => n.clone(),
            _ => return None,
        };
        self.avanzar(); // pasamos el nombre

        if self.token_actual != Token::ParentesisAbre { return None; }
        self.avanzar(); // pasamos '('

        let mut parametros = Vec::new();

        // Bucle flexible para parámetros
        while self.token_actual != Token::ParentesisCierra && self.token_actual != Token::FinDeArchivo {
            if self.token_actual == Token::Coma {
                self.avanzar(); continue;
            }

            let nombre_param = match &self.token_actual {
                Token::Identificador(n) => n.clone(),
                _ => { self.avanzar(); continue; }
            };
            self.avanzar(); // pasamos el nombre (ej. 'a')

            // Saltamos ':' si el usuario lo puso (opcional)
            if self.token_actual == Token::DosPuntos {
                self.avanzar();
            }

            let mut tipo_param = None;
            // Tomamos el tipo si el usuario lo escribió
            if let Token::Identificador(t) | Token::Tipo(t) = &self.token_actual {
                tipo_param = TipoKura::from_string(t);
                self.avanzar(); // pasamos el tipo (ej. 'Entero')
            }

            parametros.push((nombre_param, tipo_param));
        }

        if self.token_actual == Token::ParentesisCierra {
            self.avanzar(); // pasamos ')'
        }

        // --- RETORNO (Estilo Rust, Kotlin o ninguno) ---
        let mut retorno = None;

        // Saltamos ':' o '->' si están
        if self.token_actual == Token::DosPuntos || self.token_actual == Token::Flecha {
            self.avanzar();
        }

        // Leemos el tipo de retorno si lo hay
        if let Token::Identificador(t) | Token::Tipo(t) = &self.token_actual {
            retorno = TipoKura::from_string(t);
            self.avanzar(); // pasamos el tipo de retorno
        }

        // --- MAGIA AQUÍ: MODO SUPERVIVENCIA ---
        // Si por alguna desincronización quedó un token atascado (como tu error),
        // este bucle lo absorbe y lo salta hasta encontrar la bendita llave '{'
        while self.token_actual != Token::LlaveAbre && self.token_actual != Token::FinDeArchivo {
            self.avanzar();
        }

        // Si logramos llegar a la llave, entramos al cuerpo de la función
        if self.token_actual == Token::LlaveAbre {
            self.avanzar(); // pasamos '{'
        } else {
            return None; // Si llegamos al fin del archivo sin llaves, abortamos
        }

        let mut cuerpo = Vec::new();
        while self.token_actual != Token::LlaveCierra && self.token_actual != Token::FinDeArchivo {
            if let Some(decl) = self.parse_declaracion() { cuerpo.push(decl); }
        }

        if self.token_actual == Token::LlaveCierra {
            self.avanzar(); // pasamos '}'
        }

        Some(Declaracion::Funcion { nombre, parametros, retorno, cuerpo })
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
            Token::Modulo | Token::Potencia | Token::Igualdad | Token::Diferente |
            Token::MenorQue | Token::MayorQue | Token::MenorIgual | Token::MayorIgual |
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
    fn parse_reasignacion_indice(&mut self) -> Option<Declaracion> {
        // 1. Obtenemos el nombre (ej: "contador_dict")
        let nombre = match &self.token_actual {
            Token::Identificador(n) => n.clone(),
            _ => return None,
        };
        self.avanzar(); // Saltamos el nombre
        self.avanzar(); // Saltamos el '['

        let indice = self.parse_expresion()?;

        if self.token_actual != Token::CorcheteCierra { return None; }
        self.avanzar(); // Saltamos el ']'

        if self.token_actual != Token::Asignacion { return None; }
        self.avanzar(); // Saltamos el '='

        let valor = self.parse_expresion()?;
        if self.token_actual == Token::PuntoYComa { self.avanzar(); }

        // 🚀 AQUÍ ESTÁ EL TRUCO:
        // Clonamos 'nombre' en el campo 'nombre' para que la versión original
        // todavía pueda ser usada dentro del vec![] de abajo.
        Some(Declaracion::Reasignacion {
            nombre: nombre.clone(), // <-- Fotocopia aquí
            valor: Expresion::Llamada {
                nombre: "reemplazar".to_string(),
                argumentos: vec![
                    Expresion::Identificador(nombre), // <-- El original se entrega aquí
                    indice,
                    valor
                ]
            }
        })
    }
}