use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
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
    Retorno(Box<ObjetoKura>),
    Variante {
        nombre_enum: String,
        nombre_variante: String,
        valores: Vec<ObjetoKura>,
    },
    Nulo,
    Diccionario(HashMap<String, ObjetoKura>),
    Break,
    InstanciaStruct {
        nombre: String,
        campos: Rc<RefCell<HashMap<String, ObjetoKura>>>,
    },
}

#[derive(Debug, Clone)]
pub struct DefinicionEnum {
    pub nombre: String,
    pub variantes: HashMap<String, usize>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DefinicionStruct {
    pub nombre: String,
    pub campos: HashMap<String, TipoKura>,
    pub metodos: HashMap<String, ObjetoKura>,
}

// --- EL NUEVO ENTORNO INTELIGENTE ---
#[derive(Clone)]
pub struct Entorno {
    pub variables: HashMap<String, ObjetoKura>,
    pub enums: HashMap<String, DefinicionEnum>,
    pub padre: Option<Rc<RefCell<Entorno>>>, // <--- El hilo hacia el scope superior
    pub structs: HashMap<String, DefinicionStruct>,
}

impl Entorno {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Entorno {
            variables: HashMap::new(),
            enums: HashMap::new(),
            padre: None,
            structs: Default::default(),
        }))
    }

    pub fn extend(padre: Rc<RefCell<Self>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Entorno {
            variables: HashMap::new(),
            enums: HashMap::new(),
            padre: Some(padre), // Solo guardamos el puntero, cero copias!
            structs: Default::default(),
        }))
    }

    pub fn guardar(&mut self, nombre: String, valor: ObjetoKura) {
        self.variables.insert(nombre, valor);
    }

    pub fn reasignar(&mut self, nombre: &str, valor: ObjetoKura) -> bool {
        if self.variables.contains_key(nombre) {
            self.variables.insert(nombre.to_string(), valor);
            true
        } else if let Some(padre) = &self.padre {
            padre.borrow_mut().reasignar(nombre, valor)
        } else {
            false
        }
    }

    pub fn obtener(&self, nombre: &str) -> Option<ObjetoKura> {
        if let Some(valor) = self.variables.get(nombre) {
            Some(valor.clone())
        } else if let Some(padre) = &self.padre {
            padre.borrow().obtener(nombre) // Busca hacia arriba
        } else {
            None
        }
    }
}

// Nota: Ahora pasamos Rc<RefCell<Entorno>> en lugar de &mut Entorno
pub fn evaluar_programa(programa: Programa, entorno: Rc<RefCell<Entorno>>) {
    for declaracion in programa.declaraciones {
        let res = evaluar_declaracion(declaracion, Rc::clone(&entorno));
        if let ObjetoKura::Retorno(_) = res { break; }
    }
}

fn evaluar_declaracion(declaracion: Declaracion, entorno: Rc<RefCell<Entorno>>) -> ObjetoKura {
    match declaracion {
        Declaracion::Let { nombre, tipo, valor, .. } => {
            let valor_evaluado = evaluar_expresion(valor, Rc::clone(&entorno));
            if let Some(tipo_esperado) = tipo {
                let tipo_real = TipoKura::de_objeto(&valor_evaluado);
                if tipo_esperado != tipo_real {
                    println!("Error de tipo: Variable '{}' declarada como {:?} pero el valor es {:?}", nombre, tipo_esperado, tipo_real);
                    return ObjetoKura::Nulo;
                }
            }
            entorno.borrow_mut().guardar(nombre, valor_evaluado);
            ObjetoKura::Nulo
        }
        Declaracion::Print { valor } => {
            let valor_evaluado = evaluar_expresion(valor, Rc::clone(&entorno));
            imprimir_objeto(&valor_evaluado);
            println!();
            ObjetoKura::Nulo
        }
        Declaracion::Reasignacion { nombre, valor } => {
            let valor_evaluado = evaluar_expresion(valor, Rc::clone(&entorno));
            if !entorno.borrow_mut().reasignar(&nombre, valor_evaluado) {
                println!("Error: La variable '{}' no ha sido declarada.", nombre);
            }
            ObjetoKura::Nulo
        }
        Declaracion::Break => ObjetoKura::Break,
        Declaracion::If { condicion, consecuencia, alternativa } => {
            let valor_condicion = evaluar_expresion(condicion, Rc::clone(&entorno));
            let es_verdadero = match valor_condicion {
                ObjetoKura::Booleano(b) => b,
                ObjetoKura::Entero(n) => n != 0,
                _ => false,
            };

            if es_verdadero {
                let entorno_if = Entorno::extend(Rc::clone(&entorno)); // Scope del IF
                for decl in consecuencia {
                    let res = evaluar_declaracion(decl, Rc::clone(&entorno_if));
                    if matches!(res, ObjetoKura::Retorno(_) | ObjetoKura::Break) { return res; }
                }
            } else if let Some(bloque_else) = alternativa {
                let entorno_else = Entorno::extend(Rc::clone(&entorno));
                for decl in bloque_else {
                    let res = evaluar_declaracion(decl, Rc::clone(&entorno_else));
                    if matches!(res, ObjetoKura::Retorno(_) | ObjetoKura::Break) { return res; }
                }
            }
            ObjetoKura::Nulo
        }
        Declaracion::While { condicion, cuerpo } => {
            loop {
                let valor_condicion = evaluar_expresion(condicion.clone(), Rc::clone(&entorno));
                let es_verdadero = match valor_condicion {
                    ObjetoKura::Booleano(b) => b,
                    ObjetoKura::Entero(n) => n != 0,
                    _ => false,
                };
                if !es_verdadero { break; }

                let mut romper_ciclo = false;
                let entorno_while = Entorno::extend(Rc::clone(&entorno));
                for decl in cuerpo.clone() {
                    let res = evaluar_declaracion(decl, Rc::clone(&entorno_while));
                    if matches!(res, ObjetoKura::Retorno(_)) { return res; }
                    if matches!(res, ObjetoKura::Break) {
                        romper_ciclo = true;
                        break;
                    }
                }
                if romper_ciclo { break; }
            }
            ObjetoKura::Nulo
        }
        Declaracion::For { variable, iterable, cuerpo } => {
            let iterable_evaluado = evaluar_expresion(iterable, Rc::clone(&entorno));
            if let ObjetoKura::Arreglo(arr) = iterable_evaluado {
                for elemento in arr {
                    let entorno_for = Entorno::extend(Rc::clone(&entorno));
                    entorno_for.borrow_mut().guardar(variable.clone(), elemento);

                    let mut romper_ciclo = false;
                    for decl in cuerpo.clone() {
                        let res = evaluar_declaracion(decl, Rc::clone(&entorno_for));
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
        Declaracion::Funcion { nombre, parametros, retorno, cuerpo } => {
            let funcion = ObjetoKura::Funcion { parametros, retorno, cuerpo };
            entorno.borrow_mut().guardar(nombre, funcion);
            ObjetoKura::Nulo
        }
        Declaracion::Return { valor } => {
            let valor_evaluado = evaluar_expresion(valor, Rc::clone(&entorno));
            ObjetoKura::Retorno(Box::new(valor_evaluado))
        }
        Declaracion::LlamadaSuelta { nombre, argumentos } => {
            evaluar_expresion(Expresion::Llamada { nombre, argumentos }, Rc::clone(&entorno));
            ObjetoKura::Nulo
        }
        Declaracion::Enum { nombre, variantes } => {
            let mut vars_map = HashMap::new();
            for VarianteEnum { nombre: var_name, campos } in variantes {
                vars_map.insert(var_name, campos.len());
            }
            let def = DefinicionEnum { nombre: nombre.clone(), variantes: vars_map };
            entorno.borrow_mut().enums.insert(nombre, def);
            ObjetoKura::Nulo
        }
        Declaracion::Match { valor, casos } => {
            let valor_evaluado = evaluar_expresion(valor, Rc::clone(&entorno));

            for CasoMatch { patron, cuerpo } in casos {
                if patron_coincide(&patron, &valor_evaluado) {
                    let entorno_match = Entorno::extend(Rc::clone(&entorno));
                    vincular_patron(&patron, &valor_evaluado, Rc::clone(&entorno_match));

                    for decl in cuerpo {
                        let res = evaluar_declaracion(decl, Rc::clone(&entorno_match));
                        if matches!(res, ObjetoKura::Retorno(_) | ObjetoKura::Break) { return res; }
                    }
                    break;
                }
            }
            ObjetoKura::Nulo
        }
        Declaracion::Importar { elementos, archivo } => {
            let contenido = match std::fs::read_to_string(&archivo) {
                Ok(c) => c,
                Err(_) => {
                    println!("Error Kura: No se pudo abrir el modulo '{}'", archivo);
                    return ObjetoKura::Nulo;
                }
            };

            let lexer = crate::lexer::Lexer::new(&contenido);
            let mut parser = crate::parser::Parser::new(lexer);
            let programa_modulo = parser.parse_programa();

            let entorno_modulo = Entorno::new();
            evaluar_programa(programa_modulo, Rc::clone(&entorno_modulo));

            for nombre in elementos {
                if let Some(valor) = entorno_modulo.borrow().obtener(&nombre) {
                    entorno.borrow_mut().guardar(nombre, valor);
                } else {
                    println!("Error Kura: '{}' no fue encontrado dentro de '{}'", nombre, archivo);
                }
            }
            ObjetoKura::Nulo
        },
        Declaracion::Struct { nombre, campos, metodos } => {
            let mut campos_map = HashMap::new();
            for (c_nom, c_tipo) in campos {
                campos_map.insert(c_nom, c_tipo);
            }

            // --- NUEVO: Extraer funciones y guardarlas ---
            let mut metodos_map = HashMap::new();
            for m in metodos {
                if let Declaracion::Funcion { nombre: n_metodo, parametros, retorno, cuerpo } = m {
                    metodos_map.insert(n_metodo, ObjetoKura::Funcion { parametros, retorno, cuerpo });
                }
            }

            let def = DefinicionStruct { nombre: nombre.clone(), campos: campos_map, metodos: metodos_map };
            entorno.borrow_mut().structs.insert(nombre, def);
            ObjetoKura::Nulo
        }
        Declaracion::ReasignacionPropiedad { objeto, propiedad, valor } => {
            let valor_evaluado = evaluar_expresion(valor, Rc::clone(&entorno));
            if let Some(ObjetoKura::InstanciaStruct { campos, .. }) = entorno.borrow().obtener(&objeto) {
                campos.borrow_mut().insert(propiedad, valor_evaluado);
            } else {
                println!("Error: La variable '{}' no es un Struct o no existe", objeto);
            }
            ObjetoKura::Nulo
        }
    }
}

fn evaluar_expresion(expresion: Expresion, entorno: Rc<RefCell<Entorno>>) -> ObjetoKura {
    match expresion {
        Expresion::Entero(n) => ObjetoKura::Entero(n),
        Expresion::Booleano(b) => ObjetoKura::Booleano(b),
        Expresion::Cadena(texto) => ObjetoKura::Cadena(texto),
        Expresion::Identificador(nombre) => entorno.borrow().obtener(&nombre).unwrap_or(ObjetoKura::Nulo),
        Expresion::Arreglo(elementos) => {
            let mut evaluados = Vec::new();
            for el in elementos {
                evaluados.push(evaluar_expresion(el, Rc::clone(&entorno)));
            }
            ObjetoKura::Arreglo(evaluados)
        }
        Expresion::LlamadaMetodo { objeto, metodo, argumentos } => {
            let obj_evaluado = evaluar_expresion(*objeto, Rc::clone(&entorno));

            if let ObjetoKura::InstanciaStruct { nombre: nombre_struct, .. } = &obj_evaluado {
                let def_struct = entorno.borrow().structs.get(nombre_struct).cloned();

                if let Some(def) = def_struct {
                    if let Some(ObjetoKura::Funcion { parametros, cuerpo, .. }) = def.metodos.get(&metodo) {

                        if parametros.is_empty() {
                            println!("Error: El metodo '{}' debe tener 'self' como primer parametro.", metodo);
                            return ObjetoKura::Nulo;
                        }

                        let entorno_local = Entorno::extend(Rc::clone(&entorno));

                        // INYECTAR 'self'
                        let nombre_self = parametros[0].0.clone();
                        entorno_local.borrow_mut().guardar(nombre_self, obj_evaluado.clone());

                        // Pasar los demás argumentos
                        for (i, arg_expr) in argumentos.into_iter().enumerate() {
                            let arg_val = evaluar_expresion(arg_expr, Rc::clone(&entorno));
                            entorno_local.borrow_mut().guardar(parametros[i + 1].0.clone(), arg_val);
                        }

                        let mut valor_retornado = ObjetoKura::Nulo;
                        for decl in cuerpo.clone() {
                            let res = evaluar_declaracion(decl, Rc::clone(&entorno_local));
                            if let ObjetoKura::Retorno(valor) = res {
                                valor_retornado = *valor;
                                break;
                            }
                        }
                        return valor_retornado;
                    } else { println!("Error: El metodo '{}' no existe en '{}'", metodo, nombre_struct); }
                }
            } else { println!("Error: Solo puedes llamar metodos en un Struct"); }
            ObjetoKura::Nulo
        }
        Expresion::Indice { estructura, indice } => {
            let estructura_evaluada = evaluar_expresion(*estructura, Rc::clone(&entorno));
            let indice_evaluado = evaluar_expresion(*indice, Rc::clone(&entorno));

            if let (ObjetoKura::Arreglo(arr), ObjetoKura::Entero(i)) = (&estructura_evaluada, &indice_evaluado) {
                if *i >= 0 && (*i as usize) < arr.len() {
                    return arr[*i as usize].clone();
                }
            }
            if let (ObjetoKura::Diccionario(dic), ObjetoKura::Cadena(clave)) = (&estructura_evaluada, &indice_evaluado) {
                return dic.get(clave).cloned().unwrap_or(ObjetoKura::Nulo);
            }
            ObjetoKura::Nulo
        }
        Expresion::Operacion { izquierda, operador, derecha } => {
            let izq_val = evaluar_expresion(*izquierda, Rc::clone(&entorno));
            let der_val = evaluar_expresion(*derecha, Rc::clone(&entorno));

            if operador == Token::And {
                if let (ObjetoKura::Booleano(b1), ObjetoKura::Booleano(b2)) = (&izq_val, &der_val) { return ObjetoKura::Booleano(*b1 && *b2); }
            }
            if operador == Token::Or {
                if let (ObjetoKura::Booleano(b1), ObjetoKura::Booleano(b2)) = (&izq_val, &der_val) { return ObjetoKura::Booleano(*b1 || *b2); }
            }
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
            if let (ObjetoKura::Cadena(i), ObjetoKura::Cadena(d)) = (&izq_val, &der_val) {
                return match operador {
                    Token::Igualdad => ObjetoKura::Booleano(i == d),
                    Token::Diferente => ObjetoKura::Booleano(i != d),
                    Token::Suma => ObjetoKura::Cadena(format!("{}{}", i, d)),
                    _ => ObjetoKura::Nulo,
                };
            }
            ObjetoKura::Nulo
        }
        Expresion::InstanciaStruct { nombre, campos } => {
            let mut campos_map = HashMap::new();
            for (c_nom, c_expr) in campos {
                let val = evaluar_expresion(c_expr, Rc::clone(&entorno));
                campos_map.insert(c_nom, val);
            }
            ObjetoKura::InstanciaStruct {
                nombre,
                campos: Rc::new(RefCell::new(campos_map)),
            }
        }
        Expresion::AccesoPropiedad { objeto, propiedad } => {
            let obj_eval = evaluar_expresion(*objeto, Rc::clone(&entorno));
            if let ObjetoKura::InstanciaStruct { campos, .. } = obj_eval {
                return campos.borrow().get(&propiedad).cloned().unwrap_or(ObjetoKura::Nulo);
            }
            ObjetoKura::Nulo
        }
        Expresion::Diccionario(pares) => {
            let mut mapa = HashMap::new();
            for (clave, valor_expr) in pares {
                let valor_evaluado = evaluar_expresion(valor_expr, Rc::clone(&entorno));
                mapa.insert(clave, valor_evaluado);
            }
            ObjetoKura::Diccionario(mapa)
        }
        Expresion::Llamada { nombre, argumentos } => {

            if nombre == "len" && argumentos.len() == 1 {
                let arg_eval = evaluar_expresion(argumentos[0].clone(), Rc::clone(&entorno));
                match arg_eval {
                    ObjetoKura::Cadena(s) => return ObjetoKura::Entero(s.chars().count() as i64),
                    ObjetoKura::Arreglo(arr) => return ObjetoKura::Entero(arr.len() as i64),
                    _ => {
                        println!("Error: 'len' solo acepta cadenas o arreglos.");
                        return ObjetoKura::Nulo;
                    }
                }
            }

            if nombre == "push" && argumentos.len() == 2 {
                // AQUÍ HABRÁ OTRO REFACTOR LUEGO, POR AHORA LO DEJAMOS FUNCIONAL
                let arr_eval = evaluar_expresion(argumentos[0].clone(), Rc::clone(&entorno));
                let item_eval = evaluar_expresion(argumentos[1].clone(), Rc::clone(&entorno));
                if let ObjetoKura::Arreglo(mut arr) = arr_eval {
                    arr.push(item_eval);
                    return ObjetoKura::Arreglo(arr);
                } else {
                    println!("Error: 'push' espera (arreglo, elemento).");
                    return ObjetoKura::Nulo;
                }
            }

            // BUSCAMOS EN EL ENTORNO
            let funcion = entorno.borrow().obtener(&nombre);

            if let Some(ObjetoKura::Funcion { parametros, retorno, cuerpo }) = funcion {
                if argumentos.len() != parametros.len() {
                    println!("Error Kura: La funcion '{}' espera {} argumentos, recibio {}", nombre, parametros.len(), argumentos.len());
                    return ObjetoKura::Nulo;
                }

                let entorno_local = Entorno::extend(Rc::clone(&entorno));

                for (i, arg_expr) in argumentos.into_iter().enumerate() {
                    let arg_evaluado = evaluar_expresion(arg_expr, Rc::clone(&entorno));

                    if let Some(tipo_esperado) = &parametros[i].1 {
                        let tipo_real = TipoKura::de_objeto(&arg_evaluado);
                        if *tipo_esperado != tipo_real && *tipo_esperado != TipoKura::Desconocido {
                            println!("Error Kura de Tipos: El parametro '{}' espera un {:?}, pero recibió un {:?}", parametros[i].0, tipo_esperado, tipo_real);
                            return ObjetoKura::Nulo;
                        }
                    }

                    entorno_local.borrow_mut().guardar(parametros[i].0.clone(), arg_evaluado);
                }

                let mut valor_retornado = ObjetoKura::Nulo;
                for decl in cuerpo {
                    let res = evaluar_declaracion(decl, Rc::clone(&entorno_local));
                    if let ObjetoKura::Retorno(valor) = res {
                        valor_retornado = *valor;
                        break;
                    }
                }

                if let Some(tipo_ret) = retorno {
                    let tipo_real_ret = TipoKura::de_objeto(&valor_retornado);
                    if tipo_ret != tipo_real_ret && tipo_ret != TipoKura::Desconocido {
                        println!("Error Kura de Tipos: La funcion '{}' debía retornar {:?}, pero retornó {:?}", nombre, tipo_ret, tipo_real_ret);
                    }
                }

                return valor_retornado;
            }
            ObjetoKura::Nulo
        }
        Expresion::ConstructorEnum { variante, valores } => {
            let mut valores_evaluados = Vec::new();
            for val in valores {
                valores_evaluados.push(evaluar_expresion(val, Rc::clone(&entorno)));
            }
            let def_enums = entorno.borrow().enums.clone();
            for (_, def_enum) in &def_enums {
                if def_enum.variantes.contains_key(&variante) {
                    return ObjetoKura::Variante {
                        nombre_enum: def_enum.nombre.clone(),
                        nombre_variante: variante.clone(),
                        valores: valores_evaluados,
                    };
                }
            }
            ObjetoKura::Nulo
        }
    }
}

fn patron_coincide(patron: &Pattern, valor: &ObjetoKura) -> bool {
    match patron {
        Pattern::Comodin => true,
        Pattern::Identificador(_) => true,
        Pattern::Variante { nombre, bindings } => {
            if let ObjetoKura::Variante { nombre_variante, valores, .. } = valor {
                nombre == nombre_variante && valores.len() == bindings.len()
            } else { false }
        }
    }
}

fn vincular_patron(patron: &Pattern, valor: &ObjetoKura, entorno: Rc<RefCell<Entorno>>) {
    match patron {
        Pattern::Identificador(nombre) => {
            entorno.borrow_mut().guardar(nombre.clone(), valor.clone());
        }
        Pattern::Variante { bindings, .. } => {
            if let ObjetoKura::Variante { valores, .. } = valor {
                for (i, binding) in bindings.iter().enumerate() {
                    if i < valores.len() {
                        entorno.borrow_mut().guardar(binding.clone(), valores[i].clone());
                    }
                }
            }
        }
        Pattern::Comodin => {}
    }
}

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
        ObjetoKura::InstanciaStruct { nombre, campos } => {
            print!("{} {{ ", nombre);
            let mapa = campos.borrow();
            let mut count = 0;
            let len = mapa.len();
            for (k, v) in mapa.iter() {
                print!("{}: ", k);
                imprimir_objeto(v);
                count += 1;
                if count < len { print!(", "); }
            }
            print!(" }}");
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