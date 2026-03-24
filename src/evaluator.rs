#![allow(dead_code, unused)]

use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use crate::ast::{Programa, Declaracion, Expresion, CasoMatch, Pattern, VarianteEnum};
use crate::token::Token;
use crate::types::TipoKura;
use std::fs;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum ObjetoKura {
    Entero(i64),
    Flotante(f64),  // 🚀 NUEVO: Soporte para flotantes
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
    Puntero(Rc<RefCell<ObjetoKura>>),  // 🚀 NUEVO: Heap-allocated object with RC/GC
    FuncionNativa(fn(Vec<ObjetoKura>) -> ObjetoKura),
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
        let mut variables = HashMap::new();

        // ---------------------------------------------------------
        // 🚀 LIBRERÍA ESTÁNDAR: PUENTES NATIVOS RUST -> KURA
        // ---------------------------------------------------------

        // 1. native_leer_archivo(ruta)
        variables.insert("native_leer_archivo".to_string(), ObjetoKura::FuncionNativa(|args| {
            if args.len() != 1 { return ObjetoKura::Nulo; }
            if let ObjetoKura::Cadena(ruta) = &args[0] {
                match std::fs::read_to_string(ruta) {
                    Ok(contenido) => return ObjetoKura::Cadena(contenido),
                    Err(_) => return ObjetoKura::Cadena("ERROR: No se pudo leer el archivo".to_string()),
                }
            }
            ObjetoKura::Nulo
        }));
        // 3. native_anexar_archivo(ruta, contenido)
        variables.insert("native_anexar_archivo".to_string(), ObjetoKura::FuncionNativa(|args| {
            use std::io::Write;
            if args.len() != 2 { return ObjetoKura::Nulo; }
            if let (ObjetoKura::Cadena(ruta), ObjetoKura::Cadena(contenido)) = (&args[0], &args[1]) {
                let archivo = std::fs::OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(ruta);

                if let Ok(mut file) = archivo {
                    let _ = writeln!(file, "{}", contenido);
                    return ObjetoKura::Booleano(true);
                }
            }
            ObjetoKura::Booleano(false)
        }));

        // 4. native_obtener_tiempo()
        variables.insert("native_obtener_tiempo".to_string(), ObjetoKura::FuncionNativa(|_| {
            use std::time::{SystemTime, UNIX_EPOCH};
            let segs = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            ObjetoKura::Entero(segs as i64)
        }));

        // 5. native_dormir(ms)
        variables.insert("native_dormir".to_string(), ObjetoKura::FuncionNativa(|args| {
            if let Some(ObjetoKura::Entero(ms)) = args.get(0) {
                std::thread::sleep(std::time::Duration::from_millis(*ms as u64));
            }
            ObjetoKura::Nulo
        }));

        // 2. native_escribir_archivo(ruta, contenido)
        variables.insert("native_escribir_archivo".to_string(), ObjetoKura::FuncionNativa(|args| {
            if args.len() != 2 { return ObjetoKura::Nulo; }
            if let (ObjetoKura::Cadena(ruta), ObjetoKura::Cadena(contenido)) = (&args[0], &args[1]) {
                match std::fs::write(ruta, contenido) {
                    Ok(_) => return ObjetoKura::Booleano(true),
                    Err(_) => return ObjetoKura::Booleano(false),
                }
            }
            ObjetoKura::Nulo
        }));

        // 6. a_texto(valor) - Convierte cualquier valor a cadena
        variables.insert("a_texto".to_string(), ObjetoKura::FuncionNativa(|args| {
            if args.is_empty() { return ObjetoKura::Cadena("null".to_string()); }
            match &args[0] {
                ObjetoKura::Entero(n) => ObjetoKura::Cadena(n.to_string()),
                ObjetoKura::Cadena(s) => ObjetoKura::Cadena(s.clone()),
                ObjetoKura::Booleano(b) => ObjetoKura::Cadena(b.to_string()),
                ObjetoKura::Nulo => ObjetoKura::Cadena("null".to_string()),
                _ => ObjetoKura::Cadena("[objeto]".to_string()),
            }
        }));

        Rc::new(RefCell::new(Entorno {
            variables,
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
        let res = evaluar_declaracion(&declaracion, Rc::clone(&entorno));
        if let ObjetoKura::Retorno(_) = res { break; }
    }
}

fn evaluar_declaracion(declaracion: &Declaracion, entorno: Rc<RefCell<Entorno>>) -> ObjetoKura {
    match declaracion {
        Declaracion::Let { nombre, tipo, valor, .. } => {
            let valor_evaluado = evaluar_expresion(&valor, Rc::clone(&entorno));
            if let Some(tipo_esperado) = tipo {
                let tipo_real = TipoKura::de_objeto(&valor_evaluado);
                if *tipo_esperado != tipo_real {
                    println!("Error de tipo: Variable '{}' declarada como {:?} pero el valor es {:?}", nombre, tipo_esperado, tipo_real);
                    return ObjetoKura::Nulo;
                }
            }
            entorno.borrow_mut().guardar(nombre.to_string(), valor_evaluado);
            ObjetoKura::Nulo
        }
        Declaracion::Print { valor } => {
            let valor_evaluado = evaluar_expresion(&valor, Rc::clone(&entorno));
            imprimir_objeto(&valor_evaluado);
            println!();
            ObjetoKura::Nulo
        }
        Declaracion::Reasignacion { nombre, valor } => {
            let valor_evaluado = evaluar_expresion(&valor, Rc::clone(&entorno));
            if !entorno.borrow_mut().reasignar(&nombre, valor_evaluado) {
                println!("Error: La variable '{}' no ha sido declarada.", nombre);
            }
            ObjetoKura::Nulo
        }
        Declaracion::Break => ObjetoKura::Break,
        Declaracion::If { condicion, consecuencia, alternativa } => {
            let valor_condicion = evaluar_expresion(&condicion, Rc::clone(&entorno));
            let es_verdadero = match valor_condicion {
                ObjetoKura::Booleano(b) => b,
                ObjetoKura::Entero(n) => n != 0,
                _ => false,
            };

            if es_verdadero {
                let entorno_if = Entorno::extend(Rc::clone(&entorno)); // Scope del IF
                for decl in consecuencia {
                    let res = evaluar_declaracion(&decl, Rc::clone(&entorno_if));
                    if matches!(res, ObjetoKura::Retorno(_) | ObjetoKura::Break) { return res; }
                }
            } else if let Some(bloque_else) = alternativa {
                let entorno_else = Entorno::extend(Rc::clone(&entorno));
                for decl in bloque_else {
                    let res = evaluar_declaracion(&decl, Rc::clone(&entorno_else));
                    if matches!(res, ObjetoKura::Retorno(_) | ObjetoKura::Break) { return res; }
                }
            }
            ObjetoKura::Nulo
        }
        Declaracion::While { condicion, cuerpo } => {
            loop {
                let valor_condicion = evaluar_expresion(&condicion, Rc::clone(&entorno));
                let es_verdadero = match valor_condicion {
                    ObjetoKura::Booleano(b) => b,
                    ObjetoKura::Entero(n) => n != 0,
                    _ => false,
                };
                if !es_verdadero { break; }

                let mut romper_ciclo = false;
                let entorno_while = Entorno::extend(Rc::clone(&entorno));
                for decl in cuerpo.clone() {
                    let res = evaluar_declaracion(&decl, Rc::clone(&entorno_while));
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
            let iterable_evaluado = evaluar_expresion(&iterable, Rc::clone(&entorno));
            if let ObjetoKura::Arreglo(arr) = iterable_evaluado {
                for elemento in arr {
                    let entorno_for = Entorno::extend(Rc::clone(&entorno));
                    entorno_for.borrow_mut().guardar(variable.clone(), elemento);

                    let mut romper_ciclo = false;
                    for decl in cuerpo.clone() {
                        let res = evaluar_declaracion(&decl, Rc::clone(&entorno_for));
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
            let funcion = ObjetoKura::Funcion { parametros: parametros.to_vec(), retorno: retorno.clone(), cuerpo: cuerpo.to_vec() };
            entorno.borrow_mut().guardar(nombre.to_string(), funcion);
            ObjetoKura::Nulo
        }
        Declaracion::Return { valor } => {
            let valor_evaluado = evaluar_expresion(&valor, Rc::clone(&entorno));
            ObjetoKura::Retorno(Box::new(valor_evaluado))
        }
        Declaracion::LlamadaSuelta { nombre, argumentos } => {
            evaluar_expresion(&Expresion::Llamada { nombre: nombre.to_string(), argumentos: argumentos.to_vec() }, Rc::clone(&entorno));
            ObjetoKura::Nulo
        }
        Declaracion::Enum { nombre, variantes } => {
            let mut vars_map = HashMap::new();
            for VarianteEnum { nombre: var_name, campos } in variantes {
                vars_map.insert(var_name.to_string(), campos.len());
            }
            let def = DefinicionEnum { nombre: nombre.clone(), variantes: vars_map };
            entorno.borrow_mut().enums.insert(nombre.to_string(), def);
            ObjetoKura::Nulo
        }
        Declaracion::Match { valor, casos } => {
            let valor_evaluado = evaluar_expresion(&valor, Rc::clone(&entorno));

            for CasoMatch { patron, cuerpo } in casos {
                if patron_coincide(&patron, &valor_evaluado) {
                    let entorno_match = Entorno::extend(Rc::clone(&entorno));
                    vincular_patron(&patron, &valor_evaluado, Rc::clone(&entorno_match));

                    for decl in cuerpo {
                        let res = evaluar_declaracion(&decl, Rc::clone(&entorno_match));
                        if matches!(res, ObjetoKura::Retorno(_) | ObjetoKura::Break) { return res; }
                    }
                    break;
                }
            }
            ObjetoKura::Nulo
        }
        Declaracion::Importar { elementos, archivo } => {
            use std::path::PathBuf;

            let mut nombre_archivo = PathBuf::from(&archivo);
            if nombre_archivo.extension().and_then(|s| s.to_str()) != Some("kr") {
                nombre_archivo.set_extension("kr");
            }

            let rutas_busqueda = vec![
                PathBuf::from("."),
                PathBuf::from("src"),                  // 🚀 NUEVO: Buscar en src
                PathBuf::from("kura_modules"),
                PathBuf::from("C:/Kura/std"),
            ];

            let mut contenido = None;
            let mut ruta_encontrada = String::new();

            for base in rutas_busqueda {
                let intento = base.join(&nombre_archivo);
                if intento.exists() {
                    if let Ok(c) = std::fs::read_to_string(&intento) {
                        ruta_encontrada = intento.to_string_lossy().to_string();
                        contenido = Some(c);
                        break;
                    }
                }
            }

            match contenido {
                Some(contenido_final) => {
                    let lexer = crate::lexer::Lexer::new(&contenido_final);
                    let mut parser = crate::parser::Parser::new(lexer, &contenido_final);
                    let programa_modulo = parser.parse_programa();

                    let entorno_modulo = Entorno::new();
                    evaluar_programa(programa_modulo, Rc::clone(&entorno_modulo));

                    for nombre in elementos {
                        if let Some(valor) = entorno_modulo.borrow().obtener(&nombre) {
                            entorno.borrow_mut().guardar(nombre.to_string(), valor);
                        }
                        // --- CORRECCIÓN AQUÍ: Quitamos .borrow() y .borrow_mut() de .structs ---
                        else if let Some(def_struct) = entorno_modulo.borrow().structs.get(nombre.as_str()).cloned() {
                            entorno.borrow_mut().structs.insert(nombre.clone(), def_struct);
                        }
                        else {
                            println!("Error Kura: '{}' no fue encontrado en '{}'", nombre, ruta_encontrada);
                        }
                    }
                }
                None => {
                    println!("Error Kura: No se pudo encontrar el modulo '{}' en las rutas de busqueda.", archivo);
                }
            }

            ObjetoKura::Nulo
        },
        Declaracion::Exportar { nombre, es_modulo_default } => {
            // 🚀 NUEVO: Export declaration (módulo systems)
            // Por ahora, simplemente ignoramos en evaluator
            // El sistema de módulos lo procesará en el análisis
            if *es_modulo_default {
                println!("📦 Exportando como default: {}", nombre);
            } else {
                println!("📦 Exportando: {}", nombre);
            }
            ObjetoKura::Nulo
        },
        Declaracion::Struct { nombre, campos, metodos } => {
            let mut campos_map = HashMap::new();
            for (c_nom, c_tipo) in campos {
                campos_map.insert(c_nom.to_string(), c_tipo.clone());
            }

            // --- NUEVO: Extraer funciones y guardarlas ---
            let mut metodos_map = HashMap::new();
            for m in metodos {
                if let Declaracion::Funcion { nombre: n_metodo, parametros, retorno, cuerpo } = m {
                    metodos_map.insert(n_metodo.to_string(), ObjetoKura::Funcion { parametros: parametros.to_vec(), retorno: retorno.clone(), cuerpo: cuerpo.to_vec() });
                }
            }

            let def = DefinicionStruct { nombre: nombre.clone(), campos: campos_map, metodos: metodos_map };
            entorno.borrow_mut().structs.insert(nombre.to_string(), def);
            ObjetoKura::Nulo
        }
        Declaracion::LlamadaMetodoSuelta { objeto, metodo, argumentos } => {
            evaluar_expresion(&Expresion::LlamadaMetodo { objeto: objeto.clone(), metodo: metodo.to_string(), argumentos: argumentos.to_vec() }, Rc::clone(&entorno));
            ObjetoKura::Nulo
        }
        Declaracion::Delete { valor } => {
            // Evaluamos la expresión para obtener el puntero
            let obj = evaluar_expresion(valor, entorno);
            // Si es un puntero (Rc<RefCell>), vaciamos su contenido en el intérprete
            if let ObjetoKura::Puntero(ref_cell) = obj {
                *ref_cell.borrow_mut() = ObjetoKura::Nulo;
            }
            ObjetoKura::Nulo
        }
        Declaracion::ReasignacionPropiedad { objeto, propiedad, valor } => {
            let valor_evaluado = evaluar_expresion(&valor, Rc::clone(&entorno));
            if let Some(ObjetoKura::InstanciaStruct { campos, .. }) = entorno.borrow().obtener(&objeto) {
                campos.borrow_mut().insert(propiedad.to_string(), valor_evaluado);
            } else {
                println!("Error: La variable '{}' no es un Struct o no existe", objeto);
            }
            ObjetoKura::Nulo
        }
    }
}

fn evaluar_expresion(expresion: &Expresion, entorno: Rc<RefCell<Entorno>>) -> ObjetoKura {
    match expresion {
        Expresion::Entero(n) => ObjetoKura::Entero(*n),
        Expresion::Flotante(f) => ObjetoKura::Flotante(*f),  // 🚀 NUEVO
        Expresion::Booleano(b) => ObjetoKura::Booleano(*b),
        Expresion::Cadena(texto) => ObjetoKura::Cadena(texto.to_string()),
        Expresion::Identificador(nombre) => entorno.borrow().obtener(&nombre).unwrap_or(ObjetoKura::Nulo),
        Expresion::Arreglo(elementos) => {
            let mut evaluados = Vec::new();
            for el in elementos {
                evaluados.push(evaluar_expresion(&el, Rc::clone(&entorno)));
            }
            ObjetoKura::Arreglo(evaluados)
        }
        Expresion::Nuevo { tipo: _ } => {
            // En el intérprete, "malloc" se simula envolviendo un Nulo inicial
            // en un contador de referencias Rc con mutabilidad interior RefCell
            use std::rc::Rc;
            use std::cell::RefCell;
            ObjetoKura::Puntero(Rc::new(RefCell::new(ObjetoKura::Nulo)))
        }
        Expresion::Nulo => {
            ObjetoKura::Nulo
        }
        Expresion::Referencia(expr) => {
            use std::rc::Rc;
            use std::cell::RefCell;
            let obj = evaluar_expresion(expr, entorno);
            // Envuelve el objeto actual en un puntero virtual
            ObjetoKura::Puntero(Rc::new(RefCell::new(obj)))
        }
        Expresion::Desreferencia(expr) => {
            let obj = evaluar_expresion(expr, entorno);
            if let ObjetoKura::Puntero(ref_cell) = obj {
                ref_cell.borrow().clone() // Extraemos el valor clonado de la memoria
            } else {
                println!("Error de Ejecución: Intentando desreferenciar un valor que no es un Puntero.");
                ObjetoKura::Nulo
            }
        }
        Expresion::LlamadaMetodo { objeto, metodo, argumentos } => {
            let obj_evaluado = evaluar_expresion(&*objeto, Rc::clone(&entorno));

            if let ObjetoKura::InstanciaStruct { nombre: nombre_struct, .. } = &obj_evaluado {
                let def_struct = entorno.borrow().structs.get(nombre_struct).cloned();

                if let Some(def) = def_struct {
                    if let Some(ObjetoKura::Funcion { parametros, cuerpo, .. }) = def.metodos.get(metodo.as_str()) {

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
                            let arg_val = evaluar_expresion(&arg_expr, Rc::clone(&entorno));
                            entorno_local.borrow_mut().guardar(parametros[i + 1].0.clone(), arg_val);
                        }

                        let mut valor_retornado = ObjetoKura::Nulo;
                        for decl in cuerpo.clone() {
                            let res = evaluar_declaracion(&decl, Rc::clone(&entorno_local));
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
            let estructura_evaluada = evaluar_expresion(&*estructura, Rc::clone(&entorno));
            let indice_evaluado = evaluar_expresion(&*indice, Rc::clone(&entorno));

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
            let izq_val = evaluar_expresion(&*izquierda, Rc::clone(&entorno));
            let der_val = evaluar_expresion(&*derecha, Rc::clone(&entorno));

            if *operador == Token::And {
                if let (ObjetoKura::Booleano(b1), ObjetoKura::Booleano(b2)) = (&izq_val, &der_val) { return ObjetoKura::Booleano(*b1 && *b2); }
            }
            if *operador == Token::Or {
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
            // --- MEJORA: Sumar Cadena + Cualquier cosa ---
            if let ObjetoKura::Cadena(i) = &izq_val {
                if *operador == Token::Suma {
                    let der_str = match &der_val {
                        ObjetoKura::Entero(n) => n.to_string(),
                        ObjetoKura::Cadena(s) => s.clone(),
                        ObjetoKura::Booleano(b) => b.to_string(),
                        ObjetoKura::InstanciaStruct { nombre, .. } => format!("[Instancia de {}]", nombre),
                        _ => "null".to_string(),
                    };
                    return ObjetoKura::Cadena(format!("{}{}", i, der_str));
                }
            }
            ObjetoKura::Nulo
        }
        Expresion::InstanciaStruct { nombre, campos } => {
            let mut campos_map = HashMap::new();
            for (c_nom, c_expr) in campos {
                let val = evaluar_expresion(&c_expr, Rc::clone(&entorno));
                campos_map.insert(c_nom.to_string(), val);
            }
            ObjetoKura::InstanciaStruct {
                nombre: nombre.to_string(),
                campos: Rc::new(RefCell::new(campos_map)),
            }
        }
        Expresion::AccesoPropiedad { objeto, propiedad } => {
            let obj_eval = evaluar_expresion(&*objeto, Rc::clone(&entorno));
            if let ObjetoKura::InstanciaStruct { campos, .. } = obj_eval {
                return campos.borrow().get(propiedad.as_str()).cloned().unwrap_or(ObjetoKura::Nulo);
            }
            ObjetoKura::Nulo
        }
        Expresion::Diccionario(pares) => {
            let mut mapa = HashMap::new();   
            for (clave, valor_expr) in pares {
                let valor_evaluado = evaluar_expresion(&valor_expr, Rc::clone(&entorno));
                mapa.insert(clave.to_string(), valor_evaluado);
            }
            ObjetoKura::Diccionario(mapa)
        }
        Expresion::Llamada { nombre, argumentos } => {

            if nombre == "read_file" && argumentos.len() == 1 {
                let arg = evaluar_expresion(&argumentos[0], entorno);
                if let ObjetoKura::Cadena(ruta) = arg {
                    match fs::read_to_string(&ruta) {
                        Ok(contenido) => return ObjetoKura::Cadena(contenido),
                        Err(_) => {
                            println!("Error de Ejecución: No se pudo leer el archivo '{}'", ruta);
                            return ObjetoKura::Nulo;
                        }
                    }
                } else {
                    println!("Error de Ejecución: read_file requiere una cadena de texto como ruta.");
                    return ObjetoKura::Nulo;
                }
            }

            if nombre == "write_file" && argumentos.len() == 2 {
                // 🚀 AGREGAMOS .clone() A LOS ENTORNOS
                let arg_ruta = evaluar_expresion(&argumentos[0], entorno.clone());
                let arg_contenido = evaluar_expresion(&argumentos[1], entorno.clone());

                if let (ObjetoKura::Cadena(ruta), ObjetoKura::Cadena(contenido)) = (arg_ruta, arg_contenido) {
                    match fs::write(&ruta, contenido) {
                        Ok(_) => return ObjetoKura::Booleano(true), // Éxito
                        Err(_) => {
                            println!("Error de Ejecución: No se pudo escribir en el archivo '{}'", ruta);
                            return ObjetoKura::Booleano(false);
                        }
                    }
                } else {
                    println!("Error de Ejecución: write_file requiere dos cadenas (ruta y contenido).");
                    return ObjetoKura::Nulo;
                }
            }

            if nombre == "len" && argumentos.len() == 1 {
                let arg_eval = evaluar_expresion(&argumentos[0], Rc::clone(&entorno));
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
                let arr_eval = evaluar_expresion(&argumentos[0], Rc::clone(&entorno));
                let item_eval = evaluar_expresion(&argumentos[1], Rc::clone(&entorno));
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

            // --- NUEVO: EJECUTAR FUNCIONES NATIVAS DE RUST ---
            if let Some(ObjetoKura::FuncionNativa(func_rust)) = funcion.clone() {
                let mut argumentos_evaluados = Vec::new();
                for arg_expr in argumentos {
                    argumentos_evaluados.push(evaluar_expresion(&arg_expr, Rc::clone(&entorno)));
                }
                return func_rust(argumentos_evaluados); // Rust hace el trabajo sucio
            }

            if let Some(ObjetoKura::Funcion { parametros, retorno, cuerpo }) = funcion {
                if argumentos.len() != parametros.len() {
                    println!("Error Kura: La funcion '{}' espera {} argumentos, recibio {}", nombre, parametros.len(), argumentos.len());
                    return ObjetoKura::Nulo;
                }

                let entorno_local = Entorno::extend(Rc::clone(&entorno));

                for (i, arg_expr) in argumentos.into_iter().enumerate() {
                    let arg_evaluado = evaluar_expresion(&arg_expr, Rc::clone(&entorno));

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
                    let res = evaluar_declaracion(&decl, Rc::clone(&entorno_local));
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
                valores_evaluados.push(evaluar_expresion(&val, Rc::clone(&entorno)));
            }
            let def_enums = entorno.borrow().enums.clone();
            for (_, def_enum) in &def_enums {
                if def_enum.variantes.contains_key(variante.as_str()) {
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
        ObjetoKura::Flotante(f) => print!("{}", f),  // 🚀 NUEVO
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
        ObjetoKura::Puntero(ref_cell) => {
            print!("Puntero -> ");
            // Llamamos recursivamente a la misma función para imprimir lo que hay dentro!
            imprimir_objeto(&*ref_cell.borrow());
        }
        ObjetoKura::Nulo => {
            print!("null");
        }
        ObjetoKura::FuncionNativa(_) => print!("[Funcion Nativa de Rust]"),
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