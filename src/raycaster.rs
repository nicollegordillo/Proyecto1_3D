pub struct Intersect {
    pub distance: f32,
    pub wall_type: char,
    pub hit_x: f32, // Position on the wall (0 to 1)
}

pub fn cast_ray(
    maze: &[Vec<char>],
    px: f32,
    py: f32,
    angle: f32
) -> Intersect {
    let dx = angle.cos();
    let dy = angle.sin();
    let mut x = px;
    let mut y = py;

    let step_size = 0.01; // Smaller step size for more accurate wall detection
    let max_distance = 30.0;

    loop {
        x += dx * step_size;
        y += dy * step_size;

        // Check if the ray has gone out of bounds
        if x < 0.0 || y < 0.0 || x >= maze[0].len() as f32 || y >= maze.len() as f32 {
            break;
        }

        if let Some(cell) = maze.get(y as usize).and_then(|row| row.get(x as usize)) {
            if *cell == '+' || *cell == '-' || *cell == '|' {
                let distance = ((x - px).powi(2) + (y - py).powi(2)).sqrt();

                // Calculate the hit_x as the normalized distance along the wall
                let hit_x = (x % 1.0).abs(); // Assuming walls are vertical, you may need to adjust this for horizontal walls

                return Intersect { 
                    distance,
                    wall_type: *cell,
                    hit_x
                };
            }
        }

        // If the ray reaches the maximum distance without hitting anything
        if ((x - px).powi(2) + (y - py).powi(2)).sqrt() > max_distance {
            break;
        }
    }

    Intersect {
        distance: max_distance,
        wall_type: ' ', // No wall found
        hit_x: 0.0 // No intersection
    }
}


