extern crate nalgebra as na;

use std::error::Error;

pub struct Level {
    pub maze: Vec<Vec<char>>,
    pub player_start_pos: (f32, f32),
    pub cat_img: Vec<u32>,
    pub cat_width: usize,
    pub cat_height: usize,
    pub bunnies_to_collect: Option<usize>,
    pub bunny_img: Option<Vec<u32>>,
    pub bunny_width: Option<usize>,
    pub bunny_height: Option<usize>,
}

impl Level {
    pub fn load(level: usize) -> Result<Level, Box<dyn Error>> {
        match level {
            0 => {
                let (maze, player_position) = load_maze("maze.txt")?;
                let (cat_img, cat_width, cat_height) = load_and_resize_image("image/card.jpeg", 100, 100)?;
                Ok(Level {
                    maze,
                    player_start_pos: (player_position.1 as f32, player_position.0 as f32),
                    cat_img,
                    cat_width,
                    cat_height,
                    bunnies_to_collect: None,
                    bunny_img: None,
                    bunny_width: None,
                    bunny_height: None,
                })
            }
            1 => {
                let (maze, player_position) = load_maze("maze2.txt")?;
                let (bunny_img, bunny_width, bunny_height) = load_and_resize_image("image/bunny.jpeg", 100, 100)?;
                let bunnies_to_collect = maze.iter().flat_map(|row| row.iter()).filter(|&&c| c == 'b').count();
                Ok(Level {
                    maze,
                    player_start_pos: (player_position.1 as f32, player_position.0 as f32),
                    cat_img: Vec::new(),
                    cat_width: 0,
                    cat_height: 0,
                    bunnies_to_collect: Some(bunnies_to_collect),
                    bunny_img: Some(bunny_img),
                    bunny_width: Some(bunny_width),
                    bunny_height: Some(bunny_height),
                })
            }
            _ => Err("Invalid level".into()),
        }
    }
}

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
