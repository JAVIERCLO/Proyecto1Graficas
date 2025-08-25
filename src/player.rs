use raylib::prelude::*;
use raylib::consts::MouseButton;
use crate::maze::Maze;

pub struct Player {
    pub pos: Vector2,       
    pub prev_pos: Vector2,  
    pub a: f32,             
    pub move_speed: f32,
    pub rot_speed: f32,
    pub fov: f32,
}

impl Player {
    // Constructor
    pub fn new(pos: Vector2) -> Self {
        Player {
            pos,
            prev_pos: pos,
            a: 0.0,
            move_speed: 180.0,
            rot_speed: 0.7,
            fov: std::f32::consts::PI / 3.0,
        }
    }


    fn fwd(&self) -> Vector2 {
        Vector2::new(self.a.cos(), self.a.sin())
    }


    fn strafe(&self) -> Vector2 {
        Vector2::new(-self.a.sin(), self.a.cos())
    }


    pub fn can_move_to(&self, maze: &Maze, pos: Vector2) -> bool {
        let bs = maze.block_size as f32;
        let cx = (pos.x / bs).floor() as isize;
        let cy = (pos.y / bs).floor() as isize;
        !maze.is_wall(cx, cy)
    }


    pub fn process_input(&mut self, rl: &RaylibHandle, maze: &Maze, dt: f32) {

        self.prev_pos = self.pos;


        if rl.is_key_down(KeyboardKey::KEY_LEFT) {
            self.a -= self.rot_speed * dt;
        }
        if rl.is_key_down(KeyboardKey::KEY_RIGHT) {
            self.a += self.rot_speed * dt;
        }

        let mut v = Vector2::zero();
        if rl.is_key_down(KeyboardKey::KEY_W) { v += self.fwd(); }
        if rl.is_key_down(KeyboardKey::KEY_S) { v -= self.fwd(); }
        if rl.is_key_down(KeyboardKey::KEY_A) { v -= self.strafe(); }
        if rl.is_key_down(KeyboardKey::KEY_D) { v += self.strafe(); }


        if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
            let mouse_delta = rl.get_mouse_delta().x as f32;
            self.a += mouse_delta * self.rot_speed;
        }


        if v.x != 0.0 || v.y != 0.0 {
            let len = (v.x * v.x + v.y * v.y).sqrt();
            let step = self.move_speed * dt;
            let dir = v / len;
            let new_pos = self.pos + dir * step;


            if self.can_move_to(maze, new_pos) {
                self.pos = new_pos;


            }
        }
    }
}
