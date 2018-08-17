extern crate term;
extern crate vga_framebuffer;

use vga_framebuffer::{Col, Console, Position, Row};

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
                    6 => {
                        self.output.fg(term::color::YELLOW).unwrap();
                    }
                    5 => {
                        self.output.fg(term::color::MAGENTA).unwrap();
                    }
                    4 => {
                        self.output.fg(term::color::GREEN).unwrap();
                    }
                    3 => {
                        self.output.fg(term::color::CYAN).unwrap();
                    }
                    2 => {
                        self.output.fg(term::color::GREEN).unwrap();
                    }
                    1 => {
                        self.output.fg(term::color::BLUE).unwrap();
                    }
                    0 => {
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
        if self.col == vga_framebuffer::HORIZONTAL_WORDS {
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
    let mut fb = Box::new(vga_framebuffer::FrameBuffer::new());
    let max_col = Col(vga_framebuffer::TEXT_MAX_COL as u8);
    let max_row = Row(vga_framebuffer::TEXT_MAX_ROW as u8);
    fb.init(&mut d);
    fb.clear();
    fb.write_char_at('$', Position::origin()).unwrap();
    fb.write_char_at('$', Position::new(max_row, Col::origin()))
        .unwrap();
    fb.write_char_at('$', Position::new(Row::origin(), max_col))
        .unwrap();
    fb.write_char_at('$', Position::new(max_row, max_col))
        .unwrap();
    writeln!(fb, "\nThis is a test").unwrap();
    for _ in 0..628 {
        fb.isr_sol();
    }

    let mut graphics_buffer = vec![0u8; (384 / 8) * 144];
    graphics_buffer[0] = 0xAA;
    graphics_buffer[47] = 0x55;
    graphics_buffer[0 + (143 * 48)] = 0xF0;
    graphics_buffer[47 + (143 * 48)] = 0x0F;

    // Attach a graphical buffer at a scan-line. It is interpreted as
    // being a grid 48 bytes wide and as long as given. Each line
    // is output twice. We've attached it to scan-line 100.
    fb.mode2(&mut graphics_buffer[..], 100);

    for _ in 0..628 {
        fb.isr_sol();
    }
}
