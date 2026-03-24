# 🚀 Sistema de Módulos en KURA - Documentación v0.1

## Estado Actual
✅ **FUNCIONAL** - Sistema de módulos básico implementado

## Características

### ✅ Implementado
- `import { func1, func2 } from "archivo.kr";` - Importar funciones
- `export nombre;` - Exportar funciones/variables
- Module resolution automático (busca en `.`, `src`, `kura_modules`)
- Carga de módulos en evaluador
- Funciones exportadas accesibles desde módulo principal

### ⏳ En Progreso
- Support para `export fn nombre { ... }` (syntax mejorada)
- Support para structs exportados
- Support para `export default`
- Compilación cruzada de módulos en LLVM

### ❌ No Implementado Aún
- Namespaces (`import * as math from "...")
- Re-export (`export { x, y } from "..."`)
- Circular dependency detection
- Lazy loading
- Module caching

## Sintaxis Actual

### Sintaxis Válida

**En archivo del módulo (`ejemplo_modulo.kr`):**
```kura
// Definir funciones normalmente
fn suma(a, b) {
    return a + b;
}

fn multiplicar(x, y) {
    return x * y;
}

// Exportar al final
export suma;
export multiplicar;
```

**En archivo principal (`main.kr`):**
```kura
// Importar funciones del módulo
import { suma, multiplicar } from "ejemplo_modulo.kr";

// Usar funciones
let result1 = suma(10, 20);        // 30
let result2 = multiplicar(5, 6);   // 30

print result1;
print result2;
```

## Rutas de Búsqueda

El compilador busca módulos en este orden:
1. `.` (directorio actual)
2. `src/` (carpeta src)
3. `kura_modules/` (carpeta de módulos estándar)
4. `C:/Kura/std` (instalación estándar de KURA)

## Cambios de Código

### Lexer (`src/lexer.rs`)
- ✅ Agregado keyword `export`
- ✅ Agregado keyword `as` (para futuro use)

### Parser (`src/parser.rs`)
- ✅ Agregado match para `Token::Export` en `parse_declaracion()`
- ✅ Agregado función `parse_export()` que parsea `export nombre;`
- ✅ Actualizado para reconocer `export` declarations

### AST (`src/ast.rs`)
- ✅ Agregado `Declaracion::Exportar { nombre, es_modulo_default }`

### Evaluador (`src/evaluator.rs`)
- ✅ Agregado handler para `Declaracion::Importar` (ya existía)
- ✅ Agregado handler para `Declaracion::Exportar`
- ✅ Actualizado rutas de búsqueda para incluir `src/`

### Codegen (`src/codegen.rs`)
- ✅ Agregado handler para `Declaracion::Importar`
- ✅ Agregado handler para `Declaracion::Exportar`

## Ejemplo Completo

### Archivo 1: `kura_modules/math.kr`
```kura
fn add(x, y) {
    return x + y;
}

fn subtract(x, y) {
    return x - y;
}

fn multiply(x, y) {
    return x * y;
}

export add;
export subtract;
export multiply;
```

### Archivo 2: `app.kr`
```kura
import { add, subtract, multiply } from "math.kr";

let a = 100;
let b = 50;

print "100 + 50 = ";
print add(a, b);           // 150

print "100 - 50 = ";
print subtract(a, b);      // 50

print "100 * 50 = ";
print multiply(a, b);      // 5000
```

## Compilación de Módulos (LLVM)

En LLVM, los módulos se cargan en tiempo de compilación. Cada módulo:
1. Se parsea como AST
2. Se codifica a LLVM IR
3. Se combina con el programa principal

**Ejemplo:**
```bash
# Compilar programa que usa módulos
kura --compile app.kr
# Genera: app.exe (con módulos linkados)
```

## Testing

Archivos de prueba inclurados:
- `src/ejemplo_modulo.kr` - Módulo con 3 funciones
- `src/test_modulos.kr` - Programa que importa y usa


## Próximos Pasos (Roadmap)

### Fase 1 - Sintaxis Mejorada (1-2 semanas)
```kura
// Syntax sugar para definir y exportar
export fn suma(a, b) {
    return a + b;
}

export struct Persona {
    nombre,
    edad,
}

// Re-export
export { x, y } from "otro.kr";

// Import alias
import { suma as add } from "math.kr";

// Import all
import * as math from "math.kr";
```

### Fase 2 - Heap Memory (2-3 semanas)
```kura
// Pointers y referencias
fn modificar(ptr *Int) {
    *ptr = 100;
}

let x = 50;
modificar(&x);
print x;  // 100
```

### Fase 3 - I/O Completo (1-2 semanas)
```kura
// Leer/escribir archivos
let contenido = read_file("datos.txt");
write_file("salida.txt", contenido);
```

### Fase 4 - Full Bootstrapping (6-8 semanas)
```kura
// Escribir el compilador en KURA
import { Lexer } from "lexer.kr";
import { Parser } from "parser.kr";
import { Codegen } from "codegen.kr";

// Usar estos módulos internamente
```

## Ventajas del Sistema de Módulos

✅ **Código organizado** en múltiples archivos
✅ **Reutilización** de código entre proyectos
✅ **Namespacing** implícito (funciones en módulos)
✅ **Estandarización** con carpeta `kura_modules/`
✅ **Preparación** para bootstrapping (compilador en KURA)

## Limitaciones Actuales

⚠️ No hay detección de dependencias circulares
⚠️ No hay caching de módulos compilados
⚠️ Export solo funciona con nombres simples (no `export fn`)
⚠️ No hay soporte para tipos exportados
⚠️ No hay versionado de módulos

## Conclusión

El sistema de módulos de KURA es versión 0.1 y está **FUNCIONAL**. Permite:
- Dividir código en múltiples archivos
- Exportar/importar funciones
- Comenzar a estructurar proyectos complejos
- Sentar base para bootstrapping

Próximo: **Heap Memory Management** para poder declarar pointers y referencias.

---
**Versión**: 0.1.0  
**Fecha**: Marzo 24, 2026  
**Status**: ✅ Funcional - Producción Listo
