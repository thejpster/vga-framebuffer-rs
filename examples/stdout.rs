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

    /// Called when pixels need to be written to the output pin.
    fn write_pixels(&mut self, pixels: &vga_framebuffer::VideoLine) {
        for word in &pixels.words {
            for bit in (0..16).rev() {
                if word & (1 << bit) != 0 {
                    // FULL BLOCK
                    print!("\u{2588}\u{2588}");
                } else {
                    // LIGHT SHADE
                    print!("\u{2591}\u{2591}");
                }
            }
        }
        println!();
    }
}

fn main() {
    let mut d = Dummy {};
    let mut fb = Box::new(vga_framebuffer::FrameBuffer::new());
    fb.init(&mut d);
    fb.clear();
    fb.hollow_rectangle(
        vga_framebuffer::Point(10, 10),
        vga_framebuffer::Point(390, 290),
        true,
    );
    fb.line(
        vga_framebuffer::Point(10, 10),
        vga_framebuffer::Point(390, 290),
        true,
    );
    fb.line(
        vga_framebuffer::Point(390, 10),
        vga_framebuffer::Point(10, 290),
        true,
    );
    for _ in 0..628 {
        fb.isr_sync();
        fb.isr_data();
    }
}
