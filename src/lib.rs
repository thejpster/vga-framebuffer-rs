//! VGA Frame Buffer for Embedded Microcontrollers
//!
//! Generates an 800 x 600 @ 60 Hz SVGA signal from a 48 column x 36 row
//! monochrome text buffer. The image has a border.
//!
//! TODO: Implement smooth scrolling in the vertical direction with an extra
//! text row.
//!
//!
//! Width = 400 double width pixels => 400 = 8 + (48 x 8) + 8
//!
//! Height = 600 pixels => 600 = 12 + (36 x 16) + 12
//!
//! ```ignore
//! <-------------- 400 px, pixel doubled to 800 px ------------->
//! +------------------------------------------------------------+
//! |<--> 8 pixel border     ^                8 pixel border <-->|
//! |                        | 12 px border                      |
//! |                        v                                   |
//! |    +--------------------------------------------------+    |
//! |    | <--^------ 48 chars x 8 px = 384  px ----------->|    |
//! |    |    |                                             |    |
//! |    |    |                                             |    |
//! |    |    | 36 rows x 16 px = 576 px                    |    |
//! |    |    |                                             |    |
//! |    |    |                                             |    |
//! |    |    v                                             |    |
//! |    +--------------------------------------------------+    |
//! |                          ^                                 |
//! |                          | 12 px border                    |
//! |                          v                                 |
//! +------------------------------------------------------------+
//! ```
//!
//! Requires pixels to be emitted with a 20 MHz pixel clock (against a nominal
//! 40 MHz pixel clock, in order to acheive the horizontal doubling).
//!
//! See https://github.com/thejpster/monotron for an example.

#![no_std]
#![feature(const_fn)]

mod font;

pub use font::*;

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
const V_TOP_BORDER: usize = 12;
const V_BOTTOM_BORDER: usize = 12;

const V_SYNC_PULSE_FIRST: usize = 0;
const V_BACK_PORCH_FIRST: usize = V_SYNC_PULSE_FIRST + V_SYNC_PULSE;
const V_TOP_BORDER_FIRST: usize = V_BACK_PORCH_FIRST + V_BACK_PORCH;
const V_TOP_BORDER_LAST: usize = V_DATA_FIRST - 1;
const V_DATA_FIRST: usize = V_TOP_BORDER_FIRST + V_TOP_BORDER;
const V_DATA_LAST: usize = V_BOTTOM_BORDER_FIRST - 1;
const V_BOTTOM_BORDER_FIRST: usize = V_DATA_FIRST + (FONT_HEIGHT * TEXT_NUM_ROWS);
const V_BOTTOM_BORDER_LAST: usize = V_FRONT_PORCH_FIRST - 1;
const V_FRONT_PORCH_FIRST: usize = V_BOTTOM_BORDER_FIRST + V_BOTTOM_BORDER;

const PIXEL_CLOCK: u32 = 40_000_000;
const BITS_PER_WORD: usize = 8;

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

/// How many characters in a row
pub const TEXT_NUM_COLS: usize = 48;
/// Highest X co-ord for text
pub const TEXT_MAX_COL: usize = TEXT_NUM_COLS - 1;
/// Number of glyphs in a row, including borders
pub const TEXT_NUM_COLS_INC_BORDER: usize = TEXT_NUM_COLS + 2;
/// How many rows of characters on the screen
pub const TEXT_NUM_ROWS: usize = 36;
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
    /// You will receive calls to `write_pixels` as pixels are generated. Do
    /// not emit any pixels until the `line_start` timer elapses (store them
    /// in a FIFO).
    ///
    /// The H-Sync pin must rise at the start of the loop and fall after
    /// `sync_end` clock ticks. We don't control it here because that would
    /// add too much latency - you must change the H-Sync GPIO pin early in
    /// the ISR yourself.
    ///
    /// V-Sync is controlled by the current line number; you should implement
    /// `vsync_on` and `vsync_off` which this code will call at the
    /// appropriate time.
    ///
    /// * `width` - length of a line (in `clock_rate` pixels)
    /// * `sync_end` - elapsed time (in `clock_rate` pixels) before H-Sync
    ///   needs to fall
    /// * `line_start` - elapsed time (in `clock_rate` pixels) before
    ///   line_start ISR needs to fire
    /// * `clock_rate` - the pixel clock rate in Hz (e.g. 40_000_000 for 40
    ///   MHz)
    fn configure(&mut self, width: u32, sync_end: u32, line_start: u32, clock_rate: u32);

    /// Called when V-Sync needs to be high.
    fn vsync_on(&mut self);

    /// Called when V-Sync needs to be low.
    fn vsync_off(&mut self);

    /// Called word by word as pixels are calculated
    fn write_pixels(&mut self, red: u8, green: u8, blue: u8);
}

/// A point on the screen.
/// The arguments are X (column), Y (row)
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Point(pub usize, pub usize);

#[derive(Copy, Clone)]
pub struct TextRow {
    pub glyphs: [(Glyph, Attr); TEXT_NUM_COLS_INC_BORDER],
}

/// This structure describes the attributes for a Glyph.
/// They're all packed into 8 bits to save RAM.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Attr(u8);

pub const WHITE_ON_BLACK: Attr = Attr(7);
pub const YELLOW_ON_BLACK: Attr = Attr(6);
pub const MAGENTA_ON_BLACK: Attr = Attr(5);
pub const RED_ON_BLACK: Attr = Attr(4);
pub const CYAN_ON_BLACK: Attr = Attr(3);
pub const GREEN_ON_BLACK: Attr = Attr(2);
pub const BLUE_ON_BLACK: Attr = Attr(1);

/// This structure represents the framebuffer - a 2D array of monochome pixels.
///
/// The framebuffer is stored as an array of horizontal lines, where each line
/// is comprised of 8 bit words. This suits our timing needs as although the
/// SPI peripheral on an LM4F120 which can emit 16 bits at a time, 8 proves
/// easier to work with.
pub struct FrameBuffer<T>
where
    T: Hardware,
{
    line_no: usize,
    fb_line: Option<usize>,
    frame: usize,
    text_buffer: [TextRow; TEXT_NUM_ROWS],
    col: usize,
    row: usize,
    hw: Option<T>,
    attr: Attr,
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
            text_buffer: [TextRow {
                glyphs: [(Glyph::Null, WHITE_ON_BLACK); TEXT_NUM_COLS_INC_BORDER],
            }; TEXT_NUM_ROWS],
            hw: None,
            col: 0,
            row: 0,
            attr: WHITE_ON_BLACK
        }
    }

    /// Initialise the hardware (by calling the `configure` callback).
    pub fn init(&mut self, mut hw: T) {
        // This all assumes an 8-pixel font (i.e. 1 byte or two per u16)
        assert_eq!(FONT_WIDTH, 8);
        hw.configure(
            H_WHOLE_LINE,
            H_SYNC_PULSE,
            H_SYNC_PULSE + H_BACK_PORCH,
            PIXEL_CLOCK,
        );
        self.hw = Some(hw);
        // Fill in the side border
        for row in self.text_buffer.iter_mut() {
            row.glyphs[0] = (Glyph::FullBlock, WHITE_ON_BLACK);
            row.glyphs[row.glyphs.len() - 1] = (Glyph::FullBlock, WHITE_ON_BLACK);
        }
        self.clear();
    }

    /// Returns the current frame number.
    pub fn frame(&self) -> usize {
        self.frame
    }

    /// Call this at the start of every line.
    pub fn isr_sol(&mut self) {
        self.line_no += 1;

        match self.line_no {
            V_BACK_PORCH_FIRST => {
                if let Some(ref mut hw) = self.hw {
                    hw.vsync_off();
                }
                self.fb_line = None;
            }
            V_TOP_BORDER_FIRST...V_TOP_BORDER_LAST => {
                self.solid_line();
                self.fb_line = None;
            }
            V_DATA_FIRST...V_DATA_LAST => {
                let line = self.line_no - V_DATA_FIRST;
                self.calculate_pixels(line);
                self.fb_line = Some(line);
            }
            V_BOTTOM_BORDER_FIRST...V_BOTTOM_BORDER_LAST => {
                self.solid_line();
                self.fb_line = None;
            }
            V_FRONT_PORCH_FIRST => {
                // End of visible frame - increment counter
                self.frame = self.frame.wrapping_add(1);
                self.fb_line = None;
            }
            V_WHOLE_FRAME => {
                // Wrap around
                self.line_no = 0;
                if let Some(ref mut hw) = self.hw {
                    hw.vsync_on();
                }
                self.fb_line = None;
            }
            _ => {
                // No output on this line
                self.fb_line = None;
            }
        }
    }

    /// Calculate a solid line of pixels for the border.
    fn solid_line(&mut self) {
        if let Some(ref mut hw) = self.hw {
            for _ in 0..HORIZONTAL_WORDS {
                hw.write_pixels(0xFF, 0xFF, 0xFF);
            }
        }
    }

    /// Calculate the pixels for the given video line.
    ///
    /// Converts each glyph into 8 pixels, then pushes them out as pairs to
    /// the callback function (to be buffered).
    fn calculate_pixels(&mut self, line: usize) {
        let text_row = line / FONT_HEIGHT;
        let font_row = line % FONT_HEIGHT;
        if let Some(ref mut hw) = self.hw {
            if text_row < TEXT_NUM_ROWS {
                for (ch, _attr) in self.text_buffer[text_row].glyphs.iter() {
                    let w = ch.pixels(font_row);
                    // CPU isn't fast enough to calculate colours here
                    hw.write_pixels(w, w, 0xFF);
                }
            }
        }
    }

    /// Clears the screen and resets the cursor to 0,0.
    pub fn clear(&mut self) {
        for row in self.text_buffer.iter_mut() {
            for slot in row.glyphs.iter_mut().skip(1).take(TEXT_NUM_COLS) {
                *slot = (Glyph::Space, WHITE_ON_BLACK);
            }
        }
        self.row = 0;
        self.col = 0;
    }

    /// Puts a char on screen at the specified place. Unicode chars are mapped
    /// to Codepage 850 first.
    pub fn write_char_at(&mut self, ch: char, col: usize, row: usize, attr: Option<Attr>) {
        if (col < TEXT_NUM_COLS) && (row < TEXT_NUM_ROWS) {
            // Skip over the left border
            self.text_buffer[row].glyphs[col + 1] = (Glyph::map_char(ch), attr.unwrap_or(self.attr));
        }
    }

    /// Puts a glyph on screen at the specified place
    pub fn write_glyph_at(&mut self, glyph: Glyph, col: usize, row: usize, attr: Option<Attr>) {
        if (col < TEXT_NUM_COLS) && (row < TEXT_NUM_ROWS) {
            // Skip over the left border
            self.text_buffer[row].glyphs[col + 1] = (glyph, attr.unwrap_or(self.attr));
        }
    }

    /// Puts a char on screen at the current position.
    pub fn write_char(&mut self, ch: char, attr: Option<Attr>) {
        // Skip over the left border
        self.text_buffer[self.row].glyphs[self.col + 1] = (Glyph::map_char(ch), attr.unwrap_or(self.attr));
        self.col += 1;
        self.wrap_cursor();
    }

    /// Puts a glyph on screen at the current position.
    pub fn write_glyph(&mut self, glyph: Glyph, attr: Option<Attr>) {
        // Skip over the left border
        self.text_buffer[self.row].glyphs[self.col + 1] = (glyph, attr.unwrap_or(self.attr));
        self.col += 1;
        self.wrap_cursor();
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

    fn wrap_cursor(&mut self) {
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
            for slot in self.text_buffer[TEXT_MAX_ROW]
                .glyphs
                .iter_mut()
                .skip(1)
                .take(TEXT_NUM_COLS)
            {
                *slot = (Glyph::Space, WHITE_ON_BLACK);
            }
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
                '\u{007f}' => {
                    if self.col == 0 {
                        self.col = TEXT_MAX_COL;
                        if self.row > 0 {
                            self.row -= 1;
                        }
                    } else {
                        self.col -= 1;
                    }
                }
                '\r' => {
                    self.col = 0;
                }
                '\t' => {
                    let tabs = self.col / 9;
                    self.col = (tabs + 1) * 9;
                }
                ch => self.write_char(ch, None)
            }
            self.wrap_cursor();
        }
        Ok(())
    }
}

// End of file
