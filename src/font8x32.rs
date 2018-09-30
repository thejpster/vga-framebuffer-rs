use charset::Char;

use super::font8x16;

// This is just `Font8x16` but stretched to be double the height.
pub struct Font8x32;

impl super::Font for Font8x32 {
    fn pixels(&self, glyph: Char, row: usize) -> u8 {
        font8x16::Font8x16.pixels(glyph, row / 2)
    }
    fn height_pixels(&self) -> usize {
        font8x16::Font8x16.height_pixels() * 2
    }
    fn width_pixels(&self) -> usize {
        font8x16::Font8x16.width_pixels()
    }
    fn length_bytes(&self) -> usize {
        font8x16::Font8x16.length_bytes()
    }
}
