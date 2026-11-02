# Seminario de Lenguajes — Rust

Repositorio académico con las prácticas y entregables realizados en Rust para la Licenciatura en Sistemas de la Universidad Nacional de La Plata (UNLP).

El contenido conserva la organización y las implementaciones originales de los ejercicios. Cada práctica es un proyecto Cargo independiente, con su propio manifiesto, código fuente y archivo de bloqueo de dependencias.

## Estructura

```text
.
├── Práctica 1/     # 12 ejercicios introductorios
├── Práctica 2/     # 15 funciones y sus pruebas unitarias
├── Práctica 3/     # 10 ejercicios de modelado con structs y enums
├── Práctica 4/     # 5 ejercicios sobre traits, lifetimes y sistemas de dominio
├── Práctica 5/     # 6 ejercicios con errores y persistencia JSON
└── Entregables/    # entregas integradoras y sus pruebas
```

Cada carpeta contiene `Cargo.toml`, `Cargo.lock` y `src/`. Las carpetas `target/` se generan localmente al compilar y no se incluyen en el repositorio.

## Contenido de las prácticas

### Práctica 1

Doce programas introductorios que trabajan entrada por consola, operaciones aritméticas y booleanas, representación hexadecimal, tuplas, cadenas, arreglos, recorridos y búsquedas simples. Los archivos contienen funciones `main`, aunque el proyecto actual los reúne como módulos de biblioteca mediante `src/lib.rs`.

### Práctica 2

Quince funciones centradas en paridad y primalidad, sumas y conteos, rangos numéricos, transformación de arreglos, longitudes de cadenas, ordenamiento de nombres, referencias mutables y generación de una serie geométrica. Incluye 50 pruebas unitarias.

### Práctica 3

Diez ejercicios de modelado de datos y comportamiento con estructuras y enumeraciones. Incluye personas, rectángulos, fechas, triángulos, productos y estudiantes; además de sistemas para concesionaria de autos, playlist, veterinaria y biblioteca. Incluye 79 pruebas unitarias.

### Práctica 4

Cinco ejercicios que abarcan un trait para números primos, consultas sobre personas usando referencias y lifetimes, un sistema de suscripciones de streaming, un sistema de ventas con reportes y una plataforma de operaciones con criptomonedas. Incluye 5 pruebas unitarias.

### Práctica 5

Seis implementaciones con manejo de errores mediante `Result` y persistencia en archivos JSON usando `serde` y `serde_json`. Los dominios son concesionaria, playlist, veterinaria, biblioteca, streaming y plataforma de criptomonedas. Incluye 13 pruebas unitarias.

### Entregables integradores

Contiene variantes de dos entregas: una concesionaria que administra autos, calcula precios y agrupa recaudación por color; y un sistema de streaming que gestiona suscripciones, medios de pago y estadísticas por estación y mes. Incluye 24 pruebas unitarias. Hay más detalle en [`Entregables/README.md`](Entregables/README.md).

## Requisitos

- Una instalación de Rust y Cargo compatible con la edición 2024.
- Acceso a crates.io la primera vez que se compile la Práctica 5, para descargar `serde` y `serde_json` si no están en la caché local.

La forma recomendada de instalar y actualizar la herramienta es [rustup](https://rustup.rs/).

## Compilación y pruebas

Cada proyecto puede comprobarse desde la raíz del repositorio indicando su manifiesto:

```bash
cargo check --manifest-path "Práctica 1/Cargo.toml"
cargo test --manifest-path "Práctica 2/Cargo.toml"
cargo test --manifest-path "Práctica 3/Cargo.toml"
cargo test --manifest-path "Práctica 4/Cargo.toml"
cargo test --manifest-path "Práctica 5/Cargo.toml"
cargo test --manifest-path "Entregables/Cargo.toml"
```

También se puede entrar a una carpeta y usar Cargo normalmente:

```bash
cd "Práctica 3"
cargo check
cargo test
```

Los proyectos están organizados como bibliotecas: la ejecución principal consiste en compilar sus módulos y correr sus pruebas con `cargo test`. La Práctica 1 no declara objetivos binarios en `Cargo.toml`, por lo que sus funciones `main` individuales no se ejecutan mediante `cargo run` en la estructura actual.

## Material académico

Los PDF de consignas de las cinco prácticas se conservan junto a cada proyecto. Las diapositivas y los PDF teóricos de clase están excluidos mediante `.gitignore`.

Antes de redistribuir las consignas en un repositorio público, se recomienda confirmar las condiciones de publicación establecidas por la cátedra. El trabajo final de la materia no forma parte de este repositorio y se mantendrá en un repositorio independiente.
