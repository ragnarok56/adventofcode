#!/usr/bin/python3
import functools
import numpy as np

with open('in') as f:
    lines = [l.strip() for l in f.readlines()]

pairs = {
    '{': '}',
    '(': ')',
    '[': ']',
    '<': '>'
}

scorecard = {
    ')': (3, 1),
    ']': (57, 2),
    '}': (1197, 3),
    '>': (25137, 4)
}

corrupt = []
autocomplete = []

for l in lines:
    stack = []
    for c in l:
        if pairs.get(c):
            stack.append(c)
        else:
            if c != pairs.get(stack.pop()):
                corrupt.append(c)
                stack = []
                break
    # handle incomplete
    if len(stack) > 0:
        autocomplete.append([pairs.get(c) for c in stack[::-1]])

print(f'corrupt score: {sum([scorecard.get(c)[0] for c in corrupt])}')

autocomplete_scores = [
    functools.reduce(lambda a, x: a * 5 + scorecard.get(x)[1], c, 0)
    for c in autocomplete
]

print(f'autocomplete middle score: {sorted(autocomplete_scores)[int(len(autocomplete_scores) / 2)]}')
