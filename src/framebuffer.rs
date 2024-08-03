use std::fs::File;
use std::io::{self, Write};
use std::path::Path;
use std::cmp::{max, min};
use nalgebra_glm::Vec3;

pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub background_color: u32,
    pub foreground_color: u32,
    pub pixels: Vec<u32>,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Framebuffer {
        let background_color = 0xFFFFFFFF; // Default background color (white with full opacity)
        let foreground_color = 0xFF000000; // Default foreground color (black with full opacity)
        let pixels = vec![background_color; width * height];
        Framebuffer {
            width,
            height,
            background_color,
            foreground_color,
            pixels,
        }
    }

    pub fn clear(&mut self) {
        self.fill_with_color(self.background_color);
    }

    pub fn point(&mut self, x: usize, y: usize, color: u32) {
        if x < self.width && y < self.height {
            self.pixels[y * self.width + x] = color;
        }
    }

    pub fn set_background_color(&mut self, color: u32) {
        self.background_color = color;
    }

    pub fn set_foreground_color(&mut self, color: u32) {
        self.foreground_color = color;
    }

    fn fill_with_color(&mut self, color: u32) {
        for pixel in self.pixels.iter_mut() {
            *pixel = color;
        }
    }

    pub fn save_to_file(&self, filename: &str) -> io::Result<()> {
        let path = Path::new(filename);
        let file = File::create(&path)?;
        let mut writer = io::BufWriter::new(file);

        // Write BMP file headers
        let file_size = 54 + (self.width * self.height * 3);
        let reserved_bytes: [u8; 4] = [0; 4];
        let data_offset = 54;
        let file_header = [
            b'B', b'M',
            file_size as u8, (file_size >> 8) as u8, (file_size >> 16) as u8, (file_size >> 24) as u8,
            reserved_bytes[0], reserved_bytes[1], reserved_bytes[2], reserved_bytes[3],
            data_offset as u8, (data_offset >> 8) as u8, (data_offset >> 16) as u8, (data_offset >> 24) as u8,
        ];

        writer.write_all(&file_header)?;

        let info_header = [
            40, 0, 0, 0,
            (self.width as i32) as u8, ((self.width as i32) >> 8) as u8, ((self.width as i32) >> 16) as u8, ((self.width as i32) >> 24) as u8,
            (self.height as i32) as u8, ((self.height as i32) >> 8) as u8, ((self.height as i32) >> 16) as u8, ((self.height as i32) >> 24) as u8,
            1, 0,
            24, 0,
            0, 0, 0, 0,
            0, 0, 0, 0,
            0, 0, 0, 0,
            0, 0, 0, 0,
        ];

        writer.write_all(&info_header)?;

        let mut bmp_data = Vec::new();
        for y in (0..self.height).rev() {
            for x in 0..self.width {
                let pixel = self.pixels[y * self.width + x];
                let blue = (pixel & 0xFF) as u8;
                let green = ((pixel >> 8) & 0xFF) as u8;
                let red = ((pixel >> 16) & 0xFF) as u8;
                bmp_data.push(blue);
                bmp_data.push(green);
                bmp_data.push(red);
            }
        }

        writer.write_all(&bmp_data)?;
        writer.flush()?;
        Ok(())
    }

    pub fn flip_vertical(&mut self) {
        let half_height = self.height / 2;
        for y in 0..half_height {
            let swap_line = self.height - 1 - y;
            for x in 0..self.width {
                let top_index = y * self.width + x;
                let bottom_index = swap_line * self.width + x;
                self.pixels.swap(top_index, bottom_index);
            }
        }
    }

    pub fn fill_polygon(&mut self, points: &[Vec3], color: u32) {
        if points.len() < 3 {
            return; // Cannot fill a polygon with less than 3 points
        }

        // Find the bounding box of the polygon
        let mut min_x = self.width as i32;
        let mut max_x = 0;
        let mut min_y = self.height as i32;
        let mut max_y = 0;

        for point in points {
            let x = point.x as i32;
            let y = point.y as i32;
            if x < min_x {
                min_x = x;
            }
            if x > max_x {
                max_x = x;
            }
            if y < min_y {
                min_y = y;
            }
            if y > max_y {
                max_y = y;
            }
        }

        // Scanline fill algorithm
        for y in min_y..=max_y {
            let mut intersections = Vec::new();

            // Find intersections with polygon edges
            for i in 0..points.len() {
                let j = (i + 1) % points.len();
                let point_i = &points[i];
                let point_j = &points[j];

                let (x0, y0) = (point_i.x as i32, point_i.y as i32);
                let (x1, y1) = (point_j.x as i32, point_j.y as i32);

                if (y0 <= y && y < y1) || (y1 <= y && y < y0) {
                    // Calculate intersection x-coordinate
                    let x_intersect = (x0 as f32) + ((y - y0) as f32) * (x1 as f32 - x0 as f32) / ((y1 - y0) as f32);
                    intersections.push(x_intersect);
                }
            }

            // Sort intersections
            intersections.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

            // Fill between intersections
            for i in (0..intersections.len()).step_by(2) {
                let start = max(intersections[i].ceil() as i32, min_x);
                let end = min(intersections[i + 1].floor() as i32, max_x);

                for x in start..=end {
                    self.point(x as usize, y as usize, color);
                }
            }
        }
    }
}
