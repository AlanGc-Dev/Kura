use std::collections::HashMap;
use crate::ast::{Programa, Declaracion, Expresion};
use crate::token::Token;

#[derive(Debug, Clone)]
pub enum ObjetoKura {
    Entero(i64),
    Booleano(bool),
    Cadena(String),
    Arreglo(Vec<ObjetoKura>),
    Funcion { parametros: Vec<String>, cuerpo: Vec<Declaracion> }, // <-- GUARDAMOS LA FUNCIÓN EN MEMORIA
    Retorno(Box<ObjetoKura>), // <-- PARA ATRAPAR EL RETURN
    Nulo,
}

#[derive(Clone)]
pub struct Entorno {
    pub variables: HashMap<String, ObjetoKura>,
}

impl Entorno {
    pub fn new() -> Self {
        Entorno { variables: HashMap::new() }
    }
    // Clona la memoria para usarla dentro de una función
    pub fn extend(&self) -> Self {
        Entorno { variables: self.variables.clone() }
    }
    pub fn guardar(&mut self, nombre: String, valor: ObjetoKura) {
        self.variables.insert(nombre, valor);
    }
    pub fn obtener(&self, nombre: &str) -> Option<ObjetoKura> {
        self.variables.get(nombre).cloned()
    }
}

pub fn evaluar_programa(programa: Programa, entorno: &mut Entorno) {
    for declaracion in programa.declaraciones {
        let res = evaluar_declaracion(declaracion, entorno);
        if let ObjetoKura::Retorno(_) = res {
            break; // Termina el programa si hay un return global
        }
    }
}

// Ahora evaluar_declaracion devuelve un ObjetoKura para poder atrapar los "returns"
fn evaluar_declaracion(declaracion: Declaracion, entorno: &mut Entorno) -> ObjetoKura {
    match declaracion {
        Declaracion::Let { nombre, valor, .. } => {
            let valor_evaluado = evaluar_expresion(valor, entorno);
            entorno.guardar(nombre, valor_evaluado);
            ObjetoKura::Nulo
        }
        Declaracion::Print { valor } => {
            let valor_evaluado = evaluar_expresion(valor, entorno);
            imprimir_objeto(&valor_evaluado);
            println!(); // Salto de línea al final
            ObjetoKura::Nulo
        }
        Declaracion::Reasignacion { nombre, valor } => {
            let valor_evaluado = evaluar_expresion(valor, entorno);
            if entorno.variables.contains_key(&nombre) {
                entorno.guardar(nombre, valor_evaluado);
            } else {
                println!("Error: La variable '{}' no ha sido declarada.", nombre);
            }
            ObjetoKura::Nulo
        }
        Declaracion::If { condicion, consecuencia, alternativa } => {
            let valor_condicion = evaluar_expresion(condicion, entorno);
            let es_verdadero = match valor_condicion {
                ObjetoKura::Booleano(b) => b,
                ObjetoKura::Entero(n) => n != 0,
                _ => false,
            };

            if es_verdadero {
                for decl in consecuencia {
                    let res = evaluar_declaracion(decl, entorno);
                    if let ObjetoKura::Retorno(_) = res { return res; } // Burbujear el return
                }
            } else if let Some(bloque_else) = alternativa {
                for decl in bloque_else {
                    let res = evaluar_declaracion(decl, entorno);
                    if let ObjetoKura::Retorno(_) = res { return res; }
                }
            }
            ObjetoKura::Nulo
        }
        Declaracion::While { condicion, cuerpo } => {
            loop {
                let valor_condicion = evaluar_expresion(condicion.clone(), entorno);
                let es_verdadero = match valor_condicion {
                    ObjetoKura::Booleano(b) => b,
                    ObjetoKura::Entero(n) => n != 0,
                    _ => false,
                };
                if !es_verdadero { break; }

                for decl in cuerpo.clone() {
                    let res = evaluar_declaracion(decl, entorno);
                    if let ObjetoKura::Retorno(_) = res { return res; }
                }
            }
            ObjetoKura::Nulo
        }
        // --- GOLPE FINAL: LÓGICA DE FUNCIONES ---
        Declaracion::Funcion { nombre, parametros, cuerpo } => {
            let funcion = ObjetoKura::Funcion { parametros, cuerpo };
            entorno.guardar(nombre, funcion);
            ObjetoKura::Nulo
        }
        Declaracion::Return { valor } => {
            let valor_evaluado = evaluar_expresion(valor, entorno);
            ObjetoKura::Retorno(Box::new(valor_evaluado))
        }
        Declaracion::LlamadaSuelta { nombre, argumentos } => {
            evaluar_expresion(Expresion::Llamada { nombre, argumentos }, entorno);
            ObjetoKura::Nulo
        }
        Declaracion::Importar { elementos, archivo } => {
            // 1. Intentamos leer el archivo que nos pidieron
            let contenido = match std::fs::read_to_string(&archivo) {
                Ok(c) => c,
                Err(_) => {
                    println!("Error Kura: No se pudo abrir el modulo '{}'", archivo);
                    return ObjetoKura::Nulo;
                }
            };

            // 2. Creamos un mini-compilador para leer ese archivo
            let lexer = crate::lexer::Lexer::new(&contenido);
            let mut parser = crate::parser::Parser::new(lexer);
            let programa_modulo = parser.parse_programa();

            // 3. Creamos una "burbuja" de memoria temporal y ejecutamos el modulo ahi
            let mut entorno_modulo = Entorno::new();
            evaluar_programa(programa_modulo, &mut entorno_modulo);

            // 4. Extraemos SOLO lo que el usuario pidio y lo pasamos al entorno principal
            for nombre in elementos {
                if let Some(valor) = entorno_modulo.obtener(&nombre) {
                    entorno.guardar(nombre, valor);
                } else {
                    println!("Error Kura: '{}' no fue encontrado dentro de '{}'", nombre, archivo);
                }
            }

            ObjetoKura::Nulo
        },
    }
}

// Nota: Ahora recibe '&mut Entorno' porque evaluar una llamada modifica la memoria temporalmente
fn evaluar_expresion(expresion: Expresion, entorno: &mut Entorno) -> ObjetoKura {
    match expresion {
        Expresion::Entero(n) => ObjetoKura::Entero(n),
        Expresion::Booleano(b) => ObjetoKura::Booleano(b),
        Expresion::Cadena(texto) => ObjetoKura::Cadena(texto),
        Expresion::Identificador(nombre) => entorno.obtener(&nombre).unwrap_or(ObjetoKura::Nulo),
        Expresion::Arreglo(elementos) => {
            let mut evaluados = Vec::new();
            for el in elementos {
                evaluados.push(evaluar_expresion(el, entorno));
            }
            ObjetoKura::Arreglo(evaluados)
        }
        Expresion::Indice { estructura, indice } => {
            let estructura_evaluada = evaluar_expresion(*estructura, entorno);
            let indice_evaluado = evaluar_expresion(*indice, entorno);
            if let (ObjetoKura::Arreglo(arr), ObjetoKura::Entero(i)) = (estructura_evaluada, indice_evaluado) {
                if i >= 0 && (i as usize) < arr.len() {
                    return arr[i as usize].clone();
                }
            }
            ObjetoKura::Nulo
        }
        Expresion::Operacion { izquierda, operador, derecha } => {
            let izq_val = evaluar_expresion(*izquierda, entorno);
            let der_val = evaluar_expresion(*derecha, entorno);

            // Si ambos son números
            if let (ObjetoKura::Entero(i), ObjetoKura::Entero(d)) = (&izq_val, &der_val) {
                return match operador {
                    Token::Suma => ObjetoKura::Entero(i + d),
                    Token::Resta => ObjetoKura::Entero(i - d),
                    Token::Multiplicacion => ObjetoKura::Entero(i * d),
                    Token::Division => ObjetoKura::Entero(i / d),
                    Token::Igualdad => ObjetoKura::Booleano(i == d),
                    Token::MenorQue => ObjetoKura::Booleano(i < d),
                    Token::MayorQue => ObjetoKura::Booleano(i > d),
                    _ => ObjetoKura::Nulo,
                };
            }

            // Si ambos son textos (Strings)
            if let (ObjetoKura::Cadena(i), ObjetoKura::Cadena(d)) = (&izq_val, &der_val) {
                if operador == Token::Igualdad { return ObjetoKura::Booleano(i == d); }
                if operador == Token::Suma { return ObjetoKura::Cadena(format!("{}{}", i, d)); } // ¡Concatena textos!
            }

            ObjetoKura::Nulo
        }
        // --- GOLPE FINAL: EJECUTAR LA LLAMADA A FUNCIÓN ---
        Expresion::Llamada { nombre, argumentos } => {

            // 1. FUNCIONES NATIVAS (Pre-instaladas en Kura)
            if nombre == "len" && argumentos.len() == 1 {
                let arg_eval = evaluar_expresion(argumentos[0].clone(), entorno);
                match arg_eval {
                    // ¡NUEVO!: chars().count() cuenta letras reales, no bytes.
                    ObjetoKura::Cadena(s) => return ObjetoKura::Entero(s.chars().count() as i64),
                    ObjetoKura::Arreglo(arr) => return ObjetoKura::Entero(arr.len() as i64),
                    _ => {
                        println!("Error: 'len' solo acepta cadenas o arreglos.");
                        return ObjetoKura::Nulo;
                    }
                }
            }

            // NUEVA FUNCIÓN: Convierte "50" (Texto) a 50 (Número)
            if nombre == "a_numero" && argumentos.len() == 1 {
                let arg_eval = evaluar_expresion(argumentos[0].clone(), entorno);
                if let ObjetoKura::Cadena(s) = arg_eval {
                    if let Ok(num) = s.parse::<i64>() {
                        return ObjetoKura::Entero(num);
                    }
                }
                return ObjetoKura::Entero(0); // Si falla, devuelve 0
            }

            // NUEVA FUNCIÓN: Convierte 50 (Número) a "50" (Texto)
            if nombre == "a_texto" && argumentos.len() == 1 {
                let arg_eval = evaluar_expresion(argumentos[0].clone(), entorno);
                match arg_eval {
                    ObjetoKura::Entero(n) => return ObjetoKura::Cadena(n.to_string()),
                    ObjetoKura::Booleano(b) => return ObjetoKura::Cadena(b.to_string()),
                    ObjetoKura::Cadena(c) => return ObjetoKura::Cadena(c),
                    _ => return ObjetoKura::Cadena("".to_string()),
                }
            }

            if nombre == "es_letra" && argumentos.len() == 1 {
                if let ObjetoKura::Cadena(s) = evaluar_expresion(argumentos[0].clone(), entorno) {
                    if let Some(c) = s.chars().next() {
                        return ObjetoKura::Booleano(c.is_alphabetic() || c == '_');
                    }
                }
                return ObjetoKura::Booleano(false);
            }

            if nombre == "es_numero" && argumentos.len() == 1 {
                if let ObjetoKura::Cadena(s) = evaluar_expresion(argumentos[0].clone(), entorno) {
                    if let Some(c) = s.chars().next() {
                        return ObjetoKura::Booleano(c.is_numeric());
                    }
                }
                return ObjetoKura::Booleano(false);
            }

            if nombre == "char_at" && argumentos.len() == 2 {
                let texto_eval = evaluar_expresion(argumentos[0].clone(), entorno);
                let indice_eval = evaluar_expresion(argumentos[1].clone(), entorno);

                if let (ObjetoKura::Cadena(s), ObjetoKura::Entero(i)) = (texto_eval, indice_eval) {
                    // ¡NUEVO!: Validamos con seguridad sin usar unwrap() que pueda causar panic
                    if i >= 0 && (i as usize) < s.chars().count() {
                        if let Some(c) = s.chars().nth(i as usize) {
                            return ObjetoKura::Cadena(c.to_string());
                        }
                    }
                    return ObjetoKura::Cadena("".to_string()); // Índice fuera de rango seguro
                } else {
                    println!("Error: 'char_at' espera (cadena, entero).");
                    return ObjetoKura::Nulo;
                }
            }

            // NUEVA FUNCIÓN: Modifica un elemento específico de un arreglo
            if nombre == "reemplazar" && argumentos.len() == 3 {
                let arr_eval = evaluar_expresion(argumentos[0].clone(), entorno);
                let indice_eval = evaluar_expresion(argumentos[1].clone(), entorno);
                let valor_eval = evaluar_expresion(argumentos[2].clone(), entorno);

                if let (ObjetoKura::Arreglo(mut arr), ObjetoKura::Entero(i)) = (arr_eval, indice_eval) {
                    // Verificamos que el índice exista en el arreglo
                    if i >= 0 && (i as usize) < arr.len() {
                        arr[i as usize] = valor_eval; // ¡Sobrescribimos el valor!
                        return ObjetoKura::Arreglo(arr); // Devolvemos el arreglo actualizado
                    } else {
                        println!("Error Kura: Indice {} fuera de limites en 'reemplazar'", i);
                        return ObjetoKura::Nulo;
                    }
                } else {
                    println!("Error Kura: 'reemplazar' espera (Arreglo, Entero, Valor)");
                    return ObjetoKura::Nulo;
                }
            }

            if nombre == "push" && argumentos.len() == 2 {
                let arr_eval = evaluar_expresion(argumentos[0].clone(), entorno);
                let item_eval = evaluar_expresion(argumentos[1].clone(), entorno);
                if let ObjetoKura::Arreglo(mut arr) = arr_eval {
                    arr.push(item_eval);
                    return ObjetoKura::Arreglo(arr); // Devuelve el nuevo arreglo con el elemento adentro
                } else {
                    println!("Error: 'push' espera (arreglo, elemento).");
                    return ObjetoKura::Nulo;
                }
            }

            if nombre == "leer_archivo" && argumentos.len() == 1 {
                let ruta_eval = evaluar_expresion(argumentos[0].clone(), entorno);
                if let ObjetoKura::Cadena(ruta) = ruta_eval {
                    match std::fs::read_to_string(&ruta) {
                        Ok(contenido) => return ObjetoKura::Cadena(contenido),
                        Err(_) => {
                            println!("Error: No se pudo leer el archivo '{}'", ruta);
                            return ObjetoKura::Nulo;
                        }
                    }
                }
            }

            // 2. FUNCIONES DEL USUARIO (Las que tú creas con 'fn' en Kura)
            let funcion = entorno.obtener(&nombre);
            if let Some(ObjetoKura::Funcion { parametros, cuerpo }) = funcion {
                let mut entorno_local = entorno.extend(); // Creamos la memoria local

                // Le pasamos los argumentos a los parámetros
                for (i, arg_expr) in argumentos.into_iter().enumerate() {
                    let arg_evaluado = evaluar_expresion(arg_expr, entorno);
                    if i < parametros.len() {
                        entorno_local.guardar(parametros[i].clone(), arg_evaluado);
                    }
                }

                // Ejecutamos línea por línea
                for decl in cuerpo {
                    let res = evaluar_declaracion(decl, &mut entorno_local);
                    if let ObjetoKura::Retorno(valor) = res {
                        return *valor;
                    }
                }
            }
            ObjetoKura::Nulo
        }
    }
}
// Nueva función para imprimir objetos anidados infinitamente
pub fn imprimir_objeto(obj: &ObjetoKura) {
    match obj {
        ObjetoKura::Entero(n) => print!("{}", n),
        ObjetoKura::Booleano(b) => print!("{}", b),
        ObjetoKura::Cadena(t) => print!("\"{}\"", t),
        ObjetoKura::Arreglo(arr) => {
            print!("[");
            for (i, el) in arr.iter().enumerate() {
                imprimir_objeto(el);
                if i < arr.len() - 1 { print!(", "); }
            }
            print!("]");
        }
        ObjetoKura::Nulo => print!("null"),
        ObjetoKura::Funcion { .. } => print!("[Funcion]"),
        ObjetoKura::Retorno(val) => imprimir_objeto(val),
    }
}