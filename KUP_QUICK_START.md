# ⚡ KUP - Quick Start (Inicio Rápido)

**5 minutos para tener tu primer proyecto Kura**

---

## 1️⃣ Instalar KUP

### Windows (Recomendado)
```powershell
cd P:\KuraLenguaje\Kura
.\install_kup.bat  # Requiere admin

# O usar directamente:
.\target\release\kup.exe init mi_app
```

### macOS/Linux
```bash
cd ~/KuraLenguaje/Kura
chmod +x install_kup.sh
./install_kup.sh

# O usar directamente:
./target/release/kup init mi_app
```

---

## 2️⃣ Crear Proyecto (30 segundos)

```bash
kup init mi_app
cd mi_app
```

**Resultado:**
```
mi_app/
├── kura.toml       # Configuración
├── src/main.kr     # Tu código
└── kura_modules/   # Paquetes
```

---

## 3️⃣ Escribir Código (1 minuto)

**Editar `src/main.kr`:**

```kura
let nombre = "Kura";
print "¡Hola, ";
print nombre;
print "!";
```

---

## 4️⃣ Ejecutar (10 segundos)

```bash
kup run dev
```

**Salida:**
```
¡Hola, Kura!
```

---

## 5️⃣ Compilar a Nativo (2 minutos)

```bash
kup build --release
```

**Resultado:**
```
target/release/mi_app.exe  (ejecutable nativo)
```

### Ejecutar
```bash
# Windows
.\target\release\mi_app.exe

# macOS/Linux
./target/release/mi_app
```

---

## 🎯 Comandos Esenciales

```bash
kup init [nombre]        # Crear proyecto
kup run dev              # Ejecutar código
kup build --release      # Compilar
kup add <paquete>        # Agregar paquete
kup list                 # Ver paquetes
kup clean                # Limpiar
kup --help               # Ayuda
```

---

## 📚 Documentación Completa

- **Guía Completa:** `KUP_GUIA_COMPLETA.md`
- **Ejemplo Detallado:** `EJEMPLO_KUP_PRACTICO.md`
- **Resumen:** `KUP_RESUMEN_EJECUTIVO.md`

---

## 💡 Próximos Pasos

1. **Explorar Código KURA**
   ```kura
   // Variables
   let x = 10;
   let mut y = 20;
   
   // Condicionales
   if x < y {
       print "x es menor";
   }
   
   // Bucles
   while y > 0 {
       print y;
       y = y - 1;
   }
   
   // Funciones
   fn suma(a, b) {
       return a + b;
   }
   ```

2. **Agregar Dependencias**
   ```bash
   kup add math 1.0
   kup install
   ```

3. **Scripts Personalizados**
   ```toml
   [scripts]
   dev = "kura src/main.kr"
   build = "kup build --release"
   test = "kura tests/main.kr"
   ```

---

## ✅ ¡Listo!

Ya tienes KUP funcionando. Disfruta desarrollando en KURA. 🚀


