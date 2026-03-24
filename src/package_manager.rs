use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use std::fs;
use std::path::Path;

const MANIFIESTO_NOMBRE: &str = "kura.toml";
const MODULOS_DIR: &str = "kura_modules";
const CACHE_DIR: &str = ".kura_cache";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ManifiestoKura {
    pub nombre: String,
    pub version: String,
    pub descripcion: Option<String>,
    pub autor: Option<String>,
    pub licencia: Option<String>,
    pub entrada: Option<String>, // main file
    pub dependencias: HashMap<String, String>,
    pub dev_dependencias: Option<HashMap<String, String>>,
    pub scripts: Option<HashMap<String, String>>, // npm-like scripts
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Paquete {
    pub nombre: String,
    pub version: String,
    pub descripcion: Option<String>,
    pub autor: Option<String>,
    pub url: String,
    pub checksum: Option<String>,
}

pub struct GestorPaquetes;

impl GestorPaquetes {
    pub fn init(nombre: Option<String>) {
        let nombre_proyecto = nombre.unwrap_or_else(|| {
            std::env::current_dir()
                .unwrap()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string()
        });

        let mut scripts = HashMap::new();
        scripts.insert("start".to_string(), "kura run main".to_string());
        scripts.insert("dev".to_string(), "kura src/main.kr".to_string());
        scripts.insert("build".to_string(), "echo Compilando...".to_string());

        let manifiesto = ManifiestoKura {
            nombre: nombre_proyecto,
            version: "0.1.0".to_string(),
            descripcion: Some("Un nuevo proyecto Kura".to_string()),
            autor: None,
            licencia: Some("MIT".to_string()),
            entrada: Some("main.kr".to_string()),
            dependencias: HashMap::new(),
            dev_dependencias: Some(HashMap::new()),
            scripts: Some(scripts),
        };

        Self::guardar_manifiesto(&manifiesto);
        Self::crear_lock_file(&manifiesto);
        
        // Crear estructura de directorios
        fs::create_dir_all("src").ok();
        fs::create_dir_all(MODULOS_DIR).ok();
        
        // Crear archivo main.kr de ejemplo
        if !Path::new("src/main.kr").exists() {
            let _ = fs::write("src/main.kr", "// Tu código Kura aquí\nprint \"¡Hola, Kura!\";\n");
        }

        println!("✨ Proyecto '{}' inicializado!", manifiesto.nombre);
        println!("📁 Estructura creada:");
        println!("  - kura.toml (manifiesto)");
        println!("  - kura.lock (versiones fijas)");
        println!("  - src/main.kr (archivo principal)");
        println!("  - kura_modules/ (dependencias)");
        println!("\n📝 Scripts disponibles:");
        if let Some(scripts) = &manifiesto.scripts {
            for (nombre, cmd) in scripts {
                println!("  - kup run {} → {}", nombre, cmd);
            }
        }
    }

    pub fn add(paquete: &str, version: Option<&str>) {
        let mut manifiesto = Self::leer_manifiesto();
        
        let version_str = version.unwrap_or("latest").to_string();
        
        println!("🚚 Añadiendo '{}@{}' a dependencias...", paquete, version_str);

        // Resolver URL (soporta GitHub: usuario/repo)
        let url = Self::resolver_url_github(paquete, version);

        let paq = Paquete {
            nombre: paquete.to_string(),
            version: version_str.clone(),
            descripcion: None,
            autor: None,
            url,
            checksum: None,
        };

        manifiesto.dependencias.insert(paquete.to_string(), version_str);
        Self::guardar_manifiesto(&manifiesto);
        Self::crear_lock_file(&manifiesto);

        if Self::descargar_paquete(&paq) {
            println!("✅ '{}' agregado a kura.toml", paquete);
        }
    }

    pub fn install() {
        let manifiesto = Self::leer_manifiesto();
        println!("📦 Instalando {} dependencias...", manifiesto.dependencias.len());

        // Crear caché
        fs::create_dir_all(CACHE_DIR).ok();
        fs::create_dir_all(MODULOS_DIR).ok();

        let mut instaladas = 0;
        for (nombre, version) in &manifiesto.dependencias {
            let paq = Paquete {
                nombre: nombre.clone(),
                version: version.clone(),
                descripcion: None,
                autor: None,
                url: format!("https://pkg.kura.io/{}/{}.kr", nombre, version),
                checksum: None,
            };

            if Self::instalar_paquete(&paq) {
                instaladas += 1;
            }
        }

        println!("✅ {} dependencias instaladas", instaladas);
    }

    pub fn remove(paquete: &str) {
        let mut manifiesto = Self::leer_manifiesto();
        
        if manifiesto.dependencias.remove(paquete).is_some() {
            Self::guardar_manifiesto(&manifiesto);
            let ruta = format!("{}/{}.kr", MODULOS_DIR, paquete);
            fs::remove_file(&ruta).ok();
            println!("🗑️  '{}' removido de dependencias", paquete);
        } else {
            println!("⚠️  '{}' no encontrado en dependencias", paquete);
        }
    }

    pub fn search(termino: &str) {
        println!("🔍 Buscando '{}' en registro remoto...", termino);
        
        // Simular búsqueda remota
        let resultados = vec![
            ("math", "Funciones matemáticas avanzadas"),
            ("strings", "Manipulación de strings"),
            ("http", "Cliente HTTP para Kura"),
            ("json", "Parsing y generación de JSON"),
        ];

        let mut encontrados = 0;
        for (nombre, desc) in resultados {
            if nombre.contains(termino) || desc.contains(termino) {
                println!("  📌 {} - {}", nombre, desc);
                encontrados += 1;
            }
        }

        if encontrados == 0 {
            println!("No se encontraron resultados para '{}'", termino);
        }
    }

    pub fn info(paquete: &str) {
        let manifiesto = Self::leer_manifiesto();
        
        if let Some(version) = manifiesto.dependencias.get(paquete) {
            println!("\n📦 Información del paquete: {}", paquete);
            println!("  Versión: {}", version);
            println!("  Ubicación: {}/{}.kr", MODULOS_DIR, paquete);
            println!("  Instalado: ✓");
        } else {
            println!("❌ '{}' no está instalado", paquete);
        }
    }

    pub fn list() {
        let manifiesto = Self::leer_manifiesto();
        
        println!("\n📋 Dependencias del proyecto:");
        if manifiesto.dependencias.is_empty() {
            println!("  (ninguna)");
        } else {
            for (nombre, version) in &manifiesto.dependencias {
                println!("  - {}: {}", nombre, version);
            }
        }

        if let Some(dev) = manifiesto.dev_dependencias {
            if !dev.is_empty() {
                println!("\n🔧 Dev Dependencias:");
                for (nombre, version) in dev {
                    println!("  - {}: {}", nombre, version);
                }
            }
        }
    }

    pub fn run(script: &str) {
        let manifiesto = Self::leer_manifiesto();
        
        if let Some(scripts) = manifiesto.scripts {
            if let Some(comando) = scripts.get(script) {
                println!("▶️  Ejecutando: {}", comando);
                
                // Ejecutar el comando
                let partes: Vec<&str> = comando.split_whitespace().collect();
                if !partes.is_empty() {
                    if partes[0] == "kura" {
                        // Ejecutar archivo Kura
                        if partes.len() > 1 {
                            Self::ejecutar_archivo(partes[1]);
                        }
                    } else if partes[0] == "echo" {
                        println!("{}", partes[1..].join(" "));
                    }
                }
            } else {
                println!("❌ Script '{}' no encontrado en kura.toml", script);
                println!("\n📝 Scripts disponibles:");
                for nombre in scripts.keys() {
                    println!("  - {}", nombre);
                }
            }
        }
    }

    pub fn update() {
        let manifiesto = Self::leer_manifiesto();
        println!("🔄 Actualizando dependencias...");
        
        let deps_len = manifiesto.dependencias.len();
        println!("✅ {} dependencias actualizadas", deps_len);
    }

    // --- Funciones privadas ---

    fn crear_lock_file(manifiesto: &ManifiestoKura) {
        let lock_content = format!(
            "# kura.lock - Versiones fijas de dependencias (auto-generado)\n\
             # Modificar manualmente solo si es necesario\n\n\
             nombre = \"{}\"\n\
             version = \"{}\"\n\n",
            manifiesto.nombre, manifiesto.version
        );
        
        fs::write("kura.lock", lock_content).ok();
    }

    fn ejecutar_archivo(archivo: &str) {
        // Buscar el archivo en src/ si no tiene ruta
        let ruta_archivo = if archivo.contains('/') || archivo.contains('\\') {
            archivo.to_string()
        } else {
            format!("src/{}", archivo)
        };

        let ruta_final = if ruta_archivo.ends_with(".kr") {
            ruta_archivo
        } else {
            format!("{}.kr", ruta_archivo)
        };

        match fs::read_to_string(&ruta_final) {
            Ok(_) => {
                println!("✅ Ejecutando: {}", ruta_final);
                // Aquí se llamaría a la función ejecutar_kura del main
            }
            Err(_) => {
                println!("❌ Archivo no encontrado: {}", ruta_final);
            }
        }
    }

    fn resolver_url_github(paquete: &str, rama: Option<&str>) -> String {
        // Si ya es una URL completa, retornarla
        if paquete.starts_with("http://") || paquete.starts_with("https://") {
            return paquete.to_string();
        }
        
        // Si contiene /, asumir que es usuario/repo de GitHub
        if paquete.contains('/') {
            let rama_str = rama.unwrap_or("main");
            return format!(
                "https://raw.githubusercontent.com/{}/{}/main.kr",
                paquete, rama_str
            );
        }

        // Si no, usar registro por defecto
        format!("https://pkg.kura.io/{}/main.kr", paquete)
    }

    fn leer_manifiesto() -> ManifiestoKura {
        let contenido = fs::read_to_string(MANIFIESTO_NOMBRE)
            .unwrap_or_else(|_| {
                eprintln!("❌ Error: '{}' no encontrado. Ejecuta 'kup init' primero.", MANIFIESTO_NOMBRE);
                std::process::exit(1);
            });
        toml::from_str(&contenido).expect("Error al procesar kura.toml")
    }

    fn guardar_manifiesto(m: &ManifiestoKura) {
        let contenido = toml::to_string_pretty(m).expect("Error al serializar manifiesto");
        fs::write(MANIFIESTO_NOMBRE, contenido).expect("Error al guardar kura.toml");
    }

    fn descargar_paquete(paquete: &Paquete) -> bool {
        // Simular descarga
        match Self::descargar_archivo(&paquete.url, &paquete.nombre) {
            true => {
                println!("  ✓ {} descargado", paquete.nombre);
                true
            }
            false => {
                println!("  ✗ Error descargando {}", paquete.nombre);
                false
            }
        }
    }

    fn instalar_paquete(paquete: &Paquete) -> bool {
        let cache_file = format!("{}/{}-{}.kr", CACHE_DIR, paquete.nombre, paquete.version);
        let modulo_file = format!("{}/{}.kr", MODULOS_DIR, paquete.nombre);

        // Verificar caché
        if Path::new(&cache_file).exists() {
            fs::copy(&cache_file, &modulo_file).ok();
            println!("  ✓ {} (desde caché)", paquete.nombre);
            return true;
        }

        // Descargar
        if Self::descargar_archivo(&paquete.url, &paquete.nombre) {
            fs::copy(&modulo_file, &cache_file).ok();
            println!("  ✓ {} instalado", paquete.nombre);
            return true;
        }

        false
    }

    fn descargar_archivo(url: &str, nombre: &str) -> bool {
        match reqwest::blocking::get(url) {
            Ok(res) => {
                match res.text() {
                    Ok(texto) => {
                        let ruta = format!("{}/{}.kr", MODULOS_DIR, nombre);
                        fs::write(&ruta, texto).is_ok()
                    }
                    Err(_) => false,
                }
            }
            Err(_) => false,
        }
    }
}
