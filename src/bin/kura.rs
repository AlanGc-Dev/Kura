use std::fs;
use kura::{lexer, parser, evaluator, codegen::{self, OptimizationLevel}};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 || args[1] == "--help" || args[1] == "-h" {
        mostrar_ayuda();
        return;
    }

    let mut compile_mode = false;
    let mut opt_level = OptimizationLevel::Balanced;
    let mut archivo = String::new();
    let mut i = 1;

    while i < args.len() {
        match args[i].as_str() {
            "--compile" => {
                compile_mode = true;
                i += 1;
            }
            "--release" => {
                opt_level = OptimizationLevel::Aggressive;
                i += 1;
            }
            "-O0" => {
                opt_level = OptimizationLevel::None;
                i += 1;
            }
            "-O1" => {
                opt_level = OptimizationLevel::Fast;
                i += 1;
            }
            "-O2" => {
                opt_level = OptimizationLevel::Balanced;
                i += 1;
            }
            "-O3" => {
                opt_level = OptimizationLevel::Aggressive;
                i += 1;
            }
            arg => {
                archivo = arg.to_string();
                i += 1;
            }
        }
    }

    if archivo.is_empty() {
        println!("Error: No se especifico archivo.");
        mostrar_ayuda();
        return;
    }

    ejecutar_archivo(&archivo, compile_mode, opt_level);
}

fn ejecutar_archivo(ruta: &str, compile: bool, opt_level: OptimizationLevel) {
    let ruta_archivo = if ruta.contains('/') || ruta.contains('\\') {
        ruta.to_string()
    } else {
        format!("src/{}", ruta)
    };

    let ruta_final = if ruta_archivo.ends_with(".kr") {
        ruta_archivo
    } else {
        format!("{}.kr", ruta_archivo)
    };

    let codigo = fs::read_to_string(&ruta_final).unwrap_or_else(|_| {
        println!("Error: No se pudo leer '{}'", ruta_final);
        std::process::exit(1);
    });

    let lexer = lexer::Lexer::new(&codigo);
    let mut parser = parser::Parser::new(lexer, &codigo);
    let programa = parser.parse_programa();

    if compile {
        match codegen::CodeGenerator::with_optimization(opt_level) {
            Ok(mut codegen) => {
                match codegen.generate(programa) {
                    Ok(_) => {
                        codegen.print_ir();

                        let exe_name = ruta_final
                            .trim_end_matches(".kr")
                            .split('/')
                            .last()
                            .unwrap_or(&ruta_final)
                            .split('\\')
                            .last()
                            .unwrap_or(&ruta_final);

                        let exe_output = format!("{}.exe", exe_name);

                        match codegen.compile_to_exe(&exe_output) {
                            Ok(_) => {
                                println!("Compilacion exitosa: {}", exe_output);
                            }
                            Err(e) => {
                                println!("Error: {}", e);
                                std::process::exit(1);
                            }
                        }
                    }
                    Err(e) => {
                        println!("Error: {}", e);
                        std::process::exit(1);
                    }
                }
            }
            Err(e) => {
                println!("Error: {}", e);
                std::process::exit(1);
            }
        }
    } else {
        let entorno = evaluator::Entorno::new();
        evaluator::evaluar_programa(programa, entorno);
    }
}

fn mostrar_ayuda() {
    println!("Uso:");
    println!("  kura archivo.kr");
    println!("  kura --compile archivo.kr");
    println!("  kura --compile --release archivo.kr");
    println!("  kura --compile -O0/-O1/-O2/-O3 archivo.kr");
}
