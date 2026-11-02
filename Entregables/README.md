# Entregables integradores

Esta carpeta reúne cuatro archivos entregables, conservados con sus nombres originales y expuestos como módulos desde `src/lib.rs`.

## Contenido

- `Gonzalo_Lofeudo_e1_a.rs` y `Gonzalo_Lofeudo_e1_b.rs`: modelan autos y una concesionaria con capacidad máxima. Permiten agregar, buscar y eliminar autos, calcular el precio de cada vehículo según color, marca y año, y obtener la recaudación agrupada por color.
- `Gonzalo_Lofeudo_e2_v1_a.rs` y `Gonzalo_Lofeudo_e2_v1_b.rs`: modelan usuarios de un sistema de streaming, tipos de suscripción y medios de pago. Permiten crear usuarios, mejorar, empeorar o cancelar suscripciones, consultar estadísticas activas e históricas y determinar la estación con más suscripciones junto con su mes más contratado.

Las variantes `a` y `b` se mantienen separadas tal como fueron incorporadas al repositorio. En total, el proyecto contiene 24 pruebas unitarias.

## Compilar y ejecutar las pruebas

Desde la raíz del repositorio:

```bash
cargo check --manifest-path "Entregables/Cargo.toml"
cargo test --manifest-path "Entregables/Cargo.toml"
```

Desde esta carpeta:

```bash
cargo check
cargo test
```

Este proyecto es una biblioteca y no define un binario para `cargo run`; sus implementaciones se verifican mediante las pruebas unitarias.
