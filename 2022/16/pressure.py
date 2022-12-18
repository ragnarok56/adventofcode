#!/usr/bin/env python3
import re
import networkx as nx
from typing import List,Tuple
from copy import copy
import itertools

G = nx.DiGraph()
rate_map = {}


with open('in') as f:
# with open('in') as f:
    for l in f.readlines():
        match = re.search("Valve (\w+) has flow rate=(\d+); tunnels? leads? to valves? (.*)", l)
        valve = match.group(1)
        rate = int(match.group(2))
        edges = match.group(3)
        edges = [e.strip() for e in edges.split(',')]
        # print(valve, rate, edges)\
        if rate > 0:
            rate_map[valve] = rate
        G.add_node(valve, rate=rate)
        for e in edges:
            G.add_node(e)
            G.add_edge(valve, e)

# for n in G.neighbors('AA'):
#     print(n, G.nodes[n]['rate'])

dfs = nx.dfs_edges(G, 'AA')

by_rate = sorted([(n, G.nodes[n]) for n in G.nodes()], key=lambda x: x[1]['rate'], reverse=True)
# print(by_rate)

distances = {}
for n in G.nodes():
    distances[n] = nx.shortest_path_length(G, n)

def get_distance(s, e):
    return distances[s][e]

paths = []

# todo: trying to figure out how to get valid permutations of valve nodes
# that dont exceed the timer (30 minutes) to travel between
# otherwise i need to loop through 15! things and thats just not gonna happen
def permutations(prev, elements, min):
    if len(elements) <= 1:
        yield (elements, 0)
        return
    remaining = elements[1:]
    prev = elements[0:1]
    for (perm, d) in permutations(prev, remaining, min):
        for i in range(len(elements)):
            cost = d + get_distance(prev[0], perm[0])
            # print(f'{prev[0]} to {perm[0]} is {get_distance(prev[0], perm[0])}')
            if cost > 30:
                # print('cost > 30', perm[i:])
                yield (perm[i:], d)
            else: 
                # print('cost <= 30')
                yield (perm[:i] + elements[0:1] + perm[i:], cost)

paths = permutations('AA', list(rate_map.keys()), 30)
# print(next(paths))
for p in paths:
    print(p)
# for (n, r) in rate_map.items():
#     distance = 30
#     for n2 in rate_map.keys():
    
        

exit()
# print(nx.shortest_path_length(G, 'BB'))

# print(distances)
# for n in dfs:
#     print(n[0], G.nodes[n[0]]['rate'])

# for n in G.nodes():
#     for r in by_rate:
#         steps = distances[n][r[0]]
#         print(f'{n}->{r[0]}: {steps}')

# for r in by_rate:
#     steps = get_distance('DD', r[0])
#     print(f'DD->{r[0]}: {steps} - {30 - steps} [{r[1]}]')

class PressureTrack:
    def __init__(self, remaining_valves: set, minutes: int = 0, valves: List[Tuple[str,int]] = []):
        self.pressure = 0
        self.valves = valves
        self.minutes = minutes
        self.remaining_valves = remaining_valves

    def can_move(self):
        return self.minutes > 0 and len(self.remaining_valves) > 0

    def open_valve(self, n: str, rate: int):
        self.valves.append((n, rate))
        self.remaining_valves.remove(n)

    def move(self, d):
        # print(f'Minute {30 - self.minutes}: moving {d} spots (pressure current {self.pressure})')
        self.minutes = self.minutes - (d + 1)
        # print(f'valves {[v[0] for v in self.valves]} are open, releasing {sum([v[1] for v in self.valves])} * ({d+1})')
        for v in self.valves:
            self.pressure = self.pressure + (v[1] * (d + 1))
    
    def __str__(self):
        return f'Minute: {self.minutes}, Pressure: {self.pressure}, Valves: {[v[0] for v in self.valves]}'

    def __copy__(self):
        pt = PressureTrack(self.remaining_valves.copy(), self.minutes, copy(self.valves))
        pt.pressure = self.pressure
        return pt

class DFS:
    def __init__(self, graph: nx.DiGraph):
        self.graph = graph
        self.steps = 0

    def walk(self, n: str, d: int, minute: int, pt: PressureTrack):
        if minute == 0:
            return None

        self.steps = self.steps + 1

        # paths = [n]
        for x in range(d):
            pt.next()

        next_pt = copy(pt)
        
        rate = self.graph.nodes[n]['rate']
        if rate > 0:
            next_pt.open_valve(n, rate)

        print(next_pt)
        for next_valve in next_pt.remaining_valves:
            d = get_distance(n, next_valve)
            self.walk(next_valve, d, minute - d, next_pt)

    # def walk2(self, n: str, d: int, minute: int, pt: PressureTrack):
    #     if not pt.can_move():
    #         return None

    #     print(pt)

    #     for next in pt.remaining_valves:
    #         d = get_distance(n, next)
    #         pt_copy = copy(pt)
    #         pt_copy.move(d)
    #         pt_copy.open_valve(next, self.graph.nodes[n]['rate'])
    #         r = self.walk2(next, d, minute - d, pt_copy)
    #         if r is not None:
    #             yield pt_copy

    # this is the other method besides DFS to just 
    # 
    def start(self, start, minutes):
        # DD, BB, JJ, HH, EE,CC
        remaining_valves = [n for n in G.nodes() if G.nodes[n]['rate'] > 0]
        print(len(remaining_valves))
        max_pt = None
        c = 0
        # print(len(list(itertools.permutations(remaining_valves))))
        exit()
        for perm in itertools.permutations(remaining_valves):
        # for perm in [('DD', 'BB', 'JJ', 'HH', 'EE', 'CC')]:
            c = c + 1
            pt = PressureTrack(set(remaining_valves), minutes -1, [])
            n = start
            for next in perm:
                d = get_distance(n, next)
                # print(f'moving {n} to {next}')
                if pt.minutes < 0:
                    break
                n = next
                pt.move(d)
                pt.open_valve(next, rate_map[n])
            pt.move(pt.minutes)
            if max_pt is None or pt.pressure > max_pt.pressure:
                max_pt = pt
            print(c)
        print(f'Max: {max_pt}')



# steps = 0
# def walk(n: str, rem: int):
#     global steps
#     if rem == 0:
#         return None
#     paths = []
#     for neighbor in G.neighbors(n):
#         print(f'{neighbor.rjust(rem-1)}')
#         steps = steps + 1
#         r = walk(neighbor, rem - 1)
#         if r is not None:
#             paths.append(r)
#     return paths

d = DFS(G)

res = d.start('AA', 30)
# print(r, d.steps)
    
