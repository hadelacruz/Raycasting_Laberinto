# Laberinto 3D RayCasting

Este proyecto es un **juego de laberinto 3D** hecho en Rust usando la técnica de **RayCasting** (como los primeros shooters tipo Wolfenstein 3D).  
El objetivo es recorrer el laberinto en primera persona, llegar a la meta azul y ver cuánto tiempo tardas.

- Usa las teclas **W, A, S, D** para moverte.
- Presiona **F** para mostrar/ocultar el contador de FPS.
- El juego tiene música de fondo y efectos de sonido.
- Al llegar a la meta, se muestra tu tiempo final y puedes salir con **ESC**.

¡Diviértete

## ¿Cómo probarlo?

1. Instala [Rust](https://www.rust-lang.org/tools/install).
2. Coloca los archivos de audio e imágenes requeridos en la carpeta `assets/` (por ejemplo, `background_music.mp3`, `step.mp3`, `DejaVuSans.ttf`).
3. En la terminal, ejecuta:

```sh
   cargo run --release
```

## Conceptos usados
- RayCasting: Para simular la perspectiva 3D y detectar paredes.
- Buffers de píxeles: Dibujo manual de cada frame.
- Eventos de teclado: Para controlar el movimiento y la interacción.
- Audio: Música de fondo y efectos de sonido.
- Renderizado de texto: Para mostrar instrucciones, FPS y temporizador.
- Minimapa: Vista superior del laberinto y posición del jugador.