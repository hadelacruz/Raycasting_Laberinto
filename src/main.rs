mod player;
mod map;
mod raycaster;
mod render;
mod audio;
mod screens;

use pixels::{Pixels, SurfaceTexture};
use winit::event::{Event, WindowEvent, ElementState, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use render::render_3d_basic;
use std::time::{Instant, Duration};

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;
const FOV: f32 = std::f32::consts::FRAC_PI_3;
const NUM_RAYS: u32 = WIDTH;
const MAX_DEPTH: f32 = 16.0;
const MOVE_SPEED: f32 = 0.08;
const ROT_SPEED: f32 = 0.05;
const TARGET_FPS: u64 = 60;
const FRAME_TIME: Duration = Duration::from_nanos(1_000_000_000 / TARGET_FPS);

struct GameState {
    player: player::Player,
    map: map::Map,
    audio_manager: audio::AudioManager,
}

struct FpsCounter {
    frame_count: u32,
    last_time: Instant,
    fps: f32,
    frame_time: f32,
}

impl FpsCounter {
    fn new() -> Self {
        Self {
            frame_count: 0,
            last_time: Instant::now(),
            fps: 0.0,
            frame_time: 0.0,
        }
    }

    fn update(&mut self, frame_time: Duration) {
        self.frame_count += 1;
        self.frame_time = frame_time.as_secs_f32() * 1000.0;
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_time);
        
        if elapsed >= Duration::from_secs(1) {
            self.fps = self.frame_count as f32 / elapsed.as_secs_f32();
            self.frame_count = 0;
            self.last_time = now;
        }
    }

    fn get_fps(&self) -> f32 {
        self.fps
    }

    fn get_frame_time(&self) -> f32 {
        self.frame_time
    }
}

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Laberinto 3D - Humberto de la Cruz")
        .with_inner_size(winit::dpi::LogicalSize::new(WIDTH, HEIGHT))
        .build(&event_loop)
        .unwrap();

    let mut pixels = Pixels::new(WIDTH, HEIGHT, SurfaceTexture::new(WIDTH, HEIGHT, &window)).unwrap();
    let mut show_welcome = true;
    let mut show_success = false;
    let mut show_fps = true;
    let mut keys = [false; 4];
    let map = map::Map::new();
    let mut state = GameState {
        player: player::Player::new(map.start.0, map.start.1, 0.0),
        map,
        audio_manager: audio::AudioManager::new(),
    };
    
    if let Err(e) = state.audio_manager.play_background_music("assets/background_music.mp3") {
        eprintln!("No se pudo reproducir música de fondo: {}", e);
        eprintln!("Asegúrate de tener un archivo 'background_music.mp3' en la carpeta 'assets/'");
    }
    
    let mut fps_counter = FpsCounter::new();
    let mut start_time = Instant::now();
    let mut elapsed_time = 0.0;
    let mut finished_time: Option<f32> = None;

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::KeyboardInput { input, .. } => {
                    if let Some(VirtualKeyCode::Escape) = input.virtual_keycode {
                        if input.state == ElementState::Pressed {
                            *control_flow = ControlFlow::Exit;
                        }
                    }
                    if show_welcome {
                        if let Some(VirtualKeyCode::Space) = input.virtual_keycode {
                            if input.state == ElementState::Pressed {
                                show_welcome = false;
                            }
                        }
                    } else if show_success {
                        if let Some(VirtualKeyCode::Escape) = input.virtual_keycode {

                            if input.state == ElementState::Pressed {
                                // Terminar el juego
                                *control_flow = ControlFlow::Exit;
                            }
                        }
                    } else {
                        // Movimiento: W, S, A, D
                        let pressed = input.state == ElementState::Pressed;
                        match input.virtual_keycode {
                            Some(VirtualKeyCode::W) => {
                                if input.state == ElementState::Pressed && !keys[0] {
                                    state.audio_manager.play_running_loop("assets/step.mp3");
                                }
                                if input.state == ElementState::Released && keys[0] {
                                    state.audio_manager.stop_running_loop();
                                }
                                keys[0] = pressed;
                            },
                            Some(VirtualKeyCode::S) => {
                                if input.state == ElementState::Pressed && !keys[1] {
                                    state.audio_manager.play_running_loop("assets/step.mp3");
                                }
                                if input.state == ElementState::Released && keys[1] {
                                    state.audio_manager.stop_running_loop();
                                }
                                keys[1] = pressed;
                            },
                            Some(VirtualKeyCode::A) => keys[2] = pressed,
                            Some(VirtualKeyCode::D) => keys[3] = pressed,
                            Some(VirtualKeyCode::F) => {
                                if input.state == ElementState::Pressed {
                                    show_fps = !show_fps;
                                }
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            },
            Event::RedrawRequested(_) => {
                let frame_start = Instant::now();
                let frame = pixels.frame_mut();

                if !show_success && !show_welcome {
                elapsed_time = start_time.elapsed().as_secs_f32();
                }
                
                let frame = pixels.frame_mut();
                if show_welcome {
                    screens::show_welcome_screen(frame, WIDTH, HEIGHT);
                } else if show_success {
                    screens::show_success_screen(frame, WIDTH, HEIGHT);
                } else {
                    // Actualizar movimiento
                    let (dx, dy) = (state.player.angle.cos(), state.player.angle.sin());
                    if keys[0] { // W
                        let nx = state.player.x + dx * MOVE_SPEED;
                        let ny = state.player.y + dy * MOVE_SPEED;
                        if !state.map.is_wall(nx as usize, state.player.y as usize) {
                            state.player.x = nx;
                        }
                        if !state.map.is_wall(state.player.x as usize, ny as usize) {
                            state.player.y = ny;
                        }
                    }
                    if keys[1] { // S
                        let nx = state.player.x - dx * MOVE_SPEED;
                        let ny = state.player.y - dy * MOVE_SPEED;
                        if !state.map.is_wall(nx as usize, state.player.y as usize) {
                            state.player.x = nx;
                        }
                        if !state.map.is_wall(state.player.x as usize, ny as usize) {
                            state.player.y = ny;
                        }
                    }
                    if keys[2] { // A
                        state.player.angle -= ROT_SPEED;
                    }
                    if keys[3] { // D
                        state.player.angle += ROT_SPEED;
                    }
                    // Detección de meta
                    let (goal_x, goal_y) = state.map.end;
                    let dist_to_goal = ((state.player.x - goal_x as f32).powi(2) + (state.player.y - goal_y as f32).powi(2)).sqrt();
                    if dist_to_goal < 0.5 {
                        show_success = true;
                    }
                    // Renderizado 3D
                    render_3d_basic(frame, &state);
                }

                fps_counter.update(frame_start.elapsed());
                if show_fps {
                    render_fps_overlay(frame, WIDTH, HEIGHT, fps_counter.get_fps(), fps_counter.get_frame_time());
                    render_timer_overlay(frame, WIDTH, HEIGHT, elapsed_time);
                }
                
                // Mostrar FPS en pantalla
                fps_counter.update(frame_start.elapsed());
                if show_fps {
                    render_fps_overlay(frame, WIDTH, HEIGHT, fps_counter.get_fps(), fps_counter.get_frame_time());
                }
                
                pixels.render().unwrap();
                
                // Control de FPS
                let frame_time = frame_start.elapsed();
                if frame_time < FRAME_TIME {
                    *control_flow = ControlFlow::WaitUntil(
                        std::time::Instant::now() + FRAME_TIME - frame_time
                    );
                } else {
                    *control_flow = ControlFlow::Poll;
                }
            }
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            _ => {}
        }
    });
}

fn render_fps_overlay(frame: &mut [u8], width: u32, height: u32, fps: f32, frame_time: f32) {
    // Dibujar un rectángulo semi-transparente para el FPS
    let overlay_width = 180;
    let overlay_height = 40;
    let x = width - overlay_width - 10;
    let y = 10;
    
    for dy in 0..overlay_height {
        for dx in 0..overlay_width {
            let px = x + dx;
            let py = y + dy;
            if px < width && py < height {
                let idx = ((py * width + px) * 4) as usize;
                frame[idx] = 0;     // R
                frame[idx + 1] = 0; // G
                frame[idx + 2] = 0; // B
                frame[idx + 3] = 128; // A 
            }
        }
    }
    
    // Cargar la fuente para el texto FPS
    if let Ok(font_data) = std::fs::read("assets/DejaVuSans.ttf") {
        if let Some(font) = rusttype::Font::try_from_vec(font_data) {
            let fps_text = format!("FPS: {:.1}", fps);
            let frame_text = format!("Frame: {:.1}ms", frame_time);
            
            let scale = rusttype::Scale::uniform(14.0);
            let v_metrics = font.v_metrics(scale);
            
            let start_x = x as f32 + 5.0;
            let start_y = y as f32 + 15.0;
            
            // Dibujar FPS
            for glyph in font.layout(&fps_text, scale, rusttype::point(start_x, start_y + v_metrics.ascent)) {
                if let Some(bounding_box) = glyph.pixel_bounding_box() {
                    glyph.draw(|gx, gy, v| {
                        let px = gx as i32 + bounding_box.min.x;
                        let py = gy as i32 + bounding_box.min.y;
                        
                        if px >= 0 && px < width as i32 && py >= 0 && py < height as i32 {
                            let idx = ((py as u32 * width + px as u32) * 4) as usize;
                            let alpha = (v * 255.0) as u8;
                            frame[idx] = 255;     // R (blanco)
                            frame[idx + 1] = 255; // G
                            frame[idx + 2] = 255; // B
                            frame[idx + 3] = alpha;
                        }
                    });
                }
            }
            
            // Dibujar Frame time
            let start_y2 = y as f32 + 30.0;
            for glyph in font.layout(&frame_text, scale, rusttype::point(start_x, start_y2 + v_metrics.ascent)) {
                if let Some(bounding_box) = glyph.pixel_bounding_box() {
                    glyph.draw(|gx, gy, v| {
                        let px = gx as i32 + bounding_box.min.x;
                        let py = gy as i32 + bounding_box.min.y;
                        
                        if px >= 0 && px < width as i32 && py >= 0 && py < height as i32 {
                            let idx = ((py as u32 * width + px as u32) * 4) as usize;
                            let alpha = (v * 255.0) as u8;
                            frame[idx] = 255;     // R (blanco)
                            frame[idx + 1] = 255; // G
                            frame[idx + 2] = 255; // B
                            frame[idx + 3] = alpha;
                        }
                    });
                }
            }
        }
    }
}

fn render_timer_overlay(frame: &mut [u8], width: u32, height: u32, elapsed_time: f32) {
    let overlay_width = 180;
    let overlay_height = 20;
    let x = width - overlay_width - 10;
    let y = 55; 

    for dy in 0..overlay_height {
        for dx in 0..overlay_width {
            let px = x + dx;
            let py = y + dy;
            if px < width && py < height {
                let idx = ((py * width + px) * 4) as usize;
                frame[idx] = 0;
                frame[idx + 1] = 0;
                frame[idx + 2] = 0;
                frame[idx + 3] = 128;
            }
        }
    }
    if let Ok(font_data) = std::fs::read("assets/DejaVuSans.ttf") {
        if let Some(font) = rusttype::Font::try_from_vec(font_data) {
            let timer_text = format!("Tiempo: {:.2} s", elapsed_time);
            let scale = rusttype::Scale::uniform(14.0);
            let v_metrics = font.v_metrics(scale);
            let start_x = x as f32 + 5.0;
            let start_y = y as f32 + 15.0;

            for glyph in font.layout(&timer_text, scale, rusttype::point(start_x, start_y + v_metrics.ascent)) {
                if let Some(bounding_box) = glyph.pixel_bounding_box() {
                    glyph.draw(|gx, gy, v| {
                        let px = gx as i32 + bounding_box.min.x;
                        let py = gy as i32 + bounding_box.min.y;
                        if px >= 0 && px < width as i32 && py >= 0 && py < height as i32 {
                            let idx = ((py as u32 * width + px as u32) * 4) as usize;
                            let alpha = (v * 255.0) as u8;
                            frame[idx] = 255;
                            frame[idx + 1] = 255;
                            frame[idx + 2] = 0; 
                            frame[idx + 3] = alpha;
                        }
                    });
                }
            }
        }
    }
}
