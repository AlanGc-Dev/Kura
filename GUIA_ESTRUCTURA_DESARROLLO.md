# Guía de Desarrollo - Estructura del Proyecto KURA

## 📁 Estructura Organizada

Tu proyecto KURA ha sido reorganizado en 3 carpetas principales:

### 1. **language/** - El Lenguaje KURA (Código Rust)
**Ubicación**: `language/`  
**Contenido**: Todo el código Rust del intérprete KURA

**Archivos principales:**
- `lexer.rs` - Tokenización del código KURA
- `parser.rs` - Análisis sintáctico
- `ast.rs` - Árbol de sintaxis abstracta
- `evaluator.rs` - Ejecución del código
- `token.rs` - Definición de tokens
- `types.rs` - Sistema de tipos
- `codegen.rs` - Generador de código LLVM
- `package_manager.rs` - Gestor de paquetes
- `lib.rs` - Módulos públicos
- `main.rs` - Punto de entrada

**Subdirectorios:**
- `bin/` - Binarios ejecutables (kura, kup)
- `kura_modules/` - Módulos de utilidad en KURA

### 2. **tests/** - Suite de Pruebas
**Ubicación**: `tests/`  
**Contenido**: Archivos de test organizados por componente

**Subdirectorios:**
- `lexer/` - Tests del analizador léxico
- `parser/` - Tests del analizador sintáctico
- `evaluator/` - Tests de ejecución/evaluación
- `integration/` - Tests de integración

### 3. **compiler/** - Compilador en KURA
**Ubicación**: `compiler/`  
**Contenido**: Archivos del compilador escritos en KURA

**Archivos principales:**
- `compilador_final.kr` - Compilador completo
- `kura_compilador_final.kr` - Versión final
- Versiones anteriores: v2, v3, v4
- Scripts de construcción: `build_compiler_full.kr`, `compile_*.kr`
- Archivos de debug y ejemplos

---

## 🚀 Cómo Trabajar con cada Sección

### Trabajar con el Lenguaje (language/)

Si necesitas modificar el lenguaje KURA:

```bash
# Editar archivos en language/
# Ejemplo: Agregar una nueva característica

1. Editar language/token.rs       # Agregar nuevo token
2. Editar language/lexer.rs       # Reconocer el símbolo
3. Editar language/parser.rs      # Parsear la sintaxis
4. Editar language/ast.rs         # Definir nodo AST
5. Editar language/evaluator.rs   # Implementar lógica

# Compilar
cargo build

# Prueba rápida
cargo run -- tests/evaluator/test_arithmetic.kr
```

### Agregar un Test

Para agregar un nuevo test:

```bash
# 1. Crear archivo en la carpeta apropiada
# Ejemplo: Tests del lexer
touch tests/lexer/test_nuevo.kr

# 2. Escribir el test
# 3. Ejecutar
cargo run -- tests/lexer/test_nuevo.kr
```

### Trabajar con el Compilador (compiler/)

Si necesitas modificar el compilador en KURA:

```bash
# Editar archivos en compiler/
# Los archivos .kr son código KURA

# Prueba del compilador
cargo run -- compiler/compilador_final.kr

# Compilación bootstrap
cargo run -- compiler/kura_compilador_final.kr
```

---

## 📋 Estructura de Directorios Completa

```
Kura/
├── language/                    ← 🦀 Código Rust
│   ├── src/bin/                (anteriormente src/bin/)
│   ├── src/kura_modules/       (anteriormente src/kura_modules/)
│   ├── *.rs files              (lexer, parser, evaluator, etc.)
│   └── lib.rs
│
├── tests/                       ← 🧪 Tests
│   ├── lexer/                  (tests del lexer)
│   ├── parser/                 (tests del parser)
│   ├── evaluator/              (tests de evaluación)
│   └── integration/            (tests de integración)
│
├── compiler/                    ← 🔨 Compilador en KURA
│   ├── compilador_final.kr
│   ├── compilador_v*.kr
│   ├── compile_*.kr
│   └── *.kr files              (código KURA del compilador)
│
├── Cargo.toml                  (actualizado con nuevas rutas)
└── Cargo.lock
```

---

## ✅ Verificación de la Estructura

```bash
# Verificar compilación
cargo build

# Ejecutar un test simple
cargo run -- tests/evaluator/test_arithmetic.kr

# Ver archivos en cada carpeta
dir language/
dir tests/
dir compiler/
```

---

## 🔧 Configuración de Cargo (Cargo.toml)

El archivo `Cargo.toml` ha sido actualizado para:
- Apuntar a `language/lib.rs` como librería
- Apuntar a `language/bin/kura.rs` como binario principal
- Apuntar a `language/bin/kup.rs` como gestor de paquetes

```toml
[lib]
path = "language/lib.rs"

[[bin]]
name = "kura"
path = "language/bin/kura.rs"

[[bin]]
name = "kup"
path = "language/bin/kup.rs"
```

---

## 📚 Notas Importantes

1. **Mantener consistencia**: Los imports en los archivos Rust no necesitan cambiar porque Rust automáticamente resuelve módulos relativos.

2. **Convención de nombres**: 
   - Archivos de test: `test_*.kr`
   - Archivos de compilador: `compilador_*.kr` o `compile_*.kr`
   - Archivos de ejemplo: `*.kr` en raíz o `examples/`

3. **Testing**: Cada subcarpeta de `tests/` contiene pruebas específicas para su componente.

4. **Compilación**: Usar `cargo build` para compilar todo. Los archivos en `language/` se compilarán como librería y binarios.

---

## 🎯 Siguientes Pasos

- [ ] Revisar que todos los imports funcionen correctamente
- [ ] Agregar más tests en cada categoría
- [ ] Documentar cada módulo en `language/`
- [ ] Configurar CI/CD para ejecutar tests automáticamente
- [ ] Crear ejemplos adicionales en raíz (`examples/`)

¡Tu proyecto está ahora mejor organizado y listo para escalar! 🚀

