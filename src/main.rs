use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use std::fs;

mod lexer;
mod parser;
mod ast;
mod evaluator;
mod token;
mod types;

// El nombre de tu nuevo manifiesto elegante
const MANIFIESTO_NOMBRE: &str = "kura.toml";

#[derive(Serialize, Deserialize, Debug)]
struct Manifiesto {
    nombre: String,
    version: String,
    dependencias: HashMap<String, String>,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        mostrar_ayuda();
        return;
    }

    match args[1].as_str() {
        "init" => kup_init(),
        "add" => {
            if args.len() < 3 {
                println!("❌ Uso: kup add <url_de_la_libreria>");
            } else {
                kup_add(&args[2]);
            }
        },
        "install" => kup_install(),
        // Si el argumento termina en .kr, lo ejecutamos como script
        archivo if archivo.ends_with(".kr") => ejecutar_kura(archivo),
        _ => mostrar_ayuda(),
    }
}

fn kup_init() {
    let nombre_proyecto = std::env::current_dir().unwrap()
        .file_name().unwrap().to_str().unwrap().to_string();

    let nuevo_manifiesto = Manifiesto {
        nombre: nombre_proyecto,
        version: "0.1.0".to_string(),
        dependencias: HashMap::new(),
    };

    let contenido_toml = toml::to_string_pretty(&nuevo_manifiesto).unwrap();
    fs::write(MANIFIESTO_NOMBRE, contenido_toml).expect("No se pudo crear kura.toml");
    println!("✨ ¡Proyecto Kura inicializado! Se ha creado '{}'.", MANIFIESTO_NOMBRE);
}

fn kup_add(url: &str) {
    let mut manifiesto = leer_manifiesto();
    // Extraemos el nombre de la librería de la URL (ej: math.kr -> math)
    let nombre_lib = url.split('/').last().unwrap().replace(".kr", "");

    println!("🚚 Añadiendo '{}' a tus dependencias...", nombre_lib);

    if descargar_archivo(url, &nombre_lib) {
        manifiesto.dependencias.insert(nombre_lib, url.to_string());
        guardar_manifiesto(manifiesto);
        println!("✅ Registrado con éxito en kura.toml");
    }
}

fn kup_install() {
    let manifiesto = leer_manifiesto();
    println!("📦 Instalando dependencias para '{}'...", manifiesto.nombre);

    for (nombre, url) in manifiesto.dependencias {
        descargar_archivo(&url, &nombre);
    }
    println!("✨ Todas las librerías están listas en kura_modules/");
}

// --- FUNCIONES DE APOYO ---

fn leer_manifiesto() -> Manifiesto {
    let contenido = fs::read_to_string(MANIFIESTO_NOMBRE)
        .unwrap_or_else(|_| {
            println!("❌ Error: No se encuentra '{}'. Ejecuta 'kup init' primero.", MANIFIESTO_NOMBRE);
            std::process::exit(1);
        });
    toml::from_str(&contenido).expect("Error al procesar kura.toml")
}

fn guardar_manifiesto(m: Manifiesto) {
    let contenido_toml = toml::to_string_pretty(&m).unwrap();
    fs::write(MANIFIESTO_NOMBRE, contenido_toml).expect("Error al guardar kura.toml");
}

fn descargar_archivo(url: &str, nombre: &str) -> bool {
    let _ = fs::create_dir_all("kura_modules");
    match reqwest::blocking::get(url) {
        Ok(res) => {
            if let Ok(texto) = res.text() {
                let ruta = format!("kura_modules/{}.kr", nombre);
                fs::write(ruta, texto).is_ok()
            } else {
                println!("❌ Error al descargar de la URL.");
                false
            }
        }
        Err(_) => {
            println!("❌ Error de conexión al intentar descargar.");
            false
        }
    }
}

fn ejecutar_kura(ruta: &str) {
    let codigo = fs::read_to_string(ruta).unwrap_or_else(|_| {
        println!("❌ Error: No se pudo leer el archivo '{}'", ruta);
        std::process::exit(1);
    });

    let lexer = lexer::Lexer::new(&codigo);
    let mut parser = parser::Parser::new(lexer, &codigo);
    let programa = parser.parse_programa();
    let entorno = evaluator::Entorno::new();
    evaluator::evaluar_programa(programa, entorno);
}

fn mostrar_ayuda() {
    println!("🛠️  Kup: Kura Packer");
    println!("-------------------");
    println!("Uso:");
    println!("  kup init          - Inicializa un nuevo proyecto Kura");
    println!("  kup add <url>     - Descarga una librería y la añade al proyecto");
    println!("  kup install       - Descarga todas las dependencias del kura.toml");
    println!("  kup <archivo.kr>  - Ejecuta un script de Kura");
}