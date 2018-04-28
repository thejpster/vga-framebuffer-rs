#!/usr/bin/env python3

import codecs
import sys
import unicodedata

FONT_HEIGHT = 16
FONT_WIDTH = 8

cp850 = codecs.lookup("cp850")

file = sys.argv[1]
with open(file, "rb") as f:
	data = f.read()

chunk_size = int((FONT_WIDTH * FONT_HEIGHT) / 8)
glyphs = [data[i:i + chunk_size] for i in range(0, len(data), chunk_size)]

rows = []
for row in range(0, FONT_HEIGHT):
	rows.append([])

for no, glyph in enumerate(glyphs):
	print("Decoding {}".format(no))
	uni = cp850.decode(bytes([no]))
	name = unicodedata.name(uni[0], "UNKNOWN")
	camel_name = "".join(map(str.title, name.replace("-", " ").split()))
	print("Glyph {} ({}) = {} (Glyph::{})\n".format(no, uni, name, camel_name))
	for n in range(0, FONT_HEIGHT):
		rows[n].append(glyph[n])

print("pub const FONT_DATA: [[u8; 256]; 16] = [")
for (n, row) in enumerate(rows):
	print("    [ {} ],".format(",".join(map(lambda x: "0x%02x" % x, row))))
print("]")