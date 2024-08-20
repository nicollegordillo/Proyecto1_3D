use crate::framebuffer::Framebuffer;

pub const FONT: [u8; 10] = [
    // 'A' (5x5 bitmap)
    0b00100, // Row 1
    0b01010, // Row 2
    0b10001, // Row 3
    0b11111, // Row 4
    0b10001, // Row 5
    // 'B' (5x5 bitmap)
    0b11110, // Row 1
    0b10001, // Row 2
    0b11110, // Row 3
    0b10001, // Row 4
    0b11110, // Row 5
];

pub fn draw_char(framebuffer: &mut Framebuffer, x: usize, y: usize, ch: char, color: u32) {
    let idx = match ch {
        'A' => 0,
        'B' => 5,
        _ => return, // Ignore characters not in the FONT array
    };
    
    for row in 0..5 { // Adjusted to 5 rows
        let bitmap = FONT[idx + row];
        for col in 0..5 {
            if (bitmap >> (4 - col)) & 1 == 1 {
                if x + col < framebuffer.width && y + row < framebuffer.height {
                    framebuffer.point(x + col, y + row, color);
                }
            }
        }
    }
}


