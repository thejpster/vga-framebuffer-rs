# VGA Framebuffer

This crate implements basic VGA text output on an embedded microcontroller,
using nothing more than some timer peripherals and a few GPIO pins.

This crate is written to be hardware-agnostic.

* You supply an object implementing the `Hardware` trait.
* The function `isr_sol()` must be called at the start of every scan-line,
  e.g. with a Timer interrupt.
* This will then call `write_pixels()` on the given `Hardware` object with the
  RGB values for every octet (or block of 8 pixels) in that scan-line.
* These pixel values should be buffered (e.g. in an SPI FIFO) and then sent to
  the screen at the appropriate moment (e.g. when the `line_start` timer
  fires).
* The `configure()` callback gives you the appropriate timings for all of
  this.

The example code renders to a console using ANSI escape sequences to set the
colour and printing a '█' character of the appropriate colour for each pixel.
Obviously on a system with real VGA output you need to send the pixels to the
display as analog values between 0V and 0.7V, along with appropriate Horizontal
and Vertical sync signals. That is left as a platform-specific exercise for the
reader, but the [Monotron] is one example for a specific embedded platform.

## The Specs:

* 800 x 600 @ 60 Hz output signal with a 40 MHz pixel clock
* 400 x 600 effective resolution in text mode (runs pixel clock at half-speed)
* 8-colour RGB output (Red, Green, Blue, Cyan, Magenta, Yellow, Black and White)
* 8 pixel by 16 pixel bitmap font, taken from FreeBSD
* MS-DOS CodePage 850 character set
* 48 column x 36 row text output
* Double-height text support
* One foreground/background pair stored *per* character cell
* Single-buffered
	* Mutates shared memory through a pointer, but the worst case scenario is a
	  minor screen glitch which is fixed on the next frame
* Optional Mono 384x288 framebuffer, coloured with text-mode attributes.
	* Doubles each horizontal line to keep memory usage down
	* Uses an extra 13,824 bytes over text mode
	* Will suffer terrible attribute-clash, just like a ZX Spectrum :)
	* Can set a start and end scan-line for the graphics mode, to allow a mixed
	  text/graphics split-screen with reduced memory consumption

The pixels are double-width as I didn't have the CPU power to render colour
pixels at 40 MHz. I do have experimental 40 MHz support in a branch, but only in
black-and-white.

Tested on [Monotron] using a Texas
Instruments Tiva-C TM4C123, which has a Cortex-M4F core clocked at 80 MHz.

[Monotron]: https://github.com/thejpster/monotron

## Coming soon...

* Optional 3-bits-per-pixel RGB 192x288 framebuffer mode.
	* Uses an extra 20,736 bytes over text mode.
	* No attribute clash, but half the resolution.
* 8 pixel by 8 pixel font, more suited to the graphics modes
* Borderless 80 column by 25 row monochrome text mode (using a 10x24 bitmap
  font)

## Contributing

I'll happy accept a patches to enable other resolutions and/or other text
resolutions.
