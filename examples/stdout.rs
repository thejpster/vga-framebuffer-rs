extern crate vga_framebuffer;

struct Dummy;

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

    /// Called when pixels can be buffered for this line.
    fn buffer_pixels(&mut self, _pixels: &vga_framebuffer::VideoLine) {}

    /// Called when pixels need to be written to the output pin.
    fn write_pixels(&mut self, pixels: &vga_framebuffer::VideoLine) {
        for word in &pixels.words {
            for bit in (0..16).rev() {
                if word & (1 << bit) != 0 {
                    print!("@@");
                } else {
                    print!("..");
                }
            }
        }
        println!();
    }
}

use std::fmt::Write;

fn main() {
    let mut d = Dummy {};
    let mut fb = Box::new(vga_framebuffer::FrameBuffer::new());
    fb.init(&mut d);
    fb.clear();
    fb.write_char_at('$', 0, 0, false);
    fb.write_char_at('$', 0, vga_framebuffer::TEXT_MAX_ROW, false);
    fb.write_char_at('$', vga_framebuffer::TEXT_MAX_COL, 0, false);
    fb.write_char_at(
        '$',
        vga_framebuffer::TEXT_MAX_COL,
        vga_framebuffer::TEXT_MAX_ROW,
        false,
    );
    writeln!(fb, "\nThis is a test").unwrap();
    for _ in 0..628 {
        fb.isr_sol();
        fb.isr_data();
    }
}
