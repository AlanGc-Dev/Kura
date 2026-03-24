use kura::GestorPaquetes;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        mostrar_ayuda();
        return;
    }

    match args[1].as_str() {
        "init" => {
            let nombre = args.get(2).map(|s| s.to_string());
            GestorPaquetes::init(nombre)
        }
        "add" => {
            if args.len() < 3 {
                println!("❌ Uso: kup add <paquete> [version]");
                println!("   Ejemplos:");
                println!("     kup add math 1.0");
                println!("     kup add usuario/repo");
                println!("     kup add https://github.com/user/repo/raw/main/lib.kr");
            } else {
                let version = args.get(3).map(|s| s.to_string());
                GestorPaquetes::add(&args[2], version.as_deref());
            }
        }
        "remove" => {
            if args.len() < 3 {
                println!("❌ Uso: kup remove <paquete>");
            } else {
                GestorPaquetes::remove(&args[2]);
            }
        }
        "run" => {
            if args.len() < 3 {
                println!("❌ Uso: kup run <script>");
                println!("   Ejemplos: kup run start, kup run dev, kup run build");
            } else {
                GestorPaquetes::run(&args[2]);
            }
        }
        "install" => GestorPaquetes::install(),
        "update" => GestorPaquetes::update(),
        "list" => GestorPaquetes::list(),
        "search" => {
            if args.len() < 3 {
                println!("❌ Uso: kup search <termino>");
            } else {
                GestorPaquetes::search(&args[2]);
            }
        }
        "info" => {
            if args.len() < 3 {
                println!("❌ Uso: kup info <paquete>");
            } else {
                GestorPaquetes::info(&args[2]);
            }
        }
        _ => mostrar_ayuda(),
    }
}

fn mostrar_ayuda() {
    println!("\n🛠️  KUP - Kura Universal Package Manager v0.3.0");
    println!("═══════════════════════════════════════════════════");
    println!("\n📘 Comandos de Proyectos:");
    println!("  kup init [nombre]        Inicializar nuevo proyecto");
    println!("  kup install              Instalar todas las dependencias");
    println!("  kup update               Actualizar dependencias");
    println!("  kup list                 Listar dependencias instaladas");
    println!("\n📦 Comandos de Paquetes:");
    println!("  kup add <pkg> [v]        Agregar paquete a dependencias");
    println!("  kup add usuario/repo     Agregar desde GitHub");
    println!("  kup remove <pkg>         Eliminar paquete");
    println!("  kup search <term>        Buscar paquetes");
    println!("  kup info <pkg>           Ver información");
    println!("\n▶️  Scripts:");
    println!("  kup run <script>         Ejecutar script (start, dev, build)");
    println!("\n📚 Ejemplos:");
    println!("  kup init mi_proyecto     Crear proyecto");
    println!("  kup add math 1.0         Agregar paquete");
    println!("  kup add usuario/lib      Agregar desde GitHub");
    println!("  kup run start            Ejecutar script start");
    println!("  kup search http          Buscar paquetes");
    println!("\n💡 Para ejecutar archivos, usa: kura <archivo.kr>");
    println!("\n")
}
