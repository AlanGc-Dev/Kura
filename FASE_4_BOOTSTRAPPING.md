# 🚀 FASE 4 - BOOTSTRAPPING Y SISTEMA DE MÓDULOS

## 📊 Estado Actual: March 24, 2026

### ✅ COMPLETADO
1. **Cross-Compilation** (5 targets): Windows, Linux x86/ARM, macOS x86/ARM ✅
2. **Module System v0.1**: Import/Export funcional ✅
3. **Runtime Interpretation**: Cualquier código `.kr` ejecutable ✅
4. **LLVM Compilation**: Código `.kr` → executables nativos ✅

### 🎯 OBJETIVO FASE 4
**Escribir el compilador KURA EN KURA** (self-hosting)

Permitirá:
- Mejoras rápidas sin recompilar Rust
- Confiabilidad: compilador bootstraps a sí mismo
- Comunidad: otros pueden expandir el compilador fácilmente

## 🔧 Bloqueadores Identificados

### 1. ✅ RESUELTO: Sistema de Módulos
```kura
// Ahora disponible:
import { lexer_fn } from "lexer.kr";
import { parser_fn } from "parser.kr";
import { codegen_fn } from "codegen.kr";
```

### 2. ⏳ SIGUIENTE: Gestión de Heap Memory
Necesario para:
- Allocar nodos de AST dinámicamente
- Trabajar con punteros
- Manejo de memoria manual en bootstrap

```kura
// Código que necesitaremos escribir:
struct ASTNode {
    type,
    value,
    next *ASTNode,  // Pointer
}

fn allocate_node(val) *ASTNode {
    let node = new ASTNode;  // Allocate
    node.value = val;
    return node;
}
```

### 3. ⏳ TERCERO: I/O Operations
```kura
// Para leer archivos fuente:
let source = read_file("programa.kr");
let output = read_file("output.kr");
write_file("compiled.exe", output);
```

## 📋 Roadmap de Bootstrapping (12 Semanas)

### Semana 1-2: Heap Memory (NEXT)
- `new` - allocate memory
- `delete` - deallocate memory
- `*T` - pointer types
- `&ref` - references
- Garbage collection or manual management

**Objetivo**: Poder declarar structs con punteros

**Test**:
```kura
struct Node { val, next *Node }
let head = new Node;
head.val = 10;
print head.val;  // 10
delete head;
```

### Semana 3-4: I/O Operations
- `read_file(path) → String`
- `write_file(path, content) → void`
- `open_file(path) → File`
- File handle operations

**Objetivo**: Poder leer código fuente

**Test**:
```kura
let src = read_file("hello.kr");
print src;  // Imprime contenido del archivo
```

### Semana 5-7: Escribir Lexer en KURA
**Ubicación**: `kura_modules/lexer.kr`

```kura
import { Array, String } from "std.kr";

export fn tokenize(source) {
    let tokens = new Array;
    // Implementación del lexer
    return tokens;
}

export struct Token {
    type,
    value,
    line,
    column,
}
```

**Objetivo**: Reemplazar `src/lexer.rs` gradualmente

### Semana 8-9: Escribir Parser en KURA
**Ubicación**: `kura_modules/parser.kr`

```kura
import { tokenize } from "lexer.kr";
import { AST } from "ast.kr";

export fn parse(tokens) {
    // Implementación del parser
    let ast = new AST;
    // Construir árbol
    return ast;
}
```

### Semana 10-12: Escribir Codegen en KURA
**Ubicación**: `kura_modules/codegen.kr`

```kura
import { parse } from "parser.kr";

export fn codegen(ast) {
    // Generar LLVM IR desde AST
    let ir = __generate_llvm_ir(ast);
    return ir;
}
```

## 🏗️ Arquitectura de Bootstrapping

### FASE 3 (Actual)
```
Rust → Lexer → Parser → Codegen → LLVM → Executable
```

### FASE 4.1 (After Heap + I/O)
```
Rust → [Call KURA Lexer]
         ↑ (implementado en KURA, hosteado en Rust)
```

### FASE 4.2 (After Parser en KURA)
```
Rust → [Call KURA Lexer] → [Call KURA Parser]
         ↑ (KURA)           ↑ (KURA)
```

### FASE 4.3 (Full Bootstrap)
```
KURA → [KURA Lexer] → [KURA Parser] → [KURA Codegen] → LLVM → KURA
↑ (Self-hosted!)
```

## 📁 Estructura de Directorios

```
kura_modules/
├── core.kr           # Tipos y funciones base
├── lexer.kr          # Tokenizador (FASE 4.1, Semana 5-7)
├── parser.kr         # Parser (FASE 4.2, Semana 8-9)
├── codegen.kr        # Generador de código (FASE 4.3, Semana 10-12)
├── ast.kr            # Definiciones de AST
├── memory.kr         # Gestión de memoria
├── io.kr             # Operaciones de archivo
└── std.kr            # Librería estándar

src/
├── lexer.rs          # Actualmente: Lexer en Rust
├── parser.rs         # Actualmente: Parser en Rust
├── codegen.rs        # Actualmente: Codegen en Rust
└── main.rs           # Punto de entrada (intérprete + compilador)
```

## 🚀 Hitos Concretos

### Hito 1: Lexer Completo en KURA
**Definición**: lexer.kr genera exactamente los mismos tokens que src/lexer.rs

```bash
# Test:
kura --lex programa.kr                           # Actual (Rust)
kura --lex programa.kr | kura_lexer --lex programa.kr  # Comparar
```

### Hito 2: Parser Completo en KURA
**Definición**: parser.kr genera exactamente el mismo AST que src/parser.rs

```bash
# Test:
kura --parse programa.kr                         # Actual (Rust)
kura --parse programa.kr | kura_parser --parse programa.kr  # Comparar
```

### Hito 3: Compilador Self-Hosting
**Definición**: kura está escrito completamente en KURA

```bash
# Bootstrap:
kura compile compiler.kr          # Compila compilador a executable
./compiler.exe compile compiler.kr    # El compilador se auto-compila
```

## 📝 Notas: Módulos vs Bootstrapping

### Por Qué Empezamos con Módulos
✅ Permite organizar código del compilador
✅ No hay breaking changes a lenguaje
✅ Flexible: agregar una característica a la vez
✅ Reduce complejidad cognitiva

### Por Qué Falta: Heap Memory
❌ Sin `new`/`delete`, no puedo allocar nodos AST dinámicos
❌ Sin punteros, lista linkedlist imposible
❌ Sin garbage collection, memory leaks en bucles

### Por Qué Falta: I/O
❌ Sin `read_file`, no puedo leer código fuente
❌ Sin `write_file`, no puedo guardar output compilado

## ✅ Checklist para Semana Siguiente

- [ ] Implementar `new` y `delete` keywords
- [ ] Soporte para punteros (`*T`)
- [ ] Soporte para referencias (`&ref`)
- [ ] Actualizar evaluador para heap management
- [ ] Escribir tests con structs complejos
- [ ] Documentar memory model

---

**Versión**: 0.1 (Bootstrap Preparation)  
**Fecha**: March 24, 2026  
**Status**: 🟡 In Progress - Module System Complete, Heap Memory Next  
**Próximo**: Gestión de Heap Memory (2 semanas)
