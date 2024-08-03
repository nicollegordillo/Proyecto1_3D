extern crate nalgebra_glm as glm;

pub fn cast_ray(
    maze: &[Vec<char>],
    player_x: f32,
    player_y: f32,
    angle: f32,
) -> f32 {
    let mut distance = 0.0;
    let step_size = 0.1;
    let max_depth = 100.0;

    let mut x = player_x;
    let mut y = player_y;

    while distance < max_depth {
        x += step_size * angle.cos();
        y += step_size * angle.sin();
        distance += step_size;

        if maze[y as usize][x as usize] == '+' {
            break;
        }
    }

    distance
}