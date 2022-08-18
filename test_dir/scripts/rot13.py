#!/usr/bin/env python3
'''
Applies a rot13 cipher to the given data
Usage: 
    rot13.py in_path out_path
'''
import codecs
import sys

in_path = sys.argv[1]
out_path = sys.argv[2]

# Read the input file and do the rot13 transformation
out_lines = []
with open(in_path, 'r') as in_file:
    for line in in_file.readlines():
        out_lines.append(codecs.encode(line, 'rot_13'))

# Write to the output file
with open(out_path, 'w') as out_file:
    out_file.writelines(out_lines)

print(f'psidb::out_data::path::{out_path}')