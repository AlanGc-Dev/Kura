use std::fs;
use kura::{lexer, parser, evaluator, codegen::{self, OptimizationLevel, CompilationTarget}};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 || args[1] == "--help" || args[1] == "-h" {
        mostrar_ayuda();
        return;
    }

    let mut compile_mode = false;
    let mut opt_level = OptimizationLevel::Balanced;
    let mut target = CompilationTarget::WindowsX86_64;
    let mut archivo = String::new();
    let mut i = 1;

    while i < args.len() {
        match args[i].as_str() {
            "--compile" => {
                compile_mode = true;
                i += 1;
            }
            "--target" => {
                i += 1;
                if i < args.len() {
                    if let Some(t) = CompilationTarget::from_string(&args[i]) {
                        target = t;
                        i += 1;
                    } else {
                        println!("❌ Target desconocido: {}. Usa --help para ver los targets disponibles.", args[i]);
                        std::process::exit(1);
                    }
                } else {
                    println!("❌ Se requiere especificar el target después de --target");
                    std::process::exit(1);
                }
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

    ejecutar_archivo(&archivo, compile_mode, opt_level, target);
}

fn ejecutar_archivo(ruta: &str, compile: bool, opt_level: OptimizationLevel, target: CompilationTarget) {
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
        match codegen::CodeGenerator::with_target(opt_level, target) {
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
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║         KURA LANGUAGE COMPILER - CrossCompilation         ║");
    println!("╚════════════════════════════════════════════════════════════╝");
    println!("\n📖 Uso:");
    println!("  kura archivo.kr");
    println!("  kura --compile archivo.kr");
    println!("  kura --compile --release archivo.kr");
    println!("  kura --compile --target <TARGET> archivo.kr");
    println!("  kura --compile --target <TARGET> -O0/-O1/-O2/-O3 archivo.kr");
    
    println!("\n🎯 Targets Soportados:");
    println!("  windows-x86_64     Windows 64-bit (default)");
    println!("  linux-x86_64       Linux 64-bit");
    println!("  linux-arm64        Linux ARM64 (Raspberry Pi 4+)");
    println!("  macos-x86_64       macOS 64-bit (Intel)");
    println!("  macos-arm64        macOS ARM64 (Apple Silicon)");
    
    println!("\n⚡ Niveles de Optimización:");
    println!("  -O0               Sin optimización (rápida compilación)");
    println!("  -O1               Optimización mínima");
    println!("  -O2               Optimización balanceada (default)");
    println!("  -O3               Optimización máxima");
    println!("  --release         Equivalente a -O3");
    
    println!("\n📝 Ejemplos:");
    println!("  kura test.kr                                  # Ejecutar en modo intérprete");
    println!("  kura --compile test.kr                        # Compilar para Windows x64");
    println!("  kura --compile --target linux-x86_64 test.kr  # Compilar para Linux");
    println!("  kura --compile --target macos-arm64 -O3 test.kr # Compilar para macOS Apple Silicon");
    println!();
}
