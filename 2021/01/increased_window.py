#!/usr/bin/python3
import functools

measurements = []
with open('in') as f:
    measurements = [int(x) for x in f.readlines()]

measure_threes = []
for i in range(0, len(measurements) - 2):
    measure_threes.append(sum(measurements[i: i + 3]))

print(functools.reduce(lambda acc, x: (x, acc[1] + 1 if x > acc[0] else acc[1]), measure_threes, (0, -1))[1])
