use rusttype::{Font, Scale, point};
use std::fs;
use image::{self, GenericImageView, Pixel};

fn draw_multiline_text_centered(
    frame: &mut [u8],
    font: &Font,
    lines: &[&str],
    scale: Scale,
    width: u32,
    height: u32,
    color: [u8; 3],
) {
    let v_metrics = font.v_metrics(scale);
    let line_height = v_metrics.ascent - v_metrics.descent + v_metrics.line_gap;
    let total_text_height = line_height * lines.len() as f32;
    let mut y = (height as f32 - total_text_height) / 2.0 + v_metrics.ascent;

    for line in lines {
        let glyphs: Vec<_> = font.layout(line, scale, point(0.0, 0.0)).collect();
        let text_width: f32 = glyphs.iter().map(|g| g.unpositioned().h_metrics().advance_width).sum();
        let x = (width as f32 - text_width) / 2.0;
        for glyph in font.layout(line, scale, point(x, y)) {
            if let Some(bb) = glyph.pixel_bounding_box() {
                glyph.draw(|gx, gy, v| {
                    let px = gx as i32 + bb.min.x;
                    let py = gy as i32 + bb.min.y;
                    if px >= 0 && px < width as i32 && py >= 0 && py < height as i32 {
                        let idx = ((py as u32 * width + px as u32) * 4) as usize;
                        let alpha = (v * 255.0) as u8;
                        frame[idx] = color[0];
                        frame[idx + 1] = color[1];
                        frame[idx + 2] = color[2];
                        frame[idx + 3] = alpha;
                    }
                });
            }
        }
        y += line_height;
    }
}

fn draw_image_centered(
    frame: &mut [u8],
    image_data: &image::DynamicImage,
    width: u32,
    height: u32,
    target_width: u32,
    target_height: u32,
) {
    // Redimensionar la imagen al tamaño deseado
    let resized = image_data.resize(target_width, target_height, image::imageops::FilterType::Lanczos3);
    
    // Calcular posición para centrar la imagen
    let x_offset = (width - target_width) / 2;
    let y_offset = (height - target_height) / 2;
    
    // Dibujar la imagen en el frame
    for y in 0..target_height {
        for x in 0..target_width {
            let pixel = resized.get_pixel(x, y);
            let rgba = pixel.to_rgba();
            
            let frame_x = x_offset + x;
            let frame_y = y_offset + y;
            
            if frame_x < width && frame_y < height {
                let idx = ((frame_y * width + frame_x) * 4) as usize;
                frame[idx] = rgba[0];     // R
                frame[idx + 1] = rgba[1]; // G
                frame[idx + 2] = rgba[2]; // B
                frame[idx + 3] = rgba[3]; // A
            }
        }
    }
}

// pub fn show_welcome_screen(frame: &mut [u8], width: u32, height: u32) {
//     // Fondo azul oscuro
//     for y in 0..height {
//         for x in 0..width {
//             let idx = ((y * width + x) * 4) as usize;
//             frame[idx] = 20;      // R
//             frame[idx + 1] = 30;  // G
//             frame[idx + 2] = 80;  // B
//             frame[idx + 3] = 0xFF; // A
//         }
//     }

//     // Cargar la fuente
//     let font_data = fs::read("assets/DejaVuSans.ttf").expect("Error al leer la fuente");
//     let font = Font::try_from_vec(font_data).expect("Error al cargar la fuente");

//     // Texto de bienvenida y de instrucciones
//     let lines = [
//         "Bienvenido al laberinto 3D",
//         "",
//         "Usa W, A, S, D para moverte",
//         "Llega a la meta azul en el minimapa",
//         "Presiona F para alternar FPS",
//         "Presiona ESPACIO para comenzar",
//     ];
//     let scale = Scale::uniform(32.0);
//     draw_multiline_text_centered(frame, &font, &lines, scale, width, height, [255, 255, 255]);
// }

pub fn show_welcome_screen(frame: &mut [u8], width: u32, height: u32) {

    for y in 0..height {
        for x in 0..width {
            let idx = ((y * width + x) * 4) as usize;
            frame[idx] = 0;
            frame[idx + 1] = 0;
            frame[idx + 2] = 0;
            frame[idx + 3] = 0xFF;
        }
    }

    // Intentar cargar y mostrar la imagen
    if let Ok(image_data) = image::open("assets/welcome.png") {
        // Obtener dimensiones originales de la imagen
        let (img_w, img_h) = image_data.dimensions();
        let aspect_ratio = img_w as f32 / img_h as f32;

        // Calcular dimensiones para que la imagen encaje en la ventana manteniendo la relación de aspecto
        let (target_width, target_height) = if width as f32 / height as f32 > aspect_ratio {
            let h = height;
            let w = (aspect_ratio * h as f32).round() as u32;
            (w, h)
        } else {
            let w = width;
            let h = (w as f32 / aspect_ratio).round() as u32;
            (w, h)
        };

        // Dibujar la imagen centrada y redimensionada
        draw_image_centered(frame, &image_data, width, height, target_width, target_height);
    }
}

pub fn show_success_screen(frame: &mut [u8], width: u32, height: u32) {

    for y in 0..height {
        for x in 0..width {
            let idx = ((y * width + x) * 4) as usize;
            frame[idx] = 0;
            frame[idx + 1] = 0;
            frame[idx + 2] = 0;
            frame[idx + 3] = 0xFF;
        }
    }

    // Intentar cargar y mostrar la imagen
    if let Ok(image_data) = image::open("assets/success_image.png") {
        // Obtener dimensiones originales de la imagen
        let (img_w, img_h) = image_data.dimensions();
        let aspect_ratio = img_w as f32 / img_h as f32;

        // Calcular dimensiones para que la imagen encaje en la ventana manteniendo la relación de aspecto
        let (target_width, target_height) = if width as f32 / height as f32 > aspect_ratio {
            let h = height;
            let w = (aspect_ratio * h as f32).round() as u32;
            (w, h)
        } else {
            let w = width;
            let h = (w as f32 / aspect_ratio).round() as u32;
            (w, h)
        };

        // Dibujar la imagen centrada y redimensionada
        draw_image_centered(frame, &image_data, width, height, target_width, target_height);
    }
}
 