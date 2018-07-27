#!/usr/bin/env python3

import codecs
import sys
import unicodedata

cp850 = codecs.lookup("cp850")

file = sys.argv[1]
with open(file, "rb") as f:
	data = f.read()

if "8x8" in file:
	chunk_size = int((8 * 8) / 8)
elif "8x16" in file:
	chunk_size = int((8 * 16) / 8)
else:
	raise Exception("Can't understand filename")

glyphs = [data[i:i + chunk_size] for i in range(0, len(data), chunk_size)]

for no, glyph in enumerate(glyphs):
	uni = cp850.decode(bytes([no]))
	name = unicodedata.name(uni[0], "UNKNOWN")
	camel_name = "".join(map(str.title, name.replace("-", " ").split()))
	print("*" * 10)
	print("Glyph {} ({}) = {} (Glyph::{})".format(no, uni, name, camel_name))
	print("    {}, // Glyph::{}".format(",".join(map(lambda x: "0x%02x" % x, glyph)), camel_name))
	for row in glyph:
		for bit in range(0, 8):
			if row & 1 << (7 - bit):
				sys.stdout.write("X")
			else:
				sys.stdout.write(" ")
		print("")
	print("*" * 10)
