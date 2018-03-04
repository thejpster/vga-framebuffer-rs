//! VGA Frame Buffer for Embedded Microcontrollers
//!
//! Generates an 800 x 600 @ 60 Hz SVGA signal from a 400 x 300 monochrome
//! framebuffer.
//!
//! Requires pixels to be emitted with a 40 MHz pixel clock.
//!
//! See https://github.com/thejpster/monotron for an example.

#![no_std]
#![feature(const_fn)]

extern crate bresenham;

mod font;

use font::*;

// See http://tinyvga.com/vga-timing/800x600@60Hz
// These values assume a 40 MHz pixel clock
const H_VISIBLE_AREA: u32 = 800;
const H_FRONT_PORCH: u32 = 40;
const H_SYNC_PULSE: u32 = 128;
const H_BACK_PORCH: u32 = 88;
const H_WHOLE_LINE: u32 = H_VISIBLE_AREA + H_FRONT_PORCH + H_SYNC_PULSE + H_BACK_PORCH;
const V_VISIBLE_AREA: usize = 600;
const V_FRONT_PORCH: usize = 1;
const V_SYNC_PULSE: usize = 4;
const V_BACK_PORCH: usize = 23;
const V_WHOLE_FRAME: usize = V_SYNC_PULSE + V_BACK_PORCH + V_VISIBLE_AREA + V_FRONT_PORCH;
const PIXEL_CLOCK: u32 = 40_000_000;
const BITS_PER_WORD: usize = 16;

/// Number of lines in frame buffer
pub const VISIBLE_LINES: usize = 300;
/// Highest Y co-ord
pub const MAX_Y: usize = VISIBLE_LINES - 1;
/// Number of columns in frame buffer
pub const VISIBLE_COLS: usize = 400;
/// Highest X co-ord
pub const MAX_X: usize = VISIBLE_COLS - 1;
/// How many 16-bit words in a line
pub const HORIZONTAL_WORDS: usize = (VISIBLE_COLS + BITS_PER_WORD - 1) / BITS_PER_WORD;

/// How many characters in a line
pub const TEXT_NUM_COLS: usize = VISIBLE_COLS / FONT_WIDTH;
/// Highest X co-ord for text
pub const TEXT_MAX_COL: usize = TEXT_NUM_COLS - 1;
/// How many lines of characters on the screen
pub const TEXT_NUM_ROWS: usize = VISIBLE_LINES / FONT_HEIGHT;
/// Highest Y co-ord for text
pub const TEXT_MAX_ROW: usize = TEXT_NUM_ROWS - 1;

/// Implement this on your microcontroller's timer object.
pub trait Hardware {
    /// Called at start-up to configure timer.
    ///
    /// The timer must be periodic, with period `width`, which is measured
    /// clock ticks (or VGA pixels), assuming the given clock rate. If you
    /// have a clock that runs at half the given rate, then double the given
    /// values.
    ///
    /// The function `isr_sol` must be called at the start of the loop.
    ///
    /// The H-Sync pin must rise at start of loop and fall after `sync_end`
    /// VGA pixels.
    ///
    /// The function `isr_data` must be called when the `line_start` period
    /// expires. This will trigger the `write_pixels` function with the pixel
    /// data for the line. Note that these pixels must each be displayed twice
    /// as the framebuffer is half the width and half the height of the VGA
    /// output. You may need to subtract a small amount from `line_start` to
    /// account for your ISR latency.
    ///
    /// * width - length of a line (in 40 MHz pixels)
    /// * sync_end - elapsed time (in 40 MHz pixels) before H-Sync needs to fall
    /// * line_start - elapsed time (in 40 MHz pixels) before line_start ISR needs to fire
    fn configure(&mut self, width: u32, sync_end: u32, line_start: u32, clock_rate: u32);

    /// Called when V-Sync needs to be high.
    fn vsync_on(&mut self);

    /// Called when V-Sync needs to be low.
    fn vsync_off(&mut self);

    /// Called at start of line, allowing pixels to be loaded into a buffer.
    fn buffer_pixels(&mut self, pixels: &VideoLine);

    /// Called when pixels actually need to be written to the output pin.
    fn write_pixels(&mut self, pixels: &VideoLine);
}

/// A point on the screen.
/// The arguments are X (column), Y (row)
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Point(pub usize, pub usize);

#[derive(Default, Copy, Clone)]
pub struct VideoLine {
    pub words: [u16; HORIZONTAL_WORDS],
}

/// This structure represents the framebuffer - a 2D array of monochome pixels.
///
/// The framebuffer is stored as an array of horizontal lines, where each line
/// is comprised of 16 bit words. This suits the SPI peripheral on an LM4F120
/// which can emit 16 bits at a time.
pub struct FrameBuffer<T>
where
    T: Hardware,
{
    line_no: usize,
    fb_line: Option<usize>,
    frame: usize,
    buffer: [VideoLine; VISIBLE_LINES],
    hw: Option<T>,
}

/// A `TextFrameBuffer` allows you to write text to a frame buffer.
///
/// It tracks the current row and column so that repeated print statements
/// work as expected.
pub struct TextFrameBuffer<'a, T>
where
    T: Hardware,
    T: 'a,
{
    col: usize,
    row: usize,
    fb: &'a mut FrameBuffer<T>,
}

impl<T> FrameBuffer<T>
where
    T: Hardware,
{
    /// Create a new FrameBuffer
    pub const fn new() -> FrameBuffer<T> {
        FrameBuffer {
            line_no: 0,
            fb_line: None,
            frame: 0,
            buffer: [VideoLine {
                words: [0u16; HORIZONTAL_WORDS],
            }; VISIBLE_LINES],
            hw: None,
        }
    }

    pub fn init(&mut self, mut hw: T) {
        hw.configure(
            H_WHOLE_LINE,
            H_SYNC_PULSE,
            H_SYNC_PULSE + H_BACK_PORCH,
            PIXEL_CLOCK,
        );
        self.hw = Some(hw);
    }

    pub fn frame(&self) -> usize {
        self.frame
    }

    pub fn isr_sol(&mut self) {
        self.line_no += 1;

        if self.line_no == V_WHOLE_FRAME {
            self.line_no = 0;
            if let Some(ref mut hw) = self.hw {
                hw.vsync_on();
            }
        }

        if self.line_no == V_SYNC_PULSE {
            if let Some(ref mut hw) = self.hw {
                hw.vsync_off();
            }
        }

        if (self.line_no >= V_SYNC_PULSE + V_BACK_PORCH)
            && (self.line_no < V_SYNC_PULSE + V_BACK_PORCH + V_VISIBLE_AREA)
        {
            // Visible lines
            // 600 visible lines, 300 output lines each shown twice
            let line = (self.line_no - (V_SYNC_PULSE + V_BACK_PORCH)) >> 1;
            if let Some(ref mut hw) = self.hw {
                hw.buffer_pixels(&self.buffer[line]);
            }
            self.fb_line = Some(line);
        } else if self.line_no == V_SYNC_PULSE + V_BACK_PORCH + V_VISIBLE_AREA {
            // End of visible frame - increment counter
            self.frame = self.frame.wrapping_add(1);
            self.fb_line = None;
        } else {
            // Front porch
            self.fb_line = None;
        }
    }

    pub fn isr_data(&mut self) {
        if let Some(line) = self.fb_line {
            if let Some(ref mut hw) = self.hw {
                hw.write_pixels(&self.buffer[line]);
            }
        }
    }

    /// Plot a point on screen.
    pub fn draw_point(&mut self, pos: Point, set: bool) {
        if pos.0 < VISIBLE_COLS && pos.1 < VISIBLE_LINES {
            unsafe {
                self.point(pos.0, pos.1, set);
            }
        }
    }

    unsafe fn point(&mut self, x: usize, y: usize, set: bool) {
        let word_x = x / BITS_PER_WORD;
        let word_x_offset = (BITS_PER_WORD - 1) - (x % BITS_PER_WORD);
        if set {
            self.buffer[y].words[word_x] |= 1 << word_x_offset;
        } else {
            self.buffer[y].words[word_x] &= !(1 << word_x_offset);
        }
    }

    /// Draw a bitmap
    pub fn draw_bitmap(&mut self, top_left: Point, width: usize, data: &[u8]) {
        let height = (data.len() * 8) / width;
        let mut byte_idx = 0;
        let mut bit_idx = 7;
        for row in 0..height {
            for col in 0..width {
                let bit: bool = (data[byte_idx] & (1 << bit_idx)) != 0;
                if bit_idx == 0 {
                    bit_idx = 7;
                    byte_idx += 1;
                } else {
                    bit_idx -= 1;
                }
                let point = Point(col + top_left.0, row + top_left.1);
                self.draw_point(point, bit);
            }
        }
    }

    /// Draw a box using lines. The box is hollow, not filled.
    pub fn hollow_rectangle(&mut self, top_left: Point, bottom_right: Point, set: bool) {
        let top_right = Point(bottom_right.0, top_left.1);
        let bottom_left = Point(top_left.0, bottom_right.1);
        self.line(top_left, top_right, set);
        self.line(top_right, bottom_right, set);
        self.line(bottom_right, bottom_left, set);
        self.line(bottom_left, top_left, set);
    }

    /// Draw a filled box.
    pub fn filled_rectangle(&mut self, top_left: Point, bottom_right: Point, set: bool) {
        for y in top_left.1..bottom_right.1 {
            let left = Point(top_left.0, y);
            let right = Point(bottom_right.0, y);
            self.line(left, right, set);
        }
    }

    /// Draw a line.
    pub fn line(&mut self, mut start: Point, mut end: Point, set: bool) {
        start.0 = start.0.min(MAX_X);
        start.1 = start.1.min(MAX_Y);
        end.0 = end.0.min(MAX_X);
        end.1 = end.1.min(MAX_Y);
        if start == end {
            // plotting  a point
            unsafe { self.point(start.0 as usize, start.1 as usize, set) }
        } else if start.1 == end.1 {
            // Short-cut for horizontal lines, writing out whole words where possible
            if end.0 < start.0 {
                let x = end;
                end = start;
                start = x;
            }
            let mut length = (end.0 - start.0) + 1;
            let line = &mut self.buffer[start.1];
            while length > 0 {
                let word_pos = start.0 / BITS_PER_WORD;
                if ((start.0 % BITS_PER_WORD) == 0) && (length >= BITS_PER_WORD) {
                    line.words[word_pos] = if set { 0xFFFF } else { 0x0000 };
                    start.0 += BITS_PER_WORD;
                    length -= BITS_PER_WORD;
                } else {
                    let word_x_offset = (BITS_PER_WORD - 1) - (start.0 % BITS_PER_WORD);
                    if set {
                        line.words[word_pos] |= 1 << word_x_offset;
                    } else {
                        line.words[word_pos] &= !(1 << word_x_offset);
                    }
                    start.0 += 1;
                    length -= 1;
                }
            }
        } else {
            for (x, y) in bresenham::Bresenham::new(
                (start.0 as isize, start.1 as isize),
                (end.0 as isize, end.1 as isize),
            ) {
                unsafe { self.point(x as usize, y as usize, set) }
            }
        }
    }

    /// Writes 8 pixels to the framebuffer together.
    /// The offset is given in u8s.
    pub fn draw_u8(&mut self, pixels: u8, offset_u8s: usize, y: usize) {
        let x_word = offset_u8s / 2;
        let mut bits = self.buffer[y].words[x_word];
        if (offset_u8s & 1) == 1 {
            // second u8 in u16
            bits &= 0xFF00;
            bits |= pixels as u16;
        } else {
            // first u8 in u16
            bits &= 0x00FF;
            bits |= (pixels as u16) << 8;
        }
        self.buffer[y].words[x_word] = bits;
    }

    /// Writes 16 pixels to the framebuffer together.
    /// The offset is given in u16s.
    pub fn draw_u16(&mut self, pixels: u16, offset_u16s: usize, y: usize) {
        self.buffer[y].words[offset_u16s] = pixels;
    }

    pub fn clear(&mut self) {
        unsafe {
            core::ptr::write_bytes(self.buffer.as_mut_ptr(), 0x00, VISIBLE_LINES);
        }
    }
}

impl<'a, T> core::ops::Deref for TextFrameBuffer<'a, T>
where
    T: Hardware,
{
    type Target = FrameBuffer<T>;

    fn deref(&self) -> &FrameBuffer<T> {
        self.fb
    }
}

impl<'a, T> core::ops::DerefMut for TextFrameBuffer<'a, T>
where
    T: Hardware,
{
    fn deref_mut(&mut self) -> &mut FrameBuffer<T> {
        self.fb
    }
}

impl<'a, T> TextFrameBuffer<'a, T>
where
    T: Hardware,
{
    pub fn new(fb: &'a mut FrameBuffer<T>) -> TextFrameBuffer<'a, T> {
        TextFrameBuffer { col: 0, row: 0, fb }
    }

    pub fn write_char_at(&mut self, ch: char, col: usize, row: usize, flip: bool) {
        if (col < TEXT_NUM_COLS) && (row < TEXT_NUM_ROWS) {
            let pixel_y = row * FONT_HEIGHT;
            let glyph = Glyph::map_char(ch);
            for row in 0..FONT_HEIGHT {
                let mut font_byte = glyph.pixels(row);
                if flip {
                    font_byte = !font_byte;
                }
                self.fb.draw_u8(font_byte, col, pixel_y + row);
            }
        }
    }

    /// Returns Ok(()) if dimensions are OK, or Err(()) if they are out of
    /// range.
    pub fn goto(&mut self, col: usize, row: usize) -> Result<(), ()> {
        if (col < TEXT_NUM_COLS) && (row < TEXT_NUM_ROWS) {
            self.col = col;
            self.row = row;
            Ok(())
        } else {
            Err(())
        }
    }
}

impl<'a, T> core::fmt::Write for TextFrameBuffer<'a, T>
where
    T: Hardware,
{
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for ch in s.chars() {
            match ch {
                '\n' => {
                    self.col = 0;
                    self.row += 1;
                }
                '\r' => {
                    self.col = 0;
                }
                '\t' => {
                    let tabs = self.col / 9;
                    self.col = (tabs + 1) * 9;
                }
                ch => {
                    let col = self.col;
                    let row = self.row;
                    self.write_char_at(ch, col, row, false);
                    self.col += 1;
                }
            }
            if self.col == TEXT_NUM_COLS {
                self.col = 0;
                self.row += 1;
            }
            if self.row == TEXT_NUM_ROWS {
                // Should really scroll screen here...
                self.row = TEXT_NUM_ROWS - 1;
                for line in 0..VISIBLE_LINES - FONT_HEIGHT {
                    self.fb.buffer[line] = self.fb.buffer[line + FONT_HEIGHT];
                }
                for line in VISIBLE_LINES - FONT_HEIGHT..VISIBLE_LINES {
                    self.fb.buffer[line] = VideoLine {
                        words: [0u16; HORIZONTAL_WORDS],
                    };
                }
            }
        }
        Ok(())
    }
}

// End of file
