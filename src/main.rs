extern crate nalgebra as na;
extern crate nalgebra_glm as glm;
extern crate minifb;

use minifb::{Key, Window, WindowOptions};
use glm::Vec3;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

mod framebuffer;
mod player;
mod raycaster;

use framebuffer::Framebuffer;
use player::Player;
use raycaster::cast_ray;

const CELL_SIZE: usize = 20;
const FOV: f32 = std::f32::consts::PI / 3.0;

fn main() -> Result<(), Box<dyn Error>> {
    let (maze, player_position) = load_maze("maze.txt")?;
    let cell_size = CELL_SIZE;
    let width = maze[0].len() * cell_size;
    let height = maze.len() * cell_size;

    let mut framebuffer = Framebuffer::new(width, height);
    framebuffer.clear();

    let mut player = Player::new(player_position.1 as f32, player_position.0 as f32, FOV);

    render_maze(&mut framebuffer, &maze, cell_size);
    render_player(&mut framebuffer, &player, cell_size);
    render_ray(&mut framebuffer, &maze, &player, cell_size);

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

fn load_maze(filename: &str) -> Result<(Vec<Vec<char>>, (usize, usize)), Box<dyn Error>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut player_position = (0, 0);
    let maze: Vec<Vec<char>> = reader
        .lines()
        .enumerate()
        .map(|(row, line)| {
            let line_chars: Vec<char> = line.unwrap().chars().collect();
            if let Some(col) = line_chars.iter().position(|&c| c == 'p') {
                player_position = (row, col);
            }
            line_chars
        })
        .collect();
    Ok((maze, player_position))
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

fn render_player(framebuffer: &mut Framebuffer, player: &Player, cell_size: usize) {
    let x = (player.x as usize) * cell_size;
    let y = (player.y as usize) * cell_size;
    for dx in 0..cell_size {
        for dy in 0..cell_size {
            framebuffer.point(x + dx, y + dy, 0xFF00FF00);
        }
    }
}

fn render_ray(framebuffer: &mut Framebuffer, maze: &[Vec<char>], player: &Player, cell_size: usize) {
    let distance = cast_ray(maze, player.x, player.y, player.angle);
    let x_end = player.x + distance * player.angle.cos();
    let y_end = player.y + distance * player.angle.sin();

    let x0 = (player.x as usize) * cell_size;
    let y0 = (player.y as usize) * cell_size;
    let x1 = (x_end as usize) * cell_size;
    let y1 = (y_end as usize) * cell_size;

    draw_line(framebuffer, x0, y0, x1, y1, 0xFFFF0000);
}

fn draw_line(framebuffer: &mut Framebuffer, x0: usize, y0: usize, x1: usize, y1: usize, color: u32) {
    let dx = (x1 as isize - x0 as isize).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let dy = -(y1 as isize - y0 as isize).abs();
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;

    let mut x = x0 as isize;
    let mut y = y0 as isize;

    loop {
        framebuffer.point(x as usize, y as usize, color);
        if x == x1 as isize && y == y1 as isize {
            break;
        }
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x += sx;
        }
        if e2 <= dx {
            err += dx;
            y += sy;
        }
    }
}
