// maze.rs
use std::fs::File;
use std::io::{self, BufRead};

pub struct Maze {
    pub grid: Vec<Vec<char>>,
    pub width: usize,
    pub height: usize,
    pub block_size: usize,
}

impl Maze {
    pub fn from_file(path: &str, block_size: usize) -> io::Result<Self> {
        let mut grid = Vec::new();
        let file = File::open(path)?;
        for line in io::BufReader::new(file).lines() {
            let line = line?;
            grid.push(line.chars().collect::<Vec<char>>());
        }
        let height = grid.len();
        let width = grid[0].len();
        Ok(Maze {
            grid,
            width,
            height,
            block_size,
        })
    }

    pub fn tile(&self, cx: isize, cy: isize) -> char {
        if cx < 0 || cy < 0 || cx >= self.width as isize || cy >= self.height as isize {
            return '#';
        }
        self.grid[cy as usize][cx as usize]
    }

    pub fn is_wall(&self, cx: isize, cy: isize) -> bool {
        let ch = self.tile(cx, cy);
        ch == '#' || ch == '1' 
    }
}
