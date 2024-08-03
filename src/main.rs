extern crate nalgebra as na;
extern crate nalgebra_glm as glm;
extern crate minifb;

use minifb::{Key, Window, WindowOptions};
use glm::Vec3;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

mod framebuffer;

use framebuffer::Framebuffer;

const CELL_SIZE: usize = 20;

fn main() -> Result<(), Box<dyn Error>> {
    let maze = load_maze("maze.txt")?;
    let cell_size = CELL_SIZE;
    let width = maze[0].len() * cell_size;
    let height = maze.len() * cell_size;

    let mut framebuffer = Framebuffer::new(width, height);
    framebuffer.clear();
    render_maze(&mut framebuffer, &maze, cell_size);

    let mut window = Window::new(
        "Maze",
        width,
        height,
        WindowOptions {
            scale: minifb::Scale::X2,
            ..WindowOptions::default()
        },
    )?;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&framebuffer.pixels, width, height)?;
    }

    Ok(())
}

fn load_maze(filename: &str) -> Result<Vec<Vec<char>>, Box<dyn Error>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let maze: Vec<Vec<char>> = reader
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();
    Ok(maze)
}

fn render_maze(framebuffer: &mut Framebuffer, maze: &[Vec<char>], cell_size: usize) {
    for (row, line) in maze.iter().enumerate() {
        for (col, &cell) in line.iter().enumerate() {
            let color = match cell {
                '+' | '-' | '|' => 0xFF000000, // Black for walls
                'p' => 0xFF00FF00,              // Green for player
                'g' => 0xFFFF0000,              // Red for goal
                _ => 0xFFFFFFFF,                // White for empty space
            };

            for dx in 0..cell_size {
                for dy in 0..cell_size {
                    framebuffer.point(col * cell_size + dx, row * cell_size + dy, color);
                }
            }
        }
    }
}