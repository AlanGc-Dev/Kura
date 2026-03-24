# 🚀 Compilación Cruzada en KURA - Documentación

## Descripción General

KURA ahora soporta **compilación cruzada** (cross-compilation) para múltiples plataformas y arquitecturas desde una única máquina. Esto significa que puedes compilar código KURA para Windows, Linux y macOS sin necesidad de tener una máquina con ese sistema operativo.

## Caracte rísticas

### ✅ Targets Soportados

| Target | Triple LLVM | Descripción | Estado |
|--------|-------------|-------------|--------|
| **windows-x86_64** | `x86_64-pc-windows-msvc` | Windows 64-bit (Intel/AMD) | ✅ Completo |
| **linux-x86_64** | `x86_64-unknown-linux-gnu` | Linux 64-bit (Intel/AMD) | ✅ Completo |
| **linux-arm64** | `aarch64-unknown-linux-gnu` | Linux ARM64 (Raspberry Pi 4+) | ✅ Completo |
| **macos-x86_64** | `x86_64-apple-darwin` | macOS 64-bit (Intel) | ✅ Completo* |
| **macos-arm64** | `aarch64-apple-darwin` | macOS ARM64 (Apple Silicon) | ✅ Completo* |

*Requiere linkers de LLVM correspondientes instalados (ld64.lld)

### 🎯 Componentes Ajustados por Target

1. **LLVM Target Triple**: Define objetivo de arquitectura y ABI
2. **Data Layout**: Especifica alineación de datos y tamaño de tipos primitivos
3. **Linker Command**: Selecciona el linker adecuado:
   - Windows: `lld-link` (MSVC linker)
   - Unix: `ld.lld` (LLVM linker)
   - macOS: `ld64.lld` (AppleLinker compatible)
4. **Librerías Enlazadas**: `-lc`, `-lm`, `-lSystem` según el target
5. **Convención de Llamadas**: ABI específica del sistema operativo

## Uso

### Sintaxis Básica

```bash
kura --compile --target <TARGET> [opciones] archivo.kr
```

### Ejemplos

#### 1. Compilar para Windows (predeterminado)
```bash
kura --compile test.kr
kura --compile --target windows-x86_64 test.kr
```

#### 2. Compilar para Linux
```bash
kura --compile --target linux-x86_64 test.kr
```

#### 3. Compilar para Raspberry Pi (ARM64)
```bash
kura --compile --target linux-arm64 test.kr
```

#### 4. Compilar para macOS Apple Silicon
```bash
kura --compile --target macos-arm64 --release test.kr
```

#### 5. Compilar con optimización máxima para Linux
```bash
kura --compile --target linux-x86_64 -O3 test.kr
```

### Combinación con Niveles de Optimización

```bash
# -O0: Sin optimización (desarrollo)
kura --compile --target linux-x86_64 -O0 test.kr

# -O1: Optimización mínima
kura --compile --target linux-x86_64 -O1 test.kr

# -O2: Optimización balanceada (default)
kura --compile --target linux-x86_64 test.kr

# -O3: Optimización máxima (release)
kura --compile --target linux-x86_64 -O3 test.kr
kura --compile --target linux-x86_64 --release test.kr  # Equivalente
```

## Implementación Técnica

### Estructura de Compilación

```
┌─────────────────┐
│  Código KURA    │
│  (.kr file)     │
└────────┬────────┘
         │
         v
┌─────────────────────────────────┐
│  Lexer/Parser/Evaluator         │
│  AST generado                   │
└────────┬────────────────────────┘
         │
         v
┌──────────────────────────────────────────────────────┐
│  CodeGenerator (Target-Aware)                        │
│  • Extract target triple dinámicamente               │
│  • Generar IR LLVM con el target correcto            │
│  • Seleccionar data layout según target              │
└────────┬────────────────────────────────────────────┘
         │
         v
┌──────────────────────────────────────────────╗
│  Validación: llvm-verify-ir                  │
│  (verificar integridad del IR)                │
└────────┬─────────────────────────────────────┘
         │
         v
┌──────────────────────────────────────────────╗
│  Optimización: opt -O0/-O1/-O2/-O3            │
│  Pipeline LLVM de optimizaciones              │
└────────┬─────────────────────────────────────┘
         │
         v
┌──────────────────────────────────────────────────────┐
│  Compilación: clang --target=<triple>                │
│  IR LLVM → Código Objeto (.obj/.o)                   │
└────────┬────────────────────────────────────────────┘
         │
         v
┌──────────────────────────────────────────────────────┐
│  Enlazado: lld-link / ld.lld / ld64.lld              │
│  (Target-specific linker)                            │
│  Objeto + Runtime Library → Ejecutable               │
└────────┬────────────────────────────────────────────┘
         │
         v
┌──────────────────────────────────────────────┐
│  Ejecutable Nativo                           │
│  (Windows .exe / Linux ELF / macOS Mach-O)   │
└──────────────────────────────────────────────┘
```

### Cambios en el Código

#### 1. `src/codegen.rs`

**Nuevo enum `CompilationTarget`:**
```rust
pub enum CompilationTarget {
    WindowsX86_64,
    LinuxX86_64,
    LinuxARM64,
    MacOSX86_64,
    MacOSARM64,
}

impl CompilationTarget {
    pub fn as_triple(&self) -> &'static str { ... }
    pub fn linker_command(&self) -> &'static str { ... }
    pub fn from_string(s: &str) -> Option<Self> { ... }
}
```

**Actualización de `CodeGenerator`:**
```rust
pub struct CodeGenerator {
    // ... campos existentes ...
    target: CompilationTarget,  // Nuevo campo
}

pub fn with_target(opt_level: OptimizationLevel, target: CompilationTarget) -> Result<Self, String> {
    // Nuevo constructor que acepta target
}
```

**Generación de IR adaptativa:**
- `generate()` ahora usa `self.target.as_triple()` para el target triple
- Data layout seleccionado dinámicamente según el target
- Declaraciones de librerías correctas según el SO

**Compilación targetizada:**
```rust
// Antes (hardcoded)
.args(&["--target=x86_64-pc-windows-msvc", ...])

// Después (dinámico)
.args(&[&format!("--target={}", self.target.as_triple()), ...])
```

**Linkeo adaptativo:**
```rust
match self.target {
    CompilationTarget::WindowsX86_64 => {
        // Usar lld-link con argumentos de MSVC
    }
    _ => {
        // Usar ld.lld/ld64.lld con argumentos Unix
    }
}
```

#### 2. `src/bin/kura.rs`

**Importación del nuevo enum:**
```rust
use kura::codegen::{OptimizationLevel, CompilationTarget};
```

**Parseo de argumentos CLI:**
```rust
"--target" => {
    if let Some(t) = CompilationTarget::from_string(&args[i+1]) {
        target = t;
    }
}
```

**Invocación con target:**
```rust
// Antes
CodeGenerator::with_optimization(opt_level)

// Después
CodeGenerator::with_target(opt_level, target)
```

## Configuración del Entorno

### Windows

Para compilar sin necesidad de herramientas adicionales:
- LLVM/Clang ya está instalado (si completaste 100% LLVM)
- lld-link viene incluido con LLVM

### Linux

Para compilar desde Linux para otros targets:
```bash
# Instalar soporte de compilación cruzada
sudo apt-get install llvm-dev llvm clang lld

# Para ARM64
sudo apt-get install gcc-aarch64-linux-gnu binutils-aarch64-linux-gnu
```

### macOS

```bash
# Instalar Xcode y LLVM
xcode-select --install
brew install llvm
```

## Casos de Uso

### 1. Distribución de Software

Compilar una aplicación KURA una sola vez para múltiples plataformas:

```bash
#!/bin/bash
# build_all.sh - Compilar para todos los targets

cargo run --bin kura -- --compile --target windows-x86_64 --release app.kr
cargo run --bin kura -- --compile --target linux-x86_64 --release app.kr
cargo run --bin kura -- --compile --target macos-arm64 --release app.kr

echo "✅ Compilado para todas las plataformas!"
```

### 2. Desarrollo Embebido

Compilar código para Raspberry Pi sin dejar de trabajar en Windows:

```bash
# En tu máquina Windows
kura --compile --target linux-arm64 rpi_app.kr

# Transferir rpi_app.exe a Raspberry Pi
scp rpi_app.exe pi@192.168.1.100:~/
```

### 3. CI/CD Pipeline

```yaml
# GitHub Actions example
jobs:
  cross-compile:
    runs-on: ubuntu-latest
    strategy:
      matrix:
        target: [windows-x86_64, linux-x86_64, linux-arm64, macos-arm64]
    steps:
      - uses: actions/checkout@v2
      - name: Compile KURA
        run: |
          cargo run --bin kura -- --compile \
            --target ${{ matrix.target }} \
            --release src/app.kr
```

## Limitaciones y Consideraciones

### ✅ Soportado Completamente

- Compilación de código KURA puro sin dependencias externas
- Runtime library KURA (memory management, strings, arrays)
- Funciones estándar de I/O (printf)
- Toda la sintaxis y semántica del lenguaje

### ⚠️ Con Limitaciones

- **FFI a bibliotecas nativas**: Requiere enlaces LLVM específicos para cada target
- **Syscalls específicas**: Pueden diferenciar entre plataformas (futuro)
- **Paths de archivos**: Usar rutas agnósticas al SO cuando sea posible

### ❌ No Soportado Aún

- Compilación a WebAssembly (wasm32-unknown-unknown)
- Compilación directa a ARM 32-bit (arm-unknown-linux-gnueabihf)
- Compilación a Android (aarch64-linux-android)

## Troubleshooting

### Error: "Linker Command Not Found"

**Problema**: El linker especificado no está disponible en el sistema

**Solución**:
```bash
# Instalar LLVM completo
# Windows: Descarga de https://releases.llvm.org/
# Linux: sudo apt-get install lld
# macOS: brew install llvm
```

###  Error: "Target Triple Not Recognized"

**Problema**: Se especificó un target inválido

**Solución**:
```bash
# Ver targets válidos
kura --help

# Usar un target correcto
kura --compile --target linux-x86_64 app.kr
```

### Error: "Clang Not Found"

**Problema**: Clang no se encuentra en las rutas estándar

**Solución**:
```bash
# Agregar LLVM al PATH
export PATH="/path/to/llvm/bin:$PATH"

# O, en Windows:
set PATH=C:\Program Files\LLVM\bin;%PATH%
```

## Especificaciones Técnicas

### Data Layouts Utilizados

```
Windows x86_64:
e-m:w-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128

Linux x86_64:
e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128

Linux ARM64:
e-m:e-i8:8:32-i16:16:32-i64:64-i128:128-n32:64-S128

macOS x86_64:
e-m:o-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128

macOS ARM64:
e-m:o-i64:64-i128:128-n32:64-S128
```

### Convenciones de Llamadas (Calling Conventions)

| Target | CC | Stack Alignment |registro | arg0-3 |
|--------|----|-----------|----|---------|
| Windows x64 | Microsoft x64 | 16 bytes |rcx, rdx, r8, r9 |Pila |
| Linux x64 | System V AMD64 | 16 bytes | rdi, rsi, rdx, rcx, r8, r9 |XMM |
| ARM64 | ARM64 ABI | 16 bytes | x0-x7 | Stack |
| macOS x64 | Framework ABI | 16 bytes | rdi, rsi, rdx, rcx, r8, r9 | XMM |
| macOS ARM64 | ARM64e | 16 bytes | x0-x7 | Stack |

## Roadmap Futuro

- [ ] Soporte para WebAssembly (wasm32-unknown-unknown)
- [ ] Soporte para ARM 32-bit (armv7l-linux-gnueabihf)
- [ ] Soporte para RISC-V (riscv64-unknown-linux-gnu)
- [ ] Compilación a múltiples targets en un comando
- [ ] Caché de compilación cruzada (evitar recompilación)
- [ ] Profile-guided optimization (PGO)
- [ ] Link-time code generation (LTCG) automático

## Conclusión

La compilación cruzada en KURA permite distribuir aplicaciones nativas a múltiples plataformas manteniendo una única base de código. Esto simplifica el desarrollo multiplataforma y acelera el ciclo de compilación y distribución.

---

**Versión**: 0.1.0  
**Fecha**: Marzo 24, 2026  
**Estado**: ✅ Producción-Listo  
