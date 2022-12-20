#!/usr/bin/python3

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

    for d1 in droplets:
        if d1.surface_area > 0:
            for d2 in droplets:
                d1.check_surface_area(d2)

    print(sum(d.surface_area for d in droplets))

    covered_droplets = [d for d in droplets if d.surface_area < 1]

    print(covered_droplets)
    print(len(covered_droplets))

    # minx = min([d.x for d in droplets])
    # maxx = max([d.x for d in droplets])
    # miny = min([d.y for d in droplets])
    # maxy = max([d.y for d in droplets])
    # minz = min([d.z for d in droplets])
    # maxz = max([d.z for d in droplets])

    # droplet_set = set([f'{d.x},{d.y},{d.z}' for d in droplets])

    # print(minx, maxx, miny, maxy, minz, maxz)

    # for x in range(minx, maxx):
    #     for y in range(miny, maxy):
    #         for z in range(minz, maxz):
    #             if f'{x},{y},{z}' not in droplet_set:
    #                 print(x, y, z)

