# 📌 KUP - Referencia Rápida de Comandos

**Todos los comandos de KUP en una sola página**

---

## 🎯 Proyectos

```bash
kup init [nombre]           # Crear nuevo proyecto
kup install                 # Instalar todas las dependencias
kup update                  # Actualizar dependencias
kup list                    # Listar dependencias instaladas
```

---

## 🔨 Compilación

```bash
kup build                   # Compilar a debug
kup build --release         # Compilar optimizado (-O3) 
kup compile                 # Alias de build --release
kup clean                   # Limpiar caché y temporales
```

---

## 📦 Paquetes

```bash
kup add <paquete>           # Agregar paquete (última versión)
kup add <paquete> <versión> # Agregar versión específica
kup add usuario/repo        # Agregar desde GitHub
kup add https://url.kr      # Agregar desde URL
kup remove <paquete>        # Remover paquete
kup search <término>        # Buscar paquetes
kup info <paquete>          # Ver info del paquete
```

---

## ▶️ Scripts

```bash
kup run start               # Ejecutar script "start"
kup run dev                 # Ejecutar script "dev"
kup run build               # Ejecutar script "build"
kup run <script>            # Ejecutar cualquier script
```

---

## ℹ️ Información

```bash
kup --help                  # Mostrar ayuda completa
kup -h                      # Alias para --help
kup help                    # Alias para --help
kup --version               # Mostrar versión
kup -v                      # Alias para --version
```

---

## 📝 Ejemplos Comunes

### Crear proyecto y empezar
```bash
kup init mi_app
cd mi_app
kup run dev
```

### Compilar a nativo
```bash
kup build --release
.\target\release\mi_app.exe
```

### Con dependencias
```bash
kup add math 1.0
kup add usuario/libreria
kup install
```

### Scripts personalizados
```bash
# En kura.toml:
[scripts]
dev = "kura src/main.kr"
build = "kup build --release"

# Ejecutar:
kup run dev
kup run build
```

---

## 🔧 kura.toml (Configuración)

```toml
[package]
nombre = "mi_app"
version = "0.1.0"
descripcion = "Mi proyecto"
autor = "Nombre"
licencia = "MIT"
entrada = "main.kr"

[dependencias]
math = "1.0"
strings = "1.2"

[scripts]
start = "kura src/main.kr"
dev = "kura src/main.kr"
build = "kup build --release"
```

---

## 📂 Estructura de Proyecto

```
mi_app/
├── kura.toml              # Configuración
├── kura.lock              # Versiones bloqueadas
├── src/
│   └── main.kr            # Código principal
├── kura_modules/          # Paquetes instalados
└── target/
    ├── debug/             # Compilados debug
    │   ├── mi_app.exe
    │   └── mi_app.ll
    └── release/           # Compilados optimizados
        ├── mi_app.exe
        └── mi_app.ll
```

---

## ⌨️ Atajos Útiles

```bash
# Instalar + compilar
kup install && kup build --release

# Ejecutar desarrollo
kup run dev

# Compilar y limpiar
kup build --release && kup clean

# Ver todo
kup list && kup --help
```

---

## 🐛 Troubleshooting Rápido

| Error | Solución |
|-------|----------|
| "kura.toml no encontrado" | Ejecutar `kup init` |
| "Script no encontrado" | Verificar nombre en [scripts] en kura.toml |
| "LLVM no encontrado" | Instalar LLVM 10+ |
| "Permisos denegados" | Ejecutar script instalación con admin |

---

## 💾 Instalación Rápida

```powershell
# Windows
cd P:\KuraLenguaje\Kura
.\install_kup.bat

# macOS/Linux
cd ~/KuraLenguaje/Kura
chmod +x install_kup.sh
./install_kup.sh
```

---

## 📊 Flujo Típico

```
1. kup init mi_app          # Crear
   ↓
2. cd mi_app                # Entrar
   ↓
3. Editar src/main.kr       # Programar
   ↓
4. kup run dev              # Probar
   ↓
5. kup build --release      # Compilar
   ↓
6. .\target\release\mi_app  # Ejecutar
```

---

## 🎓 Documentación Completa

- **Quick Start:** `KUP_QUICK_START.md`
- **Manual:** `KUP_EJECUTABLE_README.md`
- **Tutorial:** `EJEMPLO_KUP_PRACTICO.md`
- **Guía Completa:** `KUP_GUIA_COMPLETA.md`
- **Índice:** `KUP_DOCUMENTACION_INDICE.md`

---

## 🔗 Versión: 0.4.0


