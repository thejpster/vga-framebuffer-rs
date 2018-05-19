#!/bin/python

"""Generates RGB maps."""

import sys

WHITE = 0b111
YELLOW = 0b110
CYAN = 0b011
MAGENTA = 0b101
RED = 0b100
GREEN = 0b010
BLUE = 0b001
BLACK = 0b000

def calc_bit(fg, bg, p):
	return {
		(1, 1): 0xFF,
		(1, 0): p,
		(0, 1): ~p,
		(0, 0): 0
	}[(fg, bg)]

def calc_rgb(fg, bg):
	def inner(p):
		r = calc_bit((fg >> 2) & 1, (bg >> 2) & 1, p)
		g = calc_bit((fg >> 1) & 1, (bg >> 1) & 1, p)
		b = calc_bit((fg >> 0) & 1, (bg >> 0) & 1, p)
		return (r, g, b)
	return inner

def white_on_blue(p):
	return (p, p, 0xFF)

def white_on_black(p):
	return (p, p, p)

def blue_on_white(p):
	return (~p, ~p, 0xFF)

def green_on_black(p):
	return (0, p, 0)

def generate(gen_fun):
	sys.stdout.write("[\n")
	for p in range(0, 256):
		r, g, b = gen_fun(p)
		word = ((r & 0xFF) << 16) | ((g & 0xFF) << 8) | ((b & 0xFF) << 0)
		sys.stdout.write("0x{:06x}, ".format(word))
		if ((p + 1) % 8) == 0:
			sys.stdout.write("\n")
	sys.stdout.write("],\n")

sys.stdout.write("[\n")

for fg in range(0, 8):
	for bg in range(0, 8):
		sys.stdout.write("// FG RGB = {:03b}, BG RGB = {:03b}\n".format(fg, bg))
		generate(calc_rgb(fg, bg))

sys.stdout.write("];")
