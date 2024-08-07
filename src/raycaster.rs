pub struct Intersect {
    pub distance: f32,
    pub wall_type: char,
}

pub fn cast_ray(maze: &[Vec<char>], px: f32, py: f32, angle: f32) -> Intersect {
    let dx = angle.cos();
    let dy = angle.sin();
    let mut x = px;
    let mut y = py;

    loop {
        x += dx * 0.1;
        y += dy * 0.1;

        if let Some(cell) = maze.get(y as usize).and_then(|row| row.get(x as usize)) {
            if *cell == '+' {
                let distance = ((x - px).powi(2) + (y - py).powi(2)).sqrt();
                return Intersect { distance, wall_type: *cell };
            }
        } else {
            break;
        }
    }

    Intersect {
        distance: f32::INFINITY,
        wall_type: ' ',
    }
}