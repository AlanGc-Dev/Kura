# KUP - Ejecutable Standalone

**Versión:** 0.4.0  
**Tamaño:** 3.3 MB (release)  
**Ubicación:** `target/release/kup.exe` (Windows) o `target/release/kup` (macOS/Linux)

---

## 📥 Instalación

### Opción 1: Usar el ejecutable del repositorio (RECOMENDADO)

```bash
cd P:\KuraLenguaje\Kura
.\target\release\kup.exe init mi_proyecto
```

### Opción 2: Agregar a PATH global (Windows)

```powershell
# Copiar kup.exe a carpeta del sistema
Copy-Item ".\target\release\kup.exe" "C:\Program Files\Kura\kup.exe"

# Agregar a PATH (requiere permisos de admin)
# Luego reiniciar terminal

# Usar globalmente:
kup init mi_proyecto
```

### Opción 3: Crear alias (PowerShell)

```powershell
# Editar perfil de PowerShell
notepad $PROFILE

# Agregar alias:
Set-Alias kup "P:\KuraLenguaje\Kura\target\release\kup.exe"

# Recargar perfil
. $PROFILE
```

---

## 🚀 Uso Rápido

### Crear proyecto
```bash
kup init mi_app
cd mi_app
```

### Ver ayuda
```bash
kup --help
kup -h
kup help
```

### Ver versión
```bash
kup --version
kup -v
```

### Compilar a ejecutable nativo
```bash
# Debug (rápido)
kup build

# Release (optimizado, ~3-5x más rápido)
kup build --release
kup compile  # Alias
```

### Ejecutar scripts
```bash
kup run start    # Script 'start' desde kura.toml
kup run dev      # Script 'dev'
kup run build    # Script 'build'
```

---

## 📋 Lista Completa de Comandos

### Proyectos
```bash
kup init [nombre]           # Crear nuevo proyecto
kup install                 # Instalar dependencias
kup update                  # Actualizar dependencias
kup list                    # Listar dependencias
kup build                   # Compilar (debug)
kup build --release         # Compilar (release/optimizado)
kup compile                 # Alias de build --release
kup clean                   # Limpiar caché
```

### Paquetes
```bash
kup add <paquete> [versión] # Agregar paquete
kup add usuario/repo        # Agregar desde GitHub
kup remove <paquete>        # Remover paquete
kup search <término>        # Buscar paquetes
kup info <paquete>          # Info del paquete
```

### Scripts
```bash
kup run <script>            # Ejecutar script
```

---

## 📁 Archivos Generados

Al ejecutar `kup init mi_app`:

```
mi_app/
├── kura.toml               # Configuración del proyecto
├── kura.lock               # Versiones bloqueadas
├── src/main.kr             # Archivo principal
└── kura_modules/           # Carpeta de dependencias
```

### kura.toml (ejemplo)
```toml
nombre = "mi_app"
version = "0.1.0"
descripcion = "Mi primer proyecto Kura"
autor = "Tu Nombre"
licencia = "MIT"
entrada = "main.kr"

[dependencias]
# math = "1.0"
# http = "0.5"

[scripts]
start = "kura run main"
dev = "kura src/main.kr"
build = "echo Compilando..."
```

---

## 🔄 Flujo Típico de Trabajo

```bash
# 1. Crear proyecto
kup init mi_calculadora
cd mi_calculadora

# 2. Editar src/main.kr con tu código

# 3. Desarrollo rápido
kup run dev

# 4. Agregar dependencias (opcional)
kup add math 1.0
kup install

# 5. Compilar a nativo
kup build --release

# 6. Ejecutar binario
.\target\release\mi_calculadora.exe

# 7. Limpiar (opcional)
kup clean
```

---

## 🎯 Ejemplo Completo

```bash
# Paso 1: Crear proyecto
kup init calculadora
cd calculadora

# Paso 2: Crear script en kura.toml
# (Editar [scripts] section)

# Paso 3: Escribir código en src/main.kr
# (Editar el archivo manualmente)

# Paso 4: Probar interactivamente
kup run dev

# Paso 5: Compilar optimizado
kup build --release

# Paso 6: Ejecutar nativo
.\target\release\calculadora.exe
```

---

## 💻 Requisitos del Sistema

- **Windows:** Windows 7+, .NET Runtime (incluido en binario)
- **macOS:** macOS 10.12+
- **Linux:** Ubuntu 16.04+, Debian 8+, Fedora 21+

### Opcional para compilación a nativo
- LLVM 10+ (para `kup build`)
- Clang o GCC (para linking)

---

## 🔧 Troubleshooting

### Error: "kura.toml no encontrado"
```bash
# Solución: Ejecutar en directorio con kura.toml
# O crear nuevo proyecto:
kup init
```

### Error: "LLVM no encontrado" (al compilar)
```bash
# Windows: Instalar LLVM
https://releases.llvm.org/download.html

# macOS
brew install llvm

# Linux (Debian)
sudo apt-get install llvm clang
```

### Comando no reconocido
```bash
# Windows: Agregar a PATH o usar ruta completa
.\target\release\kup.exe --help

# MacOS/Linux: Hacer ejecutable
chmod +x target/release/kup
./target/release/kup --help
```

---

## 📊 Comparación: Desarrollo vs Producción

| Aspecto | Desarrollo | Producción |
|---------|-----------|-----------|
| Comando | `kup build` | `kup build --release` |
| Ubicación | `target/debug/` | `target/release/` |
| Tamaño | Más grande | ~50% más pequeño |
| Velocidad | Normal | 3-5x más rápido |
| Debug info | Sí | No |
| Optimizaciones | Mínimas | Máximas (-O3) |

---

## 🚀 Scripts Personalizados

Ejemplo de `kura.toml` con scripts útiles:

```toml
[scripts]
dev = "kura src/main.kr"
build = "kup build --release"
test = "kura tests/main.kr"
benchmark = "kura bench/main.kr"
lint = "echo Analizando..."
format = "echo Formateando..."
clean = "kup clean"
all = "kup run build; kup run test; kup run benchmark"
```

Uso:
```bash
kup run dev         # Desarrollo
kup run build       # Compilar
kup run test        # Tests
kup run all         # Todo
```

---

## 📦 Gestión de Dependencias

### Agregar paquete
```bash
kup add math 1.0
```

### Ver paquetes instalados
```bash
kup list
```

### Remover paquete
```bash
kup remove math
```

### Instalar todas las dependencias
```bash
kup install
```

### Buscar paquetes
```bash
kup search json
```

### Ver información de paquete
```bash
kup info math
```

---

## 🔗 URLs Soportadas para Agregar Paquetes

```bash
# Versión específica del registro
kup add math 1.0

# Desde GitHub (usuario/repo)
kup add usuario/mi_libreria

# URL completa
kup add https://raw.githubusercontent.com/user/repo/main/lib.kr

# Rama específica
kup add usuario/libreria main
```

---

## 🎓 Documentación Completa

- **Guía Completa:** `KUP_GUIA_COMPLETA.md`
- **Ejemplo Práctico:** `EJEMPLO_KUP_PRACTICO.md`
- **Referencia Rápida:** `QUICK_REFERENCE.md`
- **Estructura del Proyecto:** `GUIA_ESTRUCTURA_DESARROLLO.md`

---

## 📞 Soporte

- **Reportar bugs:** GitHub Issues
- **Sugerencias:** GitHub Discussions
- **Documentación:** `/docs` en el repositorio

---

## 📄 Licencia

KUP es parte del proyecto KURA bajo licencia MIT.

---

**Versión:** 0.4.0 | **Fecha:** Marzo 2026


