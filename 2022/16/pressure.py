#!/usr/bin/env python3
import re
import networkx as nx
from typing import List,Tuple
from copy import copy
from collections import deque

G = nx.DiGraph()
rate_map = {}

with open('in') as f:
    for l in f.readlines():
        match = re.search("Valve (\w+) has flow rate=(\d+); tunnels? leads? to valves? (.*)", l)
        valve = match.group(1)
        rate = int(match.group(2))
        edges = match.group(3)
        edges = [e.strip() for e in edges.split(',')]
        if rate > 0:
            rate_map[valve] = rate
        G.add_node(valve, rate=rate)
        for e in edges:
            G.add_node(e)
            G.add_edge(valve, e)

distances = {}
for n in G.nodes():
    distances[n] = nx.shortest_path_length(G, n)

def get_distance(s, e):
    return distances[s][e]

class State:
    def __init__(self, cur, valves, elapsed, pressure):
        self.cur = cur
        self.valves = valves
        self.elapsed = elapsed
        self.pressure = pressure
    def __repr__(self):
        return f'[{self.elapsed}]:[{self.pressure}]'

max_pressure = 0
queue = deque()
seen = set()

queue.append(State('AA', tuple(), 0, 0))

def calc_pressure(time, valves):
    pressure_rate = sum([rate_map[v] for v in valves])
    return pressure_rate * time

while len(queue) > 0:
    s: State = queue.pop()
    remaining_valves = [v for v in rate_map.keys() if v not in s.valves]

    if len(remaining_valves) == 0 or s.elapsed >= 30:
        updated_pressure = s.pressure + calc_pressure(30 - s.elapsed, s.valves)
        max_pressure = max(max_pressure, updated_pressure)
        continue

    for v in remaining_valves:
        cost = get_distance(s.cur, v) + 1
        updated_elapsed = s.elapsed + cost
        if s.elapsed + cost >= 30:
            updated_pressure = s.pressure + calc_pressure(30 - s.elapsed, s.valves)
            max_pressure = max(max_pressure, updated_pressure)
            continue

        updated_pressure = s.pressure + calc_pressure(cost, s.valves)
        updated_valves = s.valves + (v,)

        key = (updated_valves, updated_pressure, updated_elapsed)
        if key not in seen:
            seen.add(key)
            queue.append(State(v, updated_valves, updated_elapsed, updated_pressure))

print(max_pressure)


