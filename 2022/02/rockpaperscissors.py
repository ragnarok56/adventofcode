#!/usr/bin/python3

def part1():

    play_lookup = {
        'X': 1,
        'Y': 2,
        'Z': 3
    }

    outcomes = {
        'A': {
            'X': 3,
            'Y': 6,
            'Z': 0
        },
        'B': {
            'X': 0,
            'Y': 3,
            'Z': 6
        },
        'C': {
            'X': 6,
            'Y': 0,
            'Z': 3
        }
    }

    score = 0
    with open('in') as f:
        for line in f.readlines():
            contest = [d.strip() for d in line.split(' ')]

            # lame
            score += outcomes[contest[0]][contest[1]]
            score += play_lookup[contest[1]]

    print(score)

def part2():
    pass

part1()
part2()