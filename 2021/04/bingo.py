#!/usr/bin/python3
import numpy as np

with open('in') as f:
    draws = [int(d) for d in f.readline().split(',')]

    boards = []
    markers = []
    while f:
        board = np.loadtxt(f, max_rows=5)
        if len(board) == 0:
            break
        boards.append((board, np.zeros_like(board)))

winning_board = None

for d in draws:
    for b in boards:
        res = np.where(b[0] == d)
        if len(res[0]) == 0:
            continue

        b[1][res[0][0]][res[1][0]] = 1  # yeesh

        # probably a better way to do this check
        has_bingo = len(np.where(np.sum(b[1], axis=0) == 5)[0] > 0) or len(np.where(np.sum(b[1], axis=1) == 5)[0] > 0)
        if has_bingo:
            winning_board = b
            break

    if winning_board:
        unmatched = np.where(winning_board[1] == 0)
        unmatched = list(zip(unmatched[0], unmatched[1]))
        sum_unmatched = sum(winning_board[0][u[0],u[1]] for u in unmatched)
        print(int(sum_unmatched * d))
        break
