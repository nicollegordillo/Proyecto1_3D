extern crate minifb;
use minifb::{Key, Window};
use crate::player::Player;

const MOVE_SPEED: f32 = 0.1;
const TURN_SPEED: f32 = std::f32::consts::PI / 30.0;

pub fn process_events(window: &Window, player: &mut Player) {
    if window.is_key_down(Key::W) {
        player.x += MOVE_SPEED * player.angle.cos();
        player.y += MOVE_SPEED * player.angle.sin();
    }
    if window.is_key_down(Key::S) {
        player.x -= MOVE_SPEED * player.angle.cos();
        player.y -= MOVE_SPEED * player.angle.sin();
    }
    if window.is_key_down(Key::A) {
        player.angle -= TURN_SPEED;
    }
    if window.is_key_down(Key::D) {
        player.angle += TURN_SPEED;
    }
}