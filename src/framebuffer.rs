use crate::raycaster::{cast_ray, Intersect};
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;
use std::cmp::{max, min};
use nalgebra_glm::Vec3;

use crate::player::Player;


pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<u32>,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Framebuffer {
            width,
            height,
            pixels: vec![0; width * height],
        }
    }

    pub fn clear(&mut self) {
        for pixel in self.pixels.iter_mut() {
            *pixel = 0xFFFFFFFF; // White background
        }
    }

    pub fn point(&mut self, x: usize, y: usize, color: u32) {
        if x < self.width && y < self.height {
            self.pixels[y * self.width + x] = color;
        }
    }

    pub fn set_background_color(&mut self, color: u32) {
        for pixel in self.pixels.iter_mut() {
            *pixel = color;
        }
    }

    pub fn set_foreground_color(&mut self, color: u32) {
        for pixel in self.pixels.iter_mut() {
            *pixel = color;
        }
    }

    pub fn render_fov(&mut self, maze: &[Vec<char>], player: &Player) {
    // Ensure walls are rendered correctly based on the player's perspective
    for ray in 0..self.width {
        // Calculate the ray angle
        let ray_angle = (player.angle - player.fov / 2.0) + (ray as f32 / self.width as f32) * player.fov;

        // Cast the ray and find the wall it hits
        let (distance, wall_hit) = self.cast_ray(player, maze, ray_angle);

        if let Some(wall_char) = wall_hit {
            let color = match wall_char {
                '+' | '-' | '|' => 0xFF000000, // Black for walls
                _ => 0xFFFFFFFF,                // Default color
            };

            // Calculate the height of the wall slice
            let wall_height = (self.height as f32 / distance) as usize;

            // Ensure wall_height is within valid range
            let wall_top = max(0, (self.height as isize - wall_height as isize) / 2) as usize;
            let wall_bottom = min(self.height, wall_top + wall_height);

            for y in wall_top..wall_bottom {
                if ray < self.width {
                    self.point(ray, y, color);
                }
            }
        }
    }
}
pub fn render_fov_with_2d(&mut self, maze: &[Vec<char>], player: &Player, cell_size: usize, cat_positions: &[na::Point3<f32>]) {
    // Render the 3D FOV
    self.clear();
    self.set_background_color(0xFF8ecae6);
    self.render_fov(maze, player);
    let ground_color = 0xFF606c38;
    let ground_start = self.height / 2+25;
    for y in ground_start..self.height {
        for x in 0..self.width {
            self.point(x, y, ground_color);
        }
    }

    // Define the size and position of the 2D map in the corner
    let map_width = maze[0].len() * cell_size;
    let map_height = maze.len() * cell_size;
    let offset_x = self.width - map_width - 10; // 10px padding from the right
    let offset_y = 10; // 10px padding from the top

    // Render the 2D maze in the corner
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
                    let x = offset_x + col * cell_size + dx;
                    let y = offset_y + row * cell_size + dy;
                    if x < self.width && y < self.height {
                        self.point(x, y, color);
                    }
                }
            }
        }
    }

    // Render the player's position on the 2D map
    let player_x = offset_x + ((player.x * cell_size as f32).floor() as usize);
    let player_y = offset_y + ((player.y * cell_size as f32).floor() as usize);

    for dx in 0..cell_size {
        for dy in 0..cell_size {
            let px = player_x + dx;
            let py = player_y + dy;
            if px < self.width && py < self.height {
                self.point(px, py, 0xFF00FF00); // Green for player
            }
        }
    }

    // Render the cat positions as small dots on the 2D map
    let cat_dot_radius = cell_size / 4; // Adjust size of the dot if needed
    for cat_position in cat_positions {
        // Extract x and y coordinates from Point3
        let cat_x = cat_position.x;
        let cat_y = cat_position.y;

        // Calculate cat position on the 2D map
        let cat_x = offset_x + (cat_x * cell_size as f32).floor() as usize;
        let cat_y = offset_y + (cat_y * cell_size as f32).floor() as usize;

        // Draw a dot representing the cat
        for dx in 0..cat_dot_radius {
            for dy in 0..cat_dot_radius {
                let dot_x = cat_x + dx;
                let dot_y = cat_y + dy;
                if dot_x < self.width && dot_y < self.height {
                    self.point(dot_x, dot_y, 0xFFFF00FF); // Purple dot for cats
                }
            }
        }
    }
}


    fn cast_ray(&self, player: &Player, maze: &[Vec<char>], ray_angle: f32) -> (f32, Option<char>) {
        // Calculate the direction of the ray
        let mut ray_x = player.x;
        let mut ray_y = player.y;

        // Normalize the angle
        let mut angle = ray_angle;
        if angle < 0.0 {
            angle += 2.0 * std::f32::consts::PI;
        } else if angle > 2.0 * std::f32::consts::PI {
            angle -= 2.0 * std::f32::consts::PI;
        }

        // Determine the step direction for x and y
        let step_x = angle.cos();
        let step_y = angle.sin();

        // Keep track of the distance the ray has traveled
        let mut distance = 0.0;

        // Maximum distance to prevent infinite loops
        let max_distance = 30.0;

        while distance < max_distance {
            ray_x += step_x * 0.1;
            ray_y += step_y * 0.1;
            distance += 0.1;

            let maze_x = ray_x as usize;
            let maze_y = ray_y as usize;

            // Check if the ray is out of bounds
            if maze_y >= maze.len() || maze_x >= maze[0].len() {
                break;
            }

            // Check if the ray has hit a wall
            match maze[maze_y][maze_x] {
                '+' | '-' | '|' => return (distance, Some(maze[maze_y][maze_x])),
                _ => {}
            }
        }

        // If no wall is hit, return maximum distance
        (max_distance, None)
    }

}     
