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
        boards.append({
            'board': board,
            'markers': np.zeros_like(board),
            'won': False
        })

losing_board = None

for d in draws:
    for b in boards:
        if b['won']:
            continue

        res = np.where(b['board'] == d)
        if len(res[0]) == 0:
            continue

        b['markers'][res[0][0]][res[1][0]] = 1  # yeesh

        # probably a better way to do this check
        has_bingo = len(np.where(np.sum(b['markers'], axis=0) == 5)[0] > 0) or len(np.where(np.sum(b['markers'], axis=1) == 5)[0] > 0)
        if has_bingo:
            if len([board for board in boards if not board['won']]) == 1:
                losing_board = b
            b['won'] = True

    if losing_board:
        unmatched = np.where(losing_board['markers'] == 0)
        unmatched = list(zip(unmatched[0], unmatched[1]))
        sum_unmatched = sum(losing_board['board'][u[0],u[1]] for u in unmatched)
        print(int(sum_unmatched * d))
        break
