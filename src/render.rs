use crate::{WIDTH, HEIGHT, FOV, NUM_RAYS, MAX_DEPTH, GameState};
use crate::raycaster::cast_ray;

pub fn render_3d_basic(frame: &mut [u8], state: &GameState) {
    // Fondo cielo y piso
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let idx = ((y * WIDTH + x) * 4) as usize;
            if y < HEIGHT / 2 {
                frame[idx] = 120; frame[idx+1] = 180; frame[idx+2] = 255; // cielo
            } else {
                frame[idx] = 218; frame[idx+1] = 219; frame[idx+2] = 195; // piso
            }
            frame[idx+3] = 0xFF;
        }
    }
    // Raycasting
    for ray in 0..NUM_RAYS {
        let ray_angle = state.player.angle - FOV/2.0 + FOV * (ray as f32) / (NUM_RAYS as f32);
        let (dist, wall_type) = cast_ray(state.player.x, state.player.y, ray_angle, &state.map, MAX_DEPTH);
        let dist = dist * (state.player.angle - ray_angle).cos(); // corrección de distorsión
        let wall_height = (HEIGHT as f32 / dist).min(HEIGHT as f32);
        let start = ((HEIGHT as f32 - wall_height) / 2.0) as usize;
        let end = ((HEIGHT as f32 + wall_height) / 2.0) as usize;
        let color = match wall_type {
            1 => [0, 82, 110],   // pared tipo 1
            2 => [50, 200, 50],   // pared tipo 2
            3 => [50, 50, 200],   // pared tipo 3
            _ => [180, 180, 180], // otro
        };
        for y in start..end.min(HEIGHT as usize) {
            let idx = ((y as u32 * WIDTH + ray) * 4) as usize;
            frame[idx] = color[0];
            frame[idx+1] = color[1];
            frame[idx+2] = color[2];
            frame[idx+3] = 0xFF;
        }
    }
    // Renderizar minimapa
    render_minimap(frame, state);
}

pub fn render_minimap(frame: &mut [u8], state: &GameState) {
    let map_scale = 6;
    let offset_x = 10;
    let offset_y = 10;
    let map_h = state.map.grid.len();
    let map_w = state.map.grid[0].len();
    // Dibujar el mapa
    for y in 0..map_h {
        for x in 0..map_w {
            let color = match state.map.grid[y][x] {
                1 => [39, 75, 92],
                2 => [50, 200, 50],
                3 => [50, 50, 200],
                _ => [40, 40, 40],
            };
            for dy in 0..map_scale {
                for dx in 0..map_scale {
                    let px = offset_x + x * map_scale + dx;
                    let py = offset_y + y * map_scale + dy;
                    if px < WIDTH as usize && py < HEIGHT as usize {
                        let idx = ((py as u32 * WIDTH + px as u32) * 4) as usize;
                        frame[idx] = color[0];
                        frame[idx+1] = color[1];
                        frame[idx+2] = color[2];
                        frame[idx+3] = 0xFF;
                    }
                }
            }
        }
    }
    // Dibujar la meta
    let (goal_x, goal_y) = state.map.end;
    let px = offset_x + goal_x * map_scale;
    let py = offset_y + goal_y * map_scale;
    for dy in 0..map_scale {
        for dx in 0..map_scale {
            let x = px + dx;
            let y = py + dy;
            if x < WIDTH as usize && y < HEIGHT as usize {
                let idx = ((y as u32 * WIDTH + x as u32) * 4) as usize;
                frame[idx] = 80;
                frame[idx+1] = 200;
                frame[idx+2] = 255;
                frame[idx+3] = 0xFF;
            }
        }
    }
    // Dibujar al jugador
    let px = offset_x + (state.player.x * map_scale as f32) as usize;
    let py = offset_y + (state.player.y * map_scale as f32) as usize;
    for dy in 0..map_scale {
        for dx in 0..map_scale {
            let x = px + dx;
            let y = py + dy;
            if x < WIDTH as usize && y < HEIGHT as usize {
                let idx = ((y as u32 * WIDTH + x as u32) * 4) as usize;
                frame[idx] = 255;
                frame[idx+1] = 255;
                frame[idx+2] = 0;
                frame[idx+3] = 0xFF;
            }
        }
    }
    // Dibujar la dirección del jugador
    let dir_len = (map_scale * 2) as f32;
    for i in 0..dir_len as usize {
        let x = px as f32 + state.player.angle.cos() * i as f32;
        let y = py as f32 + state.player.angle.sin() * i as f32;
        let x = x as usize;
        let y = y as usize;
        if x < WIDTH as usize && y < HEIGHT as usize {
            let idx = ((y as u32 * WIDTH + x as u32) * 4) as usize;
            frame[idx] = 255;
            frame[idx+1] = 255;
            frame[idx+2] = 255;
            frame[idx+3] = 0xFF;
        }
    }
} 