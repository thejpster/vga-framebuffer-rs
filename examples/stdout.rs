extern crate term;
extern crate vga_framebuffer;

use vga_framebuffer::{Attr, Col, Colour, AsciiConsole, Position, Row};

mod rust_logo;

struct Dummy {
    col: usize,
    output: Box<dyn term::Terminal<Output = std::io::Stdout>>,
}

impl<'a> vga_framebuffer::Hardware for &'a mut Dummy {
    fn configure(&mut self, width: u32, sync_end: u32, line_start: u32, clock_rate: u32) {
        println!(
            "width={}, sync_end={}, line_start={}, clock_rate={}",
            width, sync_end, line_start, clock_rate
        );
    }

    /// Called when V-Sync needs to be high.
    fn vsync_on(&mut self) {
        println!("vsync_on");
    }

    /// Called when V-Sync needs to be low.
    fn vsync_off(&mut self) {
        println!("vsync_off");
    }

    /// Called when pixels need to be written to the output pin.
    fn write_pixels(&mut self, red: u32, green: u32, blue: u32) {
        let mut old_colour = None;
        for bit in (0..8).rev() {
            let red_bit = red & (1 << bit) != 0;
            let blue_bit = blue & (1 << bit) != 0;
            let green_bit = green & (1 << bit) != 0;
            let colour: u8 = ((red_bit as u8) << 2) + ((green_bit as u8) << 1) + (blue_bit as u8);
            if old_colour != Some(colour) {
                match colour {
                    0b110 => {
                        self.output.fg(term::color::YELLOW).unwrap();
                    }
                    0b101 => {
                        self.output.fg(term::color::MAGENTA).unwrap();
                    }
                    0b100 => {
                        self.output.fg(term::color::RED).unwrap();
                    }
                    0b011 => {
                        self.output.fg(term::color::CYAN).unwrap();
                    }
                    0b010 => {
                        self.output.fg(term::color::GREEN).unwrap();
                    }
                    0b001 => {
                        self.output.fg(term::color::BLUE).unwrap();
                    }
                    0b000 => {
                        self.output.fg(term::color::BLACK).unwrap();
                    }
                    _ => {
                        self.output.fg(term::color::WHITE).unwrap();
                    }
                }
                old_colour = Some(colour);
            }
            write!(self.output, "@@").unwrap();
        }
        self.col += 1;
        if self.col == vga_framebuffer::HORIZONTAL_OCTETS {
            self.col = 0;
            println!();
        }
    }
}

use std::fmt::Write;

fn main() {
    let mut d = Dummy {
        col: 0,
        output: term::stdout().unwrap(),
    };
    let mut mode2_buffer = vec![
        0xAAu8;
        vga_framebuffer::USABLE_HORIZONTAL_OCTETS
            * vga_framebuffer::USABLE_LINES_MODE2
    ];
    let mut fb = vga_framebuffer::FrameBuffer::new();
    let max_col = Col(vga_framebuffer::TEXT_MAX_COL as u8);
    let max_row = Row(vga_framebuffer::TEXT_MAX_ROW as u8);
    fb.init(&mut d);
    fb.clear();
    fb.write_char_at(b'$', Position::origin()).unwrap();
    fb.write_char_at(b'$', Position::new(max_row, Col::origin()))
        .unwrap();
    fb.write_char_at(b'$', Position::new(Row::origin(), max_col))
        .unwrap();
    fb.write_char_at(b'$', Position::new(max_row, max_col))
        .unwrap();
    writeln!(fb, "\nThis is a test").unwrap();
    for _ in 0..628 {
        fb.isr_sol();
    }

    let mut wheel = [Colour::Red, Colour::Green, Colour::Blue].iter().cycle();
    for y in 0..=vga_framebuffer::TEXT_MAX_ROW {
        for x in 0..=vga_framebuffer::TEXT_MAX_COL {
            fb.set_attr_at(
                Position::new(Row(y as u8), Col(x as u8)),
                Attr::new(Colour::White, *wheel.next().unwrap()),
            );
        }
        wheel.next();
    }

    for (src, dest) in rust_logo::RUST_LOGO_DATA
        .iter()
        .zip(mode2_buffer.iter_mut())
    {
        // Our source is an X-Bitmap, which puts the pixels in LSB-first order.
        // We need MSB first order for Monotron.
        *dest = flip_byte(*src);
    }

    // Attach a graphical buffer at a scan-line. It is interpreted as
    // being a grid 48 bytes wide and as long as given. Each line
    // is output twice. We've attached it to the first scan-line.
    fb.mode2(&mut mode2_buffer[..], 0);

    for _ in 0..628 {
        fb.isr_sol();
    }

    let _ = fb.mode2_release();

    fb.clear();

    fb.set_custom_font(Some(&vga_framebuffer::Font8x8)).unwrap();

    writeln!(fb, "This is small text").unwrap();

    for _ in 0..628 {
        fb.isr_sol();
    }

    fb.set_custom_font(Some(&vga_framebuffer::Font8x32)).unwrap();

    fb.clear();
    writeln!(fb, "This is big text").unwrap();

    for _ in 0..628 {
        fb.isr_sol();
    }

    fb.set_custom_font(None).unwrap();

    fb.clear();
    // You have to put double-height text in twice, once for the top line and once for the bottom line.
    writeln!(fb, "\u{001b}^\u{001b}k\u{001b}RThis is double height text").unwrap();
    writeln!(fb, "\u{001b}v\u{001b}k\u{001b}GThis is double height text").unwrap();
    writeln!(fb, "\u{001b}-\u{001b}k\u{001b}WThis is normal height text").unwrap();

    for _ in 0..628 {
        fb.isr_sol();
    }
}

fn flip_byte(mut b: u8) -> u8 {
    b = (b & 0xF0) >> 4 | (b & 0x0F) << 4;
    b = (b & 0xCC) >> 2 | (b & 0x33) << 2;
    (b & 0xAA) >> 1 | (b & 0x55) << 1
}

