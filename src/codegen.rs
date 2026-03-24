use std::collections::HashMap;
use std::fs;
use std::process::Command;

use crate::ast::{Programa, Declaracion, Expresion};
use crate::token::Token;

/// CodeGenerator genera IR LLVM que se compila a código máquina nativo
/// Backend: clang (from LLVM) → object → lld-link → executable
/// Soporta optimizaciones en tiempo de compilación
#[derive(Clone, Copy, Debug)]
pub enum OptimizationLevel {
    None,      // -O0 (sin optimización)
    Fast,      // -O1 (mínima)
    Balanced,  // -O2 (balanceada - default)
    Aggressive, // -O3 (máxima)
}

pub struct CodeGenerator {
    ir_code: String,
    var_counter: usize,
    // ¡MODIFICADO! Ahora guarda (registro, tipo)
    current_scope: HashMap<String, (String, String)>,
    opt_level: OptimizationLevel,
}

impl CodeGenerator {
    pub fn new() -> Result<Self, String> {
        Ok(Self {
            ir_code: String::new(),
            var_counter: 0,
            current_scope: HashMap::new(),
            opt_level: OptimizationLevel::Balanced,
        })
    }

    pub fn with_optimization(opt_level: OptimizationLevel) -> Result<Self, String> {
        Ok(Self {
            ir_code: String::new(),
            var_counter: 0,
            current_scope: HashMap::new(),
            opt_level,
        })
    }

    pub fn generate(&mut self, programa: Programa) -> Result<String, String> {
        println!("🎯 Generando LLVM IR textual...");
        println!("📋 Compilando {} declaraciones...", programa.declaraciones.len());

        // Encabezados de LLVM IR
        self.ir_code.push_str("; LLVM IR generado por KURA\n");
        self.ir_code.push_str("target triple = \"x86_64-pc-windows-gnu\"\n");
        self.ir_code.push_str("target datalayout = \"e-m:w-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128\"\n\n");

        // Declaración de printf
        self.ir_code.push_str("declare i32 @printf(i8*, ...)\n");
        self.ir_code.push_str("declare i32 @sprintf(i8*, i8*, ...)\n\n");

        // 🚀 NUEVO: 1. Filtrar y generar primero todas las Funciones definidas por el usuario
        for stmt in &programa.declaraciones {
            if let Declaracion::Funcion { .. } = stmt {
                self.generate_statement(stmt)?;
            }
        }

        // 🚀 NUEVO: 2. Definición de la función main para las instrucciones globales
        self.ir_code.push_str("\ndefine i32 @main() {\n");
        self.ir_code.push_str("entry:\n");

        // 🚀 NUEVO: 3. Generar el cuerpo de main (saltando las funciones que ya generamos)
        for stmt in &programa.declaraciones {
            if !matches!(stmt, Declaracion::Funcion { .. }) {
                self.generate_statement(stmt)?;
            }
        }

        // Retorno 0 para el main
        self.ir_code.push_str("  ret i32 0\n");
        self.ir_code.push_str("}\n");

        println!("✅ LLVM IR generada exitosamente");
        Ok(self.ir_code.clone())
    }

    fn generate_statement(&mut self, stmt: &Declaracion) -> Result<(), String> {
        match stmt {
            Declaracion::Let { nombre, valor, .. } => {
                // Obtenemos tanto el registro como el tipo (ej: "i64" o "i8*")
                let (reg, tipo) = self.generate_expr(valor)?;
                self.current_scope.insert(nombre.clone(), (reg, tipo));
            }

            // --- NUEVO: SOPORTE PARA FUNCIONES ---
            Declaracion::Funcion { nombre, parametros, retorno: _, cuerpo } => {
                // Preparamos los parámetros (asumimos i64 por defecto para números)
                let mut params_ir = Vec::new();
                for (param_nombre, _) in parametros {
                    params_ir.push(format!("i64 %{}", param_nombre));
                }
                let signature = params_ir.join(", ");

                // Definimos la función en LLVM IR
                self.ir_code.push_str(&format!("define i64 @{}({}) {{\n", nombre, signature));
                self.ir_code.push_str("entry:\n");

                // Clonamos el scope actual para no contaminarlo
                let previous_scope = self.current_scope.clone();

                // Registramos los parámetros como variables locales para que Kura los encuentre
                for (param_nombre, _) in parametros {
                    self.current_scope.insert(param_nombre.clone(), (format!("%{}", param_nombre), "i64".to_string()));
                }

                // Compilamos el cuerpo de la función
                let mut tiene_return = false;
                for stmt in cuerpo {
                    self.generate_statement(stmt)?;
                    if matches!(stmt, Declaracion::Return { .. }) {
                        tiene_return = true;
                    }
                }

                // Si la función no tiene un return explícito, devolvemos 0 por defecto
                if !tiene_return {
                    self.ir_code.push_str("  ret i64 0\n");
                }

                self.ir_code.push_str("}\n\n");

                // Restauramos el scope global
                self.current_scope = previous_scope;
            }

            // --- NUEVO: SOPORTE PARA RETURN ---
            Declaracion::Return { valor } => {
                let (reg, _) = self.generate_expr(valor)?;
                self.ir_code.push_str(&format!("  ret i64 {}\n", reg));
            }

            // --- NUEVO: SOPORTE PARA LLAMADAS SUELTAS (Ej: saludar();) ---
            Declaracion::LlamadaSuelta { nombre, argumentos } => {
                let mut args_ir = Vec::new();
                for arg in argumentos {
                    let (reg, _) = self.generate_expr(arg)?;
                    args_ir.push(format!("i64 {}", reg));
                }
                let args_str = args_ir.join(", ");
                self.ir_code.push_str(&format!("  call i64 @{}({})\n", nombre, args_str));
            }

            // --- ACTUALIZADO: PRINT INTELIGENTE ---
            Declaracion::Print { valor } => {
                // Ahora usamos el segundo valor (el tipo) que nos devuelve generate_expr
                let (reg, tipo) = self.generate_expr(valor)?;

                if tipo == "i64" {
                    // CASO 1: Imprimir un Número Entero
                    let format_str = "@.str.fmt.i64 = private unnamed_addr constant [6 x i8] c\"%lld\\0A\\00\", align 1\n";
                    if !self.ir_code.contains("@.str.fmt.i64") {
                        let insert_pos = self.ir_code.find("declare i32 @printf").unwrap_or(0);
                        self.ir_code.insert_str(insert_pos, format_str);
                    }

                    let ptr_reg = self.new_reg();
                    self.ir_code.push_str(&format!("  {} = getelementptr inbounds [6 x i8], [6 x i8]* @.str.fmt.i64, i32 0, i32 0\n", ptr_reg));
                    self.ir_code.push_str(&format!("  call i32 (i8*, ...) @printf(i8* {}, i64 {})\n", ptr_reg, reg));
                }
                else if tipo == "i8*" {
                    // CASO 2: Imprimir una Cadena de Texto
                    let format_str = "@.str.fmt.str = private unnamed_addr constant [4 x i8] c\"%s\\0A\\00\", align 1\n";
                    if !self.ir_code.contains("@.str.fmt.str") {
                        let insert_pos = self.ir_code.find("declare i32 @printf").unwrap_or(0);
                        self.ir_code.insert_str(insert_pos, format_str);
                    }

                    let ptr_reg = self.new_reg();
                    self.ir_code.push_str(&format!("  {} = getelementptr inbounds [4 x i8], [4 x i8]* @.str.fmt.str, i32 0, i32 0\n", ptr_reg));
                    self.ir_code.push_str(&format!("  call i32 (i8*, ...) @printf(i8* {}, i8* {})\n", ptr_reg, reg));
                }
            }
            Declaracion::Reasignacion { nombre, valor } => {
                let (reg, tipo) = self.generate_expr(valor)?;
                self.current_scope.insert(nombre.clone(), (reg, tipo));
            }
            Declaracion::If { condicion, consecuencia, alternativa } => {
                let (cond_reg, _) = self.generate_expr(condicion)?;
                let if_label = self.var_counter;
                let else_label = self.var_counter + 1;
                let merge_label = self.var_counter + 2;
                self.var_counter += 3;
                
                self.ir_code.push_str(&format!("  %cond.bool = icmp ne i64 {}, 0\n", cond_reg));
                self.ir_code.push_str(&format!("  br i1 %cond.bool, label %if.{}, label %if.{}\n", if_label, else_label));
                
                self.ir_code.push_str(&format!("if.{}:\n", if_label));
                for stmt in consecuencia {
                    self.generate_statement(stmt)?;
                }
                self.ir_code.push_str(&format!("  br label %if.{}\n", merge_label));
                
                self.ir_code.push_str(&format!("if.{}:\n", else_label));
                if let Some(alt) = alternativa {
                    for stmt in alt {
                        self.generate_statement(stmt)?;
                    }
                }
                self.ir_code.push_str(&format!("  br label %if.{}\n", merge_label));
                
                self.ir_code.push_str(&format!("if.{}:\n", merge_label));
            }
            Declaracion::While { condicion, cuerpo } => {
                let loop_label = self.var_counter;
                let body_label = self.var_counter + 1;
                let exit_label = self.var_counter + 2;
                self.var_counter += 3;
                
                self.ir_code.push_str(&format!("  br label %while.{}\n", loop_label));
                self.ir_code.push_str(&format!("while.{}:\n", loop_label));
                
                let (cond_reg, _) = self.generate_expr(condicion)?;
                self.ir_code.push_str(&format!("  %loop.cond = icmp ne i64 {}, 0\n", cond_reg));
                self.ir_code.push_str(&format!("  br i1 %loop.cond, label %while.{}, label %while.{}\n", body_label, exit_label));
                
                self.ir_code.push_str(&format!("while.{}:\n", body_label));
                for stmt in cuerpo {
                    self.generate_statement(stmt)?;
                }
                self.ir_code.push_str(&format!("  br label %while.{}\n", loop_label));
                
                self.ir_code.push_str(&format!("while.{}:\n", exit_label));
            }
            _ => {
                println!("⚠️  Declaración no soportada: {:?}", stmt);
            }
        }
        Ok(())
    }

    fn generate_expr(&mut self, expr: &Expresion) -> Result<(String, String), String> {
        match expr {
            Expresion::Entero(n) => {
                Ok((n.to_string(), "i64".to_string()))
            }
            // --- NUEVO: SOPORTE PARA BOOLEANOS ---
            Expresion::Booleano(valor) => {
                // Traducimos 'true' a 1 y 'false' a 0, guardándolos como i64
                let num_str = if *valor { "1" } else { "0" };
                Ok((num_str.to_string(), "i64".to_string()))
            }
            // --- NUEVO: CREACIÓN DE ARREGLOS ---
            Expresion::Arreglo(elementos) => {
                let len = elementos.len();
                let arr_ptr = self.new_reg();

                // 1. Reservamos memoria para N números de 64 bits (i64)
                self.ir_code.push_str(&format!("  {} = alloca i64, i32 {}\n", arr_ptr, len));

                // 2. Guardamos cada elemento en su posición correspondiente
                for (i, elem) in elementos.iter().enumerate() {
                    let (val_reg, _) = self.generate_expr(elem)?;
                    let elem_ptr = self.new_reg();

                    // Calculamos la dirección de memoria de este índice
                    self.ir_code.push_str(&format!("  {} = getelementptr inbounds i64, i64* {}, i32 {}\n", elem_ptr, arr_ptr, i));
                    // Guardamos el valor ahí
                    self.ir_code.push_str(&format!("  store i64 {}, i64* {}\n", val_reg, elem_ptr));
                }

                // Devolvemos el puntero del arreglo y le decimos a Kura que es un "i64*" (Puntero a números)
                Ok((arr_ptr, "i64*".to_string()))
            }
            // --- NUEVO: LEER DE UN ARREGLO ---
            Expresion::Indice { estructura, indice } => {
                // Evaluamos el nombre de la lista (nos dará el puntero) y el número del índice
                let (arr_reg, _) = self.generate_expr(estructura)?;
                let (idx_reg, _) = self.generate_expr(indice)?;

                let elem_ptr = self.new_reg();
                // Buscamos la dirección en la memoria
                self.ir_code.push_str(&format!("  {} = getelementptr inbounds i64, i64* {}, i64 {}\n", elem_ptr, arr_reg, idx_reg));

                let val_reg = self.new_reg();
                // Extraemos (load) el valor de esa dirección
                self.ir_code.push_str(&format!("  {} = load i64, i64* {}\n", val_reg, elem_ptr));

                // Lo que sale de un arreglo de números es, obviamente, un número (i64)
                Ok((val_reg, "i64".to_string()))
            }
            Expresion::Identificador(name) => {
                if let Some((reg, tipo)) = self.current_scope.get(name) {
                    // ¡MODIFICADO! Ahora devolvemos su tipo real (i64 o i8*)
                    Ok((reg.clone(), tipo.clone()))
                } else {
                    Err(format!("Variable {} no definida", name))
                }
            }
            // --- NUEVO: EVALUACIÓN DE LLAMADAS A FUNCIONES ---
            Expresion::Llamada { nombre, argumentos } => {

                if nombre == "reemplazar" && argumentos.len() == 3 {
                    let (arr_reg, _) = self.generate_expr(&argumentos[0])?;
                    let (idx_reg, _) = self.generate_expr(&argumentos[1])?;
                    let (val_reg, _) = self.generate_expr(&argumentos[2])?;

                    let elem_ptr = self.new_reg();
                    // Buscamos la memoria
                    self.ir_code.push_str(&format!("  {} = getelementptr inbounds i64, i64* {}, i64 {}\n", elem_ptr, arr_reg, idx_reg));
                    // Sobrescribimos con el nuevo valor
                    self.ir_code.push_str(&format!("  store i64 {}, i64* {}\n", val_reg, elem_ptr));

                    // Devolvemos el mismo puntero para que Kura no pierda la referencia original
                    return Ok((arr_reg, "i64*".to_string()));
                }
                let mut args_ir = Vec::new();

                // Compilamos cada argumento
                for arg in argumentos {
                    let (reg, _) = self.generate_expr(arg)?;
                    args_ir.push(format!("i64 {}", reg));
                }
                let args_str = args_ir.join(", ");

                // Creamos un nuevo registro para guardar el resultado de la función
                let result_reg = self.new_reg();
                self.ir_code.push_str(&format!("  {} = call i64 @{}({})\n", result_reg, nombre, args_str));

                Ok((result_reg, "i64".to_string()))
            }
            // --- NUEVO: SOPORTE PARA CADENAS DE TEXTO ---
            Expresion::Cadena(texto) => {
                // LLVM requiere que las cadenas terminen en un byte nulo (\00)
                let text_len = texto.len() + 1;
                let str_name = format!("@.str.{}", self.var_counter);
                self.var_counter += 1;

                // 1. Creamos la constante global con el texto
                let global_decl = format!("{} = private unnamed_addr constant [{} x i8] c\"{}\\00\", align 1\n", str_name, text_len, texto);

                // 2. Inyectamos la constante arriba de todo (antes de las funciones)
                if let Some(insert_pos) = self.ir_code.find("declare i32 @printf") {
                    self.ir_code.insert_str(insert_pos, &global_decl);
                }

                // 3. Generamos la instrucción getelementptr para obtener la dirección de memoria
                let ptr_reg = self.new_reg();
                self.ir_code.push_str(&format!("  {} = getelementptr inbounds [{} x i8], [{} x i8]* {}, i32 0, i32 0\n", ptr_reg, text_len, text_len, str_name));

                // Retornamos el registro y le avisamos al compilador que es tipo puntero (i8*)
                Ok((ptr_reg, "i8*".to_string()))
            }
            Expresion::Operacion { izquierda, operador, derecha } => {
                let (left_reg, _) = self.generate_expr(izquierda)?;
                let (right_reg, _) = self.generate_expr(derecha)?;
                
                let result_reg = self.new_reg();
                
                let instr = match operador {
                    Token::Suma => format!("  {} = add i64 {}, {}\n", result_reg, left_reg, right_reg),
                    Token::Resta => format!("  {} = sub i64 {}, {}\n", result_reg, left_reg, right_reg),
                    Token::Multiplicacion => format!("  {} = mul i64 {}, {}\n", result_reg, left_reg, right_reg),
                    Token::Division => format!("  {} = sdiv i64 {}, {}\n", result_reg, left_reg, right_reg),
                    Token::Modulo => format!("  {} = srem i64 {}, {}\n", result_reg, left_reg, right_reg),
                    Token::MenorQue => {
                        let cmp_reg = self.new_reg();
                        self.ir_code.push_str(&format!("  {} = icmp slt i64 {}, {}\n", cmp_reg, left_reg, right_reg));
                        format!("  {} = zext i1 {} to i64\n", result_reg, cmp_reg)
                    }
                    Token::MayorQue => {
                        let cmp_reg = self.new_reg();
                        self.ir_code.push_str(&format!("  {} = icmp sgt i64 {}, {}\n", cmp_reg, left_reg, right_reg));
                        format!("  {} = zext i1 {} to i64\n", result_reg, cmp_reg)
                    }

                    Token::Igualdad => {
                        let cmp_reg = self.new_reg();
                        self.ir_code.push_str(&format!("  {} = icmp eq i64 {}, {}\n", cmp_reg, left_reg, right_reg));
                        format!("  {} = zext i1 {} to i64\n", result_reg, cmp_reg)
                    }
                    Token::Diferente => {
                        let cmp_reg = self.new_reg();
                        self.ir_code.push_str(&format!("  {} = icmp ne i64 {}, {}\n", cmp_reg, left_reg, right_reg));
                        format!("  {} = zext i1 {} to i64\n", result_reg, cmp_reg)
                    }
                    _ => return Err(format!("Operador no soportado: {:?}", operador)),
                };
                
                self.ir_code.push_str(&instr);
                Ok((result_reg, "i64".to_string()))
            }
            _ => Err(format!("Expresión no soportada: {:?}", expr)),
        }
    }

    fn new_reg(&mut self) -> String {
        let reg = format!("%r{}", self.var_counter);
        self.var_counter += 1;
        reg
    }

    pub fn print_ir(&self) {
        println!("\n🎯 ═════════════════════════════════════════════");
        println!("⭐ LLVM IR TEXTUAL GENERADO");
        println!("📝 Código LLVM intermedio:");
        println!("═════════════════════════════════════════════\n");
        println!("{}", self.ir_code);
        println!("═════════════════════════════════════════════\n");
    }

    /// Compila el IR LLVM a ejecutable usando herramientas LLVM nativas
    pub fn compile_to_exe(&self, output_file: &str) -> Result<(), String> {
        println!("🔨 Compilando LLVM IR a código máquina...");
        
        // Info de optimización
        let opt_flag = match self.opt_level {
            OptimizationLevel::None => "-O0",
            OptimizationLevel::Fast => "-O1",
            OptimizationLevel::Balanced => "-O2",
            OptimizationLevel::Aggressive => "-O3",
        };
        println!("⚡ Nivel de optimización: {:?} ({})", self.opt_level, opt_flag);

        // Guardar el IR LLVM en un archivo .ll
        let ll_file = output_file.replace(".exe", ".ll");
        fs::write(&ll_file, &self.ir_code)
            .map_err(|e| format!("Error escribiendo archivo LLVM IR: {}", e))?;

        println!("📄 Archivo IR: {}", ll_file);

        // Compilar .ll a .obj directamente SIN assembly intermedio
        let obj_file = output_file.replace(".exe", ".obj");
        
        // Rutas comunes donde podrían estar clang y llc
        let clang_paths = vec![
            "clang",
            "D:\\LLVM\\bin\\clang.exe",
            "d:\\LLVM\\bin\\clang.exe",
            "/d/LLVM/bin/clang.exe",
            "c:\\Program Files\\LLVM\\bin\\clang.exe",
        ];
        
        let mut clang_found: Option<&str> = None;
        for path in &clang_paths {
            println!("🔍 Buscando clang en: {}", path);
            match std::process::Command::new(path).arg("--version").output() {
                Ok(output) if output.status.success() => {
                    clang_found = Some(path);
                    println!("✅ Clang encontrado en: {}", path);
                    break;
                }
                _ => {}
            }
        }
        
        // Compilar directamente a objeto (NO generar assembly)
        if let Some(clang_cmd) = clang_found {
            println!("🔧 Compilando LLVM IR directamente a código máquina (objeto)...");
            let obj_compile = Command::new(clang_cmd)
                .args(&[
                    "-c",
                    opt_flag,  // OPTIMIZACIÓN LEVEL
                    "--target=x86_64-pc-windows-msvc",  // MSVC object format
                    "-fno-asynchronous-unwind-tables",
                    "-flto",   // Link Time Optimization
                    &ll_file,
                    "-o",
                    &obj_file
                ])
                .output()
                .map_err(|e| format!("Error compilando a objeto: {}", e))?;

            if !obj_compile.status.success() {
                let stderr = String::from_utf8_lossy(&obj_compile.stderr);
                let stdout = String::from_utf8_lossy(&obj_compile.stdout);
                return Err(format!("Error compilando: {} stdout: {}", stderr, stdout));
            }

            println!("✅ Objeto compilado: {}", obj_file);
        } else {
            println!("⚠️  clang no encontrado, abortando");
            return Err("clang no disponible en ninguna ubicación estándar".to_string());
        }

        // Link con lld-link (MSVC linker - compatible con clang object files)
        println!("🔗 Linkeando objeto a ejecutable con lld-link...");
        
        let out_flag = format!("-out:{}", output_file);
        let link = Command::new("lld-link")
            .args(&[
                &obj_file,
                &out_flag,
                "-subsystem:console",
                "-defaultlib:libcmt",  // CRT library
                "-defaultlib:kernel32.lib",  // Kernel32
            ])
            .output();

        let linked = if let Ok(output) = link {
            if output.status.success() {
                println!("✅ Linkeado con MSVC linker (lld-link)");
                true
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                println!("⚠️  lld-link falló, intentando gcc: {}", stderr);
                false
            }
        } else {
            false
        };

        if !linked {
            // Fallback a GCC si lld-link no funciona
            println!("🔗 Fallback: Linkeando con GCC...");
            let gcc_link = Command::new("gcc")
                .args(&[&obj_file, "-o", output_file])
                .output()
                .map_err(|e| format!("Error linkeando: {}", e))?;

            if !gcc_link.status.success() {
                let stderr = String::from_utf8_lossy(&gcc_link.stderr);
                return Err(format!("Error de linking (GCC): {}", stderr));
            }
            
            println!("✅ Linkeado con GCC (fallback)");
        }

        // Limpiar archivos intermedios
        let _ = fs::remove_file(&ll_file);
        let _ = fs::remove_file(&obj_file);
        let _ = fs::remove_file(output_file.replace(".exe", ".s"));

        println!("✅ Ejecutable nativo generado: {}", output_file);
        Ok(())
    }
}