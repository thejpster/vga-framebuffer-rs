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

mod font;

pub use console_traits::*;
pub use font::*;

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

const V_SYNC_PULSE_FIRST: usize = 0;
const V_BACK_PORCH_FIRST: usize = V_SYNC_PULSE_FIRST + V_SYNC_PULSE;
const V_TOP_BORDER_FIRST: usize = V_BACK_PORCH_FIRST + V_BACK_PORCH;
const V_TOP_BORDER_LAST: usize = V_DATA_FIRST - 1;
const V_DATA_FIRST: usize = V_TOP_BORDER_FIRST + V_TOP_BORDER;
const V_DATA_LAST: usize = V_BOTTOM_BORDER_FIRST - 1;
const V_BOTTOM_BORDER_FIRST: usize = V_DATA_FIRST + (FONT_HEIGHT * TEXT_NUM_ROWS);
const V_BOTTOM_BORDER_LAST: usize = V_FRONT_PORCH_FIRST - 1;
const V_FRONT_PORCH_FIRST: usize = V_BOTTOM_BORDER_FIRST + V_BOTTOM_BORDER;

const PIXEL_CLOCK: u32 = 20_000_000;
const BITS_PER_WORD: usize = 8;

/// Number of lines in frame buffer
pub const VISIBLE_LINES: usize = V_VISIBLE_AREA as usize;
/// Highest Y co-ord
pub const MAX_Y: usize = VISIBLE_LINES - 1;
/// Number of columns in frame buffer
pub const VISIBLE_COLS: usize = H_VISIBLE_AREA as usize;
/// Highest X co-ord
pub const MAX_X: usize = VISIBLE_COLS - 1;
/// How many 16-bit words in a line
pub const HORIZONTAL_WORDS: usize = (VISIBLE_COLS + BITS_PER_WORD - 1) / BITS_PER_WORD;

/// How many characters in a row
pub const TEXT_NUM_COLS: usize = (VISIBLE_COLS / FONT_WIDTH) - 2;
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
    fn write_pixels(&mut self, red: u32, green: u32, blue: u32);
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
#[repr(u8)]
pub enum Attr {
    Normal = 0,
    Reverse = 1,
    WhiteOnBlack = 2,
    GreenOnBlack = 3,
}

const RGB_MAPS: [[u32; 256]; 4] = [
    //    include!("../maps.txt")
    [
        0x0000ff, 0x0101ff, 0x0202ff, 0x0303ff, 0x0404ff, 0x0505ff, 0x0606ff, 0x0707ff, 0x0808ff,
        0x0909ff, 0x0a0aff, 0x0b0bff, 0x0c0cff, 0x0d0dff, 0x0e0eff, 0x0f0fff, 0x1010ff, 0x1111ff,
        0x1212ff, 0x1313ff, 0x1414ff, 0x1515ff, 0x1616ff, 0x1717ff, 0x1818ff, 0x1919ff, 0x1a1aff,
        0x1b1bff, 0x1c1cff, 0x1d1dff, 0x1e1eff, 0x1f1fff, 0x2020ff, 0x2121ff, 0x2222ff, 0x2323ff,
        0x2424ff, 0x2525ff, 0x2626ff, 0x2727ff, 0x2828ff, 0x2929ff, 0x2a2aff, 0x2b2bff, 0x2c2cff,
        0x2d2dff, 0x2e2eff, 0x2f2fff, 0x3030ff, 0x3131ff, 0x3232ff, 0x3333ff, 0x3434ff, 0x3535ff,
        0x3636ff, 0x3737ff, 0x3838ff, 0x3939ff, 0x3a3aff, 0x3b3bff, 0x3c3cff, 0x3d3dff, 0x3e3eff,
        0x3f3fff, 0x4040ff, 0x4141ff, 0x4242ff, 0x4343ff, 0x4444ff, 0x4545ff, 0x4646ff, 0x4747ff,
        0x4848ff, 0x4949ff, 0x4a4aff, 0x4b4bff, 0x4c4cff, 0x4d4dff, 0x4e4eff, 0x4f4fff, 0x5050ff,
        0x5151ff, 0x5252ff, 0x5353ff, 0x5454ff, 0x5555ff, 0x5656ff, 0x5757ff, 0x5858ff, 0x5959ff,
        0x5a5aff, 0x5b5bff, 0x5c5cff, 0x5d5dff, 0x5e5eff, 0x5f5fff, 0x6060ff, 0x6161ff, 0x6262ff,
        0x6363ff, 0x6464ff, 0x6565ff, 0x6666ff, 0x6767ff, 0x6868ff, 0x6969ff, 0x6a6aff, 0x6b6bff,
        0x6c6cff, 0x6d6dff, 0x6e6eff, 0x6f6fff, 0x7070ff, 0x7171ff, 0x7272ff, 0x7373ff, 0x7474ff,
        0x7575ff, 0x7676ff, 0x7777ff, 0x7878ff, 0x7979ff, 0x7a7aff, 0x7b7bff, 0x7c7cff, 0x7d7dff,
        0x7e7eff, 0x7f7fff, 0x8080ff, 0x8181ff, 0x8282ff, 0x8383ff, 0x8484ff, 0x8585ff, 0x8686ff,
        0x8787ff, 0x8888ff, 0x8989ff, 0x8a8aff, 0x8b8bff, 0x8c8cff, 0x8d8dff, 0x8e8eff, 0x8f8fff,
        0x9090ff, 0x9191ff, 0x9292ff, 0x9393ff, 0x9494ff, 0x9595ff, 0x9696ff, 0x9797ff, 0x9898ff,
        0x9999ff, 0x9a9aff, 0x9b9bff, 0x9c9cff, 0x9d9dff, 0x9e9eff, 0x9f9fff, 0xa0a0ff, 0xa1a1ff,
        0xa2a2ff, 0xa3a3ff, 0xa4a4ff, 0xa5a5ff, 0xa6a6ff, 0xa7a7ff, 0xa8a8ff, 0xa9a9ff, 0xaaaaff,
        0xababff, 0xacacff, 0xadadff, 0xaeaeff, 0xafafff, 0xb0b0ff, 0xb1b1ff, 0xb2b2ff, 0xb3b3ff,
        0xb4b4ff, 0xb5b5ff, 0xb6b6ff, 0xb7b7ff, 0xb8b8ff, 0xb9b9ff, 0xbabaff, 0xbbbbff, 0xbcbcff,
        0xbdbdff, 0xbebeff, 0xbfbfff, 0xc0c0ff, 0xc1c1ff, 0xc2c2ff, 0xc3c3ff, 0xc4c4ff, 0xc5c5ff,
        0xc6c6ff, 0xc7c7ff, 0xc8c8ff, 0xc9c9ff, 0xcacaff, 0xcbcbff, 0xccccff, 0xcdcdff, 0xceceff,
        0xcfcfff, 0xd0d0ff, 0xd1d1ff, 0xd2d2ff, 0xd3d3ff, 0xd4d4ff, 0xd5d5ff, 0xd6d6ff, 0xd7d7ff,
        0xd8d8ff, 0xd9d9ff, 0xdadaff, 0xdbdbff, 0xdcdcff, 0xddddff, 0xdedeff, 0xdfdfff, 0xe0e0ff,
        0xe1e1ff, 0xe2e2ff, 0xe3e3ff, 0xe4e4ff, 0xe5e5ff, 0xe6e6ff, 0xe7e7ff, 0xe8e8ff, 0xe9e9ff,
        0xeaeaff, 0xebebff, 0xececff, 0xededff, 0xeeeeff, 0xefefff, 0xf0f0ff, 0xf1f1ff, 0xf2f2ff,
        0xf3f3ff, 0xf4f4ff, 0xf5f5ff, 0xf6f6ff, 0xf7f7ff, 0xf8f8ff, 0xf9f9ff, 0xfafaff, 0xfbfbff,
        0xfcfcff, 0xfdfdff, 0xfefeff, 0xffffff,
    ],
    [
        0xffffff, 0xfefeff, 0xfdfdff, 0xfcfcff, 0xfbfbff, 0xfafaff, 0xf9f9ff, 0xf8f8ff, 0xf7f7ff,
        0xf6f6ff, 0xf5f5ff, 0xf4f4ff, 0xf3f3ff, 0xf2f2ff, 0xf1f1ff, 0xf0f0ff, 0xefefff, 0xeeeeff,
        0xededff, 0xececff, 0xebebff, 0xeaeaff, 0xe9e9ff, 0xe8e8ff, 0xe7e7ff, 0xe6e6ff, 0xe5e5ff,
        0xe4e4ff, 0xe3e3ff, 0xe2e2ff, 0xe1e1ff, 0xe0e0ff, 0xdfdfff, 0xdedeff, 0xddddff, 0xdcdcff,
        0xdbdbff, 0xdadaff, 0xd9d9ff, 0xd8d8ff, 0xd7d7ff, 0xd6d6ff, 0xd5d5ff, 0xd4d4ff, 0xd3d3ff,
        0xd2d2ff, 0xd1d1ff, 0xd0d0ff, 0xcfcfff, 0xceceff, 0xcdcdff, 0xccccff, 0xcbcbff, 0xcacaff,
        0xc9c9ff, 0xc8c8ff, 0xc7c7ff, 0xc6c6ff, 0xc5c5ff, 0xc4c4ff, 0xc3c3ff, 0xc2c2ff, 0xc1c1ff,
        0xc0c0ff, 0xbfbfff, 0xbebeff, 0xbdbdff, 0xbcbcff, 0xbbbbff, 0xbabaff, 0xb9b9ff, 0xb8b8ff,
        0xb7b7ff, 0xb6b6ff, 0xb5b5ff, 0xb4b4ff, 0xb3b3ff, 0xb2b2ff, 0xb1b1ff, 0xb0b0ff, 0xafafff,
        0xaeaeff, 0xadadff, 0xacacff, 0xababff, 0xaaaaff, 0xa9a9ff, 0xa8a8ff, 0xa7a7ff, 0xa6a6ff,
        0xa5a5ff, 0xa4a4ff, 0xa3a3ff, 0xa2a2ff, 0xa1a1ff, 0xa0a0ff, 0x9f9fff, 0x9e9eff, 0x9d9dff,
        0x9c9cff, 0x9b9bff, 0x9a9aff, 0x9999ff, 0x9898ff, 0x9797ff, 0x9696ff, 0x9595ff, 0x9494ff,
        0x9393ff, 0x9292ff, 0x9191ff, 0x9090ff, 0x8f8fff, 0x8e8eff, 0x8d8dff, 0x8c8cff, 0x8b8bff,
        0x8a8aff, 0x8989ff, 0x8888ff, 0x8787ff, 0x8686ff, 0x8585ff, 0x8484ff, 0x8383ff, 0x8282ff,
        0x8181ff, 0x8080ff, 0x7f7fff, 0x7e7eff, 0x7d7dff, 0x7c7cff, 0x7b7bff, 0x7a7aff, 0x7979ff,
        0x7878ff, 0x7777ff, 0x7676ff, 0x7575ff, 0x7474ff, 0x7373ff, 0x7272ff, 0x7171ff, 0x7070ff,
        0x6f6fff, 0x6e6eff, 0x6d6dff, 0x6c6cff, 0x6b6bff, 0x6a6aff, 0x6969ff, 0x6868ff, 0x6767ff,
        0x6666ff, 0x6565ff, 0x6464ff, 0x6363ff, 0x6262ff, 0x6161ff, 0x6060ff, 0x5f5fff, 0x5e5eff,
        0x5d5dff, 0x5c5cff, 0x5b5bff, 0x5a5aff, 0x5959ff, 0x5858ff, 0x5757ff, 0x5656ff, 0x5555ff,
        0x5454ff, 0x5353ff, 0x5252ff, 0x5151ff, 0x5050ff, 0x4f4fff, 0x4e4eff, 0x4d4dff, 0x4c4cff,
        0x4b4bff, 0x4a4aff, 0x4949ff, 0x4848ff, 0x4747ff, 0x4646ff, 0x4545ff, 0x4444ff, 0x4343ff,
        0x4242ff, 0x4141ff, 0x4040ff, 0x3f3fff, 0x3e3eff, 0x3d3dff, 0x3c3cff, 0x3b3bff, 0x3a3aff,
        0x3939ff, 0x3838ff, 0x3737ff, 0x3636ff, 0x3535ff, 0x3434ff, 0x3333ff, 0x3232ff, 0x3131ff,
        0x3030ff, 0x2f2fff, 0x2e2eff, 0x2d2dff, 0x2c2cff, 0x2b2bff, 0x2a2aff, 0x2929ff, 0x2828ff,
        0x2727ff, 0x2626ff, 0x2525ff, 0x2424ff, 0x2323ff, 0x2222ff, 0x2121ff, 0x2020ff, 0x1f1fff,
        0x1e1eff, 0x1d1dff, 0x1c1cff, 0x1b1bff, 0x1a1aff, 0x1919ff, 0x1818ff, 0x1717ff, 0x1616ff,
        0x1515ff, 0x1414ff, 0x1313ff, 0x1212ff, 0x1111ff, 0x1010ff, 0x0f0fff, 0x0e0eff, 0x0d0dff,
        0x0c0cff, 0x0b0bff, 0x0a0aff, 0x0909ff, 0x0808ff, 0x0707ff, 0x0606ff, 0x0505ff, 0x0404ff,
        0x0303ff, 0x0202ff, 0x0101ff, 0x0000ff,
    ],
    [
        0x000000, 0x010101, 0x020202, 0x030303, 0x040404, 0x050505, 0x060606, 0x070707, 0x080808,
        0x090909, 0x0a0a0a, 0x0b0b0b, 0x0c0c0c, 0x0d0d0d, 0x0e0e0e, 0x0f0f0f, 0x101010, 0x111111,
        0x121212, 0x131313, 0x141414, 0x151515, 0x161616, 0x171717, 0x181818, 0x191919, 0x1a1a1a,
        0x1b1b1b, 0x1c1c1c, 0x1d1d1d, 0x1e1e1e, 0x1f1f1f, 0x202020, 0x212121, 0x222222, 0x232323,
        0x242424, 0x252525, 0x262626, 0x272727, 0x282828, 0x292929, 0x2a2a2a, 0x2b2b2b, 0x2c2c2c,
        0x2d2d2d, 0x2e2e2e, 0x2f2f2f, 0x303030, 0x313131, 0x323232, 0x333333, 0x343434, 0x353535,
        0x363636, 0x373737, 0x383838, 0x393939, 0x3a3a3a, 0x3b3b3b, 0x3c3c3c, 0x3d3d3d, 0x3e3e3e,
        0x3f3f3f, 0x404040, 0x414141, 0x424242, 0x434343, 0x444444, 0x454545, 0x464646, 0x474747,
        0x484848, 0x494949, 0x4a4a4a, 0x4b4b4b, 0x4c4c4c, 0x4d4d4d, 0x4e4e4e, 0x4f4f4f, 0x505050,
        0x515151, 0x525252, 0x535353, 0x545454, 0x555555, 0x565656, 0x575757, 0x585858, 0x595959,
        0x5a5a5a, 0x5b5b5b, 0x5c5c5c, 0x5d5d5d, 0x5e5e5e, 0x5f5f5f, 0x606060, 0x616161, 0x626262,
        0x636363, 0x646464, 0x656565, 0x666666, 0x676767, 0x686868, 0x696969, 0x6a6a6a, 0x6b6b6b,
        0x6c6c6c, 0x6d6d6d, 0x6e6e6e, 0x6f6f6f, 0x707070, 0x717171, 0x727272, 0x737373, 0x747474,
        0x757575, 0x767676, 0x777777, 0x787878, 0x797979, 0x7a7a7a, 0x7b7b7b, 0x7c7c7c, 0x7d7d7d,
        0x7e7e7e, 0x7f7f7f, 0x808080, 0x818181, 0x828282, 0x838383, 0x848484, 0x858585, 0x868686,
        0x878787, 0x888888, 0x898989, 0x8a8a8a, 0x8b8b8b, 0x8c8c8c, 0x8d8d8d, 0x8e8e8e, 0x8f8f8f,
        0x909090, 0x919191, 0x929292, 0x939393, 0x949494, 0x959595, 0x969696, 0x979797, 0x989898,
        0x999999, 0x9a9a9a, 0x9b9b9b, 0x9c9c9c, 0x9d9d9d, 0x9e9e9e, 0x9f9f9f, 0xa0a0a0, 0xa1a1a1,
        0xa2a2a2, 0xa3a3a3, 0xa4a4a4, 0xa5a5a5, 0xa6a6a6, 0xa7a7a7, 0xa8a8a8, 0xa9a9a9, 0xaaaaaa,
        0xababab, 0xacacac, 0xadadad, 0xaeaeae, 0xafafaf, 0xb0b0b0, 0xb1b1b1, 0xb2b2b2, 0xb3b3b3,
        0xb4b4b4, 0xb5b5b5, 0xb6b6b6, 0xb7b7b7, 0xb8b8b8, 0xb9b9b9, 0xbababa, 0xbbbbbb, 0xbcbcbc,
        0xbdbdbd, 0xbebebe, 0xbfbfbf, 0xc0c0c0, 0xc1c1c1, 0xc2c2c2, 0xc3c3c3, 0xc4c4c4, 0xc5c5c5,
        0xc6c6c6, 0xc7c7c7, 0xc8c8c8, 0xc9c9c9, 0xcacaca, 0xcbcbcb, 0xcccccc, 0xcdcdcd, 0xcecece,
        0xcfcfcf, 0xd0d0d0, 0xd1d1d1, 0xd2d2d2, 0xd3d3d3, 0xd4d4d4, 0xd5d5d5, 0xd6d6d6, 0xd7d7d7,
        0xd8d8d8, 0xd9d9d9, 0xdadada, 0xdbdbdb, 0xdcdcdc, 0xdddddd, 0xdedede, 0xdfdfdf, 0xe0e0e0,
        0xe1e1e1, 0xe2e2e2, 0xe3e3e3, 0xe4e4e4, 0xe5e5e5, 0xe6e6e6, 0xe7e7e7, 0xe8e8e8, 0xe9e9e9,
        0xeaeaea, 0xebebeb, 0xececec, 0xededed, 0xeeeeee, 0xefefef, 0xf0f0f0, 0xf1f1f1, 0xf2f2f2,
        0xf3f3f3, 0xf4f4f4, 0xf5f5f5, 0xf6f6f6, 0xf7f7f7, 0xf8f8f8, 0xf9f9f9, 0xfafafa, 0xfbfbfb,
        0xfcfcfc, 0xfdfdfd, 0xfefefe, 0xffffff,
    ],
    [
        0x000000, 0x000100, 0x000200, 0x000300, 0x000400, 0x000500, 0x000600, 0x000700, 0x000800,
        0x000900, 0x000a00, 0x000b00, 0x000c00, 0x000d00, 0x000e00, 0x000f00, 0x001000, 0x001100,
        0x001200, 0x001300, 0x001400, 0x001500, 0x001600, 0x001700, 0x001800, 0x001900, 0x001a00,
        0x001b00, 0x001c00, 0x001d00, 0x001e00, 0x001f00, 0x002000, 0x002100, 0x002200, 0x002300,
        0x002400, 0x002500, 0x002600, 0x002700, 0x002800, 0x002900, 0x002a00, 0x002b00, 0x002c00,
        0x002d00, 0x002e00, 0x002f00, 0x003000, 0x003100, 0x003200, 0x003300, 0x003400, 0x003500,
        0x003600, 0x003700, 0x003800, 0x003900, 0x003a00, 0x003b00, 0x003c00, 0x003d00, 0x003e00,
        0x003f00, 0x004000, 0x004100, 0x004200, 0x004300, 0x004400, 0x004500, 0x004600, 0x004700,
        0x004800, 0x004900, 0x004a00, 0x004b00, 0x004c00, 0x004d00, 0x004e00, 0x004f00, 0x005000,
        0x005100, 0x005200, 0x005300, 0x005400, 0x005500, 0x005600, 0x005700, 0x005800, 0x005900,
        0x005a00, 0x005b00, 0x005c00, 0x005d00, 0x005e00, 0x005f00, 0x006000, 0x006100, 0x006200,
        0x006300, 0x006400, 0x006500, 0x006600, 0x006700, 0x006800, 0x006900, 0x006a00, 0x006b00,
        0x006c00, 0x006d00, 0x006e00, 0x006f00, 0x007000, 0x007100, 0x007200, 0x007300, 0x007400,
        0x007500, 0x007600, 0x007700, 0x007800, 0x007900, 0x007a00, 0x007b00, 0x007c00, 0x007d00,
        0x007e00, 0x007f00, 0x008000, 0x008100, 0x008200, 0x008300, 0x008400, 0x008500, 0x008600,
        0x008700, 0x008800, 0x008900, 0x008a00, 0x008b00, 0x008c00, 0x008d00, 0x008e00, 0x008f00,
        0x009000, 0x009100, 0x009200, 0x009300, 0x009400, 0x009500, 0x009600, 0x009700, 0x009800,
        0x009900, 0x009a00, 0x009b00, 0x009c00, 0x009d00, 0x009e00, 0x009f00, 0x00a000, 0x00a100,
        0x00a200, 0x00a300, 0x00a400, 0x00a500, 0x00a600, 0x00a700, 0x00a800, 0x00a900, 0x00aa00,
        0x00ab00, 0x00ac00, 0x00ad00, 0x00ae00, 0x00af00, 0x00b000, 0x00b100, 0x00b200, 0x00b300,
        0x00b400, 0x00b500, 0x00b600, 0x00b700, 0x00b800, 0x00b900, 0x00ba00, 0x00bb00, 0x00bc00,
        0x00bd00, 0x00be00, 0x00bf00, 0x00c000, 0x00c100, 0x00c200, 0x00c300, 0x00c400, 0x00c500,
        0x00c600, 0x00c700, 0x00c800, 0x00c900, 0x00ca00, 0x00cb00, 0x00cc00, 0x00cd00, 0x00ce00,
        0x00cf00, 0x00d000, 0x00d100, 0x00d200, 0x00d300, 0x00d400, 0x00d500, 0x00d600, 0x00d700,
        0x00d800, 0x00d900, 0x00da00, 0x00db00, 0x00dc00, 0x00dd00, 0x00de00, 0x00df00, 0x00e000,
        0x00e100, 0x00e200, 0x00e300, 0x00e400, 0x00e500, 0x00e600, 0x00e700, 0x00e800, 0x00e900,
        0x00ea00, 0x00eb00, 0x00ec00, 0x00ed00, 0x00ee00, 0x00ef00, 0x00f000, 0x00f100, 0x00f200,
        0x00f300, 0x00f400, 0x00f500, 0x00f600, 0x00f700, 0x00f800, 0x00f900, 0x00fa00, 0x00fb00,
        0x00fc00, 0x00fd00, 0x00fe00, 0x00ff00,
    ],
];

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
    hw: Option<T>,
    attr: Attr,
    pos: Position,
    mode: ControlCharMode,
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
                glyphs: [(Glyph::Null, Attr::Normal); TEXT_NUM_COLS_INC_BORDER],
            }; TEXT_NUM_ROWS],
            hw: None,
            pos: Position {
                row: Row(0),
                col: Col(0),
            },
            attr: Attr::Normal,
            mode: ControlCharMode::Interpret,
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
            row.glyphs[0] = (Glyph::FullBlock, Attr::Normal);
            row.glyphs[row.glyphs.len() - 1] = (Glyph::FullBlock, Attr::Normal);
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
                for (ch, attr) in self.text_buffer[text_row].glyphs.iter() {
                    let mut w = ch.pixels(font_row);
                    let rgb_addr = (RGB_MAPS.as_ptr() as usize) + ((*attr as usize) * 1024_usize)
                        + (w as usize * 4_usize);
                    let rgb_word = unsafe { core::ptr::read(rgb_addr as *const u32) };
                    hw.write_pixels(rgb_word >> 16, rgb_word >> 8, rgb_word);
                }
            }
        }
    }

    /// Clears the screen and resets the cursor to 0,0.
    pub fn clear(&mut self) {
        for row in self.text_buffer.iter_mut() {
            for slot in row.glyphs.iter_mut().skip(1).take(TEXT_NUM_COLS) {
                *slot = (Glyph::Space, Attr::Normal);
            }
        }
        self.pos = Position::origin();
    }

    /// Puts a glyph on screen at the specified place
    pub fn write_glyph_at(&mut self, glyph: Glyph, pos: Position, attr: Option<Attr>) {
        if (pos.col <= self.get_width()) && (pos.row <= self.get_height()) {
            // Skip over the left border
            self.text_buffer[pos.row.0 as usize].glyphs[pos.col.0 as usize + 1] =
                (glyph, attr.unwrap_or(self.attr));
        }
    }

    /// Puts a glyph on screen at the current position.
    pub fn write_glyph(&mut self, glyph: Glyph, attr: Option<Attr>) {
        // Skip over the left border
        self.text_buffer[self.pos.row.0 as usize].glyphs[self.pos.col.0 as usize + 1] =
            (glyph, attr.unwrap_or(self.attr));
        self.move_cursor_right().unwrap();
    }

    /// Change the current character attribute
    pub fn set_attr(&mut self, attr: Attr) {
        self.attr = attr;
    }
}

impl<T> Console for FrameBuffer<T>
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

    /// Called when the screen needs to scroll up one row.
    fn scroll_screen(&mut self) -> Result<(), Self::Error> {
        for line in 0..TEXT_NUM_ROWS - 1 {
            self.text_buffer[line] = self.text_buffer[line + 1];
        }
        for slot in self.text_buffer[TEXT_MAX_ROW]
            .glyphs
            .iter_mut()
            .skip(1)
            .take(TEXT_NUM_COLS)
        {
            *slot = (Glyph::Space, Attr::Normal);
        }
        Ok(())
    }

    /// Write a single Unicode char to the screen at the given position
    /// without updating the current position.
    fn write_char_at(&mut self, ch: char, pos: Position) -> Result<(), Self::Error> {
        if (pos.col <= self.get_width()) && (pos.row <= self.get_height()) {
            // Skip over the left border
            self.text_buffer[pos.row.0 as usize].glyphs[pos.col.0 as usize + 1] =
                (Glyph::map_char(ch), self.attr);
        }
        Ok(())
    }
}

impl<T> core::fmt::Write for FrameBuffer<T>
where
    T: Hardware,
{
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write_string(s).unwrap();
        Ok(())
    }
}

// End of file
