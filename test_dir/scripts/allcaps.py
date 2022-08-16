#!/usr/bin/env python3
'''
Converts the input data to all caps
Usage:
    allcaps.py in_path out_path
'''
import sys

in_path = sys.argv[1]
out_path = sys.argv[2]

# Read the input file and capitalize the text
out_lines = []
with open(in_path, 'r') as in_file:
    for line in in_file.readlines():
        out_lines.append(line.upper())

# Write to the output file
with open(out_path, 'w') as out_file:
    out_file.writelines(out_lines)
