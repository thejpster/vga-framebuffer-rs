# VGA Framebuffer

This crate implements basic VGA text output on an embedded microcontroller,
using nothing more than some timer peripherals and a few GPIO pins.

This crate is written to be hardware-agnostic.

* You supply an object implementing the `Hardware` trait.
* The function `isr_sol()` must be called at the start of every scan-line,
  e.g. with a Timer interrupt.
* This will then call `write_pixels()` on the given `Hardware` object with the
  RGB values for every byte in that scan-line.
* These pixel values should be buffered (e.g. in an SPI FIFO) and then sent to
  the screen at the appropriate moment (e.g. when the `line_start` timer
  fires).
* The `configure()` callback gives you the appropriate timings for all of
  this.

## The Specs:

* 800 x 600 @ 60 Hz output signal with a 40 MHz pixel clock.
* 8-colour RGB output (Red, Green, Blue, Cyan, Magenta, Yellow, Black and White).
* Double-width horizontal pixels, giving 400 x 600 output.
* 8x16 font glyphs.
* MS-DOS CodePage 850 character set.
* 48 column x 36 row text output.
* One foreground/background pair stored *per* character cell.
* Single-buffered.
	* Uses `unsafe` mutation of a static, but the worst case scenario is a
	  minor screen glitch which is fixed on the next frame.

The pixels are double-width as I was unable to get 40 MHz SPI output
functioning correctly on my particular micro. 20 MHz works just fine.

Tested on [Monotron](https://github.com/thejpster/monotron) using a Texas
Instruments Stellaris LM4F120 clocked at 80 MHz.

## Coming soon...

* Optional Mono 384x288 framebuffer, coloured with text-mode attributes.
	* Doubles each horizontal line to keep memory usage down.
	* Uses an extra 13,824 bytes over text mode.
	* Will suffer terrible attribute-clash, just like a ZX Spectrum :)
* Optional 3-bits-per-pixel RGB 192x288 framebuffer mode.
	* Uses an extra 20,736 bytes over text mode.
	* No attribute clash, but half the resolution.
* 8x8 pixel font suitable for either of the above.
* Set a start and end scan-line for the graphics mode:
	* Allows a mixed text/graphics split-screen with reduced memory consumption.

## Contributing

I'll happy accept a patches to enable other resolutions and/or other text
resolutions.
