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

extern crate console_traits;

mod charset;
mod font8x16;
mod font8x8;

pub use charset::*;
pub use console_traits::*;
use font8x16::Font8x16;
// use font8x8::Font8x8;
use core::sync::atomic::{AtomicUsize, Ordering};

pub trait Font {
    fn pixels(&self, glyph: Char, row: usize) -> u8;
    fn height_pixels(&self) -> usize;
    fn width_pixels(&self) -> usize;
    fn length_bytes(&self) -> usize;
}

// See http://tinyvga.com/vga-timing/800x600@60Hz
// These values have been adjusted to assume a 20 MHz pixel clock
const H_VISIBLE_AREA: u32 = 400;
const H_FRONT_PORCH: u32 = 20;
const H_SYNC_PULSE: u32 = 64;
const H_BACK_PORCH: u32 = 44;
const H_WHOLE_LINE: u32 = H_VISIBLE_AREA + H_FRONT_PORCH + H_SYNC_PULSE + H_BACK_PORCH;
const V_VISIBLE_AREA: usize = 600;
const V_FRONT_PORCH: usize = 1;
const V_SYNC_PULSE: usize = 4;
const V_BACK_PORCH: usize = 23;
const V_WHOLE_FRAME: usize = V_SYNC_PULSE + V_BACK_PORCH + V_VISIBLE_AREA + V_FRONT_PORCH;
const V_TOP_BORDER: usize = 12;
const V_BOTTOM_BORDER: usize = 12;

const MAX_FONT_HEIGHT: usize = 16;
const MAX_FONT_WIDTH: usize = 8;
const V_SYNC_PULSE_FIRST: usize = 0;
const V_BACK_PORCH_FIRST: usize = V_SYNC_PULSE_FIRST + V_SYNC_PULSE;
const V_TOP_BORDER_FIRST: usize = V_BACK_PORCH_FIRST + V_BACK_PORCH;
const V_TOP_BORDER_LAST: usize = V_DATA_FIRST - 1;
const V_DATA_FIRST: usize = V_TOP_BORDER_FIRST + V_TOP_BORDER;
const V_DATA_LAST: usize = V_BOTTOM_BORDER_FIRST - 1;
const V_BOTTOM_BORDER_FIRST: usize = V_DATA_FIRST + (MAX_FONT_HEIGHT * TEXT_NUM_ROWS);
const V_BOTTOM_BORDER_LAST: usize = V_FRONT_PORCH_FIRST - 1;
const V_FRONT_PORCH_FIRST: usize = V_BOTTOM_BORDER_FIRST + V_BOTTOM_BORDER;

const PIXEL_CLOCK: u32 = 20_000_000;

/// Top/bottom border height
pub const TOP_BOTTOM_BORDER_HEIGHT: usize = 12;
// How many pixels in the left and right borders
pub const LEFT_RIGHT_BORDER_WIDTH: usize = 8;

/// Number of lines in frame buffer
pub const USABLE_LINES: usize = 576;
/// Number of lines in the mode2 frame buffer (which is line-doubled)
pub const USABLE_LINES_MODE2: usize = 288;
/// Number of columns in frame buffer
pub const USABLE_COLS: usize = 384;
/// Highest Y co-ord
pub const MAX_Y: usize = USABLE_LINES - 1;
/// Highest X co-ord
pub const MAX_X: usize = USABLE_COLS - 1;

/// How many words in a line (including the border)
pub const HORIZONTAL_OCTETS: usize = 50;
/// How many words in a line (excluding the border)
pub const USABLE_HORIZONTAL_OCTETS: usize = 48;

/// How many characters in a row
pub const TEXT_NUM_COLS: usize = USABLE_COLS / MAX_FONT_WIDTH;
/// Highest X co-ord for text
pub const TEXT_MAX_COL: usize = TEXT_NUM_COLS - 1;
/// How many rows of characters on the screen
pub const TEXT_NUM_ROWS: usize = USABLE_LINES / MAX_FONT_HEIGHT;
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
    fn write_pixels(&mut self, red: u32, green: u32, blue: u32);
}

/// A point on the screen.
/// The arguments are X (column), Y (row)
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Point(pub usize, pub usize);

#[derive(Copy, Clone)]
pub struct TextRow {
    pub glyphs: [(Char, Attr); TEXT_NUM_COLS],
}

/// This structure describes the attributes for a Char.
/// They're all packed into 8 bits to save RAM.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Attr(u8);

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Colour {
    White = 7,
    Yellow = 6,
    Magenta = 5,
    Red = 4,
    Cyan = 3,
    Green = 2,
    Blue = 1,
    Black = 0,
}

impl Attr {
    pub const fn new(fg: Colour, bg: Colour) -> Attr {
        Attr(((fg as u8) * 8) + (bg as u8))
    }

    pub fn set_fg(&mut self, fg: Colour) -> &mut Attr {
        self.0 = ((fg as u8) * 8) + (self.0 & 0x7);
        self
    }

    pub fn set_bg(&mut self, bg: Colour) -> &mut Attr {
        self.0 = (self.0 & 0x38) + (bg as u8);
        self
    }
}

// White on Blue
const DEFAULT_ATTR: Attr = Attr((7 * 8) + 1);

impl core::default::Default for Attr {
    fn default() -> Self {
        DEFAULT_ATTR
    }
}

const RGB_MAPS: [[u32; 256]; 64] = include!("maps.txt");

/// Represents Mode2 1-bpp graphics
pub struct Mode2<'a> {
    buffer: &'a mut [u8],
    start: usize,
    end: usize,
}

/// This structure represents the framebuffer - a 2D array of monochome pixels.
///
/// The framebuffer is stored as an array of horizontal lines, where each line
/// is comprised of 8 bit words. This suits our timing needs as although the
/// SPI peripheral on an LM4F120 which can emit 16 bits at a time, 8 proves
/// easier to work with.
pub struct FrameBuffer<'a, T>
where
    T: Hardware,
{
    line_no: AtomicUsize,
    frame: usize,
    text_buffer: [TextRow; TEXT_NUM_ROWS],
    hw: Option<T>,
    attr: Attr,
    pos: Position,
    mode: ControlCharMode,
    escape_mode: EscapeCharMode,
    mode2: Option<Mode2<'a>>,
}

impl<'a, T> FrameBuffer<'a, T>
where
    T: Hardware,
{
    /// Create a new FrameBuffer
    pub const fn new() -> FrameBuffer<'a, T> {
        FrameBuffer {
            line_no: AtomicUsize::new(0),
            frame: 0,
            text_buffer: [TextRow {
                glyphs: [(Char::Null, DEFAULT_ATTR); TEXT_NUM_COLS],
            }; TEXT_NUM_ROWS],
            hw: None,
            pos: Position {
                row: Row(0),
                col: Col(0),
            },
            attr: DEFAULT_ATTR,
            mode: ControlCharMode::Interpret,
            escape_mode: EscapeCharMode::Waiting,
            mode2: None,
        }
    }

    /// Initialise the hardware (by calling the `configure` callback).
    pub fn init(&mut self, mut hw: T) {
        // This all assumes an 8-pixel font (i.e. 1 byte or two per u16)
        assert_eq!(MAX_FONT_WIDTH, 8);
        hw.configure(
            H_WHOLE_LINE,
            H_SYNC_PULSE,
            H_SYNC_PULSE + H_BACK_PORCH,
            PIXEL_CLOCK,
        );
        self.hw = Some(hw);
        self.clear();
    }

    /// Enable mode2 - a 1-bit-per-pixel graphical buffer which is coloured
    /// according to the colour attributes for the matching text cells.
    /// Supply a u8 slice that is some multiple of USABLE_HORIZONTAL_OCTETS long.
    /// The buffer will be line-doubled and so can be up to 288 lines long.
    pub fn mode2(&mut self, buffer: &'a mut [u8], start_line: usize) {
        let length = buffer.len();
        let buffer_lines = length / USABLE_HORIZONTAL_OCTETS;
        let mode2 = Mode2 {
            buffer: buffer,
            start: start_line,
            // Framebuffer is line-doubled
            end: start_line + (2 * buffer_lines),
        };
        self.mode2 = Some(mode2);
    }

    pub fn mode2_shift(&mut self, new_start_line: usize) {
        if let Some(mode2) = self.mode2.as_mut() {
            mode2.start = new_start_line;
        }
    }

    /// Releases the memory for mode2. The rendering code may keep
    /// reading this memory buffer up until the end of the frame.
    pub fn mode2_release(&mut self) -> Option<(&'a mut [u8], usize)> {
        let mut mode2_opt = None;
        core::mem::swap(&mut self.mode2, &mut mode2_opt);
        if let Some(mode2) = mode2_opt {
            Some((mode2.buffer, mode2.start))
        } else {
            None
        }
    }

    /// Returns the current frame number.
    pub fn frame(&self) -> usize {
        self.frame
    }

    /// Returns the current visible line number or None in the blanking period.
    pub fn line(&self) -> Option<usize> {
        let line = self.line_no.load(Ordering::Relaxed);
        if line >= V_DATA_FIRST && line <= V_DATA_LAST {
            Some(line - V_DATA_FIRST)
        } else {
            None
        }
    }

    /// Returns the number of lines since startup.
    pub fn total_line(&self) -> u64 {
        let line_a = self.line_no.load(Ordering::Relaxed);
        let mut f = self.frame;
        let line_b = self.line_no.load(Ordering::Relaxed);
        if line_b < line_a {
            // We wrapped - read new frame
            f = self.frame;
        }
        ((f as u64) * (V_WHOLE_FRAME as u64)) + (line_b as u64)
    }

    /// Call this at the start of every line.
    pub fn isr_sol(&mut self) {
        let current_line = self.line_no.fetch_add(1, Ordering::Relaxed);

        match current_line {
            V_BACK_PORCH_FIRST => {
                if let Some(ref mut hw) = self.hw {
                    hw.vsync_off();
                }
            }
            V_TOP_BORDER_FIRST...V_TOP_BORDER_LAST => {
                self.solid_line();
            }
            V_DATA_FIRST...V_DATA_LAST => {
                let line = current_line - V_DATA_FIRST;
                self.calculate_pixels(line);
            }
            V_BOTTOM_BORDER_FIRST...V_BOTTOM_BORDER_LAST => {
                self.solid_line();
            }
            V_FRONT_PORCH_FIRST => {
                // End of visible frame - increment counter
                self.frame = self.frame.wrapping_add(1);
            }
            V_WHOLE_FRAME => {
                // Wrap around
                self.line_no.store(0, Ordering::Relaxed);
                if let Some(ref mut hw) = self.hw {
                    hw.vsync_on();
                }
            }
            _ => {
                // No output on this line
            }
        }
    }

    /// Calculate a solid line of pixels for the border.
    /// @todo allow the border colour/pattern to be set
    fn solid_line(&mut self) {
        if let Some(ref mut hw) = self.hw {
            // Middle bit
            for _ in 0..HORIZONTAL_OCTETS {
                hw.write_pixels(0xFF, 0xFF, 0xFF);
            }
        }
    }

    /// Calculate the pixels for the given video line.
    ///
    /// Converts each glyph into 8 pixels, then pushes them out as RGB
    /// triplets to the callback function (to be buffered).
    fn calculate_pixels(&mut self, line: usize) {
        let text_row = line / MAX_FONT_HEIGHT;
        let font_row = line % MAX_FONT_HEIGHT;
        let font = Font8x16;
        if let Some(ref mut hw) = self.hw {
            // Left border
            hw.write_pixels(0xFF, 0xFF, 0xFF);

            let mut need_text = true;
            if let Some(mode2) = self.mode2.as_ref() {
                if line >= mode2.start && line < mode2.end && text_row < TEXT_NUM_ROWS {
                    // Pixels in the middle

                    // Our framebuffer is line-doubled
                    let framebuffer_line = (line - mode2.start) >> 1;

                    // Find the block of bytes for this scan-line
                    let start = framebuffer_line * USABLE_HORIZONTAL_OCTETS;
                    let framebuffer_bytes =
                        &mode2.buffer[start..(start + USABLE_HORIZONTAL_OCTETS)];

                    // Write out the bytes with colour from the text-buffer
                    for ((_, attr), bitmap) in self.text_buffer[text_row]
                        .glyphs
                        .iter()
                        .zip(framebuffer_bytes.iter())
                    {
                        let w = *bitmap;
                        let rgb_addr = (RGB_MAPS.as_ptr() as usize)
                            + (attr.0 as usize * 1024_usize)
                            + (w as usize * 4_usize);
                        let rgb_word = unsafe { core::ptr::read(rgb_addr as *const u32) };
                        hw.write_pixels(rgb_word >> 16, rgb_word >> 8, rgb_word);
                    }
                    need_text = false;
                }
            }

            if need_text {
                // Characters in the middle
                if text_row < TEXT_NUM_ROWS {
                    for (ch, attr) in self.text_buffer[text_row].glyphs.iter() {
                        let w = font.pixels(*ch, font_row);
                        let rgb_addr = (RGB_MAPS.as_ptr() as usize)
                            + (attr.0 as usize * 1024_usize)
                            + (w as usize * 4_usize);
                        let rgb_word = unsafe { core::ptr::read(rgb_addr as *const u32) };
                        hw.write_pixels(rgb_word >> 16, rgb_word >> 8, rgb_word);
                    }
                }
            }

            // Right border
            hw.write_pixels(0xFF, 0xFF, 0xFF);
        }
    }

    /// Clears the screen and resets the cursor to 0,0.
    pub fn clear(&mut self) {
        for row in self.text_buffer.iter_mut() {
            for slot in row.glyphs.iter_mut() {
                *slot = (Char::Space, self.attr);
            }
        }
        self.pos = Position::origin();
    }

    /// Puts a glyph on screen at the specified place
    pub fn write_glyph_at(&mut self, glyph: Char, pos: Position, attr: Option<Attr>) {
        if (pos.col <= self.get_width()) && (pos.row <= self.get_height()) {
            self.text_buffer[pos.row.0 as usize].glyphs[pos.col.0 as usize] =
                (glyph, attr.unwrap_or(self.attr));
        }
    }

    /// Puts a glyph on screen at the current position.
    pub fn write_glyph(&mut self, glyph: Char, attr: Option<Attr>) {
        self.text_buffer[self.pos.row.0 as usize].glyphs[self.pos.col.0 as usize] =
            (glyph, attr.unwrap_or(self.attr));
        self.move_cursor_right().unwrap();
    }

    /// Changes the attribute for a given position, leaving the glyph unchanged.
    pub fn set_attr_at(&mut self, pos: Position, attr: Attr) {
        self.text_buffer[pos.row.0 as usize].glyphs[pos.col.0 as usize].1 = attr;
    }

    /// Change the current character attribute
    pub fn set_attr(&mut self, attr: Attr) -> Attr {
        let old = self.attr;
        self.attr = attr;
        old
    }

    /// Get the current character attribute
    pub fn get_attr(&mut self) -> Attr {
        self.attr
    }
}

impl<'a, T> BaseConsole for FrameBuffer<'a, T>
where
    T: Hardware,
{
    type Error = ();

    /// Gets the last col on the screen.
    fn get_width(&self) -> Col {
        Col(TEXT_MAX_COL as u8)
    }

    /// Gets the last row on the screen.
    fn get_height(&self) -> Row {
        Row(TEXT_MAX_ROW as u8)
    }

    /// Set the horizontal position for the next text output.
    fn set_col(&mut self, col: Col) -> Result<(), Self::Error> {
        self.pos.col = col;
        Ok(())
    }

    /// Set the vertical position for the next text output.
    fn set_row(&mut self, row: Row) -> Result<(), Self::Error> {
        self.pos.row = row;
        Ok(())
    }

    /// Set the horizontal and vertical position for the next text output.
    fn set_pos(&mut self, pos: Position) -> Result<(), Self::Error> {
        self.pos = pos;
        Ok(())
    }

    /// Get the current screen position.
    fn get_pos(&self) -> Position {
        self.pos
    }

    /// Set the control char mode
    fn set_control_char_mode(&mut self, mode: ControlCharMode) {
        self.mode = mode;
    }

    /// Get the current control char mode
    fn get_control_char_mode(&self) -> ControlCharMode {
        self.mode
    }

    /// Set the escape char mode
    fn set_escape_char_mode(&mut self, mode: EscapeCharMode) {
        self.escape_mode = mode;
    }

    /// Get the current escape char mode
    fn get_escape_char_mode(&self) -> EscapeCharMode {
        self.escape_mode
    }

    /// Called when the screen needs to scroll up one row.
    fn scroll_screen(&mut self) -> Result<(), Self::Error> {
        for line in 0..TEXT_NUM_ROWS - 1 {
            self.text_buffer[line] = self.text_buffer[line + 1];
        }
        for slot in self.text_buffer[TEXT_MAX_ROW].glyphs.iter_mut() {
            *slot = (Char::Space, DEFAULT_ATTR);
        }
        Ok(())
    }
}

impl<'a, T> AsciiConsole for FrameBuffer<'a, T>
where
    T: Hardware,
{
    /// Handle an escape char.
    /// We take a, b, c, d, e, f, g, h as being a background colour and A..H as being a foreground colour.
    /// 'Z' means clear the screen.
    fn handle_escape(&mut self, escaped_char: u8) -> bool {
        match escaped_char {
            b'W' => {
                self.attr.set_fg(Colour::White);
            }
            b'Y' => {
                self.attr.set_fg(Colour::Yellow);
            }
            b'M' => {
                self.attr.set_fg(Colour::Magenta);
            }
            b'R' => {
                self.attr.set_fg(Colour::Red);
            }
            b'C' => {
                self.attr.set_fg(Colour::Cyan);
            }
            b'G' => {
                self.attr.set_fg(Colour::Green);
            }
            b'B' => {
                self.attr.set_fg(Colour::Blue);
            }
            b'K' => {
                self.attr.set_fg(Colour::Black);
            }
            b'w' => {
                self.attr.set_bg(Colour::White);
            }
            b'y' => {
                self.attr.set_bg(Colour::Yellow);
            }
            b'm' => {
                self.attr.set_bg(Colour::Magenta);
            }
            b'r' => {
                self.attr.set_bg(Colour::Red);
            }
            b'c' => {
                self.attr.set_bg(Colour::Cyan);
            }
            b'g' => {
                self.attr.set_bg(Colour::Green);
            }
            b'b' => {
                self.attr.set_bg(Colour::Blue);
            }
            b'k' => {
                self.attr.set_bg(Colour::Black);
            }
            b'Z' => {
                self.clear();
            }
            _ => {}
        }
        // We only have single char sequences
        true
    }

    /// Write a single Unicode char to the screen at the given position
    /// without updating the current position.
    fn write_char_at(&mut self, ch: u8, pos: Position) -> Result<(), Self::Error> {
        if (pos.col <= self.get_width()) && (pos.row <= self.get_height()) {
            // Skip over the left border
            self.text_buffer[pos.row.0 as usize].glyphs[pos.col.0 as usize] =
                (Char::from_byte(ch), self.attr);
        }
        Ok(())
    }
}

impl<'a, T> core::fmt::Write for FrameBuffer<'a, T>
where
    T: Hardware,
{
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for ch in s.chars() {
            self.write_character(Char::map_char(ch) as u8).map_err(|_| core::fmt::Error)?;
        }
        Ok(())
    }
}

// End of file
