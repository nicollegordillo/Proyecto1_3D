extern crate nalgebra as na;
extern crate nalgebra_glm as glm;
extern crate minifb;
use std::cmp::{min, max};
use minifb::{Key, Window, WindowOptions};
use glm::Vec3;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

mod framebuffer;
mod player;
mod raycaster;
mod input;

use framebuffer::Framebuffer;
use player::Player;
use raycaster::cast_ray;
use input::process_events;

const CELL_SIZE: usize = 20;
const FOV: f32 = std::f32::consts::PI / 3.0;

fn render_top_down(framebuffer: &mut Framebuffer, maze: &[Vec<char>], player: &Player, cell_size: usize) {
    framebuffer.clear();
    render_maze(framebuffer, maze, cell_size);
    render_player(framebuffer, player, cell_size);
}

fn render_first_person(framebuffer: &mut Framebuffer, maze: &[Vec<char>], player: &Player) {
    framebuffer.clear();
    framebuffer::Framebuffer::render_fov(framebuffer, maze, player);
}

fn main() -> Result<(), Box<dyn Error>> {
    let (maze, player_position) = load_maze("maze.txt")?;
    let cell_size = 50;  // Example value, adjust as needed
    let width = maze[0].len() * cell_size;
    let height = maze.len() * cell_size;

    let mut framebuffer = Framebuffer::new(width, height);
    framebuffer.clear();

    let mut player = Player::new(player_position.1 as f32, player_position.0 as f32, FOV); // Example FOV value, adjust as needed

    let mut window = Window::new(
        "Maze",
        width,
        height,
        WindowOptions {
            scale: minifb::Scale::X2,
            ..WindowOptions::default()
        },
    )?;

    let mut top_down_view = true;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        process_events(&window, &mut player, &maze);

        if window.is_key_pressed(Key::Tab, minifb::KeyRepeat::No) {
            top_down_view = !top_down_view;
        }

        if top_down_view {
            render_top_down(&mut framebuffer, &maze, &player, cell_size);
        } else {
            render_first_person(&mut framebuffer, &maze, &player);
        }

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

pub fn render_3D(framebuffer: &mut Framebuffer, maze: &[Vec<char>], player: &Player) {
    let num_rays = framebuffer.width;

    let hw = framebuffer.width as f32 / 2.0; // precalculated half width
    let hh = framebuffer.height as f32 / 2.0; // precalculated half height
    framebuffer.set_background_color(0x0c0b38);
    framebuffer.set_foreground_color(0xebdc7f);

    for i in 0..num_rays {
        let current_ray = i as f32 / num_rays as f32; // current ray divided by total rays
        let a = player.angle - (player.fov / 2.0) + (player.fov * current_ray);
        let intersect = cast_ray(maze, player.x, player.y, a);

        let stake_height = framebuffer.height as f32 / intersect.distance;
        let stake_top = (hh - (stake_height / 2.0)).max(0.0) as usize;
        let stake_bottom = (hh + (stake_height / 2.0)).min(framebuffer.height as f32) as usize;

        for y in stake_top..stake_bottom {
            if i < framebuffer.width as usize && y < framebuffer.height as usize {
                framebuffer.point(i, y, 0xebdc7f);
            } else {
                println!("Point out of bounds: i = {}, y = {}", i, y);
            }
        }
    }
}
