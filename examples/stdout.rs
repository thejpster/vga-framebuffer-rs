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
    let mut tfb = vga_framebuffer::TextFrameBuffer::new(&mut fb);
    tfb.clear();
    tfb.hollow_rectangle(
        vga_framebuffer::Point(10, 10),
        vga_framebuffer::Point(390, 290),
        true,
    );
    tfb.line(
        vga_framebuffer::Point(10, 10),
        vga_framebuffer::Point(390, 290),
        true,
    );
    tfb.line(
        vga_framebuffer::Point(390, 10),
        vga_framebuffer::Point(10, 290),
        true,
    );
    tfb.write_char_at('$', 0, 0, false);
    tfb.write_char_at('$', 0, vga_framebuffer::TEXT_MAX_ROW, false);
    tfb.write_char_at('$', vga_framebuffer::TEXT_MAX_COL, 0, false);
    tfb.write_char_at(
        '$',
        vga_framebuffer::TEXT_MAX_COL,
        vga_framebuffer::TEXT_MAX_ROW,
        false,
    );
    writeln!(tfb, "\nThis is a test").unwrap();
    let bitmap: [u8; 8] = [
        0b00111100, 0b01000010, 0b10100101, 0b10000001, 0b10100101, 0b10011001, 0b01000010,
        0b00111100,
    ];
    for x in 0..10 {
        for y in 0..10 {
            let p = vga_framebuffer::Point(100 + x * 10, 100 + y * 10);
            tfb.draw_bitmap(p, 8, &bitmap);
        }
    }
    for _ in 0..628 {
        tfb.isr_sol();
        tfb.isr_data();
    }
}
