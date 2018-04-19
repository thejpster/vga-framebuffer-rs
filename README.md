# VGA Framebuffer

This crate implements basic VGA text output on an embedded microcontroller,
using nothing more than some timer peripherals and a few GPIO pins.

## The Specs:

* 800 x 600 @ 60 Hz output signal with a 40 MHz pixel clock.
* Monochrome (your choice of red, green or blue)
* Double-width horizontal pixels, giving 400 x 600 output
* 8x16 font glyphs
* 40 column x 36 row text output
* Single-buffered

The pixels are double-width as I was unable to get 40 MHz SPI output
functioning correctly on my particular micro, while 20 MHz works just fine.

## Contributing

I'll happy accept a patches to enable other resolutions and/or other text
resolutions.
