# 🛠️ KUP - Kura Universal Package Manager

**Versión:** 0.4.0  
**Tamaño Ejecutable:** 3.3 MB (release)  
**Plataforma:** Windows, macOS, Linux  
**Archivo de Configuración:** `kura.toml`

---

## 📋 Tabla de Contenidos

1. [Descripción](#descripción)
2. [Instalación](#instalación)
3. [Inicio Rápido](#inicio-rápido)
4. [Comandos Principales](#comandos-principales)
5. [Gestión de Dependencias](#gestión-de-dependencias)
6. [Scripts del Proyecto](#scripts-del-proyecto)
7. [Compilación](#compilación)
8. [Estructura de Archivos](#estructura-de-archivos)
9. [Ejemplos Prácticos](#ejemplos-prácticos)
10. [Troubleshooting](#troubleshooting)

---

## 📖 Descripción

**KUP** es el gestor de paquetes universal para el lenguaje **KURA**. Inspirado en herramientas como **npm** (Node.js), **pip** (Python) y **cargo** (Rust), proporciona:

✅ Gestión de dependencias  
✅ Comandos y scripts tipo Node.js  
✅ Compilación nativa con LLVM  
✅ Configuración en TOML  
✅ Integración con GitHub  
✅ Sistema de caché  

---

## 🚀 Instalación

### Opción 1: Desde el repositorio compilado
```bash
# Windows
cd P:\KuraLenguaje\Kura
.\target\release\kup.exe init mi_proyecto

# macOS/Linux
./target/release/kup init mi_proyecto
```

### Opción 2: Compilar desde source
```bash
cd P:\KuraLenguaje\Kura
cargo build --bin kup --release
```

### Opción 3: Agregar a PATH (opcional)
Copiar `kup.exe` a una carpeta en tu PATH para usarlo globalmente.

```powershell
# Windows
Copy-Item .\target\release\kup.exe "C:\Program Files\Kura\kup.exe"

# Luego puedes usar:
kup init mi_proyecto
```

---

## ⚡ Inicio Rápido

### Crear un nuevo proyecto
```bash
kup init calculadora
cd calculadora
```

### Estructura generada
```
calculadora/
├── kura.toml          # Manifiesto del proyecto
├── kura.lock          # Versiones fijas (auto-generado)
├── src/
│   └── main.kr        # Código principal
└── kura_modules/      # Dependencias instaladas
```

### Ejemplo de `kura.toml`
```toml
nombre = "calculadora"
version = "0.1.0"
descripcion = "Una calculadora simple en Kura"
autor = "Tu Nombre"
licencia = "MIT"
entrada = "main.kr"

[dependencias]
# Aquí van los paquetes

[scripts]
start = "kura run src/main.kr"
dev = "kura src/main.kr"
build = "kup build --release"
```

---

## 📦 Comandos Principales

### Proyectos
| Comando | Descripción |
|---------|-------------|
| `kup init [nombre]` | Crear nuevo proyecto Kura |
| `kup install` | Instalar todas las dependencias |
| `kup update` | Actualizar dependencias a última versión |
| `kup list` | Listar dependencias instaladas |
| `kup build` | Compilar a ejecutable nativo (debug) |
| `kup build --release` | Compilar con optimizaciones máximas |
| `kup compile` | Alias de `kup build --release` |

### Paquetes
| Comando | Descripción |
|---------|-------------|
| `kup add <pkg> [v]` | Agregar paquete a dependencias |
| `kup add usuario/repo` | Agregar desde GitHub |
| `kup remove <pkg>` | Eliminar paquete |
| `kup search <termino>` | Buscar paquetes en registro |
| `kup info <pkg>` | Ver información del paquete |

### Scripts
| Comando | Descripción |
|---------|-------------|
| `kup run <script>` | Ejecutar script definido en `kura.toml` |

---

## 📚 Gestión de Dependencias

### Agregar un paquete
```bash
# Versión específica
kup add math 1.0

# Última versión
kup add strings

# Desde GitHub
kup add usuario/libreria_cool

# URL completa
kup add https://github.com/user/repo/raw/main/lib.kr
```

### Instalar todas las dependencias
```bash
kup install
```

Descarga todos los paquetes especificados en `kura.toml` y los guarda en `kura_modules/`.

### Ver dependencias instaladas
```bash
kup list
```

**Salida:**
```
📋 Dependencias del proyecto:
  - math: 1.0
  - strings: 1.2
  - http: 0.5

🔧 Dev Dependencias:
  - test_framework: 1.0
```

### Remover un paquete
```bash
kup remove math
```

---

## 🔄 Scripts del Proyecto

Define scripts en `kura.toml` bajo `[scripts]`:

```toml
[scripts]
start = "kura src/main.kr"
dev = "kura src/main.kr"
build = "kup build --release"
test = "kura test/main.kr"
benchmark = "echo Corriendo benchmarks..."
```

### Ejecutar un script
```bash
kup run start
kup run dev
kup run build
kup run test
```

**Salida:**
```
▶️  Ejecutando: kura src/main.kr
✅ Ejecutando: src/main.kr
```

---

## 🔨 Compilación

### Modo Debug
```bash
kup build
```

- Crea binario en `target/debug/`
- Sin optimizaciones
- Ideal para desarrollo

### Modo Release
```bash
kup build --release
# O equivalentemente:
kup compile
```

- Crea binario en `target/release/`
- Optimizaciones `-O3`
- Más rápido (~3-5x)
- Ideal para producción

### Salida de compilación
```
🔨 Compilando proyecto 'mi_app' desde 'src/main.kr'...
⚡ Modo release (-O3)
📄 LLVM IR guardado: target/release/mi_app.ll
✅ Ejecutable listo: target/release/mi_app.exe
   Ejecuta con: ./target/release/mi_app.exe
```

### Ejecución
```bash
# Windows
.\target\release\mi_app.exe

# macOS/Linux
./target/release/mi_app
```

---

## 📂 Estructura de Archivos

```
proyecto/
├── kura.toml                 # Manifiesto (nombre, versión, scripts)
├── kura.lock                 # Versiones bloqueadas (lock file)
├── src/
│   ├── main.kr              # Entrada principal
│   ├── util.kr              # Módulos adicionales
│   └── ...
├── kura_modules/            # Dependencias instaladas
│   ├── math.kr
│   ├── strings.kr
│   └── ...
├── target/
│   ├── debug/
│   │   ├── proyecto.exe
│   │   └── proyecto.ll
│   └── release/
│       ├── proyecto.exe     # Binario optimizado
│       └── proyecto.ll      # LLVM IR
├── .kura_cache/             # Caché de descargas
└── tests/                    # Tests (opcional)
```

---

## 💡 Ejemplos Prácticos

### Ejemplo 1: Calculadora Simple
```bash
# Crear proyecto
kup init calculadora
cd calculadora

# Editar src/main.kr
```

```kura
let x = 10;
let y = 5;
print "Suma: ";
print x + y;
print "Resta: ";
print x - y;
print "Multiplicación: ";
print x * y;
print "División: ";
print x / y;
```

```bash
# Ejecutar
kup run start

# Compilar
kup build --release

# Ejecutar binario nativo
.\target\release\calculadora.exe
```

---

### Ejemplo 2: Proyecto con Dependencias
```bash
# Crear proyecto
kup init server_web
cd server_web

# Agregar dependencias
kup add http 1.0
kup add json 0.5
kup install

# Editar src/main.kr para usar dependencias
kup run dev

# Compilar para producción
kup build --release
```

---

### Ejemplo 3: Scripts Personalizados
```toml
[scripts]
dev = "kura src/main.kr"
build = "kup build --release"
test = "kura tests/main.kr"
lint = "echo Analizando código..."
format = "echo Formateando código..."
all = "kup run lint; kup run test; kup run build"
```

```bash
kup run dev       # Desarrollo rápido
kup run test      # Ejecutar tests
kup run build     # Compilar
```

---

## 🐛 Troubleshooting

### Error: "kura.toml no encontrado"
**Solución:** Ejecuta `kup init` primero
```bash
kup init
```

---

### Error: "Paquete no descargado"
**Posibles causas:**
1. URL incorrecta
2. Conexión de red
3. Servidor no disponible

**Solución:**
```bash
# Verificar conexión
ping github.com

# Intentar manualmente
kup add usuario/repo
```

---

### Error: "LLVM no encontrado" (al compilar)
**Solución:** Verificar que LLVM está instalado
```bash
# Windows
where llc
where llvm-link

# macOS/Linux
which llc
which llvm-link
```

---

### El ejecutable es muy lento
**Soluciones:**
1. Usar `--release`: `kup build --release`
2. Verificar que no hay loops infinitos en el código
3. Usar profiler

---

### Scripts no se ejecutan
**Solución:** Verificar sintaxis en `kura.toml`
```toml
[scripts]
start = "kura src/main.kr"  # ✅ Correcto
run = kura src/main.kr       # ❌ Incorrecto (sin comillas)
```

---

## 🔗 Integración con GitHub

### Agregar desde GitHub
```bash
# Usuario/repositorio
kup add usuario/libreria

# Con rama específica
kup add usuario/libreria main
```

**Resolución automática:**
```
usuario/libreria → https://raw.githubusercontent.com/usuario/libreria/main/main.kr
```

---

## 📊 Archivo de Caché

**Ubicación:** `.kura_cache/`

**Ventajas:**
- ✅ Descargas más rápidas
- ✅ Funciona sin conexión (si está cacheado)
- ✅ Reduce tráfico de red

**Limpiar caché:**
```bash
rm -r .kura_cache
# Windows
Remove-Item ".kura_cache" -Recurse -Force
```

---

## 📖 Documentación Oficial

- [Guía de KURA](./GUIA_ESTRUCTURA_DESARROLLO.md)
- [Referencia Rápida](./QUICK_REFERENCE.md)
- [Instrucciones de Compilación](./EXE_COMPILADOR_STANDALONE.md)

---

## 🎯 Próximos Pasos

- [ ] Registrar proyecto en registro público
- [ ] Publicar paquetes en `pkg.kura.io`
- [ ] Agregar soporte para workspaces
- [ ] Implementar hook pre/post scripts
- [ ] Agregar GUI para KUP

---

## 📝 Licencia

KUP es parte del proyecto KURA bajo licencia MIT.

---

**¿Necesitas ayuda?** Abre un issue en el repositorio o consulta la documentación.


