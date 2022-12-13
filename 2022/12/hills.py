#!/usr/bin/python3
import heapq

class Node:
    def __init__(self, value, position, parent=None):
        self.position = position
        self.value = value
        self.parent = parent
        self.f = 0
        self.h = 0
        self.g = 0
    def ord(self):
        return ord(self.value)
    def __hash__(self):
        return hash(self.position)
    def __eq__(self, node):
        return self.position == node.position
    def __lt__(self, other):
      return self.f < other.f
    def __gt__(self, other):
      return self.f > other.f
    def __str__(self):
        return f'{self.value}: {self.position} [{self.f}, {self.g}, {self.h}]'

def get_neighbors(hm, node):
    bounds = (len(hm), len(hm[0]))
    ns = [(-1, 0), (1, 0), (0, -1), (0, 1)]
    for n in ns:
        ni = node.position[0] + n[0]
        nj = node.position[1] + n[1]
        if 0 <= ni < bounds[0] and 0 <= nj < bounds[1]:
            neighbor = hm[ni][nj]
            can_move = neighbor.ord() < node.ord() or abs(neighbor.ord() - node.ord()) <= 1
            if can_move:
                yield Node(neighbor.value, neighbor.position, node)

def astar(hm, start, goal):
    _open = []
    _openSet = {}
    _closed = set()
    heapq.heapify(_open)
    heapq.heappush(_open, start)
    while len(_open) > 0:
        current = heapq.heappop(_open)
        _closed.add(current)
        if current == goal:
            break
        for n in get_neighbors(hm, current):
            if n in _closed:
                continue
            n.g = current.g + 1
            x1, y1 = n.position
            x2, y2 = goal.position
            n.h = abs(y2 - y1) + abs(x2 - x1)
            n.f = n.h + n.g
            c = _openSet.get(n)
            if c and c.g < n.g:
                continue
            heapq.heappush(_open, n)
            _openSet[n] = n
    path = []
    while current.parent is not None:
        path.append(current.position)
        current = current.parent
    path.append(current.position)
    return path[::-1]

def part1():
    hm = []
    start = None
    end = None
    with open('in') as f:
        for (i, line) in enumerate(f.readlines()):
            hm.append([])
            for (j, h) in enumerate(line.strip()):
                if h == 'S':
                    h = 'a'
                    start = Node(h, (i, j))
                if h == 'E':
                    h = 'z'
                    end = Node(h, (i, j))
                hm[i].append(Node(h, (i, j)))

    r = astar(hm, start, end)
    print(len(r) - 1)

    # this prints the path
    # for (i, row) in enumerate(hm):
    #     for (j, value) in enumerate(row):
    #         path_cell = any(p for p in r if p == (i, j))
    #         print('#' if path_cell else '.', end="")
    #     print()


def part2():
    hm = []
    start = []
    end = None
    with open('in') as f:
        for (i, line) in enumerate(f.readlines()):
            hm.append([])
            for (j, h) in enumerate(line.strip()):
                if h == 'S' or h == 'a':
                    h = 'a'
                    start.append(Node(h, (i, j)))
                if h == 'E':
                    h = 'z'
                    end = Node(h, (i, j))
                hm[i].append(Node(h, (i, j)))

    filtered_start = []
    for s in start:
        if any(n for n in get_neighbors(hm, s) if n.value == 'b'):
            filtered_start.append(s)

    results = []
    for s in filtered_start:
        r = astar(hm, s, end)
        results.append(r)

    print(min(len(r) for r in results) - 1)

if __name__ == "__main__":
    part1()
    part2()