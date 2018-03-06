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
pub const VISIBLE_LINES: usize = 600;
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
    /// The function `isr_sol` must be called at the start of the loop. You will
    /// receive callbacks with pixel data as it is generated. Do not emit
    /// any pixels until `line_start` elapses (store them in a FIFO).
    ///
    /// The H-Sync pin must rise at start of loop and fall after `sync_end`
    /// VGA pixels.
    ///
    /// * width - length of a line (in 40 MHz pixels)
    /// * sync_end - elapsed time (in 40 MHz pixels) before H-Sync needs to fall
    /// * line_start - elapsed time (in 40 MHz pixels) before line_start ISR needs to fire
    fn configure(&mut self, width: u32, sync_end: u32, line_start: u32, clock_rate: u32);

    /// Called when V-Sync needs to be high.
    fn vsync_on(&mut self);

    /// Called when V-Sync needs to be low.
    fn vsync_off(&mut self);

    /// Called word by word as pixels are calculated
    fn write_pixels(&mut self, pixels: u16);
}

/// A point on the screen.
/// The arguments are X (column), Y (row)
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Point(pub usize, pub usize);

#[derive(Copy, Clone)]
pub struct VideoLine {
    pub words: [u16; HORIZONTAL_WORDS],
}

#[derive(Copy, Clone)]
pub struct TextLine {
    pub chars: [Glyph; TEXT_NUM_COLS],
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
    // buffer: [VideoLine; VISIBLE_LINES],
    video_line: VideoLine,
    text_buffer: [TextLine; TEXT_NUM_ROWS],
    col: usize,
    row: usize,
    hw: Option<T>,
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
            text_buffer: [TextLine {
                chars: [Glyph::Space; TEXT_NUM_COLS],
            }; TEXT_NUM_ROWS],
            video_line: VideoLine {
                words: [0u16; HORIZONTAL_WORDS],
            },
            hw: None,
            col: 0,
            row: 0,
        }
    }

    /// Initialise the hardware (by calling the `configure` callback).
    pub fn init(&mut self, mut hw: T) {
        hw.configure(
            H_WHOLE_LINE,
            H_SYNC_PULSE,
            H_SYNC_PULSE + H_BACK_PORCH,
            PIXEL_CLOCK,
        );
        self.hw = Some(hw);
    }

    /// Returns the current frame number.
    pub fn frame(&self) -> usize {
        self.frame
    }

    /// Call this at the start of every line.
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
            // let line = (self.line_no - (V_SYNC_PULSE + V_BACK_PORCH)) >> 1;
            let line = self.line_no - (V_SYNC_PULSE + V_BACK_PORCH);
            self.calculate_pixels(line);
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

    /// Calculate the pixels for the given video line.
    ///
    /// Converts each glyph into 8 pixels, then pushes them out as pairs to
    /// the callback function (to be buffered).
    fn calculate_pixels(&mut self, line: usize) {
        let text_row = line / FONT_HEIGHT;
        let font_row = line % FONT_HEIGHT;
        if text_row < TEXT_NUM_ROWS {
            for (ch_pair, word) in self.text_buffer[text_row]
                .chars
                .chunks(2)
                .zip(self.video_line.words.iter_mut())
            {
                let b0 = ch_pair[0].pixels(font_row) as u16;
                let b1 = ch_pair[1].pixels(font_row) as u16;
                let w = (b0 << 8) | b1;
                if let Some(ref mut hw) = self.hw {
                    hw.write_pixels(w);
                }
                *word = w;
            }
        } else {
            for word in self.video_line.words.iter_mut() {
                *word = 0;
            }
        }
    }

    /// Clears the screen and resets the cursor to 0,0.
    pub fn clear(&mut self) {
        for row in self.text_buffer.iter_mut() {
            for slot in row.chars.iter_mut() {
                *slot = Glyph::Space;
            }
        }
        self.row = 0;
        self.col = 0;
    }

    /// Puts a char on screen at the specified place
    pub fn write_char_at(&mut self, ch: char, col: usize, row: usize, _flip: bool) {
        if (col < TEXT_NUM_COLS) && (row < TEXT_NUM_ROWS) {
            self.text_buffer[row].chars[col] = Glyph::map_char(ch);
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

impl<T> core::fmt::Write for FrameBuffer<T>
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
                // Scroll screen
                self.row = TEXT_NUM_ROWS - 1;
                for line in 0..TEXT_NUM_ROWS - 1 {
                    self.text_buffer[line] = self.text_buffer[line + 1];
                }
                self.text_buffer[TEXT_MAX_ROW] = TextLine {
                    chars: [Glyph::Space; TEXT_NUM_COLS],
                };
            }
        }
        Ok(())
    }
}

// End of file
