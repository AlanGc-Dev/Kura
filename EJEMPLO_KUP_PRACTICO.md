# 🎯 Ejemplo Práctico: Crear un Proyecto con KUP

## Paso 1: Crear el proyecto

```bash
kup init mi_calculadora
cd mi_calculadora
```

**Salida esperada:**
```
✨ Proyecto 'mi_calculadora' inicializado!
📁 Estructura creada:
  - kura.toml (manifiesto)
  - kura.lock (versiones fijas)
  - src/main.kr (archivo principal)
  - kura_modules/ (dependencias)

📝 Scripts disponibles:
  - kup run start → kura run main
  - kup run dev → kura src/main.kr
  - kup run build → echo Compilando...
```

---

## Paso 2: Editar el archivo principal

**Archivo: `src/main.kr`**

```kura
// Calculadora simple en KURA
let x = 15;
let y = 3;

print "=== CALCULADORA KURA ===";
print "";

print "Números: ";
print x;
print " y ";
print y;
print "";

print "Suma: ";
print x + y;

print "Resta: ";
print x - y;

print "Multiplicación: ";
print x * y;

print "División: ";
print x / y;

print "Módulo: ";
print x % y;

print "Potencia: ";
print x ** 2;

print "";
print "¡Calculadora Terminada!";
```

---

## Paso 3: Ejecutar el proyecto en desarrollo

```bash
kup run dev
```

**Salida esperada:**
```
▶️  Ejecutando: kura src/main.kr
=== CALCULADORA KURA ===

Números: 15 y 3

Suma: 18
Resta: 12
Multiplicación: 45
División: 5
Módulo: 0
Potencia: 225

¡Calculadora Terminada!
```

---

## Paso 4: Verificar el proyecto

```bash
kup list
```

**Salida:**
```
📋 Dependencias del proyecto:
  (ninguna)

🔧 Dev Dependencias:
  (ninguna)
```

---

## Paso 5: Compilar a ejecutable nativo

### Modo Debug (rápido para desarrollo)
```bash
kup build
```

**Salida:**
```
🔨 Compilando proyecto 'mi_calculadora' desde 'src/main.kr'...
📄 LLVM IR guardado: target/debug/mi_calculadora.ll
✅ Ejecutable listo: target/debug/mi_calculadora.exe
   Ejecuta con: ./target/debug/mi_calculadora.exe
```

### Modo Release (optimizado para producción)
```bash
kup build --release
# O equivalentemente:
kup compile
```

**Salida:**
```
🔨 Compilando proyecto 'mi_calculadora' desde 'src/main.kr'...
⚡ Modo release (-O3)
📄 LLVM IR guardado: target/release/mi_calculadora.ll
✅ Ejecutable listo: target/release/mi_calculadora.exe
   Ejecuta con: ./target/release/mi_calculadora.exe
```

---

## Paso 6: Ejecutar el binario compilado

```bash
# Windows
.\target\release\mi_calculadora.exe

# macOS/Linux
./target/release/mi_calculadora
```

**Salida:**
```
=== CALCULADORA KURA ===

Números: 15 y 3

Suma: 18
Resta: 12
Multiplicación: 45
División: 5
Módulo: 0
Potencia: 225

¡Calculadora Terminada!
```

---

## Paso 7: Ver el archivo generado

El LLVM IR (Intermediate Representation) se guarda automáticamente:

```bash
# Ver el IR
type target/release/mi_calculadora.ll

# O en Linux/macOS
cat target/release/mi_calculadora.ll
```

---

## Paso 8: Limpiar archivos temporales

```bash
kup clean
```

**Salida:**
```
🧹 Limpiando caché y archivos temporales...
  ✓ .kura_cache/
  ✓ target/debug/
  ✓ temporal.ll
  ✓ temporal.obj
✅ 4 elementos limpiados
```

---

## Paso 9: Personalizar Scripts

Editar `kura.toml` para agregar scripts personalizados:

```toml
[scripts]
start = "kura src/main.kr"
dev = "kura src/main.kr"
build = "kup build --release"
test = "kura tests/test.kr"
clean = "kup clean"
all = "kup run build; kup run test"
```

Ejecutar scripts personalizados:
```bash
kup run start    # Ejecutar desarrollo
kup run build    # Compilar optimizado
kup run test     # Tests
kup run clean    # Limpiar
kup run all      # Hacer todo
```

---

## Paso 10: Agregar Dependencias (Ejemplo)

```bash
kup add math 1.0
```

**Salida:**
```
🚚 Añadiendo 'math@1.0' a dependencias...
✅ 'math' agregado a kura.toml
```

Ver dependencias:
```bash
kup list
```

**Salida:**
```
📋 Dependencias del proyecto:
  - math: 1.0
```

---

## 📊 Estructura final del proyecto

```
mi_calculadora/
├── kura.toml                    # Manifiesto
├── kura.lock                    # Lock file
├── src/
│   └── main.kr                  # Código fuente
├── kura_modules/                # Dependencias
│   └── math.kr                  # (si se instaló)
├── target/
│   ├── debug/
│   │   ├── mi_calculadora.exe
│   │   └── mi_calculadora.ll
│   └── release/
│       ├── mi_calculadora.exe   # Binario optimizado
│       └── mi_calculadora.ll    # IR optimizado
└── .kura_cache/                 # Caché (se limpia con kup clean)
```

---

## 🎯 Resumen de Comandos Usados

```bash
# Crear y navegar
kup init mi_calculadora
cd mi_calculadora

# Desarrollo
kup run dev          # Ejecutar interactivamente
kup list            # Ver dependencias

# Compilación
kup build           # Debug
kup build --release # Release (optimizado)

# Mantenimiento
kup clean           # Limpiar caché
kup list            # Listar paquetes
```

---

## ✅ Próximos Pasos

1. **Publicar el proyecto**: Agregar a GitHub
2. **Compartir paquetes**: `kup add usuario/mi_calculadora`
3. **Agregar más funcionalidades**: Editar `src/main.kr`
4. **Crear módulos reutilizables**: Organizar código en `src/`
5. **Distribuir ejecutable**: `./target/release/mi_calculadora.exe`

---

**¡Listo!** Ahora tienes un proyecto Kura funcional con empaquetado, scripts y compilación nativa. 🚀


