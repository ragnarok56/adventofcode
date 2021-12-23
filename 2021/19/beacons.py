import numpy as np
import itertools
import collections
import math as m
# from scipy.spatial.transform import Rotation as R

scanners = []
with open('in_test2') as f:
    scanner_beacons = map(lambda x: x.strip(), ''.join(f.readlines()).replace('\n', ' ').split('---')[::2][1:])
    for sb in scanner_beacons:
        scanners.append([list(map(int, x.split(','))) for x in sb.split(' ')])

rotation_mult = list(set(itertools.permutations([1,1,-1,-1], 2)))
sm = []
origin = np.array([0,0,0])

test = np.array([808, -476, -593])

def rotate(coord, offset, invert, rm, i, j, p=None):
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
                # print(f'{offset}, {invert}, {(rm[0] * invert, rm[1] * invert)}')
                def yield_rotate(coord):
                    return rotate(coord, offset, invert, rm, i, j)
                yield yield_rotate
                # def rotate(coord, p=None):
                #     for _ in range(offset):
                #         coord = coord[1:3] + [coord[0]]
                #     if (j + i) % 2 != 0:
                #         coord = [coord[0], coord[2], coord[1]]
                #     if p:
                #         return f'{offset} {invert}, {rm}'
                #     return ([coord[0] * invert, coord[1] * rm[0], coord[2] * rm[1]], offset, invert, rm)
                # yield rotate
                    

for r in gen_r():
    print(f'{r([1, 2, 3])}')
# exit(0)



# def Rx(theta):
#   return np.array([[ 1, 0           , 0           ],
#                    [ 0, m.cos(theta),-m.sin(theta)],
#                    [ 0, m.sin(theta), m.cos(theta)]])
  
# def Ry(theta):
#   return np.array([[ m.cos(theta), 0, m.sin(theta)],
#                    [ 0           , 1, 0           ],
#                    [-m.sin(theta), 0, m.cos(theta)]])
  
# def Rz(theta):
#   return np.array([[ m.cos(theta), -m.sin(theta), 0 ],
#                    [ m.sin(theta), m.cos(theta) , 0 ],
#                    [ 0           , 0            , 1 ]])

# axes, 0 = x, 1 = y, 2 = z
# def gen_rotations():
#     for x in range(0,3):
#         for y in range(0, 3):
#             for z in range(0, 3):
#                 def rotate(coord):
#                     # print(coord)
#                     # print(f'{x}, {y}, {z}, {x1}, {y1}')
#                     a = Rx(m.radians(90 * x)) * coord
#                     # print(a)
#                     print(m.radians(90 * y))
#                     b = Ry(m.radians(90 * y)) * a
#                     c = Rz(m.radians(90 * z)) * b
#                     return c
#                 yield rotate

def gen_rot():
    x = lambda x: x[0]
    y = lambda x: x[1]
    z = lambda x: x[2]
    rotations = [
        lambda c: (x(c), y(c), z(c)),
        lambda c: (x(c), -z(c), y(c)),
        lambda c: (x(c), -y(c), -z(c)),
        lambda c: (x(c), z(c), -y(c)),
        lambda c: (-x(c), -y(c), z(c)),
        lambda c: (-x(c), -z(c), -y(c)),
        lambda c: (-x(c), y(c), -z(c)),
        lambda c: (-x(c), z(c), y(c)),
        lambda c: (-z(c), x(c), -y(c))
    ]
    for r in rotations:
        yield r

# for r in gen_rot():
#     print(f'{r((1, 2, 3))}')
# exit(0)
# result = set()
# for r in gen_rotations():
#     print(f'{r(test)}')
    # result.add(r(test))
# print(len(result))

# print(len(list(gen_rotations())))
# exit(1)
# def gen_rotate():
#     for offset in range(3):
#         for r in invert_rotations:
#             def rotate(coord):
#                 # shift "offset" times
#                 for _ in range(offset):
#                     coord = coord[1:3] + [coord[0]]
#                 return (coord[0] * r[0], coord[1] * r[1], coord[2] * r[2])

#             yield (rotate, offset, r)

# for r in gen_rotate():
#     print(f'{r[1], r[2]}')
# exit(1)
    # def rotate(coord):
    #     for _ in range(3):
    #         coord = coord[1:3] + [coord[0]]
    #         for r in invert_rotations:
    #             yield (coord[0] * r[0], coord[1] * r[1], coord[2] * r[2])

    # return rotate

def create_rotations(scanners):
    scanner_map = {}

    for i, s in enumerate(scanners):
        print(f'scanner: {i}')
        all_rotations = []
        for coord in s:
            rotation_set = set(rotate(coord))
            all_rotations.append(rotation_set)

        scanner_map[i] = all_rotations

    print(scanner_map)
    return scanner_map

scanner_coords = []
for (a1, b1) in itertools.permutations(enumerate(scanners), 2):
    # print(f'{a1} -- {b1}')

    track = collections.defaultdict(list)
    for a in a1[1]:
        for r in gen_r():
            # compare 1 rotation with all beacons
            for b in b1[1]:
                diff = (a[0] - r(b)[0][0],a[1] - r(b)[0][1],a[2] - r(b)[0][2])
                track[diff].append((r(b), r))
                # print(f'{a} | {r(b)} -- {diff}')
                # for br in r(b):
                #     print(f'{a} | {br} -- {a[0] - br[0]},{a[1] - br[1]},{a[2] - br[2]}')
    # print('  matches:')
    # matches = [v for v in sorted(track.items(), key=lambda x: len(x[1])) if len(v) > 1]
    # matches = 0
    # print(f'{a1[0]}:{b1[0]}')

    for k, v in sorted(track.items(), key=lambda x: len(x[1])):
        if len(v) == 12:
            scanner_coords.append((a1[0], b1[0], k, v[1][0]))
            # print(f'{a1[0]}:{b1[0]} [{len(v)}]    {k} - {v}')

for s in scanner_coords:
    print(f'{s[0]} [{s[1]}] - {s[2]} rotation: {s[3]}')

scanner_map = {}
# for s in scanner_coords:
#     scanner_map[s[0]].update({
#         s[1]: s[2]
#     }

def search(start, end, visited=None, r=None):
    print(f'{start} -> {end}: {r}')
    for s in scanner_coords:
        if s[0] in visited:
            continue
        visited.append(s[0])
        print(visited)
        if s[0] == start:
            if s[0] == start and s[1] == end:
                print('found it')
                # rotated = r([s[2][0], s[2][1], s[2][2]], True) if r else s[2]
                rotated = rotate(s[2], r[1], r[2], r[3], r[4], r[5])[0] if r else s[2]
                # print(rotated)
                # print(res)
                return rotated, visited
            res, visited = search(s[1], end, visited, s[3])
            if res:
                # print(r)
                # rotated = r(s[2]) if r else s[2]
                rotated = rotate(s[2], r[1], r[2], r[3], r[4], r[5])[0] if r else s[2]
                # print(rotated)
                # print(res)
                # rotated = s[2]
                return [rotated[0] + res[0], rotated[1] + res[1], rotated[2] + res[2]], visited
        
    return None, visited
        # if s[1] == start and s[0] == end:
        #     print('found opposite')
        #     # inverted = [-s[2][0], -s[2][1], -s[2][2]]
        #     # rotated = rotate(inverted, r[1], r[2], r[3], r[4], r[5])[0] if r else inverted
            
        #     return rotated 

print(search(0, 2, []))


def gen_scanner_map():
    smap = create_rotations(scanners)

    for (a, b) in itertools.permutations(smap.keys(), 2):
        test = {}
        for a1 in smap[a]:
            for a1_r in a1:
                for b1 in smap[b]:
                    for b1_r in b1:
                        key = f'{a1_r}--{b1_r}'
                        test[key] = {
                            'x': a1_r[0] - b1_r[0],
                            'y': a1_r[1] - b1_r[1],
                            'z': a1_r[2] - b1_r[2],
                            'closer': False
                        }
                        print(f'{key}: {test[key]["x"]},{test[key]["y"]},{test[key]["z"]}')
                    # if not b_orientation:
                    #     for m in invert_rotations:
                    #         if a1[0] - b1[0] - m[0] < test[key]["x"] and a1[1] - b1[1] - m[1] < test[key]["y"] and a1[2] - b1[2] - m[2] < test[key]["z"]:
                    #             print(f'closer: {m}')
                    #             b_orientation = m









def beacon_dist():
    pair_dist = {}

    for i, s in enumerate(scanners):
        s_np = list(map(np.array, s))
        for (a, b) in itertools.combinations(s_np, 2):
            dist = str(np.linalg.norm(a-b))
            if dist in pair_dist:
                pair_dist[dist].update({
                    i: (a.tolist(), b.tolist())
                })
            else:
                pair_dist[dist] = {
                    i: (a.tolist(), b.tolist())
                }
    for s in scanners:
        print(s)

    # print(max(s[0] for s in sorted(scanners, key=lambda x: x[0])))
    # print(max(s[1] for s in sorted(scanners, key=lambda x: x[1])))

    # dmap = (
    #     np.zeros(
    #         max(s[0] for s in sorted(scanners, key=lambda x: x[0])),
    #         max(s[1] for s in sorted(scanners, key=lambda x: x[1])),
    #         max(s[2] for s in sorted(scanners, key=lambda x: x[2]))
    #     )
    # )
    # print(dmap)

    def build(pairs):
        print(pairs)
        print('')

    for k, v in pair_dist.items():
        if len(v.items()) > 1:
            print(k)
            build(v)



# print(pair_dist)

# for s1, s2 in itertools.combinations(scanner_map.keys(), 2):
#     print(f'{s1}:{s2}')

#     print(sorted(list(scanner_map[s1])))
#     print(sorted(list(scanner_map[s2])))

#     print(scanner_map[s1].intersection(scanner_map[s2]))


#     exit(0)
    # print(sorted([np.linalg.norm(origin-x) for x in s]))
    # for b in s:

    #     dist = np.linalg.norm(origin-b)
    #     print(dist)
    # v = np.array(b)
    # r = R.from_quat([0, 0, np.sin(np.pi/4), np.cos(np.pi/4)])
    # print(r.as_euler('zyx', degrees=True))
    # print(r.as_matrix())
    # print(r.apply(v))
    # exit(0)

