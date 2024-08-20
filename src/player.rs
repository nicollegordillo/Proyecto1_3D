pub struct Player {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub angle: f32,
    pub fov: f32,
}

impl Player {
    pub fn new(x: f32, y: f32, z:f32, fov: f32) -> Self {
        Self { x, y, z, angle: 0.0, fov }
    }

    pub fn move_forward(&mut self, distance: f32, maze: &[Vec<char>]) {
        let new_x = self.x + distance * self.angle.cos();
        let new_y = self.y + distance * self.angle.sin();

        if !self.check_collision(new_x, new_y, maze) {
            self.x = new_x;
            self.y = new_y;
        }
    }

    pub fn move_backward(&mut self, distance: f32, maze: &[Vec<char>]) {
        let new_x = self.x - distance * self.angle.cos();
        let new_y = self.y - distance * self.angle.sin();

        if !self.check_collision(new_x, new_y, maze) {
            self.x = new_x;
            self.y = new_y;
        }
    }

    pub fn turn_left(&mut self, angle: f32) {
        self.angle -= angle;
    }

    pub fn turn_right(&mut self, angle: f32) {
        self.angle += angle;
    }

    fn check_collision(&self, x: f32, y: f32, maze: &[Vec<char>]) -> bool {
        let maze_x = x as usize;
        let maze_y = y as usize;

        if maze_y >= maze.len() || maze_x >= maze[0].len() {
            return true;
        }

        match maze[maze_y][maze_x] {
            '+' | '-' | '|' => true, // These characters represent walls
            _ => false,
        }
    }
}