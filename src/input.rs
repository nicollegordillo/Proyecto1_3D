extern crate minifb;
use minifb::{Key, Window};
use crate::player::Player;

const MOVE_SPEED: f32 = 0.1;
const TURN_SPEED: f32 = std::f32::consts::PI / 30.0;

pub fn process_events(window: &Window, player: &mut Player, maze: &[Vec<char>], prev_mouse_x: &mut Option<f32>) {
    let move_speed = 0.1; // Adjust the speed to suit your game
    let turn_speed = 0.05;

    if window.is_key_down(Key::W) {
        player.move_forward(move_speed, maze);
    }

    if window.is_key_down(Key::S) {
        player.move_backward(move_speed, maze);
    }

    if window.is_key_down(Key::A) {
        player.turn_left(turn_speed);
    }

    if window.is_key_down(Key::D) {
        player.turn_right(turn_speed);
    }

    // Mouse-based horizontal movement
    if let Some((mouse_x, _mouse_y)) = window.get_mouse_pos(minifb::MouseMode::Pass) {
        if let Some(prev_x) = *prev_mouse_x {
            let delta_x = mouse_x - prev_x;

            // Adjust the player's angle based on mouse movement
            player.angle += delta_x * TURN_SPEED;
        }

        // Update the previous mouse position
        *prev_mouse_x = Some(mouse_x);
    }
}

fn is_wall(maze: &[Vec<char>], x: f32, y: f32) -> bool {
    let cell_x = x as usize;
    let cell_y = y as usize;
    if cell_x >= maze[0].len() || cell_y >= maze.len() {
        return true;
    }
    maze[cell_y][cell_x] == '+'
}