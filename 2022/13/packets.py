#!/usr/bin/python3
import json
from functools import cmp_to_key

def check(left, right):
    # print(f'Compare {left} vs {right}')
    if isinstance(left, int):
        if isinstance(right, int):
            if left < right:
                return 1
            elif left > right:
                return -1
            else:
                return 0
        elif isinstance(right, list):
            return check([left], right)

    if isinstance(left, list):
        if isinstance(right, list):
            for pairs in zip(left, right):
                result = check(pairs[0], pairs[1])
                if result == 0:
                    continue
                return result
            if len(left) < len(right):
                return 1
            if len(left) > len(right):
                return -1
            return 0
        elif isinstance(right, int):
            return check(left, [right])

with open('in') as f:
    input = [l for l in f.readlines() if len(l.strip()) > 0]
    it = iter(input)
    pairs = zip(it,it)
    total = 0
    for (i, pair) in enumerate(pairs):
        left = json.loads(pair[0])
        right = json.loads(pair[1])
        result = check(left, right)

        if result == 1:
            total += i + 1
    print(total)

    all_packets = [[[2]], [[6]]]
    for packet in input:
        all_packets.append(json.loads(packet))

    all_packets = sorted(all_packets, key=cmp_to_key(check), reverse=True)

    total = 1
    for (i, p) in enumerate(all_packets):
        p_str = json.dumps(p)
        if p_str == '[[2]]' or p_str == '[[6]]':
            total *= i + 1

    print(total)
