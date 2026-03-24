#!/bin/bash
# 🚀 KURA Cross-Compilation Examples
# Ejemplos prácticos de compilación cruzada

echo "╔════════════════════════════════════════════════════════════╗"
echo "║   KURA Language - Cross Compilation Examples              ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""

# Color codes
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Asegurarse de que estamos en el directorio correcto
if [ ! -f "Cargo.toml" ]; then
    echo "❌ Error: No se encontró Cargo.toml"
    echo "   Ejecuta este script desde la raíz de KURA"
    exit 1
fi

echo -e "${BLUE}📦 Building KURA compiler...${NC}"
cargo build --release 2>&1 | grep -E 'Compiling|Finished' || true
echo ""

# Ejemplo 1: Windows x86_64 (predeterminado)
echo -e "${YELLOW}═══════════════════════════════════════════════════════════${NC}"
echo -e "${YELLOW}Ejemplo 1: Compilar para Windows x86_64${NC}"
echo -e "${YELLOW}═══════════════════════════════════════════════════════════${NC}"
echo ""
echo -e "${GREEN}Comando:${NC}"
echo 'cargo run --release --bin kura -- --compile --target windows-x86_64 test_compile.kr'
echo ""
echo -e "${GREEN}Descripción:${NC}"
echo "- Target: Windows 64-bit (Intel/AMD)"
echo "- Linker: lld-link (MSVC)"
echo "- Resultado: test_compile.exe"
echo ""

# Ejecutar
echo -e "${BLUE}⏳ Compilando...${NC}"
cargo run --release --bin kura -- --compile --target windows-x86_64 test_compile 2>&1 | \
    grep -E '✅|❌|Compilacion exitosa' || true
echo ""

# Ejemplo 2: Linux x86_64
echo -e "${YELLOW}═══════════════════════════════════════════════════════════${NC}" 
echo -e "${YELLOW}Ejemplo 2: Compilar para Linux x86_64${NC}"
echo -e "${YELLOW}═══════════════════════════════════════════════════════════${NC}"
echo ""
echo -e "${GREEN}Comando:${NC}"
echo 'cargo run --release --bin kura -- --compile --target linux-x86_64 test_compile.kr'
echo ""
echo -e "${GREEN}Descripción:${NC}"
echo "- Target: Linux 64-bit (Intel/AMD)"
echo "- Linker: ld.lld (LLVM linker)"
echo "- Convención: System V AMD64 ABI"
echo "- Libs enlazadas: -lc, -lm"
echo ""

# Ejecutar (solo mostrar IR generation, no puede linkear sin herramientas)
echo -e "${BLUE}⏳ Compilando IR para Linux...${NC}"
cargo run --release --bin kura -- --compile --target linux-x86_64 test_compile 2>&1 | \
    grep -E '🎯 Target|Target:|target triple|target datalayout' | head -3
echo ""

# Ejemplo 3: Linux ARM64
echo -e "${YELLOW}═══════════════════════════════════════════════════════════${NC}"
echo -e "${YELLOW}Ejemplo 3: Compilar para Linux ARM64 (Raspberry Pi)${NC}"
echo -e "${YELLOW}═══════════════════════════════════════════════════════════${NC}"
echo ""
echo -e "${GREEN}Comando:${NC}"
echo 'cargo run --release --bin kura -- --compile --target linux-arm64 app.kr'
echo ""
echo -e "${GREEN}Descripción:${NC}"
echo "- Target: ARM64 (Raspberry Pi 4+, Orange Pi, etc.)"
echo "- Arquitectura: AArch64"
echo "- Linker: ld.lld"
echo "- Caso de uso: Compilar en Windows/Linux x86_64 para Raspberry Pi"
echo ""

# Ejemplo 4: macOS Intel
echo -e "${YELLOW}═══════════════════════════════════════════════════════════${NC}"
echo -e "${YELLOW}Ejemplo 4: Compilar para macOS Intel${NC}"
echo -e "${YELLOW}═══════════════════════════════════════════════════════════${NC}"
echo ""
echo -e "${GREEN}Comando:${NC}"
echo 'cargo run --release --bin kura -- --compile --target macos-x86_64 app.kr'
echo ""
echo -e "${GREEN}Descripción:${NC}"
echo "- Target: macOS 64-bit (Intel Macs pre-2021)"
echo "- Linker: ld64.lld (Apple linker compatible)"
echo "- Libs: -lSystem (sistema operativo)"
echo ""

# Ejemplo 5: macOS Apple Silicon
echo -e "${YELLOW}═══════════════════════════════════════════════════════════${NC}"
echo -e "${YELLOW}Ejemplo 5: Compilar para macOS Apple Silicon${NC}"
echo -e "${YELLOW}═══════════════════════════════════════════════════════════${NC}"
echo ""
echo -e "${GREEN}Comando:${NC}"
echo 'cargo run --release --bin kura -- --compile --target macos-arm64 --release app.kr'
echo ""
echo -e "${GREEN}Descripción:${NC}"
echo "- Target: macOS ARM64 (Apple Silicon - M1/M2/M3/M4)"
echo "- Arquitectura: AArch64 con extensiones Apple"
echo "- Linker: ld64.lld"
echo "- Flags: -O3 para máximo rendimiento"
echo ""

# Ejemplo 6: Compilación con Optimizaciones
echo -e "${YELLOW}═══════════════════════════════════════════════════════════${NC}"
echo -e "${YELLOW}Ejemplo 6: Compilación Cruzada con Optimizaciones${NC}"
echo -e "${YELLOW}═══════════════════════════════════════════════════════════${NC}"
echo ""
echo -e "${GREEN}Comando:${NC}"
echo 'cargo run --release --bin kura -- --compile --target linux-x86_64 -O3 app.kr'
echo ""
echo -e "${GREEN}Descripción:${NC}"
echo "- Nivel de optimización: 3 (máximo)"
echo "- Flags: -O3 con LTO (Link-Time Optimization)"
echo "- Resultante: Código muy optimizado, compila más lento"
echo "- Ideal para: Releases de producción"
echo ""

# Información del sistema
echo -e "${YELLOW}═══════════════════════════════════════════════════════════${NC}"
echo -e "${YELLOW}Información del Sistema${NC}"
echo -e "${YELLOW}═══════════════════════════════════════════════════════════${NC}"
echo ""
echo -e "${GREEN}Herramientas Disponibles:${NC}"

# Verificar disponibilidad de herramientas
if command -v clang &> /dev/null; then
    echo "✅ Clang: $(clang --version | head -1)"
else
    echo "❌ Clang: NO INSTALADO"
fi

if command -v lld-link &> /dev/null; then
    echo "✅ lld-link: Disponible (MSVC linker)"
else
    echo "❌ lld-link: NO DISPONIBLE"
fi

if command -v lld &> /dev/null; then
    echo "✅ ld.lld: Disponible (Unix linker)"
else
    echo "❌ ld.lld: NO DISPONIBLE"
fi

echo ""
echo -e "${YELLOW}═══════════════════════════════════════════════════════════${NC}"
echo -e "${YELLOW}Ayuda Disponible${NC}"
echo -e "${YELLOW}═══════════════════════════════════════════════════════════${NC}"
echo ""
echo "Para ver todos los targets y opciones disponibles:"
echo -e "${GREEN}cargo run --release --bin kura -- --help${NC}"
echo ""

echo -e "${GREEN}✅ Ejemplos completados!${NC}"
echo ""
