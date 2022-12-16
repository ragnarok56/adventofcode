#!/usr/bin/python3
import re

class Sensor:
    def __init__(self, pos, beacon):
        self.pos = pos
        self.beacon = beacon
        self.d = abs(pos[0] - beacon[0]) + abs(pos[1] - beacon[1])

sensors = []
beacons = []

with open('in') as f:
    for line in f.readlines():
        (s, b) = line.split(':')
        s_match = re.search("Sensor at x=(-?\d+), y=(-?\d+)", s)
        sensor = (int(s_match.group(1)), int(s_match.group(2)))

        b_match = re.search(" closest beacon is at x=(-?\d+), y=(-?\d+)", b)
        beacon = (int(b_match.group(1)), int(b_match.group(2)))
        sensors.append(Sensor(sensor, beacon))
        beacons.append(beacon)

def part1():
    cells_no_beacons = set()
    row = 2000000
    for s in sensors:
        if (s.pos[1] <= row and s.pos[1] + s.d >= row) or (s.pos[1] >= row and s.pos[1] - s.d <= row):
            y_diff = abs(row - s.pos[1])
            remaining = s.d - y_diff
            for x in range(s.pos[0] - remaining, s.pos[0] + remaining + 1):
                spot = (x, row)
                cells_no_beacons.add(spot)

    for b in beacons:
        if b in cells_no_beacons:
            cells_no_beacons.remove(b)

    print(len(cells_no_beacons))

def part2():
    # brute force again, but without dumb "tracking each spot" logic
    bounds = 4000000

    def check_ranges(ranges):
        ranges = iter(sorted(ranges))
        _, current_stop = next(ranges)
        for start, stop in ranges:
            if start > current_stop:
                if start > current_stop + 1:
                    return current_stop
                current_stop = start, stop
            else:
                current_stop = max(current_stop, stop)

        return None

    for row in range(0, bounds):
        ranges = []
        for s in sensors:
            if (s.pos[1] <= row <= s.pos[1] + s.d) or (s.pos[1] >= row >= s.pos[1] - s.d):
                y_diff = abs(row - s.pos[1])
                remaining = s.d - y_diff
                r = max(s.pos[0] - remaining, 0), min(s.pos[0] + remaining, bounds)
                ranges.append(r)
                # print(f'Sensor: {s.pos}, Beacon: {s.beacon}, offset: {y_diff}, rem: {remaining}, range: {r}')

        res = check_ranges(ranges)
        print(row)
        if res is not None:
            print(res * 4000000 + row)
            break

if __name__ == '__main__':
    part1()
    part2()