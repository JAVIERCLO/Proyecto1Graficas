mod framebuffer;
mod maze;
mod player;
mod caster;
mod textures;

use raylib:: prelude::*;
use framebuffer::Framebuffer;
use maze::Maze;
use player::Player;
use caster::{render3d, draw_minimap};
use textures::TextureManager;

fn main() {
    let (mut rl, th) = raylib::init()
        .size(800, 600)
        .title("Nivel 1")
        .vsync()
        .build();

        rl.set_target_fps(20);

    // Cargar el mapa
    let maze = match Maze::from_file("assets/levels/level1.txt", 100) {
        Ok(m) => m,
        Err(e) => { eprintln!("Error al cargar el mapa: {e}"); return; }
    };


    let mut player = Player::new(Vector2::new(150.0, 150.0));


    let texman = TextureManager::new(&mut rl, &th);


    let mut fb = Framebuffer::new(800, 600, Color::BLACK);


    while !rl.window_should_close() {
        // Procesar entradas
        player.process_input(&rl, &maze, 1.0 / 20.0);


        fb.clear();


        render3d(&mut fb, &maze, &player, &texman);


        draw_minimap(&mut fb, &maze, &player);


        fb.swap_buffers(&mut rl, &th);

    }
}
