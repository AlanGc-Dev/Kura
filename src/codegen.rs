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
    current_scope: HashMap<String, String>, // nombre -> registro LLVM
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
        
        // Definición de la función main
        self.ir_code.push_str("define i32 @main() {\n");
        self.ir_code.push_str("entry:\n");

        // Generar el cuerpo
        for stmt in &programa.declaraciones {
            self.generate_statement(stmt)?;
        }

        // Retorno 0
        self.ir_code.push_str("  ret i32 0\n");
        self.ir_code.push_str("}\n");

        println!("✅ LLVM IR generada exitosamente");
        println!("📊 {} declaraciones compiladas", programa.declaraciones.len());

        Ok(self.ir_code.clone())
    }

    fn generate_statement(&mut self, stmt: &Declaracion) -> Result<(), String> {
        match stmt {
            Declaracion::Let { nombre, valor, .. } => {
                let (reg, _type) = self.generate_expr(valor)?;
                // En LLVM textual, no necesitamos alloca para valores simples
                // Solo guardamos en el scope
                self.current_scope.insert(nombre.clone(), reg);
            }
            Declaracion::Print { valor } => {
                let (reg, _type) = self.generate_expr(valor)?;
                
                // @.str format string - %lld\0A\00 = 6 bytes (including null terminator)
                // Insertar string de formato antes de main
                let format_str = "@.str.fmt = private unnamed_addr constant [6 x i8] c\"%lld\\0A\\00\", align 1\n";
                if !self.ir_code.contains("@.str.fmt") {
                    let insert_pos = self.ir_code.find("define i32 @main").unwrap();
                    self.ir_code.insert_str(insert_pos, format_str);
                }
                
                self.ir_code.push_str("  %str.ptr = getelementptr inbounds [5 x i8], [5 x i8]* @.str.fmt, i32 0, i32 0\n");
                self.ir_code.push_str(&format!("  call i32 (i8*, ...) @printf(i8* %str.ptr, i64 {})\n", reg));
            }
            Declaracion::Reasignacion { nombre, valor } => {
                let (reg, _type) = self.generate_expr(valor)?;
                self.current_scope.insert(nombre.clone(), reg);
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
            Expresion::Identificador(name) => {
                if let Some(reg) = self.current_scope.get(name) {
                    Ok((reg.clone(), "i64".to_string()))
                } else {
                    Err(format!("Variable {} no definida", name))
                }
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