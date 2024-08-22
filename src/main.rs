extern crate image;
extern crate nalgebra as na;
extern crate nalgebra_glm as glm;
extern crate minifb;

use font::draw_char;
use image::GenericImageView;
use minifb::{Key, Window, WindowOptions};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::{Instant, Duration};

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
    PlayingA,
    PlayingB,
    SuccessScreen,
    FailScreen,
}

fn is_colliding_with_wall(maze: &[Vec<char>], x: f32, y: f32, size: f32) -> bool {
    let cell_x_start = (x - size / 2.0) as usize;
    let cell_x_end = (x + size / 2.0) as usize;
    let cell_y_start = (y - size / 2.0) as usize;
    let cell_y_end = (y + size / 2.0) as usize;

    for y in cell_y_start..=cell_y_end {
        for x in cell_x_start..=cell_x_end {
            if x < maze[0].len() && y < maze.len() {
                if maze[y][x] != ' ' { // Assumes walls are any cell not equal to ' '
                    return true;
                }
            }
        }
    }
    false
}

fn is_within_wall(maze: &[Vec<char>], x: f32, y: f32) -> bool {
    let cell_x = x as usize;
    let cell_y = y as usize;

    // Ensure the position is within maze bounds
    if cell_x < maze[0].len() && cell_y < maze.len() {
        // Check if the cell contains a wall (not a space)
        maze[cell_y][cell_x] != ' '
    } else {
        // Out of bounds
        true
    }
}

fn load_level(level: usize) -> Result<(Vec<Vec<char>>, (usize, usize), Vec<u32>, usize, usize, Option<usize>), Box<dyn Error>> {
    match level {
        0 => {
            let (maze, player_position) = load_maze("maze.txt")?;
            let (cat_img, cat_width, cat_height) = load_and_resize_image("image/card.jpeg", 100, 100)?;
            Ok((maze, player_position, cat_img, cat_width, cat_height, None)) // Level A: No bunnies to collect
        }
        1 => {
            let (maze, player_position) = load_maze("maze2.txt")?;
            let (bunny_img, bunny_width, bunny_height) = load_and_resize_image("image/bunny.jpeg", 100, 100)?;
            let bunnies_to_collect = maze.iter().flat_map(|row| row.iter()).filter(|&&c| c == 'b').count(); // Count the bunnies
            Ok((maze, player_position, bunny_img, bunny_width, bunny_height, Some(bunnies_to_collect)))
        }
        _ => Err("Invalid level".into()),
    }
}

// Define a struct to hold the game level data
struct GameLevel {
    maze: Vec<Vec<char>>,
    player_position: (usize, usize),
    cat_img: Vec<u32>,
    cat_width: usize,
    cat_height: usize,
    cat_positions: Vec<na::Point3<f32>>,
    bunnies_to_collect: Option<usize>, // Only used in Level B
}

fn main() -> Result<(), Box<dyn Error>> {
    let player_start_pos = (1.0, 1.0);

    let mut framebuffer = Framebuffer::new(600, 600);
    let mut window = Window::new("Maze", 600, 600, WindowOptions::default())?;
    let mut player = Player::new(player_start_pos.1, player_start_pos.0, 0.0, FOV);
    let mut prev_mouse_x: Option<f32> = None;

    let (start_screen_img, start_screen_width, start_screen_height) = load_and_resize_image("image/alice_start.jpeg", 600, 600)?;
    let (success_screen_img, success_screen_width, success_screen_height) = load_and_resize_image("image/alice_success.jpeg", 600, 600)?;
    let (fail_screen_img, fail_screen_width, fail_screen_height) = load_and_resize_image("image/alice_fail.jpeg", 600, 600)?;
    let (wall_texture_img, wall_texture_width, wall_texture_height) = load_and_resize_image("image/wall.jpeg", 100, 200)?;
    let mut game_state = GameState::StartScreen;
    let mut selected_level = 0;
    let wall_texture = wall_texture_img.to_vec();
    let mut buttons = vec![
        Button::new(100, 400, BUTTON_WIDTH, BUTTON_HEIGHT, "A"),
        Button::new(100, 450, BUTTON_WIDTH, BUTTON_HEIGHT, "B"),
    ];

    let mut selected_button = 0;

    // Store game level data
    let mut game_level: Option<GameLevel> = None;


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
                    selected_level = selected_button;
                    game_state = state;

                    // Load level data only once
                    game_level = match selected_level {
                        0 => {
                            let (maze, player_position) = load_maze("maze.txt")?;
                            let (cat_img, cat_width, cat_height) = load_and_resize_image("image/card.jpeg", 100, 100)?;
                            let cat_positions = vec![
                                na::Point3::new(2.0, 3.0, 0.0),
                                na::Point3::new(5.0, 7.0, 0.0),
                                // Add more positions as needed
                            ];
                            Some(GameLevel {
                                maze,
                                player_position,
                                cat_img,
                                cat_width,
                                cat_height,
                                cat_positions,
                                bunnies_to_collect: None, // No bunnies in Level A
                            })
                        }
                        1 => {
                            let (maze, player_position) = load_maze("maze2.txt")?;
                            let (bunny_img, bunny_width, bunny_height) = load_and_resize_image("image/bunny.jpeg", 100, 100)?;
                            let cat_positions = vec![
                                na::Point3::new(2.0, 3.0, 0.0),
                                na::Point3::new(5.0, 7.0, 0.0),
                                // Add more positions as needed
                            ];
                            let bunnies_to_collect = maze.iter().flat_map(|row| row.iter()).filter(|&&c| c == 'b').count(); // Count the bunnies
                            Some(GameLevel {
                                maze,
                                player_position,
                                cat_img: bunny_img,
                                cat_width: bunny_width,
                                cat_height: bunny_height,
                                cat_positions,
                                bunnies_to_collect: Some(bunnies_to_collect),
                            })
                        }
                        _ => None,
                    };

                    // Initialize player position for the level
                    if let Some(level) = &game_level {
                        player = Player::new(player_start_pos.1, player_start_pos.0, 0.0, FOV);
                    }
                }
            }
            GameState::PlayingA | GameState::PlayingB => {
                if let Some(level) = &game_level {
                    process_events(&window, &mut player, &level.maze, &mut prev_mouse_x);

                    let (next_x, next_y) = (player.x, player.y);

                    if !is_colliding_with_wall(&level.maze, next_x, next_y, 1.0) {
                        player.x = next_x;
                        player.y = next_y;
                    }

                    framebuffer.render_fov_with_2d(&level.maze, &player, CELL_SIZE, &level.cat_positions,&wall_texture, wall_texture_width,wall_texture_height);

                    // Render the cat images in static positions
                    for cat_position in &level.cat_positions {
                        render_cat_in_3d(
                            &mut framebuffer,
                            &level.cat_img,
                            level.cat_width,
                            level.cat_height,
                            cat_position,
                            &player,
                            &level.maze,
                        );
                    }

                    // Check for collision with any cat
                    let player_pos = na::Point2::new(player.x, player.y);
                    for cat_position in &level.cat_positions {
                        let cat_pos = na::Point2::new(cat_position.x, cat_position.y);
                        if (player_pos - cat_pos).magnitude() < 0.5 {
                            game_state = GameState::FailScreen;
                            break;
                        }
                    }

                    // Check for success condition
                    if level.maze[player.y as usize][player.x as usize] == 'g' {
                        game_state = GameState::SuccessScreen;
                    }
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




const MAX_SCALE: f32 = 2.0; // Limit the maximum scale to avoid excessively large sprites

fn project_to_2d(cat_position: &na::Point3<f32>, player: &Player, framebuffer_width: f32, framebuffer_height: f32) -> Option<(usize, usize, f32)> {
    let dx = cat_position.x - player.x;
    let dy = cat_position.y - player.y;

    // Calculate the angle between the player and the cat
    let angle_to_cat = dy.atan2(dx) - player.angle;
    
    // Ensure the angle is within the player's FOV
    if angle_to_cat.abs() > player.fov / 2.0 {
        return None; // Cat is outside the player's FOV, don't render it
    }

    // Calculate distance to the cat
    let distance = (dx.powi(2) + dy.powi(2)).sqrt();

    // Project the position to 2D screen space
    let screen_x = ((angle_to_cat / player.fov) + 0.5) * framebuffer_width;
    let scale = (400.0 / distance).min(MAX_SCALE); // Adjust the scale based on distance for perspective, and clamp

    Some((screen_x as usize, (framebuffer_height / 2.0) as usize, scale))
}

fn render_cat_in_3d(
    framebuffer: &mut Framebuffer, 
    cat_img: &[u32], 
    cat_width: usize, 
    cat_height: usize, 
    cat_position: &na::Point3<f32>, 
    player: &Player, 
    maze: &[Vec<char>]
) {
    let framebuffer_width = framebuffer.width as f32;
    let framebuffer_height = framebuffer.height as f32;

    if let Some((x, y, scale)) = project_to_2d(cat_position, player, framebuffer_width, framebuffer_height) {
        let reduced_scale = scale * 0.5;

        let scaled_width = (cat_width as f32 * reduced_scale).min(framebuffer_width);
        let scaled_height = (cat_height as f32 * reduced_scale).min(framebuffer_height);

        let angle_to_cat = (cat_position.y - player.y).atan2(cat_position.x - player.x);
        let intersection = cast_ray(maze, player.x, player.y, angle_to_cat);

        let distance_to_cat = ((cat_position.x - player.x).powi(2) + (cat_position.y - player.y).powi(2)).sqrt();
        let cat_visible = intersection.distance >= distance_to_cat;

        if !cat_visible {
            return;
        }

        let start_x = x.saturating_sub(scaled_width as usize / 2);
        let start_y = y.saturating_sub(scaled_height as usize / 2);
        let end_x = (start_x + scaled_width as usize).min(framebuffer_width as usize);
        let end_y = (start_y + scaled_height as usize).min(framebuffer_height as usize);

        for dy in 0..scaled_height as usize {
            for dx in 0..scaled_width as usize {
                let pixel_index = (dy * cat_height / scaled_height as usize) * cat_width + (dx * cat_width / scaled_width as usize);
                if pixel_index < cat_img.len() {
                    let pixel = cat_img[pixel_index];
                    if (pixel >> 24) & 0xFF != 0 { // Only draw if the pixel is not fully transparent
                        let dest_x = start_x + dx;
                        let dest_y = start_y + dy;
                        if dest_x < framebuffer_width as usize && dest_y < framebuffer_height as usize {
                            framebuffer.point(dest_x, dest_y, pixel);
                        }
                    }
                }
            }
        }
    }
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

fn render_maze(framebuffer: &mut Framebuffer, maze: &[Vec<char>], cell_size: usize, cat_positions: &[na::Point3<f32>], cat_img: &[u32], cat_width: usize, cat_height: usize) {
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

    // Render cats
    for cat_position in cat_positions {
        let x = (cat_position.x * CELL_SIZE as f32) as usize;
        let y = (cat_position.y * CELL_SIZE as f32) as usize;
        render_image(framebuffer, cat_img, cat_width, cat_height, x, y);
    }
}

fn check_cat_collisions(player: &Player, cat_positions: &[na::Point3<f32>]) -> bool {
    for cat_position in cat_positions {
        if (player.x - cat_position.x).abs() < 0.5 && (player.y - cat_position.y).abs() < 0.5 {
            return true;
        }
    }
    false
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
    if window.is_key_down(Key::P) {
        match *selected_button {
            0 => Some(GameState::PlayingA), // Level 1
            1 => Some(GameState::PlayingB), // Level 2
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
