use crate::{map::Map, player::Player};

pub fn cast_ray(player_x: f32, player_y: f32, angle: f32, map: &crate::map::Map, max_depth: f32) -> (f32, u8) {
    let mut dist = 0.0;
    let mut hit_wall = false;
    let mut wall_type = 0;
    while dist < max_depth && !hit_wall {
        let rx = player_x + angle.cos() * dist;
        let ry = player_y + angle.sin() * dist;
        let mx = rx as isize;
        let my = ry as isize;
        if mx < 0 || my < 0 || mx >= map.grid[0].len() as isize || my >= map.grid.len() as isize {
            hit_wall = true;
            wall_type = 1;
        } else if map.grid[my as usize][mx as usize] != 0 {
            hit_wall = true;
            wall_type = map.grid[my as usize][mx as usize];
        }
        dist += 0.02;
    }
    (dist, wall_type)
} 