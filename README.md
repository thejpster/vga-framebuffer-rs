# VGA Framebuffer

This crate implements a basic VGA framebuffer on an embedded microcontroller, using nothing more than some timer peripherals and a few GPIO pins.

## The Specs:

* 800 x 600 @ 60 Hz output signal
* Monochrome (your choice of red, green or blue)
* 400 x 300 pixel framebuffer
* Requires just under 15 KiB of RAM
* Single-buffered
* Line and box drawing

