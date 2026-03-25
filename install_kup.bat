@echo off
REM Script de instalación de KUP para Windows
REM Instala KUP (Kura Universal Package Manager) en el PATH

setlocal enabledelayedexpansion

echo.
echo ═══════════════════════════════════════════════════════
echo 🛠️  Instalador de KUP - Kura Universal Package Manager
echo ═══════════════════════════════════════════════════════
echo.

REM Detectar si se ejecuta como administrador
net session >nul 2>&1
if %errorLevel% neq 0 (
    echo ❌ Error: Este script requiere permisos de administrador.
    echo    Ejecuta el script con clic derecho → "Ejecutar como administrador"
    pause
    exit /b 1
)

REM Rutas
set "KURA_SOURCE=%~dp0target\release\kup.exe"
set "KURA_DIR=C:\Program Files\Kura"
set "KURA_DEST=%KURA_DIR%\kup.exe"

REM Verificar que el archivo fuente existe
if not exist "%KURA_SOURCE%" (
    echo ❌ Error: No se encontró %KURA_SOURCE%
    echo    Ejecuta primero: cargo build --bin kup --release
    pause
    exit /b 1
)

echo 📦 Verificando archivos...
echo    Origen: %KURA_SOURCE%
echo    Destino: %KURA_DEST%
echo.

REM Crear directorio si no existe
if not exist "%KURA_DIR%" (
    echo 📁 Creando directorio: %KURA_DIR%
    mkdir "%KURA_DIR%"
    if %errorLevel% neq 0 (
        echo ❌ Error al crear directorio
        pause
        exit /b 1
    )
)

REM Copiar ejecutable
echo 📋 Copiando kup.exe...
copy /Y "%KURA_SOURCE%" "%KURA_DEST%"
if %errorLevel% neq 0 (
    echo ❌ Error al copiar archivo
    pause
    exit /b 1
)

REM Agregar al PATH si no está
echo 🔗 Verificando PATH...
reg query "HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Control\Session Manager\Environment" /v PATH | findstr /i "%KURA_DIR%"
if %errorLevel% neq 0 (
    echo 📝 Agregando %KURA_DIR% al PATH...
    for /f "tokens=2*" %%a in ('reg query "HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Control\Session Manager\Environment" /v PATH ^| findstr /i path') do (
        set "OLD_PATH=%%b"
    )
    set "NEW_PATH=!OLD_PATH!;%KURA_DIR%"
    reg add "HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Control\Session Manager\Environment" /v PATH /t REG_EXPAND_SZ /d "!NEW_PATH!" /f
) else (
    echo ✓ Ya está en PATH
)

REM Verificar instalación
echo.
echo 🧪 Verificando instalación...
where kup >nul 2>&1
if %errorLevel% eq 0 (
    echo ✅ ¡Instalación exitosa!
    echo    Puedes usar: kup --help
    echo.
    echo 📚 Próximos pasos:
    echo    1. Abre una nueva terminal (para refrescar PATH)
    echo    2. Ejecuta: kup --help
    echo    3. Crea un proyecto: kup init mi_app
) else (
    echo ⚠️  KUP instalado pero requiere reiniciar terminal/ordenador
    echo    Abre una nueva terminal (cmd, PowerShell) para usar kup
    echo.
    echo 📚 Próximos pasos:
    echo    1. Abre una nueva terminal
    echo    2. Ejecuta: kup --help
)

echo.
echo 📝 Nota: Para desinstalar, elimina C:\Program Files\Kura\kup.exe
echo        y remueve %KURA_DIR% del PATH (Variables de Entorno)
echo.

pause

