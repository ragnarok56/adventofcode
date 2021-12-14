import sys
import networkx as nx
from collections import Counter

small_cave_visits = int(sys.argv[1]) if len(sys.argv) > 1 else 1

G = nx.Graph()

BIG = 1
SMALL = 0

with open('in') as f:
    for l in f.readlines():
        [x, y] = [n.strip() for n in l.split('-')]
        G.add_node(x, type=SMALL if x.lower() == x else BIG)
        G.add_node(y, type=SMALL if y.lower() == y else BIG)
        G.add_edge(x, y)

paths = ['start']
num_paths = 0

def explore(cur):
    global num_paths
    for n in G.neighbors(cur):
        if n == 'start':
            continue

        if G.nodes[n]['type'] == SMALL and n in paths:
            if max([v for k, v in Counter(paths).items() if k.lower() == k and k not in ['start', 'end']]) >= small_cave_visits:
                continue

        if n == 'end':
            num_paths = num_paths + 1
            continue

        paths.append(n)
        explore(n)
        paths.pop()


for n in G.neighbors('start'):
    paths.append(n)
    explore(n)
    paths.pop()

print(num_paths)