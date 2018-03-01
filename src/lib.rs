//! VGA Frame Buffer for Embedded Microcontrollers
#![no_std]

extern crate bresenham;

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
const PIXEL_CLOCK: u32 = 80_000_000;
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

/// Implement this on your microcontroller's timer object.
pub trait Hardware {
    /// Called at start-up to configure timer.
    ///
    /// The timer must be periodic, with period `width`, which is measured
    /// clock ticks (or VGA pixels), assuming the given clock rate. If you
    /// have a clock that runs at half the given rate, then double the given
    /// values.
    ///
    /// The function `isr_sync` must be called at the start of the loop.
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
    /// * width - length of a line (in 80 MHz pixels)
    /// * sync_end - elapsed time (in 80 MHz pixels) before H-Sync needs to fall
    /// * line_start - elapsed time (in 80 MHz pixels) before line_start ISR needs to fire
    fn configure(&mut self, width: u32, sync_end: u32, line_start: u32, clock_rate: u32);

    /// Called when V-Sync needs to be high.
    fn vsync_on(&mut self);

    /// Called when V-Sync needs to be low.
    fn vsync_off(&mut self);

    /// Called when pixels need to be written to the output pin.
    fn write_pixels(&mut self, pixels: &VideoLine);
}

/// A point on the screen.
/// The arguments are X (column), Y (row)
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Point(pub usize, pub usize);

#[derive(Default, Copy, Clone)]
pub struct VideoLine {
    pub words: [u16; HORIZONTAL_WORDS],
}

/// The main structure.
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

impl<T> FrameBuffer<T>
where
    T: Hardware,
{
    /// Create a new FrameBuffer
    pub fn new() -> FrameBuffer<T> {
        FrameBuffer {
            line_no: 0,
            fb_line: None,
            frame: 0,
            buffer: [VideoLine::default(); VISIBLE_LINES],
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

    pub fn isr_sync(&mut self) {
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
            self.fb_line = Some((self.line_no - (V_SYNC_PULSE + V_BACK_PORCH)) >> 1);
        } else if self.line_no == V_SYNC_PULSE + V_BACK_PORCH + V_VISIBLE_AREA {
            self.frame = self.frame.wrapping_add(1);
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

    pub fn clear(&mut self) {
        unsafe {
            core::ptr::write_bytes(self.buffer.as_mut_ptr(), 0x00, VISIBLE_LINES);
        }
    }
}
