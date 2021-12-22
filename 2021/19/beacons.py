import numpy as np
import itertools
import collections
from scipy.spatial.transform import Rotation as R

scanners = []
with open('in_test2') as f:
    scanner_beacons = map(lambda x: x.strip(), ''.join(f.readlines()).replace('\n', ' ').split('---')[::2][1:])
    for sb in scanner_beacons:
        scanners.append([list(map(int, x.split(','))) for x in sb.split(' ')])

invert_rotations = list(set(itertools.permutations([1,1,1,-1,-1,-1], 3)))
sm = []
origin = np.array([0,0,0])

# for i, s in enumerate(scanners):
#     print(f'{i}: {s}')

# exit(0)

def gen_rotate():
    for offset in range(3):
        for r in invert_rotations:
            def rotate(coord):
                # shift "offset" times
                for _ in range(offset):
                    coord = coord[1:3] + [coord[0]]
                return (coord[0] * r[0], coord[1] * r[1], coord[2] * r[2])

            yield (rotate, offset, r)

for r in gen_rotate():
    print(f'{r[1], r[2]}')
exit(1)
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

for (a1, b1) in itertools.permutations(enumerate(scanners), 2):
    # print(f'{a1} -- {b1}')

    track = collections.defaultdict(list)
    for a in a1[1]:
        for r in gen_rotate():
            # compare 1 rotation with all beacons
            for b in b1[1]:
                diff = (a[0] - r[0](b)[0],a[1] - r[0](b)[1],a[2] - r[0](b)[2])
                track[diff].append((r[0](b), (r[1], r[2])))
                # print(f'{a} | {r(b)} -- {diff}')
                # for br in r(b):
                #     print(f'{a} | {br} -- {a[0] - br[0]},{a[1] - br[1]},{a[2] - br[2]}')
    # print('  matches:')
    # matches = [v for v in sorted(track.items(), key=lambda x: len(x[1])) if len(v) > 1]
    # matches = 0
    # print(f'{a1[0]}:{b1[0]}')
    for k, v in sorted(track.items(), key=lambda x: len(x[1])):
        if len(v) > 2:
            print(f'{a1[0]}:{b1[0]} [{len(v)}]    {k} - {v}')


def gen_scanner_map():
    smap = create_rotations(scanners)

    for (a, b) in itertools.combinations(smap.keys(), 2):
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

