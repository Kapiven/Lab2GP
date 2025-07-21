# Lab2GP

# Conway's Game of Life — Rust Implementation

Este proyecto es una implementación del juego de la vida de Conway utilizando **Rust** y la librería gráfica `minifb` para el manejo de un framebuffer pixelado.

## Resultados

Hay un gif en el repositorio :D
![Game of Life](juego.gif)
<img width="392" height="848" alt="image" src="https://github.com/user-attachments/assets/f6a9ce99-2f38-4f3a-ab78-849ad8e6633d" />


## ¿Qué es el Juego de la Vida?

El Juego de la Vida es un autómata celular desarrollado por John Conway. No requiere intervención del usuario: es un "juego de cero jugadores", donde las celdas evolucionan automáticamente según reglas simples.

Cada celda del mundo puede estar:
- **Viva** (color blanco)
- **Muerta** (color negro)

### Reglas por cada generación (frame):
1. Cualquier celda viva con menos de 2 vecinos vivos muere (soledad).
2. Cualquier celda viva con 2 o 3 vecinos vive.
3. Cualquier celda viva con más de 3 vecinos muere (sobrepoblación).
4. Cualquier celda muerta con exactamente 3 vecinos vive (reproducción).

---

## Características

- Algoritmo completo del Juego de la Vida.
- Framebuffer optimizado con píxeles escalados.
- Múltiples patrones clásicos: glider, blinker, bloque.
- Animación fluida con `minifb`.
- Diseño extensible para agregar más organismos.

---

## Requisitos

- Rust ≥ 1.60
- Cargo (gestor de paquetes de Rust)

---

## Instalación y ejecución

1. Clona el repositorio:

   ```bash
   git clone https://github.com/Kapiven/Lab2GP.git
   cd game_of_life
   ```

2. compila y ejecuta:
    cargo run

## Autora

- Karen Pineda 

