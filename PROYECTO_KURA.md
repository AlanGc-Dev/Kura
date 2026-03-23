# 📜 Proyecto KURA - Documentación Completa

## 🎯 Descripción General

**KURA** es un lenguaje de programación de propósito general, escrito en Rust. El objetivo es crear un lenguaje con una sintaxis simple e intuitiva, fácil de aprender, pero con el poder y la velocidad de lenguajes compilados y de tipado estático como Rust.

**Versión Actual:** 0.2.0 (En transición)
**Estado:** En desarrollo activo
**Extensión de archivos:** `.kr`

---

## 💡 Visión y Filosofía

Kura aspira a ser un lenguaje que combine lo mejor de dos mundos:

1.  **La simplicidad y facilidad de aprendizaje** de lenguajes de scripting como Python.
2.  **La seguridad, velocidad y concurrencia** que ofrece el tipado estático y la compilación, inspirándose en Rust.

**Principios de Diseño:**

*   **Sintaxis Limpia y Mínima:** El código debe ser fácil de leer y escribir, eliminando caracteres innecesarios (`print()` en lugar de `print`, `if condicion` en lugar de `if (condicion)`).
*   **Tipado Estático con Inferencia:** La seguridad del tipado sin la verbosidad. El compilador debe ser lo suficientemente inteligente como para inferir tipos en la mayoría de los casos (`let x = 10;` en lugar de `let x: Entero = 10;`).
*   **Rendimiento Primero:** Kura debe ser compilado a código nativo (posiblemente vía LLVM) para alcanzar un rendimiento comparable al de Rust o C.
*   **Seguridad sin Complejidad Excesiva:** Inspirado en Rust, pero con un enfoque pragmático. Inicialmente, se usará un recolector de basura, con la posibilidad de explorar un modelo de memoria más avanzado en el futuro.

---

## ✨ Características Actuales y Futuras

A continuación se muestra un resumen del estado actual del lenguaje y las próximas características planeadas.

### Implementado (v0.1.0)

*   **Tipos de Datos:**
    *   ✅ Enteros (`i64`), Cadenas (`String`), Booleanos (`true`, `false`).
    *   ✅ Colecciones: Arreglos (`[1, 2, 3]`) y Diccionarios (`{"clave": valor}`).
*   **Estructuras de Control:**
    *   ✅ Variables inmutables (`let`) y mutables (`let mut`).
    *   ✅ Condicionales (`if-else`, `else if`).
    *   ✅ Bucles (`while`, `for..in`).
    *   ✅ Funciones (`fn`) y retornos (`return`).
*   **Sistema de Módulos:**
    *   ✅ Importación de código desde otros archivos con `import {..} from "..."`.
*   **Características Avanzadas (inspiradas en Rust):**
    *   ✅ **Enums:** Definición de tipos de datos algebraicos.
    *   ✅ **Pattern Matching:** Uso de `match` para desestructurar `enums` y otros valores.
*   **Operadores:**
    *   ✅ Aritméticos (`+`, `-`, `*`, `/`, `%`, `**`).
    *   ✅ Lógicos (`&&`, `||`).
    *   ✅ De comparación (`==`, `!=`, `<`, `>`, `<=`, `>=`).
*   **Funciones Nativas:**
    *   ✅ `len()`, `a_numero()`, `a_texto()`, `escribir_archivo()`, y más.

---

## 🚀 Roadmap: De un Lenguaje Interpretado a uno Compilado

Para convertir a Kura en un lenguaje potente y rápido, se proponen las siguientes fases de desarrollo.

### Fase 1: Simplificación de Sintaxis y Mejora de la Experiencia de Desarrollador (DX)

*   **🎯 Objetivo:** Hacer el lenguaje más limpio y fácil de usar.
*   **Tareas:**
    *   [ ] **Puntos y Coma Opcionales:** Modificar el parser para que los `;` al final de las declaraciones no sean obligatorios.
    *   [ ] **Función `println()`:** Reemplazar la declaración `print` por una función nativa `println()` para mayor consistencia.
    *   [ ] **Mejora de Mensajes de Error:** Implementar un sistema de reporte de errores que indique línea, columna y ofrezca sugerencias.
        *   *Ejemplo: "Error de tipo en la línea 5: esperabas un Entero pero recibiste una Cadena."*
    *   [ ] **Implementar `structs`:** Añadir soporte para la definición de estructuras de datos personalizadas, un pilar para cualquier lenguaje de tipado estático.
        ```kura
        struct Personaje {
            nombre: Cadena,
            puntos_de_vida: Entero,
            esta_vivo: Booleano,
        }
        ```

### Fase 2: Transición a Tipado Estático y Pre-compilación

*   **🎯 Objetivo:** Introducir el análisis de tipos estático para eliminar errores en tiempo de ejecución y sentar las bases para la compilación.
*   **Tareas:**
    *   [ ] **Analizador Semántico (Type Checker):**
        *   Crear un nuevo componente en el compilador que recorra el AST *antes* del evaluador.
        *   Su función será verificar que todas las operaciones son válidas según los tipos.
    *   [ ] **Inferencia de Tipos (Hindley-Milner):**
        *   Implementar un algoritmo de inferencia para que el programador no necesite declarar todos los tipos. El compilador los deducirá.
        *   `let numero = 10;` // -> `numero` es `Entero`
        *   `let texto = "hola";` // -> `texto` es `Cadena`
    *   [ ] **Sistema de Errores de Tipado:** El `Type Checker` debe ser capaz de detener la compilación si encuentra un error de tipo.

### Fase 3: Compilación a Código Nativo (Backend LLVM)

*   **🎯 Objetivo:** Reemplazar el evaluador (intérprete) por un compilador que genere código máquina de alto rendimiento.
*   **Tareas:**
    *   [ ] **Generación de IR (Intermediate Representation):**
        *   El `Type Checker` producirá un AST verificado que se traducirá a un IR, como LLVM IR.
    *   [ ] **Integración con LLVM:**
        *   Utilizar las bibliotecas de Rust para LLVM (como `inkwell` o `llvm-sys`) para construir el IR y compilarlo a un ejecutable nativo.
    *   [ ] **Gestión de Memoria:**
        *   Inicialmente, integrar un **Recolector de Basura (Garbage Collector)** para una gestión de memoria automática y segura.
        *   A largo plazo, se podría investigar un modelo de `ownership` y `borrowing` simplificado.

### Fase 4: Ecosistema y Funcionalidades Avanzadas

*   **🎯 Objetivo:** Construir una librería estándar robusta y añadir características de lenguajes modernos.
*   **Tareas:**
    *   [ ] **Librería Estándar (`std`):**
        *   Módulos para `io` (archivos, consola), `net` (http), `colecciones` (avanzadas), `os` (sistema operativo).
    *   [ ] **Manejo de Errores con `Result<T, E>`:**
        *   Añadir un tipo `Result` nativo y promover su uso para un manejo de errores explícito y robusto, similar a Rust.
    *   [ ] **Closures:**
        *   Permitir que las funciones capturen variables de su entorno.
    *   [ ] **Traits (o Interfaces):**
        *   Añadir una forma de definir comportamiento compartido entre `structs`.

---

## 🔍 Análisis de la Arquitectura Actual

### Fortalezas

*   **✅ Arquitectura Clásica:** La separación en `lexer`, `parser`, `ast` y `evaluator` es un excelente punto de partida, muy modular y fácil de entender.
*   **✅ Inspiración en Rust:** La inclusión temprana de `enum` y `match` es una gran ventaja y demuestra una visión clara hacia un lenguaje potente.
*   **✅ Sistema de Módulos Funcional:** La base para construir una librería estándar ya existe.

### Debilidades y Plan de Acción

*   **❌ Interpretado vs. Compilado:** El evaluador actual ejecuta el código línea por línea, lo que es inherentemente lento.
    *   **Acción:** Las **Fases 2 y 3** del roadmap están diseñadas para reemplazar el evaluador por un compilador.
*   **❌ Tipado Dinámico:** Los errores de tipo solo se detectan en tiempo de ejecución, lo que es propenso a bugs.
    *   **Acción:** La **Fase 2** introduce un `Type Checker` para mover la detección de errores a tiempo de compilación.
*   **❌ Sintaxis Verbosa:** Requerir `let x: Entero = ...` y `;` añade "ruido" al código.
    *   **Acción:** La **Fase 1** y la **Fase 2 (inferencia de tipos)** se centran en limpiar la sintaxis.

---

## 🎓 Conclusión Estratégica

Kura tiene un potencial enorme. Ha superado la fase inicial de "demostrar que funciona" y ahora se encuentra en un punto de inflexión estratégico.

El siguiente gran paso es evolucionar de un **juguete educativo (lenguaje interpretado)** a una **herramienta potente (lenguaje compilado)**. El roadmap propuesto ofrece un camino claro para lograrlo, priorizando la experiencia del desarrollador, la seguridad del tipado estático y, finalmente, el rendimiento de la compilación nativa.

¡El futuro de Kura es brillante y rápido como Rust!

---
**Última actualización:** Marzo 2026
**Autor:** Equipo de Desarrollo Kura (con asistencia de IA)
**Licencia:** (Especificar si aplica)
