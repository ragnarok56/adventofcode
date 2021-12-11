#!/usr/bin/python3
import numpy as np
import sys

# positive int only does that many steps, -1 to continue till all octopi flash
steps = int(sys.argv[1])
# display matrix on screen to "watch it"
animate = (sys.argv[2] if len(sys.argv) > 2 else '').lower() == 'true'

with open('in_test') as f:
    input = [" ".join(list(l)) for l in f.readlines()]
    board = np.genfromtxt(input)

if animate:
    import os
    os.system('clear')
    print(board)

rangeX = range(board.shape[0])
rangeY = range(board.shape[1])

flashed_counter = 0
all_flashed_step = None
s = 0

# kinda lame loop condition checks
check_steps = lambda: s < steps
check_all_flashed = lambda: all_flashed_step is None

if steps > 0:
    should_continue = check_steps
else:
    should_continue = check_all_flashed

while should_continue():
    board = board + 1
    energized_positions = np.where(board > 9)
    while (len(energized_positions[0]) > 0):
        board[energized_positions] = 0

        # trigger adjacent octopi
        for p in zip(energized_positions[0], energized_positions[1]):
            for x in range(-1, 2):
                for y in range(-1, 2):
                    adjacent = (p[0] + x, p[1] + y)

                    if (adjacent[0] in rangeX and adjacent[1] in rangeY and adjacent != p and board[adjacent] != 0):
                        board[adjacent] = board[adjacent] + 1

        # determine if any newly energized octopi
        energized_positions = np.where(board > 9)

    # count octopi that flashed
    flashed_counter = flashed_counter + len(np.where(board == 0)[0])

    # check if all flashed
    if len(np.where(board == 0)[0]) == board.shape[0] * board.shape[1]:
        all_flashed_step = s + 1

    if animate:
        import time
        time.sleep(.1)
        os.system('clear')
        # output board so its easier to see flashes
        flash_board = np.where(board == 0, board, ' ')
        print(np.where(flash_board == ' ', flash_board, '0'))

    s = s + 1

print(f'total flashes: {flashed_counter}')
print(f'first all flash step: {all_flashed_step}')