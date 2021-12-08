#!/usr/bin/python3

with open('in') as f:
    codes = f.readlines()

code_length = len(codes[0]) - 1
place_counts = [0] * code_length

for c in codes:
    c = c.rstrip()
    for i, b in enumerate(c):
        place_counts[i] = place_counts[i] + int(b)

min_bit_count = len(codes) / 2

gamma = int("".join('1' if x > min_bit_count else '0' for x in place_counts), 2)

epsilon = (gamma ^ int(''.join('1' * code_length), 2))

print(gamma * epsilon)