#!/usr/bin/python3
import functools

calc_map = {
    'forward': lambda acc, x: (acc[0] + x, acc[1] + (x * acc[2]), acc[2]),
    'down': lambda acc, x: (acc[0], acc[1], acc[2] + x),
    'up': lambda acc, x: (acc[0], acc[1], acc[2] - x)
}

def update_coords(acc, cmd):
    (c, x) = cmd.split(' ')
    return calc_map[c](acc, int(x))

with open('in') as f:
    # (horizontal, depth, aim)
    pos = functools.reduce(update_coords, f.readlines(), (0, 0, 0))
    print(pos[0] * pos[1])