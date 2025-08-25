use raylib::prelude::*;
use crate::{framebuffer::Framebuffer, maze::Maze, player::Player, textures::TextureManager};

#[derive(Clone, Copy, Debug)]
pub struct Hit {
    pub distance: f32, 
    pub tile: char,    
    pub side: i32,     
    pub wall_x: f32,   
}

#[inline]
fn is_wall_ch(ch: char) -> bool {
    !matches!(ch, ' ' | 'S' | 'E' | 'e')
}

pub fn cast_ray(maze: &Maze, player: &Player, ray_angle: f32) -> Hit {
    let bs = maze.block_size as f32;


    let pos_x = player.pos.x / bs;
    let pos_y = player.pos.y / bs;


    let dir_x = ray_angle.cos();
    let dir_y = ray_angle.sin();


    let mut map_x = pos_x.floor() as i32;
    let mut map_y = pos_y.floor() as i32;


    let delta_x = if dir_x == 0.0 { f32::INFINITY } else { (1.0 / dir_x).abs() };
    let delta_y = if dir_y == 0.0 { f32::INFINITY } else { (1.0 / dir_y).abs() };


    let (step_x, mut side_x) = if dir_x < 0.0 {
        (-1, (pos_x - map_x as f32) * delta_x)
    } else {
        ( 1, (map_x as f32 + 1.0 - pos_x) * delta_x)
    };
    let (step_y, mut side_y) = if dir_y < 0.0 {
        (-1, (pos_y - map_y as f32) * delta_y)
    } else {
        ( 1, (map_y as f32 + 1.0 - pos_y) * delta_y)
    };

    let mut side = 0;            
    let mut hit_tile = '#';


    loop {
        if side_x < side_y {
            side_x += delta_x;
            map_x += step_x;
            side = 0;
        } else {
            side_y += delta_y;
            map_y += step_y;
            side = 1;
        }

        if map_x < 0 || map_y < 0 || (map_x as usize) >= maze.width || (map_y as usize) >= maze.height {
            hit_tile = '#';
            break;
        }
        hit_tile = maze.grid[map_y as usize][map_x as usize];
        if is_wall_ch(hit_tile) { break; }
    }


    let perp_cells = if side == 0 {
        (map_x as f32 - pos_x + (1 - step_x) as f32 * 0.5) / dir_x
    } else {
        (map_y as f32 - pos_y + (1 - step_y) as f32 * 0.5) / dir_y
    }.abs();


    let wall_hit = if side == 0 { pos_y + perp_cells * dir_y } else { pos_x + perp_cells * dir_x };
    let mut wall_x = wall_hit - wall_hit.floor(); 


    if side == 0 && dir_x > 0.0 { wall_x = 1.0 - wall_x; }
    if side == 1 && dir_y < 0.0 { wall_x = 1.0 - wall_x; }

    Hit {
        distance: perp_cells * bs,
        tile: hit_tile,
        side,
        wall_x,
    }
}

pub fn render3d(fb: &mut Framebuffer, maze: &Maze, player: &Player, texman: &TextureManager) {
    let hh = (fb.height / 2) as u32;
    fb.set_current_color(Color::new(60, 80, 140, 255));
    for y in 0..hh { for x in 0..fb.width { fb.set_pixel(x, y); } }

    fb.set_current_color(Color::new(40, 40, 40, 255));
    for y in hh..fb.height { for x in 0..fb.width { fb.set_pixel(x, y); } }

    let dist_to_proj = (fb.width as f32 * 0.5) / (player.fov * 0.5).tan();

    for x in 0..fb.width {

        let t = x as f32 / fb.width as f32;
        let ray_angle = player.a - player.fov * 0.5 + player.fov * t;

        let hit = cast_ray(maze, player, ray_angle);
        let dist = hit.distance.max(1.0);

        let line_h = (maze.block_size as f32 / dist) * dist_to_proj;

        let half = fb.height as f32 * 0.5;
        let mut top = (half - line_h * 0.5) as i32;
        let mut bot = (half + line_h * 0.5) as i32;
        if top < 0 { top = 0; }
        if bot >= fb.height as i32 { bot = fb.height as i32 - 1; }


        if texman.has_image(hit.tile) {
            let (tw, th) = texman.image_size(hit.tile).unwrap_or((128, 128));
            let tx = (hit.wall_x * tw as f32).clamp(0.0, (tw - 1) as f32) as u32;

            for y in top..=bot {
                let dy = y as f32 - (half - line_h * 0.5);
                let ty = ((dy / line_h) * (th - 1) as f32).clamp(0.0, (th - 1) as f32) as u32;

                let mut c = texman.get_pixel_color(hit.tile, tx, ty).unwrap_or(Color::WHITE);
                if hit.side == 1 {
                    c = Color::new((c.r as f32 * 0.7) as u8, (c.g as f32 * 0.7) as u8, (c.b as f32 * 0.7) as u8, c.a);
                }
                fb.set_current_color(c);
                fb.set_pixel(x, y as u32);
            }
        } else {

            let mut c = Color::LIGHTGRAY;
            if hit.side == 1 {
                c = Color::new((c.r as f32 * 0.7) as u8, (c.g as f32 * 0.7) as u8, (c.b as f32 * 0.7) as u8, c.a);
            }
            fb.set_current_color(c);
            for y in top..=bot { fb.set_pixel(x, y as u32); }
        }
    }
}



pub fn draw_minimap(fb: &mut Framebuffer, maze: &Maze, player: &Player) {

    let scale: i32 = 4;
    let map_w = maze.width as i32 * scale;
    let map_h = maze.height as i32 * scale;


    let pad: i32 = 8;
    let origin_x = (fb.width as i32 - map_w - pad).max(0);
    let origin_y = pad;


    for y in 0..map_h {
        for x in 0..map_w {
            fb.set_current_color(Color::new(20, 20, 20, 255));
            fb.set_pixel((origin_x + x) as u32, (origin_y + y) as u32);
        }
    }


    for j in 0..maze.height as i32 {
        for i in 0..maze.width as i32 {
            let ch = maze.grid[j as usize][i as usize];
            let is_wall = !matches!(ch, ' ' | 'S' | 'E' | 'e');
            let color = if is_wall { Color::WHITE } else { Color::DARKGRAY };

            fb.set_current_color(color);
            for yy in 0..scale {
                for xx in 0..scale {
                    let sx = origin_x + i * scale + xx;
                    let sy = origin_y + j * scale + yy;
                    if sx >= 0 && sy >= 0 && sx < fb.width as i32 && sy < fb.height as i32 {
                        fb.set_pixel(sx as u32, sy as u32);
                    }
                }
            }
        }
    }


    let px = origin_x as f32 + (player.pos.x / maze.block_size as f32) * scale as f32;
    let py = origin_y as f32 + (player.pos.y / maze.block_size as f32) * scale as f32;

    fb.set_current_color(Color::YELLOW);
    for dy in -1..=1 {
        for dx in -1..=1 {
            let sx = (px as i32 + dx).clamp(0, fb.width as i32 - 1);
            let sy = (py as i32 + dy).clamp(0, fb.height as i32 - 1);
            fb.set_pixel(sx as u32, sy as u32);
        }
    }


    let len = (6 * scale) as f32;
    let ex = px + player.a.cos() * len;
    let ey = py + player.a.sin() * len;
    draw_line_pixels(fb, px as i32, py as i32, ex as i32, ey as i32, Color::YELLOW);
}


fn draw_line_pixels(
    fb: &mut Framebuffer,
    mut x0: i32,
    mut y0: i32,
    x1: i32,
    y1: i32,
    color: Color,
) {
    fb.set_current_color(color);

    let dx = (x1 - x0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let dy = -(y1 - y0).abs();
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;

    loop {
        if x0 >= 0 && y0 >= 0 && x0 < fb.width as i32 && y0 < fb.height as i32 {
            fb.set_pixel(x0 as u32, y0 as u32);
        }
        if x0 == x1 && y0 == y1 { break; }
        let e2 = 2 * err;
        if e2 >= dy { err += dy; x0 += sx; }
        if e2 <= dx { err += dx; y0 += sy; }
    }
}
