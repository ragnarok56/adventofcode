#!/usr/bin/python3
import functools
import numpy as np

with open('in') as f:
    input = [" ".join(list(l)) for l in f.readlines()]
    board = np.genfromtxt(input)

rows = range(board.shape[0])
cols = range(board.shape[1])
low_points = []

def check_neighbors(x, i, j):
    # check up
    if i > 0 and x >= board[i - 1][j]:
        return
    # check down
    if i < board.shape[0] - 1 and x >= board[i + 1][j]:
        return
    # check left
    if j > 0 and x >= board[i][j - 1]:
        return
    # check right
    if j < board.shape[1] - 1 and x >= board[i][j + 1]:
        return

    return True

for i in rows:
    for j in cols:
        point = board[i][j]
        is_low = check_neighbors(point, i, j)
        if is_low:
            low_points.append((point, i, j))

risk = sum([1 + int(p[0]) for p in low_points])

print(f'risk: {risk}')

# dfs each low point and break on 9s

basins = []

def explore(basin, i, j):
    # check up
    if i > 0:
        neigh_point = (i - 1, j)
        if neigh_point not in basin and board[i - 1][j] != 9:
            basin.add(neigh_point)
            explore(basin, i - 1, j)
    # check down
    if i < board.shape[0] - 1:
        neigh_point = (i + 1, j)
        if neigh_point not in basin and board[i + 1][j] != 9:
            basin.add(neigh_point)
            explore(basin, i + 1, j)
    # check left
    if j > 0:
        neigh_point = (i, j - 1)
        if neigh_point not in basin and board[i][j - 1] != 9:
            basin.add(neigh_point)
            explore(basin, i, j - 1)
    # check right
    if j < board.shape[1] - 1:
        neigh_point = (i, j + 1)
        if neigh_point not in basin and board[i][j + 1] != 9:
            basin.add(neigh_point)
            explore(basin, i, j + 1)

    return True

for (_, i, j) in low_points:
    basin = set()
    explore(basin, i, j)
    basins.append(len(basin))

largest_basin_score = functools.reduce(lambda a, x: a * x, sorted(basins, reverse=True)[:3])

print(f'largest basins: {largest_basin_score}')

