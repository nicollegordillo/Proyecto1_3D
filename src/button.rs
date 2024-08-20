use crate::{font::draw_char, framebuffer::Framebuffer};

pub struct Button {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
    pub text: String,
    pub is_selected: bool,
}

impl Button {
    pub fn new(x: usize, y: usize, width: usize, height: usize, text: &str) -> Self {
        Button {
            x,
            y,
            width,
            height,
            text: text.to_string(),
            is_selected: false,
        }
    }

    pub fn draw(&self, framebuffer: &mut Framebuffer) {
        let color = if self.is_selected { 0xFFFFFFFF } else { 0xFF808080 }; // White if selected, gray otherwise
        let text_color = if self.is_selected { 0xFF000000 } else { 0xFFFFFFFF }; // Black text if selected, white otherwise

        // Draw button background
        for x in self.x..self.x + self.width {
            for y in self.y..self.y + self.height {
                if x < framebuffer.width && y < framebuffer.height {
                    framebuffer.point(x, y, color);
                }
            }
        }

        // Draw text in the center of the button
        let text_x = self.x + (self.width - (self.text.len() * 6)) / 2; // Center text horizontally
        let text_y = self.y + (self.height - 7) / 2; // Center text vertically

        for (i, ch) in self.text.chars().enumerate() {
            draw_char(framebuffer, text_x + i * 6, text_y, ch, text_color);
        }
    }
}



