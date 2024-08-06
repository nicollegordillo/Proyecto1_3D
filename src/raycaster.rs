pub struct Intersect {
    pub distance: f32,
    pub wall_type: char,
}

pub fn cast_ray(maze: &[Vec<char>], player_x: f32, player_y: f32, angle: f32) -> Intersect {
    let mut distance = 0.0;
    let step_size = 0.1;
    let max_depth = 100.0;

    let mut x = player_x;
    let mut y = player_y;

    while distance < max_depth {
        x += step_size * angle.cos();
        y += step_size * angle.sin();

        if x < 0.0 || x >= maze[0].len() as f32 || y < 0.0 || y >= maze.len() as f32 {
            break;
        }

        if maze[y as usize][x as usize] == '+' {
            return Intersect {
                distance,
                wall_type: '+',
            };
        }

        distance += step_size;
    }

    Intersect {
        distance,
        wall_type: ' ',
    }
}