# 🎯 KUP - Resumen Ejecutivo

**Fecha:** Marzo 2026  
**Versión:** 0.4.0  
**Estado:** ✅ Completo y Funcional  
**Ejecutable:** 3.3 MB (Release, Standalone)

---

## 📋 Resumen

KUP es el **Kura Universal Package Manager** - un gestor de paquetes moderno e intuitivo para el lenguaje de programación KURA. Inspirado en herramientas populares como npm (Node.js), cargo (Rust) y pip (Python), proporciona todas las herramientas necesarias para:

- 📦 **Gestión de Dependencias**: Agregar, actualizar y remover paquetes
- 🔄 **Scripts del Proyecto**: Ejecutar comandos personalizados como en npm
- 🔨 **Compilación Nativa**: Compilar a ejecutables con LLVM
- 📄 **Configuración TOML**: Archivos de configuración simples y claros
- 🌐 **Integración GitHub**: Usar librerías directamente desde repositorios
- ⚡ **Rendimiento**: Caché de paquetes, compilación optimizada

---

## 🚀 Características Principales

### 1. Gestión de Proyectos
```bash
kup init [nombre]          # Crear nuevo proyecto
kup install                # Instalar dependencias
kup list                   # Listar paquetes
kup update                 # Actualizar paquetes
```

### 2. Gestión de Paquetes
```bash
kup add math 1.0           # Agregar paquete versión específica
kup add usuario/repo       # Agregar desde GitHub
kup remove math            # Remover paquete
kup search json            # Buscar paquetes
kup info math              # Info del paquete
```

### 3. Scripts (Node.js-like)
```bash
kup run dev                # Ejecutar script "dev"
kup run build              # Ejecutar script "build"
kup run test               # Ejecutar script "test"
```

### 4. Compilación Nativa
```bash
kup build                  # Compilar debug
kup build --release        # Compilar optimizado (-O3)
kup compile                # Alias de --release
```

### 5. Mantenimiento
```bash
kup clean                  # Limpiar caché y temporales
kup --help                 # Ver ayuda
kup --version              # Ver versión
```

---

## 📁 Estructura Generada

```
mi_proyecto/
├── kura.toml              # Manifiesto (nombre, versión, scripts)
├── kura.lock              # Lock file (versiones bloqueadas)
├── src/
│   └── main.kr            # Código principal
├── kura_modules/          # Paquetes instalados
├── target/
│   ├── debug/             # Binarios debug
│   └── release/           # Binarios optimizados
└── .kura_cache/           # Caché local
```

---

## 📝 Archivo de Configuración (kura.toml)

```toml
[package]
nombre = "mi_app"
version = "0.1.0"
descripcion = "Mi primer proyecto Kura"
autor = "Tu Nombre"
licencia = "MIT"
entrada = "main.kr"

[dependencias]
math = "1.0"
strings = "1.2"

[scripts]
start = "kura src/main.kr"
dev = "kura src/main.kr"
build = "kup build --release"
test = "kura tests/main.kr"
```

---

## 🎯 Casos de Uso

### 1. Desarrollo Rápido
```bash
kup init app
cd app
kup run dev              # Ejecutar interactivamente
```

### 2. Compilación Optimizada
```bash
kup build --release      # Compilar con -O3
./target/release/app     # Ejecutar nativo (~3-5x más rápido)
```

### 3. Gestión de Dependencias
```bash
kup add usuario/libreria # Agregar desde GitHub
kup install              # Instalar todo
kup list                 # Ver paquetes
```

### 4. Distribución
```bash
kup build --release      # Compilar
# Compartir ./target/release/app.exe (ejecutable nativo)
```

---

## 💻 Requisitos

### Mínimos
- Windows 7+, macOS 10.12+, Linux (Ubuntu 16.04+, Debian 8+)
- 10 MB de espacio en disco

### Para compilación nativa (opcional)
- LLVM 10+ 
- Clang o GCC
- 50 MB de espacio

---

## 🔧 Instalación

### Windows (Recomendado)
```bash
# 1. Clonar repositorio
git clone https://github.com/KuraLenguaje/Kura.git
cd Kura

# 2. Compilar
cargo build --bin kup --release

# 3. Instalar (requiere admin)
.\install_kup.bat

# 4. Usar
kup --help
```

### macOS/Linux
```bash
# 1. Clonar repositorio
git clone https://github.com/KuraLenguaje/Kura.git
cd Kura

# 2. Compilar
cargo build --bin kup --release

# 3. Instalar
chmod +x install_kup.sh
./install_kup.sh

# 4. Usar
kup --help
```

### Uso Directo (sin instalar)
```bash
# Windows
.\target\release\kup.exe --help

# macOS/Linux
./target/release/kup --help
```

---

## 📊 Comparación con Herramientas Similares

| Característica | KUP | npm | cargo | pip |
|---|---|---|---|---|
| Lenguaje Destino | KURA | JS | Rust | Python |
| Configuración | TOML | JSON | TOML | TXT |
| Scripts Personalizados | ✅ | ✅ | ✅ | ❌ |
| Compilación Nativa | ✅ | ❌ | ✅ | ❌ |
| GitHub Integration | ✅ | ✅ | ✅ | ✅ |
| Caché Local | ✅ | ✅ | ✅ | ✅ |
| Tamaño Ejecutable | 3.3 MB | 50 MB+ | 140 MB+ | 10 MB |

---

## 🎓 Documentación

| Documento | Descripción |
|-----------|-------------|
| `KUP_GUIA_COMPLETA.md` | Guía detallada de todos los comandos |
| `KUP_EJECUTABLE_README.md` | Instrucciones de instalación y uso |
| `EJEMPLO_KUP_PRACTICO.md` | Ejemplo paso-a-paso de un proyecto |
| `QUICK_REFERENCE.md` | Referencia rápida de comandos |

---

## 📈 Roadmap Futuro

### Fase 2 (v0.5.0)
- [ ] Registros públicos de paquetes
- [ ] Sistema de versiones semánticas
- [ ] Publicación automática de paquetes
- [ ] Configuración de autenticación

### Fase 3 (v0.6.0)
- [ ] Workspaces (múltiples proyectos)
- [ ] Hooks pre/post scripts
- [ ] GUI para KUP
- [ ] Integración con CI/CD

### Fase 4 (v1.0.0)
- [ ] Repositorio central pkg.kura.io
- [ ] Certificación de paquetes
- [ ] Plugin system
- [ ] Transpilación

---

## 🎯 Objetivos Alcanzados

✅ **Gestor de Paquetes Funcional**
- Crear, instalar, actualizar y remover paquetes
- Sistema de caché eficiente
- Soporte para GitHub

✅ **Sistema de Scripts**
- Ejecución de comandos personalizados
- Definición flexible en TOML
- Compatible con npm-style workflows

✅ **Compilación Nativa**
- LLVM como backend
- Compilación debug y release
- Generación de IR (Intermediate Representation)

✅ **Configuración TOML**
- Manifiesto simple y legible
- Versionado con lock file
- Soporte para dev dependencies

✅ **Ejecutable Standalone**
- 3.3 MB tamaño (optimizado)
- Sin dependencias externas
- Multiplataforma (Windows, macOS, Linux)

---

## 💡 Ejemplos Rápidos

### Proyecto Básico
```bash
kup init hola
cd hola
kup build --release
./target/release/hola
```

### Con Dependencias
```bash
kup init calculadora
cd calculadora
kup add math 1.0
kup install
kup run dev
```

### Scripts Personalizados
```bash
kup init servidor
cd servidor
# Editar [scripts] en kura.toml
kup run dev
kup run build
kup run test
```

---

## 📞 Soporte y Contribución

- **Documentación:** `https://kura.dev/docs`
- **GitHub:** `https://github.com/KuraLenguaje/Kura`
- **Issues:** Reportar en GitHub Issues
- **Contribuciones:** Pull Requests bienvenidos

---

## 📄 Licencia

KUP es parte del proyecto KURA bajo licencia **MIT**.

---

## ✨ Conclusión

KUP es un gestor de paquetes moderno, eficiente e intuitivo que trae las mejores prácticas de herramientas como npm y cargo al lenguaje KURA. Con su enfoque en simplicidad, rendimiento y compatibilidad con GitHub, proporciona una experiencia de desarrollo fluida y productiva.

**Estado:** ✅ **Listo para producción**

---

**Versión:** 0.4.0 | **Fecha:** Marzo 2026 | **Mantenedor:** Equipo KURA


