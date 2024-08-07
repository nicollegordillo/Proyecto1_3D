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
        let num_rays = self.width;
        let half_fov = player.fov / 2.0;
        let angle_step = player.fov / num_rays as f32;

        for i in 0..num_rays {
            let ray_angle = player.angle - half_fov + i as f32 * angle_step;
            let intersect = cast_ray(maze, player.x, player.y, ray_angle);
            let distance = intersect.distance * (ray_angle - player.angle).cos(); // Correct distance for fish-eye effect
            let wall_height = (self.height as f32 / distance) as usize;

            let start = self.height.saturating_sub(wall_height) / 2;
            let end = min(start + wall_height, self.height);

            for y in start..end {
                self.point(i, y, 0xFF0000FF); // Example color, change as needed
            }
        }
    }
}
