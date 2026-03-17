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
            match valor_evaluado {
                ObjetoKura::Entero(n) => println!("{}", n),
                ObjetoKura::Booleano(b) => println!("{}", b),
                ObjetoKura::Cadena(texto) => println!("{}", texto),
                ObjetoKura::Arreglo(arr) => {
                    print!("[");
                    for (i, el) in arr.iter().enumerate() {
                        match el {
                            ObjetoKura::Entero(n) => print!("{}", n),
                            ObjetoKura::Booleano(b) => print!("{}", b),
                            ObjetoKura::Cadena(t) => print!("\"{}\"", t),
                            _ => print!("null"),
                        }
                        if i < arr.len() - 1 { print!(", "); }
                    }
                    println!("]");
                }
                ObjetoKura::Nulo => println!("null"),
                ObjetoKura::Funcion { .. } => println!("[Funcion Interna]"),
                ObjetoKura::Retorno(_) => {}
            }
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
            if let (ObjetoKura::Entero(i), ObjetoKura::Entero(d)) = (izq_val, der_val) {
                match operador {
                    Token::Suma => ObjetoKura::Entero(i + d),
                    Token::Resta => ObjetoKura::Entero(i - d),
                    Token::Multiplicacion => ObjetoKura::Entero(i * d),
                    Token::Division => ObjetoKura::Entero(i / d),
                    Token::Igualdad => ObjetoKura::Booleano(i == d),
                    Token::MenorQue => ObjetoKura::Booleano(i < d),
                    Token::MayorQue => ObjetoKura::Booleano(i > d),
                    _ => ObjetoKura::Nulo,
                }
            } else {
                ObjetoKura::Nulo
            }
        }
        // --- GOLPE FINAL: EJECUTAR LA LLAMADA A FUNCIÓN ---
        Expresion::Llamada { nombre, argumentos } => {
            let funcion = entorno.obtener(&nombre);
            if let Some(ObjetoKura::Funcion { parametros, cuerpo }) = funcion {
                let mut entorno_local = entorno.extend(); // Creamos la memoria local

                // Le pasamos los argumentos a los parámetros (ej: vida = 100)
                for (i, arg_expr) in argumentos.into_iter().enumerate() {
                    let arg_evaluado = evaluar_expresion(arg_expr, entorno);
                    if i < parametros.len() {
                        entorno_local.guardar(parametros[i].clone(), arg_evaluado);
                    }
                }

                // Ejecutamos línea por línea el cuerpo de la función
                for decl in cuerpo {
                    let res = evaluar_declaracion(decl, &mut entorno_local);
                    // Si encontramos un return, detenemos la función y devolvemos el valor
                    if let ObjetoKura::Retorno(valor) = res {
                        return *valor;
                    }
                }
            }
            ObjetoKura::Nulo // Si no hay return, devuelve nulo (Void)
        }
    }
}