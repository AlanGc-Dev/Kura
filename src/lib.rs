// Módulos compartidos entre kura y kup
pub mod lexer;
pub mod parser;
pub mod ast;
pub mod evaluator;
pub mod token;
pub mod types;
pub mod package_manager;
pub mod codegen;

pub use package_manager::GestorPaquetes;
