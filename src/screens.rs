pub fn show_welcome_screen(frame: &mut [u8], width: u32, height: u32) {
    // Fondo azul oscuro
    for y in 0..height {
        for x in 0..width {
            let idx = ((y * width + x) * 4) as usize;
            frame[idx] = 20;      // R
            frame[idx + 1] = 30;  // G
            frame[idx + 2] = 80;  // B
            frame[idx + 3] = 0xFF; // A
        }
    }
    // Texto: por simplicidad, no dibujamos texto real, solo una "caja" blanca como placeholder
    let box_w = width / 2;
    let box_h = height / 8;
    let start_x = (width - box_w) / 2;
    let start_y = (height - box_h) / 2;
    for y in start_y..(start_y + box_h) {
        for x in start_x..(start_x + box_w) {
            let idx = ((y * width + x) * 4) as usize;
            frame[idx] = 255;
            frame[idx + 1] = 255;
            frame[idx + 2] = 255;
            frame[idx + 3] = 0xFF;
        }
    }
    // Aquí podrías usar una librería de texto para dibujar "Presiona ESPACIO para comenzar"
}

pub fn show_success_screen(frame: &mut [u8], width: u32, height: u32) {
    // Fondo verde
    for y in 0..height {
        for x in 0..width {
            let idx = ((y * width + x) * 4) as usize;
            frame[idx] = 40;      // R
            frame[idx + 1] = 180; // G
            frame[idx + 2] = 80;  // B
            frame[idx + 3] = 0xFF; // A
        }
    }
    // "Mensaje" de éxito (caja blanca como placeholder)
    let box_w = width / 2;
    let box_h = height / 8;
    let start_x = (width - box_w) / 2;
    let start_y = (height - box_h) / 2;
    for y in start_y..(start_y + box_h) {
        for x in start_x..(start_x + box_w) {
            let idx = ((y * width + x) * 4) as usize;
            frame[idx] = 255;
            frame[idx + 1] = 255;
            frame[idx + 2] = 255;
            frame[idx + 3] = 0xFF;
        }
    }
    // Aquí podrías usar una librería de texto para dibujar "¡Felicidades! Llegaste a la meta. Presiona ESPACIO para reiniciar."
} 