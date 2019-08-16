//! # VGA Frame Buffer for Embedded Microcontrollers
//!
//! This is designed for a system with a 40 MHz system clock, or some multiple
//! thereof. The nominal output is an 800 x 600 @ 60 Hz SVGA signal, with 3
//! bits per pixel. Because that requires a very large amount of frame buffer
//! ( 800 x 600 x 3 / 8 = 175 KiB ), we offer a few graphics modes which all
//! generate a compatible signal which may double or quadruple the pixel
//! width, or repeat lines. Some modes also use a border.
//!
//! # Video Modes
//!
//! ## 48 x 36 colour text mode (Mode 0)
//!
//! Each text cell has an 8-bit character, a 3-bit foreground colour and a
//! 3-bit background colour. With an 8x16 font, we run the pixel clock at half
//! speed to double the width of each character (so they are effectively 16x16
//! glyphs). You can optionally swap the font, supplying a `[u8; 256 * 16]`.
//! We include the FreeBSD Console Font (Code Page 850) and a Teletext font.
//!
//! ```ignore
//! <------------------------ 400 px ---------------------------->
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
//! When you select this mode, you can supply a font in the form of an array
//! of `256 * 16` bytes, where each block of 16 bytes are the 16 rows of a
//! glyph.
//!
//! ## 384 x 288 block-colour graphics mode (Mode 1)
//!
//! An extension of Mode 0, here you supply a 384 x 288 mono bitmap which is
//! used as a source for the foreground/background rather than using the
//! character cells and the font. You can also specify which scan lines on
//! screen the bitmap is attached, allowing part graphics, part text modes
//! (e.g. for flight sims, or Turtle graphics).
//!
//! ```ignore
//! <------------------------ 400 px ---------------------------->
//! +------------------------------------------------------------+
//! |<--> 8 pixel border     ^                8 pixel border <-->|
//! |                        | 12 px border                      |
//! |                        v                                   |
//! |    +--------------------------------------------------+    |
//! |    | <--^---------------- 384  px ------------------->|    |
//! |    |    |                                             |    |
//! |    |    |                                             |    |
//! |    |    | 288 px doubled                              |    |
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
//! ## 80 x 36 mono text mode (Mode 2)
//!
//! Each text cell has an 8-bit character but no attributes. With a 10x16
//! font, we run the pixel clock at the full 40 MHz speed to get an 80 column
//! display, but we don't have enough CPU time on an 80 MHz Tiva-C for
//! anything other that white/black. You can optionally swap the font,
//! supplying a `[u16; 256 * 24]`. We supply a font which is a mix of Lucida
//! Console 10x16 and 8x16 FreeBSD Console (Code Page 850).
//!
//! When you select this mode, you can supply a font in the form of an array
//! of `256 * 16 * 2` bytes, where each block of 32 bytes are the 16 rows of a
//! glyph. Only the right hand (least significant) 10 bits of each two-byte
//! row are rendered.
//!
//! ```ignore
//! <------------------------ 800 px ---------------------------->
//! +------------------------------------------------------------+
//! |                        ^                                   |
//! |                        | 12 px border                      |
//! |                        v                                   |
//! +------------------------------------------------------------+
//! | <-------^------ 80 chars x 10 px = 800  px --------------->|
//! |         |                                                  |
//! |         |                                                  |
//! |         | 36 rows x 16 px = 576 px                         |
//! |         |                                                  |
//! |         |                                                  |
//! |         v                                                  |
//! +------------------------------------------------------------+
//! |                          ^                                 |
//! |                          | 12 px border                    |
//! |                          v                                 |
//! +------------------------------------------------------------+
//! ```
//!
//! ## Cursor support
//!
//! There is optional cursor support. Rather than try and check each text cell
//! at render time to see if it is in the cursor position, we swap chars in
//! and out of the text buffer as the cursor moves. It's a little more
//! expensive, but the cost is at text write time, not at render time (and so
//! it won't break sync).
//!
//! See https://github.com/thejpster/monotron for an example.

#![no_std]
#![cfg_attr(feature = "const_fn", feature(const_fn))]

extern crate console_traits;
#[macro_use]
extern crate const_ft;

mod charset;
pub mod freebsd_cp850_10x16;
pub mod freebsd_cp850_8x16;
pub mod freebsd_teletext_8x16;

pub use charset::*;
pub use console_traits::*;
use core::sync::atomic::{AtomicUsize, Ordering};

// See http://tinyvga.com/vga-timing/800x600@60Hz
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

const PIXEL_CLOCK: u32 = 40_000_000;

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
pub const HORIZONTAL_OCTETS: usize = 80;
/// How many words in a line (excluding the border)
pub const USABLE_HORIZONTAL_OCTETS: usize = 80;

/// How many characters in a row
pub const TEXT_NUM_COLS: usize = USABLE_HORIZONTAL_OCTETS;
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
    /// Note this function may be called when changing video modes, so make
    /// sure your implementation can handle that (e.g. by disabling interrupts
    /// before modifying the peripheral state).
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
    fn write_pixels(&mut self, xrgb: XRGBColour);

    /// Called to write out a whole block, not just 8 bits of it
    fn write_solid(&mut self);

    /// Called word by word as pixels are calculated (in mono mode)
    fn write_mono_pixels(&mut self, pixels: u16);
}

/// A point on the screen.
/// The arguments are X (column), Y (row)
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Point(pub usize, pub usize);

/// You can set this on a row to make the text double-height. This was common
/// on the BBC Micro in Mode 7/Teletext mode.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum DoubleHeightMode {
    Normal,
    Top,
    Bottom,
}

#[derive(Copy, Clone)]
pub struct TextRow {
    pub double_height: DoubleHeightMode,
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

impl Colour {
    /// Generate 8 pixels in RGB which are all this colour
    pub fn into_pixels(self) -> XRGBColour {
        match self {
            Colour::White => XRGBColour::new(0xFF, 0xFF, 0xFF),
            Colour::Yellow => XRGBColour::new(0xFF, 0xFF, 0x00),
            Colour::Magenta => XRGBColour::new(0xFF, 0x00, 0xFF),
            Colour::Red => XRGBColour::new(0xFF, 0x00, 0x00),
            Colour::Cyan => XRGBColour::new(0x00, 0xFF, 0xFF),
            Colour::Green => XRGBColour::new(0x00, 0xFF, 0x00),
            Colour::Blue => XRGBColour::new(0x00, 0x00, 0xFF),
            Colour::Black => XRGBColour::new(0x00, 0x00, 0x00),
        }
    }
}

/// Represents 8 pixels, each of which can be any 3-bit RGB colour
#[derive(Debug, Copy, Clone)]
pub struct XRGBColour(pub u32);

impl XRGBColour {
    /// Create a new block of 8 coloured pixels by mixing 8 red/black pixels,
    /// 8 green/black pixels and 8 blue/black pixels.
    pub const fn new(red: u8, green: u8, blue: u8) -> XRGBColour {
        XRGBColour(((red as u32) << 16) | ((green as u32) << 8) | (blue as u32))
    }

    /// Get the 8 red/black pixels in the bottom 8 bits
    pub const fn red(self) -> u32 {
        (self.0 >> 16) & 0xFF
    }

    /// Get the 8 green/black pixels in the bottom 8 bits
    pub const fn green(self) -> u32 {
        (self.0 >> 8) & 0xFF
    }

    /// Get the 8 blue/black pixels in the bottom 8 bits
    pub const fn blue(self) -> u32 {
        self.0 & 0xFF
    }

    /// Pixel must be in the range 0..7, where 0 is the rightmost pixel
    pub const fn pixel_has_red(self, pixel: u8) -> bool {
        ((self.0 >> (16 + (7 - pixel))) & 1) == 1
    }

    /// Pixel must be in the range 0..7, where 0 is the rightmost pixel
    pub const fn pixel_has_green(self, pixel: u8) -> bool {
        ((self.0 >> (8 + (7 - pixel))) & 1) == 1
    }

    /// Pixel must be in the range 0..7, where 0 is the rightmost pixel
    pub const fn pixel_has_blue(self, pixel: u8) -> bool {
        ((self.0 >> (7 - pixel)) & 1) == 1
    }
}

impl Attr {
    const FG_BITS: u8 = 0b0011_1000;
    const BG_BITS: u8 = 0b0000_0111;

    const_ft! {
        pub fn new(fg: Colour, bg: Colour) -> Attr {
            Attr(((fg as u8) << 3) + (bg as u8))
        }
    }

    pub fn set_fg(&mut self, fg: Colour) -> &mut Attr {
        self.0 = ((fg as u8) << 3) + (self.0 & !Self::FG_BITS);
        self
    }

    pub fn set_bg(&mut self, bg: Colour) -> &mut Attr {
        self.0 = (self.0 & !Self::BG_BITS) + (bg as u8);
        self
    }

    pub fn as_u8(self) -> u8 {
        self.0
    }
}

// White on Blue
const DEFAULT_ATTR: Attr = Attr((7 * 8) + 1);

const CURSOR: Char = Char::LowLine;

impl core::default::Default for Attr {
    fn default() -> Self {
        DEFAULT_ATTR
    }
}

static RGB_MAPS: [XRGBColour; 256 * 64] = include!("maps.txt");

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
#[repr(C)]
pub struct FrameBuffer<'a, T>
where
    T: Hardware,
{
    line_no: AtomicUsize,
    frame: usize,
    // Add one extra row because 600 doesn't divide by 16
    text_buffer: [TextRow; TEXT_NUM_ROWS + 1],
    hw: Option<T>,
    attr: Attr,
    pos: Position,
    mode: ControlCharMode,
    escape_mode: EscapeCharMode,
    mode2: Option<Mode2<'a>>,
    font: Option<*const u8>,
    cursor_visible: bool,
    under_cursor: Char,
}

impl<'a, T> FrameBuffer<'a, T>
where
    T: Hardware,
{
    /// Create a new FrameBuffer
    const_ft! {
        pub fn new() -> FrameBuffer<'a, T> {
            FrameBuffer {
                line_no: AtomicUsize::new(0),
                frame: 0,
                text_buffer: [TextRow {
                    double_height: DoubleHeightMode::Normal,
                    glyphs: [(Char::Null, DEFAULT_ATTR); TEXT_NUM_COLS],
                }; TEXT_NUM_ROWS + 1],
                hw: None,
                pos: Position {
                    row: Row(0),
                    col: Col(0),
                },
                attr: DEFAULT_ATTR,
                mode: ControlCharMode::Interpret,
                escape_mode: EscapeCharMode::Waiting,
                mode2: None,
                font: None,
                cursor_visible: true,
                under_cursor: Char::Space
            }
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

    pub fn borrow_hw_mut(&mut self) -> Option<&mut T> {
        if let Some(x) = &mut self.hw {
            Some(x)
        } else {
            None
        }
    }

    pub fn borrow_hw(&self) -> Option<&T> {
        if let Some(x) = &self.hw {
            Some(x)
        } else {
            None
        }
    }

    pub fn set_cursor_visible(&mut self, visible: bool) {
        if visible != self.cursor_visible {
            if visible {
                self.cursor_visible = true;
                self.under_cursor = self.current_cell().0;
                self.current_cell().0 = CURSOR;
            } else {
                self.cursor_visible = false;
                self.current_cell().0 = self.under_cursor;
            }
        }
    }

    /// Enable mode2 - a 1-bit-per-pixel graphical buffer which is coloured
    /// according to the colour attributes for the matching text cells.
    /// Supply a u8 slice that is some multiple of USABLE_HORIZONTAL_OCTETS long.
    /// The buffer will be line-doubled and so can be up to 288 lines long.
    pub fn mode2(&mut self, buffer: &'a mut [u8], start_line: usize) {
        let length = buffer.len();
        let buffer_lines = length / USABLE_HORIZONTAL_OCTETS;
        let mode2 = Mode2 {
            buffer,
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

    pub fn map_line(&mut self, _visible_line: u16, _rendered_line: u16) {
        // Do nothing
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
        match self.line_no.load(Ordering::Relaxed) {
            V_BOTTOM_BORDER_FIRST..=V_BOTTOM_BORDER_LAST => {
                self.solid_line();
            }
            V_TOP_BORDER_FIRST..=V_TOP_BORDER_LAST => {
                self.solid_line();
            }
            V_DATA_FIRST..=V_DATA_LAST => {
                self.calculate_pixels();
            }
            V_BACK_PORCH_FIRST => {
                if let Some(ref mut hw) = self.hw {
                    hw.vsync_off();
                }
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
        self.line_no.fetch_add(1, Ordering::Relaxed);
    }

    /// Calculate a solid line of pixels for the border.
    /// @todo allow the border colour/pattern to be set
    fn solid_line(&mut self) {
        if let Some(ref mut hw) = self.hw {
            // Middle bit
            for _ in 0..HORIZONTAL_OCTETS {
                hw.write_solid();
            }
        }
    }

    /// Calculate the pixels for the given video line.
    ///
    /// Converts each glyph into 8 pixels, then pushes them out as RGB
    /// triplets to the callback function (to be buffered).
    fn calculate_pixels(&mut self) {
        let real_line = self.line_no.load(Ordering::Relaxed) - V_DATA_FIRST;
        let text_row = line / MAX_FONT_HEIGHT;
        let row = &self.text_buffer[text_row];
        let font_row = match row.double_height {
            DoubleHeightMode::Normal => line % MAX_FONT_HEIGHT,
            DoubleHeightMode::Top => (line % MAX_FONT_HEIGHT) / 2,
            DoubleHeightMode::Bottom => ((line % MAX_FONT_HEIGHT) + MAX_FONT_HEIGHT) / 2,
        };
        let font_table = freebsd_cp850_10x16::FONT_DATA.as_ptr();
        if let Some(ref mut hw) = self.hw {
            // let mut need_text = true;
            // if let Some(mode2) = self.mode2.as_ref() {
            //     if line >= mode2.start && line < mode2.end && text_row < TEXT_NUM_ROWS {
            //         // Pixels in the middle

            //         // Our framebuffer is line-doubled
            //         let framebuffer_line = (line - mode2.start) >> 1;

            //         // Find the block of bytes for this scan-line
            //         let start = framebuffer_line * USABLE_HORIZONTAL_OCTETS;
            //         let framebuffer_bytes =
            //             &mode2.buffer[start..(start + USABLE_HORIZONTAL_OCTETS)];

            //         // Write out the bytes with colour from the text-buffer
            //         for ((_, attr), bitmap) in self.text_buffer[text_row]
            //             .glyphs
            //             .iter()
            //             .zip(framebuffer_bytes.iter())
            //         {
            //             let w = *bitmap;
            //             // RGB_MAPs is a lookup of (pixels, fg, bg) -> (r,g,b)
            //             // Each row is 4 bytes. The row index is
            //             // 0bFFFBBBPPPPPPPP, where F = foreground, B =
            //             // background, P = 8-bit pixels.
            //             let rgb_addr = unsafe {
            //                 RGB_MAPS
            //                     .as_ptr()
            //                     .offset(((attr.0 as isize) * 256_isize) + (w as isize))
            //             };
            //             let rgb_word = unsafe { *rgb_addr };
            //             hw.write_pixels(rgb_word);
            //         }
            //         need_text = false;
            //     }
            // }

            // if need_text {
            // Characters in the middle
            let font_table = unsafe { font_table.add(font_row) };
            for (ch, _attr) in row.glyphs.iter() {
                let index = (*ch as isize) * (MAX_FONT_HEIGHT as isize);
                let mono_pixels = unsafe { *font_table.offset(index) };
                // RGB_MAPs is a lookup of (pixels, fg, bg) -> (r,g,b)
                // Each row is 4 bytes. The row index is
                // 0bFFFBBBPPPPPPPP, where F = foreground, B =
                // background, P = 8-bit pixels.
                // let rgb_addr = unsafe {
                //     RGB_MAPS
                //         .as_ptr()
                //         .offset(((attr.0 as isize) * 256_isize) + (mono_pixels as isize))
                // };
                // let rgb_word = unsafe { *rgb_addr };
                // hw.write_pixels(rgb_word);
                hw.write_mono_pixels(mono_pixels);
            }
            // }
        }
    }

    /// Change the current font
    pub fn set_custom_font(&mut self, new_font: Option<&'static [u8]>) {
        self.font = match new_font {
            // The given font
            Some(x) => {
                assert_eq!(x.len(), 256 * MAX_FONT_HEIGHT);
                Some(x.as_ptr())
            }
            // The default font
            None => None,
        };
    }

    /// Clears the screen and resets the cursor to 0,0.
    pub fn clear(&mut self) {
        for row in self.text_buffer.iter_mut() {
            for slot in row.glyphs.iter_mut() {
                *slot = (Char::Space, self.attr);
            }
            row.double_height = DoubleHeightMode::Normal;
        }
        self.pos = Position::origin();
    }

    /// Puts a glyph on screen at the specified place
    pub fn write_glyph_at(&mut self, glyph: Char, pos: Position, attr: Option<Attr>) {
        if self.cursor_visible && (pos.row == self.pos.row) && (pos.col == self.pos.col) {
            self.under_cursor = glyph;
            self.text_buffer[pos.row.0 as usize].glyphs[pos.col.0 as usize].1 =
                attr.unwrap_or(self.attr);
        } else {
            if (pos.col <= self.get_width()) && (pos.row <= self.get_height()) {
                self.text_buffer[pos.row.0 as usize].glyphs[pos.col.0 as usize] =
                    (glyph, attr.unwrap_or(self.attr));
            }
        }
    }

    /// Read a glyph on screen at the specified place
    pub fn read_glyph_at(&mut self, pos: Position) -> Option<(Char, Attr)> {
        if self.cursor_visible && (pos.row == self.pos.row) && (pos.col == self.pos.col) {
            Some((
                self.under_cursor,
                self.text_buffer[pos.row.0 as usize].glyphs[pos.col.0 as usize].1,
            ))
        } else {
            if (pos.col <= self.get_width()) && (pos.row <= self.get_height()) {
                Some(self.text_buffer[pos.row.0 as usize].glyphs[pos.col.0 as usize])
            } else {
                None
            }
        }
    }

    /// Puts a glyph on screen at the current position.
    pub fn write_glyph(&mut self, glyph: Char, attr: Option<Attr>) {
        if self.cursor_visible {
            self.under_cursor = glyph;
            self.current_cell().1 = attr.unwrap_or(self.attr);
        } else {
            *self.current_cell() = (glyph, attr.unwrap_or(self.attr));
        }
        self.move_cursor_right().unwrap();
    }

    /// Write a single Unicode char to the screen at the current position.
    pub fn write_char(&mut self, ch: u8, attr: Option<Attr>) {
        self.write_glyph(Char::from_byte(ch), attr);
    }

    /// Changes the attribute for a given position, leaving the glyph unchanged.
    pub fn set_attr_at(&mut self, pos: Position, attr: Attr) {
        self.text_buffer[pos.row.0 as usize].glyphs[pos.col.0 as usize].1 = attr;
    }

    /// Change font height for a given line.
    pub fn set_line_mode_at(&mut self, row: Row, double_height: DoubleHeightMode) {
        self.text_buffer[row.0 as usize].double_height = double_height;
    }

    /// Change font height for the current line.
    pub fn set_line_mode(&mut self, double_height: DoubleHeightMode) {
        self.text_buffer[self.pos.row.0 as usize].double_height = double_height;
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

    fn current_cell(&mut self) -> &mut (Char, Attr) {
        &mut self.text_buffer[self.pos.row.0 as usize].glyphs[self.pos.col.0 as usize]
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
        if self.cursor_visible {
            self.current_cell().0 = self.under_cursor;
            self.pos.col = col;
            self.under_cursor = self.current_cell().0;
            self.current_cell().0 = CURSOR;
        } else {
            self.pos.col = col;
        }
        Ok(())
    }

    /// Set the vertical position for the next text output.
    fn set_row(&mut self, row: Row) -> Result<(), Self::Error> {
        if self.cursor_visible {
            self.current_cell().0 = self.under_cursor;
            self.pos.row = row;
            self.under_cursor = self.current_cell().0;
            self.current_cell().0 = CURSOR;
        } else {
            self.pos.row = row;
        }
        Ok(())
    }

    /// Set the horizontal and vertical position for the next text output.
    fn set_pos(&mut self, pos: Position) -> Result<(), Self::Error> {
        if self.cursor_visible {
            self.current_cell().0 = self.under_cursor;
            self.pos = pos;
            self.under_cursor = self.current_cell().0;
            self.current_cell().0 = CURSOR;
        } else {
            self.pos = pos;
        }
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
        let old_cursor = self.cursor_visible;
        self.set_cursor_visible(false);
        for line in 0..TEXT_NUM_ROWS - 1 {
            self.text_buffer[line] = self.text_buffer[line + 1];
        }
        for slot in self.text_buffer[TEXT_MAX_ROW].glyphs.iter_mut() {
            *slot = (Char::Space, self.attr);
        }
        self.set_cursor_visible(old_cursor);
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
            b'^' => {
                self.set_line_mode(DoubleHeightMode::Top);
            }
            b'v' => {
                self.set_line_mode(DoubleHeightMode::Bottom);
            }
            b'-' => {
                self.set_line_mode(DoubleHeightMode::Normal);
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
        if self.cursor_visible && (pos.row == self.pos.row) && (pos.col == self.pos.col) {
            self.under_cursor = Char::from_byte(ch);
            self.text_buffer[pos.row.0 as usize].glyphs[pos.col.0 as usize].1 = self.attr;
        } else {
            if (pos.col <= self.get_width()) && (pos.row <= self.get_height()) {
                self.text_buffer[pos.row.0 as usize].glyphs[pos.col.0 as usize] =
                    (Char::from_byte(ch), self.attr);
            }
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
            self.write_character(Char::map_char(ch) as u8)
                .map_err(|_| core::fmt::Error)?;
        }
        Ok(())
    }
}

// End of file
