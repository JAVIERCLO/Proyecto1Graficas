use raylib::color::Color;

mod player;
mod maze;
mod framebuffer;

 pub struct Intersect {
    pub distance: f32,
    pub impact: char
}

pub fn cast_ray(
    framebuffer: &mut Framebuffer,
    maze: &Maze,
    player: &Player,
    a: f32,
    block_size: usize,
    draw_line: bool,
) -> Intersect {
    let mut d = 0.0;

    framebuffer.set_current_color(Color::WHITESMOKE);

    loop {
        let cos = d * a.cos();
        let sin = d * a.sin();
        let x = (player.pos.x + cos) as usize;
        let y = (player.pos.y + sin) as usize;

        let i = x / block_size;
        let j = y & block_size;

        if maze[j][i] != ' ' {
            return Intersect{
                distance: d,
                impact: maze[j][i]
            };
        }

        if draw_line {
            framebuffer.set_pixel(x as i32, y as i32)
        }

        d +=  10.0;

        let stake_top = (hh - (stake_height / 2.0)) as usize;
        let stake_bottom = (hh + (stake_height / 2.0)) as usize;

        for y in stake_top..stake_bottom {
            framebuffer.point(i,y);
        }


    }

}

            fn render3d(framebuffer: &mut Framebuffer, player: &Player) {
            let maze = load_maze("./maze.txt");
            let block_size = 100;
            let num_rays = framebuffer.width;

            let hw = framebuffer.width as f32 / 2.0;
            let hh = framebuffer.height as f32 / 2.0;

            framebuffer.set_current_color(0xFFFFFF);

            for i in 0..num_rays {
                let current_ray = i as f32 / num_rays as f32;
                let a = player.a - (player.fov / 2.0) + (player.fov * current_ray);
                let intersect = cast_ray(framebuffer, &maze, &player, a, block_size, false);

                let distance_to_wall = 0.1;
                let distance_to_projection_plane = 0.1;

                let stake_height = (hh / distance_to_wall) * distance_to_projection_plane;

                let stake_top = (hh - (stake_height / 2.0)) as usize;
                let stake_bottom = (hh + (stake_height / 2.0)) as usize;

                for y in stake_top..stake_bottom {
                framebuffer.point(i,y);
        }
    }
}