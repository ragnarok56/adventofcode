#!/usr/bin/python3
import functools

measurements = []
with open('in') as f:
    measurements = [int(x) for x in f.readlines()]

print(functools.reduce(lambda acc, x: (x, acc[1] + 1 if x > acc[0] else acc[1]), measurements, (0, -1))[1])
