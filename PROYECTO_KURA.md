# 📜 Proyecto KURA - Documentación Completa

## 🎯 Descripción General

**KURA** es un lenguaje de programación interpretado de propósito general, escrito en Rust. El objetivo es crear un lenguaje simple, intuitivo y con una sintaxis clara, especialmente diseñado para ser fácil de aprender y entender.

**Versión Actual:** 0.1.0  
**Estado:** En desarrollo activo  
**Extensión de archivos:** `.kr`

---

## 📁 Estructura del Proyecto

```
Kura/
├── Cargo.toml              # Configuración del proyecto Rust
├── src/
│   ├── main.rs             # Punto de entrada, gestor de archivos y flujo principal
│   ├── token.rs            # Definición de tokens (palabras clave, símbolos, literales)
│   ├── lexer.rs            # Analizador léxico (tokenización del código)
│   ├── parser.rs           # Analizador sintáctico (construcción del AST)
│   ├── ast.rs              # Árbol de sintaxis abstracta (estructura del programa)
│   └── evaluator.rs        # Motor de evaluación (ejecución del programa)
└── target/                 # Artefactos compilados de Rust
```

---

## 🔧 Componentes Principales

### 1. **Lexer** (`lexer.rs`)
- **Propósito:** Analiza el código fuente carácter por carácter y genera tokens.
- **Responsabilidades:**
  - Reconoce palabras clave (`let`, `mut`, `print`, `if`, `while`, `fn`, `return`, etc.)
  - Identifica símbolos y operadores (`+`, `-`, `*`, `/`, `==`, `&&`, `||`, `->`)
  - Parsea identificadores, números enteros y cadenas de texto
  - Maneja espacios en blanco y comentarios

### 2. **Token** (`token.rs`)
- **Definición:** Enumeración que lista todos los tipos de tokens reconocibles.
- **Palabras clave** incluidas:
  - Variables: `let`, `mut`
  - Control de flujo: `if`, `else`, `while`, `break`
  - Funciones: `fn`, `return`
  - Operaciones: `print`
  - Booleanos: `true`, `false`
  - Operadores lógicos: `and`, `or`
- **Operadores matemáticos:** `+`, `-`, `*`, `/`
- **Operadores de comparación:** `==`, `<`, `>`
- **Símbolos especiales:** `:`, `=`, `;`, `->`, `[`, `]`, `(`, `)`, `{`, `}`

### 3. **Parser** (`parser.rs`)
- **Propósito:** Convierte la secuencia de tokens en un Árbol de Sintaxis Abstracta (AST).
- **Manejo de:**
  - Declaraciones de variables (`let`)
  - Declaraciones de funciones (`fn`)
  - Sentencias de control (`if`, `while`, `break`)
  - Expresiones matemáticas y lógicas
  - Llamadas a funciones
  - Reasignaciones de variables
  - Importaciones de módulos

### 4. **AST** (`ast.rs`)
- **Estructura:** Define los nodos del árbol de sintaxis.
- **Tipos principales:**
  - **Programa:** Conjunto de declaraciones
  - **Declaraciones:** 
    - `Let` - declaración de variables
    - `Print` - impresión en pantalla
    - `If/Else` - condicionales
    - `While` - bucles
    - `Funcion` - definición de funciones
    - `Return` - retorno de valores
    - `LlamadaSuelta` - llamada a función sin asignación
    - `Reasignacion` - modificación de variables
    - `Importar` - carga de módulos
  - **Expresiones:**
    - Literales: `Entero`, `Cadena`, `Booleano`
    - Variables: `Identificador`
    - Operaciones: `Operacion` (con operador y operandos)
    - Colecciones: `Arreglo`, `Diccionario`
    - Acceso: `Indice` (para arrays/dicts)
    - Funciones: `Llamada`

### 5. **Evaluator** (`evaluator.rs`)
- **Propósito:** Ejecuta el programa interpretado.
- **Características:**
  - Mantiene un entorno (`Entorno`) que almacena variables en memoria
  - Evalúa declaraciones y expresiones
  - Maneja tipos: `Entero`, `Booleano`, `Cadena`, `Arreglo`, `Funcion`, `Diccionario`, `Nulo`
  - Controla el flujo con `Break` y `Retorno`
  - Soporta funciones con parámetros y cuerpo
  - Gestiona diccionarios con HashMap

---

## 🎮 Cómo Funciona el Flujo Principal

```
Entrada (.kr file)
    ↓
[LEXER] - Tokenización
    ↓
[PARSER] - Construcción del AST
    ↓
[EVALUATOR] - Interpretación y ejecución
    ↓
Salida (Printed Results)
```

**Flujo en `main.rs`:**
1. Lee argumentos de línea de comandos (ruta del archivo `.kr`)
2. Valida que sea un archivo `.kr`
3. Lee el contenido del archivo
4. Crea un Lexer y Parser
5. Genera el AST
6. Crea un Entorno (memoria)
7. Evalúa el programa

---

## ✨ Características Implementadas

### Tipos de Datos
- ✅ Enteros (`i64`)
- ✅ Cadenas de texto (`String`)
- ✅ Booleanos (`true`, `false`)
- ✅ Arreglos (`[1, 2, 3]`)
- ✅ Diccionarios (`{"clave": valor}`)
- ✅ Funciones

### Declaraciones
- ✅ Variables inmutables: `let x: Entero = 5;`
- ✅ Variables mutables: `let mut x: Entero = 5;`
- ✅ Print: `print x;`
- ✅ Reasignación: `x = 10;`
- ✅ Condicionales: `if condicion { } else { }`
- ✅ Bucles: `while condicion { }`
- ✅ Break: salir de bucles
- ✅ Funciones: `fn nombre(param1, param2) { }`
- ✅ Return: `return valor;`
- ✅ Importaciones: `import f1, f2 from "archivo.kr";`

### Expresiones y Operadores
- ✅ Operadores matemáticos: `+`, `-`, `*`, `/`
- ✅ Comparación: `==`, `<`, `>`
- ✅ Lógica: `&&`, `||`
- ✅ Acceso a índices: `array[0]`, `dict["key"]`
- ✅ Llamadas a funciones: `funcion(arg1, arg2)`

---

## 🚀 Cómo Usar KURA

### Compilar el proyecto
```bash
cargo build --release
```

### Ejecutar un archivo Kura
```bash
./target/release/Kura archivo.kr
```

### Ejemplo de programa Kura (Mi_Programa.kr)
```kura
let x: Entero = 10;
let mut y: Entero = 5;

print x;
print y;

y = 20;
print y;

if x < y {
    print "x es menor que y";
} else {
    print "x es mayor o igual a y";
}

while y > 0 {
    print y;
    y = y - 1;
}

fn saludar(nombre) {
    print nombre;
}

saludar("Mundo");

let numeros: Arreglo = [1, 2, 3, 4, 5];
print numeros[0];
```

---

## 🎯 Objetivos de Optimización y Mejora

### 🔴 Prioridad Alta

1. **Simplificar la Sintaxis**
   - Considerar hacer innecesaria la declaración de tipos (inferencia de tipos)
   - Ejemplo actual: `let x: Entero = 5;` → Posible: `let x = 5;`
   - Simplificar sintaxis de funciones

2. **Mejorar Mensajes de Error**
   - Agregar línea y columna en errores
   - Mensajes más descriptivos
   - Sugerencias de corrección

3. **Performance y Optimización**
   - Optimizar evaluación de expresiones
   - Cacheo de funciones evaluadas
   - Reducir clonaciones innecesarias

### 🟡 Prioridad Media

4. **Características de Lenguaje**
   - Operador módulo `%`
   - Operadores de potencia `**`
   - Operadores de asignación compuesta `+=`, `-=`, etc.
   - Strings interpolados (template literals)
   - Tuplas
   - Enums

5. **Control de Flujo Avanzado**
   - `match` para pattern matching
   - `for` loops
   - `continue` en bucles
   - Try/catch para manejo de errores

6. **Funciones Integradas (Built-ins)**
   - `len()` - longitud de arrays/strings
   - `push()` / `pop()` para arreglos
   - `keys()` / `values()` para diccionarios
   - `parseInt()`, `parseFloat()`, `toString()`
   - Funciones matemáticas: `sqrt()`, `pow()`, `max()`, `min()`

### 🟢 Prioridad Baja

7. **Documentación y Testing**
   - Ejemplos de programas completos
   - Tests unitarios
   - Documentación de API interna

8. **Funcionalidades Avanzadas**
   - Sistema de módulos mejorado
   - Closures
   - Decoradores
   - Program transpilation a JavaScript/Python

---

## 💡 Opciones de Refactorización para Simplificar

### Opción 1: Inferencia de Tipos
**Cambio:**
```kura
// Actual
let x: Entero = 5;
let s: Cadena = "hola";

// Propuesto
let x = 5;
let s = "hola";
```
**Ventajas:** Sintaxis más limpia, menos verbosa  
**Cambios necesarios:** Modificar parser y evaluator para inferir tipos

### Opción 2: Sintaxis de Función Simplificada
**Cambio:**
```kura
// Actual
fn saludar(nombre) { print nombre; }

// Propuesto (mismo, pero considerar)
def saludar(nombre) { print nombre; }
```

### Opción 3: Importación Simplificada
**Cambio:**
```kura
// Actual
import f1, f2 from "archivo.kr";

// Propuesto
use "archivo.kr";
// o
include "archivo.kr";
```

---

## 🔍 Análisis Actual de Código

### Fortalezas
✅ Estructura bien organizada (lexer → parser → ast → evaluator)  
✅ Sistema modular en Rust  
✅ Soporte para funciones, bucles y condicionales  
✅ Tipos de datos básicos y colecciones  
✅ Sistema de importación de módulos  

### Áreas de Mejora
- Inferencia de tipos (reducir verbosidad)
- Mensajes de error más informativos
- Manejo de excepciones
- Más funciones integradas
- Optimización de performance
- Sistema de tipos más robusto

---

## 📝 Notas Adicionales para IAs

### Para Continuar el Desarrollo

1. **Entender primero:**
   - El flujo: Lexer → Parser → AST → Evaluator
   - Cómo se agrega un nuevo token requiere cambio en `token.rs`, `lexer.rs`, `parser.rs`, `ast.rs`, y `evaluator.rs`

2. **Pasos típicos para añadir características:**
   - Agregue el token en `token.rs`
   - Agregue reconocimiento en `lexer.rs`
   - Agregue parsing en `parser.rs`
   - Agregue nodo AST en `ast.rs`
   - Agregue evaluación en `evaluator.rs`

3. **Testing:**
   - Cree archivos `.kr` de prueba
   - Ejecute con: `cargo run -- archivo.kr`

4. **Compiler command:** 
   - Debug: `cargo build`
   - Release: `cargo build --release`
   - Run: `cargo run -- archivo.kr`

---

## 🎓 Conclusión

KURA es un proyecto educativo y experimental para crear un lenguaje de programación simple. El código está bien estructurado para extensión y mejora. Las prioridades actuales son:

1. ✨ Simplificar la sintaxis (especialmente tipos)
2. 🐛 Mejorar manejo de errores
3. 📈 Agregar más funciones integradas
4. 🚀 Optimizar performance

---

**Última actualización:** Marzo 2026  
**Autor:** Equipo de Desarrollo Kura  
**Licencia:** (Especificar si aplica)

