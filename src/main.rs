// NOTA: Este archivo no se usa. Los binarios están en src/bin/
// - src/bin/kura.rs (intérprete)
// - src/bin/kup.rs (gestor de paquetes)
//
// Cargo usa la configuración en Cargo.toml:
// [[bin]]
// name = "kura"
// path = "src/bin/kura.rs"
//
// [[bin]]
// name = "kup"
// path = "src/bin/kup.rs"

fn main() {
    println!("ERROR: Usa 'kura' o 'kup' en lugar de este binario.");
    println!("Ejecuta: cargo run --release --bin kura <archivo.kr>");
    println!("O: cargo run --release --bin kup <comando>");
}
