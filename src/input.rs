extern crate minifb;
use minifb::{Key, Window};
use crate::player::Player;

const MOVE_SPEED: f32 = 0.1;
const TURN_SPEED: f32 = std::f32::consts::PI / 30.0;

pub fn process_events(window: &Window, player: &mut Player, maze: &[Vec<char>]) {
    if window.is_key_down(Key::W) {
        let new_x = player.x + MOVE_SPEED * player.angle.cos();
        let new_y = player.y + MOVE_SPEED * player.angle.sin();
        if !is_wall(maze, new_x, new_y) {
            player.x = new_x;
            player.y = new_y;
        }
    }
    if window.is_key_down(Key::S) {
        let new_x = player.x - MOVE_SPEED * player.angle.cos();
        let new_y = player.y - MOVE_SPEED * player.angle.sin();
        if !is_wall(maze, new_x, new_y) {
            player.x = new_x;
            player.y = new_y;
        }
    }
    if window.is_key_down(Key::A) {
        player.angle -= TURN_SPEED;
    }
    if window.is_key_down(Key::D) {
        player.angle += TURN_SPEED;
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