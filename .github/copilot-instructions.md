---
name: kura-project-instructions
description: "Instrucciones para contribuir al desarrollo del lenguaje KURA: simplificar sintaxis, optimizar código, añadir características. Aplica a archivos Rust en src/."
applyTo: "src/**/*.rs"
---

# Instrucciones para IAs: Desarrollo del Lenguaje KURA

## Descripción General del Proyecto

KURA es un lenguaje de programación interpretado de propósito general, escrito en Rust. El objetivo es crear un lenguaje simple, intuitivo y con una sintaxis clara, especialmente diseñado para ser fácil de aprender y entender. Versión actual: 0.1.0. Extensión de archivos: `.kr`.

**Flujo principal:** Lexer → Parser → AST → Evaluator.

## Estructura del Código

- `main.rs`: Punto de entrada, gestor de archivos y flujo principal.
- `token.rs`: Definición de tokens (palabras clave, símbolos, literales).
- `lexer.rs`: Analizador léxico (tokenización del código).
- `parser.rs`: Analizador sintáctico (construcción del AST).
- `ast.rs`: Árbol de sintaxis abstracta (estructura del programa).
- `evaluator.rs`: Motor de evaluación (ejecución del programa).

## Características Actuales

### Tipos de Datos
- Enteros (`i64`), Cadenas (`String`), Booleanos (`true`, `false`), Arreglos, Diccionarios, Funciones.

### Declaraciones
- Variables: `let x: Entero = 5;`, `let mut x: Entero = 5;`
- Print: `print x;`
- Reasignación: `x = 10;`
- Condicionales: `if condicion { } else { }`
- Bucles: `while condicion { }`
- Break: salir de bucles
- Funciones: `fn nombre(param1, param2) { }`
- Return: `return valor;`
- Importaciones: `import f1, f2 from "archivo.kr";`

### Expresiones y Operadores
- Matemáticos: `+`, `-`, `*`, `/`
- Comparación: `==`, `<`, `>`
- Lógica: `&&`, `||`
- Acceso: `array[0]`, `dict["key"]`
- Llamadas: `funcion(arg1, arg2)`

## Objetivos de Mejora Prioritarios

### Alta Prioridad
1. **Simplificar la Sintaxis (Inferencia de Tipos)**: Eliminar declaración explícita de tipos. Ejemplo: Cambiar `let x: Entero = 5;` a `let x = 5;`. Modificar parser y evaluator para inferir tipos automáticamente.
2. **Mejorar Mensajes de Error**: Agregar línea y columna en errores, mensajes más descriptivos con sugerencias de corrección.
3. **Optimización de Performance**: Reducir clonaciones innecesarias, cacheo de funciones, optimizar evaluación de expresiones.

### Media Prioridad
4. **Características Adicionales**: Operador módulo `%`, operadores de potencia `**`, asignación compuesta `+=`, strings interpolados, tuplas, enums.
5. **Control de Flujo Avanzado**: `match`, `for` loops, `continue`, try/catch.
6. **Funciones Integradas**: `len()`, `push()`, `pop()`, `keys()`, `values()`, funciones matemáticas.

### Baja Prioridad
7. **Testing y Documentación**: Tests unitarios, ejemplos completos.
8. **Funcionalidades Avanzadas**: Closures, decoradores, transpilation.

## Guías para Contribuir

### Pasos para Añadir una Nueva Característica
1. **Agregar Token**: En `token.rs`, añadir el nuevo token (e.g., `Modulo` para `%`).
2. **Reconocimiento en Lexer**: En `lexer.rs`, añadir lógica para reconocer el símbolo (e.g., `%`).
3. **Parsing**: En `parser.rs`, añadir reglas de parsing para el nuevo token.
4. **Nodo AST**: En `ast.rs`, añadir el nodo correspondiente (e.g., `Operacion` con operador `Modulo`).
5. **Evaluación**: En `evaluator.rs`, implementar la lógica de evaluación (e.g., `a % b`).
6. **Testing**: Crear archivos `.kr` de prueba y ejecutar con `cargo run -- archivo.kr`.

### Compilación y Ejecución
- Debug: `cargo build`
- Release: `cargo build --release`
- Run: `cargo run -- archivo.kr`

### Ejemplos de Código KURA
```kura
let x = 10;  // Inferir tipo Entero
let mut y = 5;
print x;
y = 20;
if x < y {
    print "x es menor";
}
while y > 0 {
    print y;
    y = y - 1;
}
fn saludar(nombre) {
    print nombre;
}
saludar("Mundo");
let nums = [1, 2, 3];
print nums[0];
```

### Notas para IAs
- Mantener consistencia con el estilo Rust existente.
- Priorizar cambios que simplifiquen la sintaxis sin romper compatibilidad.
- Si hay dudas, consultar PROYECTO_KURA.md para más detalles.
- Enfocarse en optimización: evitar clones innecesarios, usar referencias donde sea posible.