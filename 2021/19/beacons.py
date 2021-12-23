import itertools
import collections

scanners = []
with open('in') as f:
    scanner_beacons = map(lambda x: x.strip(), ''.join(f.readlines()).replace('\n', ' ').split('---')[::2][1:])
    for sb in scanner_beacons:
        scanners.append([list(map(int, x.split(','))) for x in sb.split(' ')])

def rotate(coord, params, p=None):
    (_, offset, invert, rm, i, j) = params
    for _ in range(offset):
        coord = coord[1:3] + [coord[0]]
    if (j + i) % 2 != 0:
        coord = [coord[0], coord[2], coord[1]]
    if p:
        return f'{offset} {invert}, {rm}'
    return ([coord[0] * invert, coord[1] * rm[0], coord[2] * rm[1]], offset, invert, rm, i, j)

def gen_r():
    for offset in range(0, 3):
        for i, invert in enumerate([1, -1]):
            for j, rm in enumerate([[1,1],[-1,1],[-1,-1],[1,-1]]):
                def yield_rotate(coord):
                    return rotate(coord, (None, offset, invert, rm, i, j))
                yield yield_rotate
          

scanner_coords = []
for (a1, b1) in itertools.permutations(enumerate(scanners), 2):
    print(f'{a1[0]} -- {b1[0]}')

    track = collections.defaultdict(list)
    for a in a1[1]:
        for r in gen_r():
            # compare 1 rotation with all beacons
            for b in b1[1]:
                diff = (a[0] - r(b)[0][0],a[1] - r(b)[0][1],a[2] - r(b)[0][2])
                track[diff].append((r(b), r))

    for k, v in sorted(track.items(), key=lambda x: len(x[1])):
        if len(v) == 12:
            scanner_coords.append((a1[0], b1[0], k, v[1][0]))

# s[0] = relative to
# s[1] = scanner
# s[2] = scanner position relative to s[0]

all_beacon_coords = []

def search(start, end, visited):
    if start not in visited:
        visited.add(start)
        scanners_from_start = [s for s in scanner_coords if s[0] == start]
        for s in scanners_from_start:
            if s[0] == start and s[1] == end:
                fixed_beacons = []
                for b in scanners[end]:
                    rotated = rotate([b[0], b[1], b[2]], s[3])[0]
                    fixed_beacon = [rotated[0] + s[2][0], rotated[1] + s[2][1], rotated[2] + s[2][2]]
                    fixed_beacons.append(fixed_beacon)
                return (s[2], fixed_beacons)
            res = search(s[1], end, visited)
            if res:
                (scanner, beacons) = res
                rotated = rotate([scanner[0], scanner[1], scanner[2]], s[3])[0]
                fixed_scanner = [rotated[0] + s[2][0], rotated[1] + s[2][1], rotated[2] + s[2][2]]
                fixed_beacons = []
                for b in beacons:
                    rotated = rotate([b[0], b[1], b[2]], s[3])[0]
                    fixed_beacon = [rotated[0] + s[2][0], rotated[1] + s[2][1], rotated[2] + s[2][2]]
                    fixed_beacons.append(fixed_beacon)

                return (fixed_scanner, fixed_beacons)

all_beacon_coords = [(b[0], b[1], b[2]) for b in scanners[0]]
all_scanner_coords = [(0, 0, 0)]

for i, s in enumerate(scanners):
    if i == 0:
        continue
    visited = set()
    print(f'checking scanner {i}')
    scanner, beacons = search(0, i, visited)
    all_scanner_coords.append(scanner)
    for b in [(b[0], b[1], b[2]) for b in beacons]:
        if b not in all_beacon_coords:
            all_beacon_coords.append(b)

print(len(all_beacon_coords))

largest_distance = 0
for a, b in itertools.combinations(all_scanner_coords, 2):
    distance = abs(a[0]-b[0]) + abs(a[1]-b[1]) + abs(a[2]-b[2])
    if distance > largest_distance:
        largest_distance = distance

print(largest_distance)
