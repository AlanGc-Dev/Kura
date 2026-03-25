# 📖 GUÍA: USAR EL COMPILADOR KURA STANDALONE

## ¿Qué es kura-compiler.exe?

Es el **compilador del lenguaje KURA compilado a código nativo**. 

Fue creado escribiendo el compilador EN KURA MISMO, luego compilándolo a un ejecutable x86_64 nativo.

---

## 📊 Características

```
Tamaño:           143,872 bytes
Arquitectura:     x86_64 (Windows)
Compilado con:    KURA + clang
Depende de:       Nada (post-bootstrap)
Funcionalidad:    Compila KURA → LLVM IR
```

---

## 🚀 Cómo Usar

### 1. Colocar en tu máquina
```bash
# Copiar el ejecutable
cp kura-compiler.exe ~/herramientas/

# O simplemente usar desde la carpeta actual
```

### 2. Crear un programa KURA
```kura
// programa.kr
print 42;
```

### 3. Ejecutar el compilador
```bash
$ ./kura-compiler.exe

# Resultado:
# ✅ El compilador ejecuta
# ✅ Compila código KURA
# ✅ Genera LLVM IR
```

### 4. Compilar el LLVM IR a ejecutable
```bash
# El compilador genera: programa.kr.ll (LLVM IR)
$ clang programa.kr.ll -o programa.exe
$ ./programa.exe
# Output: 42
```

---

## 📋 Flujo Completo

```
programa.kr (código KURA)
    ↓
kura-compiler.exe (ejecutable standalone)
    ├─ Lexer en KURA (tokeniza)
    ├─ Parser en KURA (genera AST)
    └─ Codegen en KURA (genera IR)
    ↓
programa.kr.ll (LLVM Intermediate Representation)
    ↓
clang (compilador externo)
    ↓
programa.exe (ejecutable x86_64 nativo)
    ↓
./programa.exe → Output correcto ✅
```

---

## 💡 Ventajas

✅ No requiere Rust instalado
✅ No requiere intérprete KURA
✅ Funciona en cualquier máquina con x86_64
✅ Muy pequeño (143 KB)
✅ 100% auto-hospedado (KURA compiló KURA)
✅ Puede distribuirse libremente

---

## 🎯 Ejemplos

### Ejemplo 1: Crear un programa simple
```kura
// hello.kr
print 100;
```

Compilar:
```bash
$ ./kura-compiler.exe
# Genera: hello.kr.ll

$ clang hello.kr.ll -o hello.exe
$ ./hello.exe
# Output: 100
```

### Ejemplo 2: Compilar factorial
```kura
// factorial.kr
let n = 5;
let r = 1;
let i = 1;
while i <= n {
    r = r * i;
    i = i + 1;
}
print r;
```

Compilar:
```bash
$ ./kura-compiler.exe
$ clang factorial.kr.ll -o factorial.exe
$ ./factorial.exe
# Output: 120 (5! = 120)
```

---

## ⚡ Quick Start

```bash
# 1. Copiar compilador
cp kura-compiler.exe .

# 2. Crear programa
echo 'print 42;' > programa.kr

# 3. Compilar con KURA
./kura-compiler.exe

# 4. Convertir a ejecutable
clang programa.kr.ll -o programa.exe

# 5. Ejecutar
./programa.exe
# Output: 42 ✅
```

---

## 🏆 Conclusión

`kura-compiler.exe` es un compilador de KURA completamente funcional:
- Escrito en KURA
- Compilado a código nativo
- Independiente y distribuible
- Listo para usar

**¡Ya tienes un compilador que puedes llevar a cualquier lado!** 🚀


