extern crate nalgebra_glm as glm;

pub struct Player {
    pub x: f32,
    pub y: f32,
    pub angle: f32,
    pub fov: f32,
}

impl Player {
    pub fn new(x: f32, y: f32, fov: f32) -> Player {
        Player {
            x,
            y,
            angle: 0.0,
            fov,
        }
    }
}