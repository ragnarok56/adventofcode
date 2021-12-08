#!/usr/bin/python3
from collections import Counter

with open('in') as f:
    coords = []
    for line in f.readlines():
        coords.append([list(map(lambda x: int(x), pair.split(','))) for pair in line.split(' -> ')])

vent_map = Counter()
for coord in coords:
    points = []
    x1, x2 = coord[0][0], coord[1][0]
    y1, y2 = coord[0][1], coord[1][1]
    # only check vertical/horizontal lines
    if x1 == x2:
        sorted_coords = sorted(coord, key=lambda x: x[1])
        for y in range(sorted_coords[0][1],sorted_coords[1][1] + 1):
            points.append(f'{x1},{y}')
    elif y1 == y2:
        sorted_coords = sorted(coord, key=lambda x: x[0])
        for x in range(sorted_coords[0][0],sorted_coords[1][0] + 1):
            points.append(f'{x},{y1}')

    for p in points:
        vent_map.update({p: 1})

print(len([p for p in vent_map.values() if p > 1]))