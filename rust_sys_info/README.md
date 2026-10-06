# 🦀 rust_sys_info — CLI Telemetry System

Módulo de telemetría y diagnósticos de sistema en CLI desarrollado en Rust como parte del *Rust Systems Programming Track*. 

El proyecto demuestra el uso de tipos compuestos, garantías de seguridad en tiempo de compilación, inmutabilidad, *shadowing* y expresiones puras.

---

## 🚀 Características y Funcionalidades

- **Interacción E/S**: Lectura limpia de entradas de usuario vía `std::io::stdin`.
- **Métricas de CPU**: Manejo de especificaciones de hardware mediante tuplas heterogéneas `(u32, f64, bool)`.
- **Lecturas de Núcleos**: Almacenamiento contiguo de telemetría de CPU mediante arreglos de tamaño fijo `[u8; 4]`.
- **Procesamiento de Métricas**: Cálculo de promedio de uso de CPU mediante referencias prestadas (`&[u8]`) y *casting* explícito de tipos.
- **Clasificación Dinámica**: Clasificación del estado de carga (`[ALTO]` vs `[BAJO]`) aprovechando `if` como expresión.

---

## 🛠️ Conceptos de Rust Demostrados

| Concepto | Implementación / Garantía |
| :--- | :--- |
| **Shadowing** | Reutilización de identificadores (`nombre`, `consultas`) para transformación de tipos sin mutabilidad. |
| **Strict Type Safety** | Prohibición de conversión implícita de tipos (uso obligatorio de `as` / `.try_into()`). |
| **Static Bounds Checking** | Verificación estática y prevención en compilación de índices fuera de rango en arrays. |
| **Explicit Signatures** | Definición explícita de contratos de tipo en funciones (`fn promedio_uso(...) -> f64`). |
| **Expression-Based Flow** | Evaluación homogénea de tipos en bloques condicionales `if-else`. |

---

## 🖥️ Ejemplo de Ejecución

```text
=== RUST SYS INFO ===
Ingresa tu nombre:
Hugo
---------------------------------
Usuario: Hugo
Consultas acumuladas: 1
Arquitectura: Cores: 4, Frecuencia: 3.2 GHz, 64-bit: true
Uso por núcleo: Core 0: 23%, Core 1: 45%, Core 2: 12%, Core 3: 67%
Promedio de carga total: 36.75% [BAJO]