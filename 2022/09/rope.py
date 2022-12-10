#!/usr/bin/python3

move_map = {
    'R': lambda x, v: (x[0] + v, x[1]),
    'L': lambda x, v: (x[0] - v, x[1]),
    'U': lambda x, v: (x[0], x[1] + v),
    'D': lambda x, v: (x[0], x[1] - v)
}

def touching(x, y):
    return abs(x[0] - y[0]) < 2 and abs(x[1] - y[1]) < 2

def process_knot(H, T):
    if not touching(H, T):
        if H[1] == T[1]:
            tail_pos = H[0] - T[0] > 0
            T = (T[0] + (1 if tail_pos else -1), T[1])
        elif H[0] == T[0]:
            tail_pos = H[1] - T[1] > 0
            T = (T[0], T[1] + (1 if tail_pos else -1))
        else:
            lateral = H[0] > T[0]
            vertical = H[1] > T[1]
            T = (T[0] + (1 if lateral else -1), T[1] + (1 if vertical else -1))
    return T

def part(num_knots):
    knots = []
    for i in range(num_knots):
        knots.append((0,0))
    coords = {knots[0]}
    with open('in') as f:
        for line in f.readlines():
            direction, amount = line.split(' ')
            for i in range(int(amount)):
                knots[0] = move_map[direction](knots[0], 1)
                for j in range(1, num_knots):
                    knots[j] = process_knot(knots[j - 1], knots[j])
                coords.add(knots[num_knots - 1])

    print(len(coords))

if __name__ == "__main__":
    part(2)
    part(10)