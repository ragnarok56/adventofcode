#!/usr/bin/python3
import functools

calc_map = {
    'forward': lambda acc, x: (acc[0] + x, acc[1]),
    'down': lambda acc, x: (acc[0], acc[1] + x),
    'up': lambda acc, x: (acc[0], acc[1] - x)
}

def update_coords(acc, cmd):
    (c, x) = cmd.split(' ')
    return calc_map[c](acc, int(x))

with open('in') as f:
    # (horizontal, depth)
    pos = functools.reduce(update_coords, f.readlines(), (0, 0))
    print(pos[0] * pos[1])