extern crate image;
extern crate nalgebra as na;
extern crate nalgebra_glm as glm;
extern crate minifb;

use image::GenericImageView;
use minifb::{Key, Window, WindowOptions};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

mod framebuffer;
mod input;
mod player;
mod raycaster;
mod button;
mod font;

use button::Button;
use framebuffer::Framebuffer;
use input::process_events;
use player::Player;
use raycaster::cast_ray;

const CELL_SIZE: usize = 20;
const FOV: f32 = std::f32::consts::PI / 3.0;

const BUTTON_WIDTH: usize = 100;
const BUTTON_HEIGHT: usize = 30;

#[derive(PartialEq)]
enum GameState {
    StartScreen,
    Playing,
    SuccessScreen,
    FailScreen,
}

fn main() -> Result<(), Box<dyn Error>> {
    let (maze, player_position) = load_maze("maze.txt")?;
    let mut framebuffer = Framebuffer::new(800, 800);
    let mut window = Window::new("Maze", 800, 800, WindowOptions::default())?;
    let player_start_pos = (5.0, 5.0);
    let mut player = Player::new(player_start_pos.1, player_start_pos.0, FOV);
    let mut prev_mouse_x: Option<f32> = None;

   let (start_screen_img, start_screen_width, start_screen_height) = load_and_resize_image("image/start_screen.jpeg", 800, 800)?;
   let (success_screen_img, success_screen_width, success_screen_height) = load_and_resize_image("image/success_screen.jpeg", 800, 800)?;
   let (fail_screen_img, fail_screen_width, fail_screen_height) = load_and_resize_image("image/fail_screen.jpeg", 800, 800)?;


   let (cat_img, cat_width, cat_height) = load_and_resize_image("image/cat.png", 50, 50)?; // Adjust size as needed
   let mut cat_position = (5.0, 20.0);

   let mut game_state = GameState::StartScreen;
   let mut selected_level = 0;

   let mut buttons = vec![
        Button::new(250, 200, BUTTON_WIDTH, BUTTON_HEIGHT, "A"),
        Button::new(250, 250, BUTTON_WIDTH, BUTTON_HEIGHT, "B"),
    ];

    let mut selected_button = 0;

while window.is_open() && !window.is_key_down(Key::Escape) {
    match game_state {
        GameState::StartScreen => {
            framebuffer.clear();
            render_image(&mut framebuffer, &start_screen_img, start_screen_width, start_screen_height, 0, 0);

            for (i, button) in buttons.iter_mut().enumerate() {
                button.is_selected = i == selected_button;
                button.draw(&mut framebuffer);
            }

            if let Some(state) = process_start_screen_input(&window, &mut selected_button) {
                game_state = state;
                if game_state == GameState::Playing {
                    player = Player::new(player_position.1 as f32, player_position.0 as f32, FOV);
                }
            }
        }
        GameState::Playing => {
            process_events(&window, &mut player, &maze, &mut prev_mouse_x);
            framebuffer.render_fov_with_2d(&maze, &player, CELL_SIZE);

            // Update the cat's position to follow the player
            cat_position = (
                cat_position.0 + 0.05 * (player.x - cat_position.0),
                cat_position.1 + 0.05 * (player.y - cat_position.1),
            );

            // Render the cat image
            render_image(
                &mut framebuffer,
                &cat_img,
                cat_width,
                cat_height,
                (cat_position.0 * CELL_SIZE as f32) as usize,
                (cat_position.1 * CELL_SIZE as f32) as usize,
            );

            // Check for collision with the cat
            if (player.x - cat_position.0).abs() < 0.5 && (player.y - cat_position.1).abs() < 0.5 {
                game_state = GameState::FailScreen;
            }

            if maze[player.y as usize][player.x as usize] == 'g' {
                game_state = GameState::SuccessScreen;
            }
        }
        GameState::SuccessScreen => {
            framebuffer.clear();
            render_image(&mut framebuffer, &success_screen_img, success_screen_width, success_screen_height, 0, 0);
            if window.is_key_down(Key::Enter) {
                game_state = GameState::StartScreen;
            }
        }
        GameState::FailScreen => {
            framebuffer.clear();
            render_image(&mut framebuffer, &fail_screen_img, fail_screen_width, fail_screen_height, 0, 0);
            if window.is_key_down(Key::Enter) {
                game_state = GameState::StartScreen;
            }
        }
    }

    window.update_with_buffer(&framebuffer.pixels, framebuffer.width, framebuffer.height)?;
}



    Ok(())
}

fn load_and_resize_image(path: &str, new_width: usize, new_height: usize) -> Result<(Vec<u32>, usize, usize), Box<dyn Error>> {
    let img = image::open(path)?;
    let img = img.resize(new_width as u32, new_height as u32, image::imageops::FilterType::Nearest);
    let img = img.to_rgba8();
    let pixels = img
        .pixels()
        .map(|p| {
            let rgba = p.0;
            ((rgba[0] as u32) << 16) | ((rgba[1] as u32) << 8) | (rgba[2] as u32) | ((rgba[3] as u32) << 24)
        })
        .collect();
    Ok((pixels, new_width, new_height))
}



fn render_image(framebuffer: &mut Framebuffer, image: &[u32], image_width: usize, image_height: usize, x_offset: usize, y_offset: usize) {
    let framebuffer_width = framebuffer.width;
    let framebuffer_height = framebuffer.height;

    for y in 0..image_height {
        for x in 0..image_width {
            let pixel_index = y * image_width + x;
            if pixel_index < image.len() {
                let pixel = image[pixel_index];
                let dest_x = x_offset + x;
                let dest_y = y_offset + y;
                if dest_x < framebuffer_width && dest_y < framebuffer_height {
                    framebuffer.point(dest_x, dest_y, pixel);
                }
            }
        }
    }
}




// Implement load_maze, render_maze, render_player, etc.


fn load_maze(filename: &str) -> Result<(Vec<Vec<char>>, (usize, usize)), Box<dyn Error>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    
    let maze: Vec<Vec<char>> = reader
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();

    // No need to find the player's position in the maze file
    Ok((maze, (1, 1))) // Just return a dummy player position since it's manually set
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

fn process_start_screen_input(window: &Window, selected_button: &mut usize) -> Option<GameState> {
    if window.is_key_down(Key::Down) {
        *selected_button = (*selected_button + 1) % 2; // Assuming 2 buttons
        std::thread::sleep(std::time::Duration::from_millis(150)); // Simple debounce
    }
    if window.is_key_down(Key::Up) {
        *selected_button = (*selected_button + 1 + 2 - 1) % 2; // Wrap around
        std::thread::sleep(std::time::Duration::from_millis(150)); // Simple debounce
    }
    if window.is_key_down(Key::Enter) {
        match *selected_button {
            0 => Some(GameState::Playing), // Level 1
            1 => Some(GameState::Playing), // Level 2
            _ => None,
        }
    } else {
        None
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
