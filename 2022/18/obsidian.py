#!/usr/bin/python3
import numpy as np

class Droplet:
    def __init__(self, pos):
        self.x = pos[0]
        self.y = pos[1]
        self.z = pos[2]
        self.pos = pos
        self.surface_area = 6

    def check_surface_area(self, other):
        if str(self) == str(other):
            return
        if self.x == other.x and self.y == other.y and abs(self.z - other.z) < 2:
            # print('z', self, other)
            self.surface_area -= 1
            return
        if self.y == other.y and self.z == other.z and abs(self.x - other.x) < 2:
            # print('x', self, other)
            self.surface_area -= 1
            return
        if self.z == other.z and self.x == other.x and abs(self.y - other.y) < 2:
            # print('y', self, other)
            self.surface_area -= 1
            return

    def __repr__(self):
        return f'{self.x},{self.y},{self.z}: [{self.surface_area}]'


with open('in_test') as f:
    droplets = [Droplet([int(x) for x in l.strip().split(',')]) for l in f.readlines()]

    # print(sum(d.surface_area for d in droplets))

    for d1 in droplets:
        if d1.surface_area > 0:
            for d2 in droplets:
                d1.check_surface_area(d2)

    print(sum(d.surface_area for d in droplets))
