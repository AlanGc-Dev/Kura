#!/bin/bash

# Script de instalación de KUP para macOS/Linux
# Instala KUP (Kura Universal Package Manager) en el PATH

clear

echo "═══════════════════════════════════════════════════════"
echo "🛠️  Instalador de KUP - Kura Universal Package Manager"
echo "═══════════════════════════════════════════════════════"
echo ""

# Detectar plataforma
UNAME=$(uname)
if [[ "$UNAME" == "Darwin" ]]; then
    OS="macOS"
    INSTALL_PATH="/usr/local/bin/kup"
elif [[ "$UNAME" == "Linux" ]]; then
    OS="Linux"
    INSTALL_PATH="/usr/local/bin/kup"
else
    echo "❌ Sistema operativo no soportado: $UNAME"
    exit 1
fi

echo "📦 Sistema: $OS"
echo ""

# Rutas
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
KUP_SOURCE="$SCRIPT_DIR/target/release/kup"
KUP_DEST="$INSTALL_PATH"

# Verificar que el archivo fuente existe
if [ ! -f "$KUP_SOURCE" ]; then
    echo "❌ Error: No se encontró $KUP_SOURCE"
    echo "   Ejecuta primero: cargo build --bin kup --release"
    exit 1
fi

echo "📋 Verificando permisos..."

# Crear directorio si no existe (con permisos)
if [ ! -d "$(dirname "$INSTALL_PATH")" ]; then
    echo "📁 Creando directorio: $(dirname "$INSTALL_PATH")"
    sudo mkdir -p "$(dirname "$INSTALL_PATH")"
fi

# Copiar ejecutable con permisos
echo "📝 Copiando kup a $INSTALL_PATH..."
sudo cp "$KUP_SOURCE" "$INSTALL_PATH"
if [ $? -ne 0 ]; then
    echo "❌ Error al copiar archivo"
    exit 1
fi

# Hacer ejecutable
echo "🔐 Estableciendo permisos de ejecución..."
sudo chmod +x "$INSTALL_PATH"

# Verificar instalación
echo ""
echo "🧪 Verificando instalación..."
if command -v kup &> /dev/null; then
    KUP_VERSION=$(kup --version)
    echo "✅ ¡Instalación exitosa!"
    echo "   $KUP_VERSION"
    echo "   Ubicación: $INSTALL_PATH"
else
    echo "⚠️  KUP instalado pero requiere una nueva terminal"
    echo "   Abre una nueva ventana de terminal para usar kup"
fi

echo ""
echo "📚 Próximos pasos:"
echo "   1. Abre una nueva terminal"
echo "   2. Ejecuta: kup --help"
echo "   3. Crea un proyecto: kup init mi_app"
echo ""

echo "📝 Desinstalación:"
echo "   Para desinstalar, ejecuta:"
echo "   sudo rm $INSTALL_PATH"
echo ""

echo "✨ ¡Listo! Disfruta de KUP"
echo ""

