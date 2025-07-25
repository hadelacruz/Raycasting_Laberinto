mod player;
mod map;
mod raycaster;
mod render;
mod audio;
mod screens;

use pixels::{Pixels, SurfaceTexture};
use winit::event::{Event, WindowEvent, ElementState, KeyboardInput, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use render::render_3d_basic;

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;
const FOV: f32 = std::f32::consts::FRAC_PI_3; // 60 grados
const NUM_RAYS: u32 = WIDTH;
const MAX_DEPTH: f32 = 16.0;
const MOVE_SPEED: f32 = 0.08;
const ROT_SPEED: f32 = 0.05;

struct GameState {
    player: player::Player,
    map: map::Map,
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
    let mut keys = [false; 4];
    let map = map::Map::new();
    let mut state = GameState {
        player: player::Player::new(map.start.0, map.start.1, 0.0),
        map,
    };

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
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
                        if let Some(VirtualKeyCode::Space) = input.virtual_keycode {
                            if input.state == ElementState::Pressed {
                                // Reiniciar juego
                                state.player.x = state.map.start.0;
                                state.player.y = state.map.start.1;
                                state.player.angle = 0.0;
                                show_success = false;
                                show_welcome = true;
                            }
                        }
                    } else {
                        // Movimiento: W, S, A, D
                        let pressed = input.state == ElementState::Pressed;
                        match input.virtual_keycode {
                            Some(VirtualKeyCode::W) => keys[0] = pressed,
                            Some(VirtualKeyCode::S) => keys[1] = pressed,
                            Some(VirtualKeyCode::A) => keys[2] = pressed,
                            Some(VirtualKeyCode::D) => keys[3] = pressed,
                            _ => {}
                        }
                    }
                }
                _ => {}
            },
            Event::RedrawRequested(_) => {
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
                pixels.render().unwrap();
            }
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            _ => {}
        }
    });
}
