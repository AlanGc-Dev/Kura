

use std::collections::HashMap;
use crate::ast::{Programa, Declaracion, Expresion, CasoMatch, Pattern, VarianteEnum};
use crate::token::Token;
use crate::types::TipoKura;
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum ObjetoKura {
    Entero(i64),
    Booleano(bool),
    Cadena(String),
    Arreglo(Vec<ObjetoKura>),
    Funcion {
        parametros: Vec<(String, Option<TipoKura>)>,
        retorno: Option<TipoKura>,
        cuerpo: Vec<Declaracion>
    },
    Retorno(Box<ObjetoKura>), // <-- PARA ATRAPAR EL RETURN
    Variante {                  // <-- NUEVO: Para instancias de enum
        nombre_enum: String,
        nombre_variante: String,
        valores: Vec<ObjetoKura>,
    },
    Nulo,
    Diccionario(HashMap<String, ObjetoKura>), // <-- NUEVO
    Break,
}

// Estructura para guardar definiciones de enum
#[derive(Debug, Clone)]
pub struct DefinicionEnum {
    pub nombre: String,
    pub variantes: HashMap<String, usize>, // nombre_variante -> num_campos
}

#[derive(Clone)]
pub struct Entorno {
    pub variables: HashMap<String, ObjetoKura>,
    pub enums: HashMap<String, DefinicionEnum>, // <-- NUEVO: Guardar definiciones de enum
}

impl Entorno {
    pub fn new() -> Self {
        Entorno { 
            variables: HashMap::new(),
            enums: HashMap::new(),
        }
    }
    // Clona la memoria para usarla dentro de una función
    pub fn extend(&self) -> Self {
        Entorno { 
            variables: self.variables.clone(),
            enums: self.enums.clone(),
        }
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
        Declaracion::Let { nombre, tipo, valor, .. } => {
            let valor_evaluado = evaluar_expresion(valor, entorno);
            if let Some(tipo_esperado) = tipo {
                let tipo_real = TipoKura::de_objeto(&valor_evaluado);
                if tipo_esperado != tipo_real {
                    println!("Error de tipo: Variable '{}' declarada como {:?} pero el valor es {:?}", nombre, tipo_esperado, tipo_real);
                    return ObjetoKura::Nulo;
                }
            }
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
        Declaracion::Break => ObjetoKura::Break,
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
                    // Atrapamos Return O Break y lo burbujeamos
                    if matches!(res, ObjetoKura::Retorno(_) | ObjetoKura::Break) { return res; }
                }
            } else if let Some(bloque_else) = alternativa {
                for decl in bloque_else {
                    let res = evaluar_declaracion(decl, entorno);
                    if matches!(res, ObjetoKura::Retorno(_) | ObjetoKura::Break) { return res; }
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

                let mut romper_ciclo = false;
                for decl in cuerpo.clone() {
                    let res = evaluar_declaracion(decl, entorno);
                    if matches!(res, ObjetoKura::Retorno(_)) { return res; }
                    if matches!(res, ObjetoKura::Break) {
                        romper_ciclo = true;
                        break; // Rompe el for
                    }
                }
                if romper_ciclo { break; } // Rompe el loop principal de Rust
            }
            ObjetoKura::Nulo
        }
        Declaracion::For { variable, iterable, cuerpo } => {
            let iterable_evaluado = evaluar_expresion(iterable, entorno);
            if let ObjetoKura::Arreglo(arr) = iterable_evaluado {
                for elemento in arr {
                    entorno.guardar(variable.clone(), elemento);
                    let mut romper_ciclo = false;
                    for decl in cuerpo.clone() {
                        let res = evaluar_declaracion(decl, entorno);
                        if matches!(res, ObjetoKura::Retorno(_)) { return res; }
                        if matches!(res, ObjetoKura::Break) {
                            romper_ciclo = true;
                            break;
                        }
                    }
                    if romper_ciclo { break; }
                }
            }
            ObjetoKura::Nulo
        }

        // --- GOLPE FINAL: LÓGICA DE FUNCIONES ---
        Declaracion::Funcion { nombre, parametros, retorno, cuerpo } => {
            let funcion = ObjetoKura::Funcion { parametros, retorno, cuerpo };
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
        Declaracion::Enum { nombre, variantes } => {
            // Crear definición del enum
            let mut vars_map = HashMap::new();
            for VarianteEnum { nombre: var_name, campos } in variantes {
                vars_map.insert(var_name, campos.len());
            }
            let def = DefinicionEnum {
                nombre: nombre.clone(),
                variantes: vars_map,
            };
            entorno.enums.insert(nombre, def);
            ObjetoKura::Nulo
        }
        Declaracion::Match { valor, casos } => {
            let valor_evaluado = evaluar_expresion(valor, entorno);
            
            for CasoMatch { patron, cuerpo } in casos {
                if patron_coincide(&patron, &valor_evaluado, entorno) {
                    // Vincular variables del patrón
                    vincular_patron(&patron, &valor_evaluado, entorno);
                    
                    // Ejecutar el cuerpo
                    for decl in cuerpo {
                        let res = evaluar_declaracion(decl, entorno);
                        if matches!(res, ObjetoKura::Retorno(_) | ObjetoKura::Break) {
                            return res;
                        }
                    }
                    break;
                }
            }
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

            // Si es un Arreglo...
            if let (ObjetoKura::Arreglo(arr), ObjetoKura::Entero(i)) = (&estructura_evaluada, &indice_evaluado) {
                if *i >= 0 && (*i as usize) < arr.len() {
                    return arr[*i as usize].clone();
                }
            }
            // ¡NUEVO!: Si es un Diccionario...
            if let (ObjetoKura::Diccionario(dic), ObjetoKura::Cadena(clave)) = (&estructura_evaluada, &indice_evaluado) {
                return dic.get(clave).cloned().unwrap_or(ObjetoKura::Nulo);
            }
            ObjetoKura::Nulo
        }
        Expresion::Operacion { izquierda, operador, derecha } => {
            let izq_val = evaluar_expresion(*izquierda, entorno);
            let der_val = evaluar_expresion(*derecha, entorno);

            // ¡NUEVO!: Operadores lógicos
            if operador == Token::And {
                if let (ObjetoKura::Booleano(b1), ObjetoKura::Booleano(b2)) = (&izq_val, &der_val) {
                    return ObjetoKura::Booleano(*b1 && *b2);
                }
            }
            if operador == Token::Or {
                if let (ObjetoKura::Booleano(b1), ObjetoKura::Booleano(b2)) = (&izq_val, &der_val) {
                    return ObjetoKura::Booleano(*b1 || *b2);
                }
            }
            // Si ambos son números
            if let (ObjetoKura::Entero(i), ObjetoKura::Entero(d)) = (&izq_val, &der_val) {
                return match operador {
                    Token::Suma => ObjetoKura::Entero(i + d),
                    Token::Resta => ObjetoKura::Entero(i - d),
                    Token::Multiplicacion => ObjetoKura::Entero(i * d),
                    Token::Division => ObjetoKura::Entero(i / d),
                    Token::Modulo => ObjetoKura::Entero(i % d),
                    Token::Potencia => ObjetoKura::Entero(i.pow(*d as u32)),
                    Token::Igualdad => ObjetoKura::Booleano(i == d),
                    Token::Diferente => ObjetoKura::Booleano(i != d),
                    Token::MenorQue => ObjetoKura::Booleano(i < d),
                    Token::MayorQue => ObjetoKura::Booleano(i > d),
                    Token::MenorIgual => ObjetoKura::Booleano(i <= d),
                    Token::MayorIgual => ObjetoKura::Booleano(i >= d),
                    _ => ObjetoKura::Nulo,
                };
            }

            // Si ambos son textos (Strings)
            if let (ObjetoKura::Cadena(i), ObjetoKura::Cadena(d)) = (&izq_val, &der_val) {
                return match operador {
                    Token::Igualdad => ObjetoKura::Booleano(i == d),
                    Token::Diferente => ObjetoKura::Booleano(i != d),
                    Token::Suma => ObjetoKura::Cadena(format!("{}{}", i, d)), // ¡Concatena textos!
                    _ => ObjetoKura::Nulo,
                };
            }

            ObjetoKura::Nulo
        }
        Expresion::Diccionario(pares) => {
            let mut mapa = HashMap::new();
            for (clave, valor_expr) in pares {
                let valor_evaluado = evaluar_expresion(valor_expr, entorno);
                mapa.insert(clave, valor_evaluado);
            }
            ObjetoKura::Diccionario(mapa)
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

                if let (ObjetoKura::Arreglo(mut arr), ObjetoKura::Entero(i)) = (arr_eval.clone(), indice_eval.clone()) {
                    // Verificamos que el índice exista en el arreglo
                    if i >= 0 && (i as usize) < arr.len() {
                        arr[i as usize] = valor_eval; // ¡Sobrescribimos el valor!
                        return ObjetoKura::Arreglo(arr); // Devolvemos el arreglo actualizado
                    } else {
                        println!("Error Kura: Indice {} fuera de limites en 'reemplazar'", i);
                        return ObjetoKura::Nulo;
                    }
                }else if let (ObjetoKura::Diccionario(mut dic), ObjetoKura::Cadena(clave)) = (arr_eval, indice_eval) {
                    dic.insert(clave, valor_eval);
                    return ObjetoKura::Diccionario(dic);
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

            if nombre == "pop" && argumentos.len() == 1 {
                let arr_eval = evaluar_expresion(argumentos[0].clone(), entorno);
                if let ObjetoKura::Arreglo(mut arr) = arr_eval {
                    if let Some(elem) = arr.pop() {
                        return elem; // Devuelve el elemento removido
                    } else {
                        println!("Error: 'pop' en arreglo vacío.");
                        return ObjetoKura::Nulo;
                    }
                } else {
                    println!("Error: 'pop' espera un arreglo.");
                    return ObjetoKura::Nulo;
                }
            }

            if nombre == "sort" && argumentos.len() == 1 {
                let arr_eval = evaluar_expresion(argumentos[0].clone(), entorno);
                if let ObjetoKura::Arreglo(arr) = arr_eval {
                    // Ordenar solo si todos son números
                    let mut nums = Vec::new();
                    for elem in &arr {
                        if let ObjetoKura::Entero(n) = elem {
                            nums.push(*n);
                        } else {
                            println!("Error: 'sort' solo funciona con arreglos de números.");
                            return ObjetoKura::Nulo;
                        }
                    }
                    nums.sort();
                    let sorted = nums.into_iter().map(ObjetoKura::Entero).collect();
                    return ObjetoKura::Arreglo(sorted);
                } else {
                    println!("Error: 'sort' espera un arreglo.");
                    return ObjetoKura::Nulo;
                }
            }

            // ¡NUEVO!: Convertir números y booleanos a texto
            if nombre == "a_cadena" && argumentos.len() == 1 {
                let arg_eval = evaluar_expresion(argumentos[0].clone(), entorno);
                match arg_eval {
                    ObjetoKura::Entero(n) => return ObjetoKura::Cadena(n.to_string()),
                    ObjetoKura::Booleano(b) => return ObjetoKura::Cadena(b.to_string()),
                    ObjetoKura::Cadena(c) => return ObjetoKura::Cadena(c),
                    _ => return ObjetoKura::Cadena("null".to_string()),
                }
            }

            // ¡NUEVO!: Escribir archivos nativos en el disco duro
            if nombre == "escribir_archivo" && argumentos.len() == 2 {
                let ruta_eval = evaluar_expresion(argumentos[0].clone(), entorno);
                let contenido_eval = evaluar_expresion(argumentos[1].clone(), entorno);

                if let (ObjetoKura::Cadena(ruta), ObjetoKura::Cadena(contenido)) = (ruta_eval, contenido_eval) {
                    // --- AÑADE ESTA LÍNEA DE AQUÍ ABAJO ---
                    println!(">>> RUST: Guardando {} bytes en {}", contenido.len(), ruta);

                    if std::fs::write(&ruta, contenido).is_ok() {
                        return ObjetoKura::Booleano(true);
                    }
                }
                return ObjetoKura::Nulo;
            }

            // 1.5 CONSTRUCTORES DE ENUM (Ok(valor), Err(error), etc)
            // Búscar si nombre es un constructor conocido
            // Primero verificar si existe el constructor
            if let Some(def_enum) = entorno.enums.iter().find(|(_, e)| e.variantes.contains_key(&nombre)).map(|(_, e)| e.clone()) {
                let mut valores = Vec::new();
                for arg_expr in argumentos {
                    valores.push(evaluar_expresion(arg_expr, entorno));
                }
                return ObjetoKura::Variante {
                    nombre_enum: def_enum.nombre.clone(),
                    nombre_variante: nombre.clone(),
                    valores,
                };
            }

            // 2. FUNCIONES DEL USUARIO
            let funcion = entorno.obtener(&nombre);
            if let Some(ObjetoKura::Funcion { parametros, retorno, cuerpo }) = funcion {

                if argumentos.len() != parametros.len() {
                    println!("Error Kura: La funcion '{}' espera {} argumentos, recibio {}", nombre, parametros.len(), argumentos.len());
                    return ObjetoKura::Nulo;
                }

                let mut entorno_local = entorno.extend();
                for (i, arg_expr) in argumentos.into_iter().enumerate() {
                    let arg_evaluado = evaluar_expresion(arg_expr, entorno);

                    // Validar tipo del parámetro opcionalmente
                    if let Some(tipo_esperado) = &parametros[i].1 {
                        let tipo_real = TipoKura::de_objeto(&arg_evaluado);
                        if *tipo_esperado != tipo_real && *tipo_esperado != TipoKura::Desconocido {
                            println!("Error Kura de Tipos: El parametro '{}' espera un {:?}, pero recibió un {:?}", parametros[i].0, tipo_esperado, tipo_real);
                            return ObjetoKura::Nulo;
                        }
                    }

                    entorno_local.guardar(parametros[i].0.clone(), arg_evaluado);
                }

                let mut valor_retornado = ObjetoKura::Nulo;
                for decl in cuerpo {
                    let res = evaluar_declaracion(decl, &mut entorno_local);
                    if let ObjetoKura::Retorno(valor) = res {
                        valor_retornado = *valor;
                        break;
                    }
                }

                // Validar retorno opcionalmente
                if let Some(tipo_ret) = retorno {
                    let tipo_real_ret = TipoKura::de_objeto(&valor_retornado);
                    if tipo_ret != tipo_real_ret && tipo_ret != TipoKura::Desconocido {
                        println!("Error Kura de Tipos: La funcion '{}' debía retornar {:?}, pero retornó {:?}", nombre, tipo_ret, tipo_real_ret);
                    }
                }

                return valor_retornado;
            } else {
                println!("Error Kura: La funcion '{}' no existe. ¿Olvidaste el import?", nombre);
            }
            ObjetoKura::Nulo
        }
        Expresion::ConstructorEnum { variante, valores } => {
            // Los constructores de enum se manejan como llamadas normales
            let mut valores_evaluados = Vec::new();
            for val in valores {
                valores_evaluados.push(evaluar_expresion(val, entorno));
            }
            // Buscar si es un constructor de enum válido
            for (_, def_enum) in &entorno.enums {
                if def_enum.variantes.contains_key(&variante) {
                    return ObjetoKura::Variante {
                        nombre_enum: def_enum.nombre.clone(),
                        nombre_variante: variante.clone(),
                        valores: valores_evaluados,
                    };
                }
            }
            // Si no lo encuentra, error
            println!("Error: Variante de enum '{}' no encontrada", variante);
            ObjetoKura::Nulo
        }
    }
}

// Métodos auxiliares para pattern matching
fn patron_coincide(patron: &Pattern, valor: &ObjetoKura, _entorno: &Entorno) -> bool {
    match patron {
        Pattern::Comodin => true,
        Pattern::Identificador(_) => true,
        Pattern::Variante { nombre, bindings } => {
            if let ObjetoKura::Variante { nombre_variante, valores, .. } = valor {
                nombre == nombre_variante && valores.len() == bindings.len()
            } else {
                false
            }
        }
    }
}

fn vincular_patron(patron: &Pattern, valor: &ObjetoKura, entorno: &mut Entorno) {
    match patron {
        Pattern::Identificador(nombre) => {
            entorno.guardar(nombre.clone(), valor.clone());
        }
        Pattern::Variante { bindings, .. } => {
            if let ObjetoKura::Variante { valores, .. } = valor {
                for (i, binding) in bindings.iter().enumerate() {
                    if i < valores.len() {
                        entorno.guardar(binding.clone(), valores[i].clone());
                    }
                }
            }
        }
        Pattern::Comodin => {}
    }
}

// Nueva función para imprimir objetos anidados infinitamente
pub fn imprimir_objeto(obj: &ObjetoKura) {
    match obj {
        ObjetoKura::Entero(n) => print!("{}", n),
        ObjetoKura::Booleano(b) => print!("{}", b),
        ObjetoKura::Cadena(t) => print!("{}", t),
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
        ObjetoKura::Variante { nombre_enum: _, nombre_variante, valores } => {
            print!("{}(", nombre_variante);
            for (i, val) in valores.iter().enumerate() {
                imprimir_objeto(val);
                if i < valores.len() - 1 { print!(", "); }
            }
            print!(")");
        }
        ObjetoKura::Retorno(val) => imprimir_objeto(val),
        ObjetoKura::Break => print!("break"),
        ObjetoKura::Diccionario(dic) => {
            print!("{{");
            let mut count = 0;
            let len = dic.len();
            for (clave, valor) in dic {
                print!("\"{}\": ", clave);
                imprimir_objeto(valor);
                count += 1;
                if count < len { print!(", "); }
            }
            print!("}}");
        }
    }
}