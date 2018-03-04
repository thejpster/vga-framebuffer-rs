# VGA Framebuffer

This crate implements a basic VGA framebuffer on an embedded microcontroller,
using nothing more than some timer peripherals and a few GPIO pins.

## The Specs:

* 800 x 600 @ 60 Hz output signal with a 40 MHz pixel clock.
* Monochrome (your choice of red, green or blue)
* 400 x 300 pixel framebuffer
* Requires just under 15 KiB of RAM
* Single-buffered
* Line and box drawing

## Contributing

I'd happy accept a patch to enable 640 x 480 @ 60Hz (enabled with a `cfg`
flag), but it requires a 36 MHz pixel clock and my test board can't do that so
I can't test it.
