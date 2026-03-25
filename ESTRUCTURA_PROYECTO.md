# Estructura del Proyecto KURA - Organización

## Descripción General
El proyecto KURA ha sido reorganizado en una estructura clara y mantenible para facilitar el desarrollo, testing y compilación del lenguaje.

## Estructura de Carpetas

```
Kura/
├── language/                    # 📦 Lenguaje KURA (código Rust)
│   ├── ast.rs                   # Árbol de sintaxis abstracta
│   ├── evaluator.rs             # Motor de evaluación
│   ├── lexer.rs                 # Analizador léxico
│   ├── parser.rs                # Analizador sintáctico
│   ├── token.rs                 # Definición de tokens
│   ├── types.rs                 # Tipos de datos
│   ├── codegen.rs               # Generador de código (LLVM)
│   ├── package_manager.rs       # Gestor de paquetes
│   ├── lib.rs                   # Módulos públicos
│   ├── main.rs                  # Punto de entrada
│   ├── bin/                      # Binarios ejecutables
│   │   ├── kura.rs              # Intérprete principal
│   │   └── kup.rs               # Package manager
│   └── kura_modules/            # Módulos de utilidad
│       ├── lexer.kr
│       ├── parser.kr
│       ├── eval.kr
│       └── ...
│
├── tests/                        # 🧪 Suite de pruebas
│   ├── lexer/                    # Tests del analizador léxico
│   │   ├── test_lexer_simple.kr
│   │   ├── test_lexer_completo.kr
│   │   └── test_lexer_bootstrap.kr
│   │
│   ├── parser/                   # Tests del analizador sintáctico
│   │   └── test_parser_bootstrap.kr
│   │
│   ├── evaluator/                # Tests del evaluador
│   │   ├── test_arithmetic.kr
│   │   ├── test_bool.kr
│   │   ├── test_arr.kr
│   │   ├── test_func.kr
│   │   ├── test_for.kr
│   │   ├── test_if.kr
│   │   ├── test_io.kr
│   │   ├── test_match.kr
│   │   ├── test_str.kr
│   │   ├── test_struct.kr
│   │   └── test_heap.kr
│   │
│   └── integration/               # Tests de integración
│       ├── test_compile.kr
│       ├── test_bootstrap.kr
│       ├── test_codegen_bootstrap.kr
│       ├── test_features_completas.kr
│       ├── test_modulos.kr
│       └── ...
│
├── compiler/                     # 🔨 Compilador en KURA
│   ├── compilador_final.kr       # Compilador final
│   ├── compilador_v2.kr          # Versión 2
│   ├── compilador_v3.kr          # Versión 3
│   ├── compilador_v4.kr          # Versión 4
│   ├── kura_compilador_final.kr  # Compilador completo
│   ├── build_compiler_full.kr    # Script de construcción
│   ├── compile_simple.kr         # Compilación simple
│   ├── compile_full.kr           # Compilación completa
│   ├── compile_pares.kr
│   ├── compile_potencias.kr
│   ├── compile_suma.kr
│   ├── compile_factorial.kr
│   ├── debug_dict.kr             # Debug/Ejemplos
│   ├── debug_if.kr
│   ├── ejemplo_modulo.kr
│   ├── benchmark.kr
│   └── ...
│
├── Cargo.toml                    # Configuración de Rust
├── Cargo.lock                    # Lock de dependencias
│
├── 📄 Archivos de Raíz
│   ├── factorial.kr              # Ejemplo: factorial
│   ├── pares.kr                  # Ejemplo: números pares
│   ├── potencias.kr              # Ejemplo: potencias
│   ├── simple.kr                 # Ejemplo simple
│   ├── suma_1_a_10.kr            # Ejemplo: suma
│   └── bench_*.kr                # Benchmarks
│
└── 📚 Documentación
    ├── QUICK_REFERENCE.md
    ├── PROYECTO_KURA.md
    └── ...
```

## Organización por Sección

### 1. **language/** - Código del Lenguaje (Rust)
Contiene toda la implementación del lenguaje KURA en Rust:
- **Core**: Lexer, Parser, AST, Evaluador
- **Features**: Generador de código (LLVM), Gestor de paquetes
- **Binarios**: Intérprete (`kura`) y gestor de paquetes (`kup`)
- **Módulos**: Funciones reutilizables en KURA

### 2. **tests/** - Suite de Pruebas
Organizada por componente:
- **lexer/**: Pruebas de tokenización
- **parser/**: Pruebas de parsing
- **evaluator/**: Pruebas de ejecución (aritmética, funciones, control de flujo, etc.)
- **integration/**: Pruebas de integración (compilación, bootstrap, características completas)

### 3. **compiler/** - Compilador en KURA
Todos los archivos para el compilador bootstrapped en KURA:
- Versiones del compilador (v2, v3, v4)
- Scripts de construcción
- Ejemplos y debugging
- Benchmarks

## Cómo Usar

### Compilar el Proyecto
```bash
cargo build              # Debug
cargo build --release   # Release
```

### Ejecutar el Intérprete
```bash
cargo run -- archivo.kr
```

### Ejecutar Tests
```bash
cargo test
```

### Ejecutar un Archivo de Test Específico
```bash
cargo run -- tests/evaluator/test_arithmetic.kr
cargo run -- tests/lexer/test_lexer_simple.kr
```

## Ventajas de esta Estructura

✅ **Claridad**: Separación clara entre lenguaje, tests y compilador  
✅ **Mantenibilidad**: Fácil de localizar archivos específicos  
✅ **Escalabilidad**: Estructura preparada para crecer  
✅ **Testing Organizado**: Tests agrupados por componente  
✅ **Bootstrapping**: Compilador en KURA separado del lenguaje  

## Próximos Pasos

1. Actualizar imports en archivos binarios si es necesario
2. Añadir más tests en cada categoría
3. Mejorar documentación de cada módulo
4. Implementar sistema de CI/CD para tests

