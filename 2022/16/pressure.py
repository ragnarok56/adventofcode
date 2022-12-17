#!/usr/bin/env python3
import re
import networkx as nx
from typing import List,Tuple
from copy import copy

G = nx.DiGraph()

with open('in_test') as f:
# with open('in') as f:
    for l in f.readlines():
        match = re.search("Valve (\w+) has flow rate=(\d+); tunnels? leads? to valves? (.*)", l)
        valve = match.group(1)
        rate = int(match.group(2))
        edges = match.group(3)
        edges = [e.strip() for e in edges.split(',')]
        print(valve, rate, edges)
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
    def __init__(self, minutes: int = 0, valves: List[Tuple[str,int]] = [], visited: set = set()):
        self.pressure = 0
        self.visited = visited
        self.valves = valves
        self.minutes = minutes
        self.remaining_valves: set

    def open_valve(self, n: str, rate: int):
        self.valves.append((n, rate))
        self.visited.add(n)
        self.remaining_valves.remove(n)

    def next(self):
        self.minutes = self.minutes - 1
        for v in self.valves:
            self.pressure = self.pressure + v[1]
    
    def __str__(self):
        return f'Minute: {self.minutes}, Pressure: {self.pressure}, Open Valves: {len(self.valves)}, Valves: {sorted([v[0] for v in self.valves])}'

    def __copy__(self):
        pt = PressureTrack(self.minutes, self.valves, self.visited)
        pt.pressure = self.pressure
        pt.remaining_valves = self.remaining_valves.copy()
        return pt


class DFS:
    def __init__(self, graph: nx.DiGraph):
        self.graph = graph
        self.steps = 0

    def walk(self, n, minute, pt: PressureTrack):
        if minute == 0:
            return None

        self.steps = self.steps + 1

        paths = [n]
        pt.next()
        next_pt = copy(pt)
        
        rate = self.graph.nodes[n]['rate']
        if rate > 0:
            next_pt.open_valve(n, rate)

        print(next_pt)
        for neighbor in self.graph.neighbors(n):
            if neighbor not in next_pt.visited:
                r = self.walk(neighbor, minute - 1, next_pt)
                if r is not None:
                    paths.append(r)
        return paths

    def start(self, n, minutes):
        pt = PressureTrack(minutes)
        
        valve_nodes = set([n for n in G.nodes() if G.nodes[n]['rate'] > 0])

        r = self.walk(n, self.total, pt)
        return r



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

r = d.start('AA', 30)

print(r, d.steps)
    
