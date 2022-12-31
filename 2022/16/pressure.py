#!/usr/bin/env python3
import re
import networkx as nx
from collections import deque
from itertools import combinations

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

def part(total_time = 30, part2 = False):
    max_pressure = 0
    queue = deque()
    seen = set()
    max_pressures = {}

    queue.append(State('AA', tuple(), 0, 0))

    def calc_pressure(time, valves):
        pressure_rate = sum([rate_map[v] for v in valves])
        return pressure_rate * time

    while len(queue) > 0:
        s: State = queue.pop()
        remaining_valves = [v for v in rate_map.keys() if v not in s.valves]

        # track all sub-routes since human/elephant paths aren't all
        # guaranteed to hit a max number of valves, so the optimal path
        # may contain a full path for 1 and not the other, so having
        # even partial paths determined is necessary to figure out the max
        # combined pressure for both
        updated_pressure = s.pressure + calc_pressure(total_time - s.elapsed, s.valves)
        key = (s.valves, s.pressure, s.elapsed)
        if key not in max_pressures:
            max_pressures[key] = updated_pressure
        else:
            max_pressures[key] = max(updated_pressure, max_pressures[key])

        if len(remaining_valves) == 0 or s.elapsed >= total_time:
            updated_pressure = s.pressure + calc_pressure(total_time - s.elapsed, s.valves)
            max_pressure = max(max_pressure, updated_pressure)
            continue

        for v in remaining_valves:
            cost = get_distance(s.cur, v) + 1
            updated_elapsed = s.elapsed + cost
            if s.elapsed + cost >= total_time:
                updated_pressure = s.pressure + calc_pressure(total_time - s.elapsed, s.valves)
                max_pressure = max(max_pressure, updated_pressure)
                continue

            updated_pressure = s.pressure + calc_pressure(cost, s.valves)
            updated_valves = s.valves + (v,)

            key = (updated_valves, updated_pressure, updated_elapsed)
            if key not in seen:
                seen.add(key)
                queue.append(State(v, updated_valves, updated_elapsed, updated_pressure))

    if not part2:
        print(max_pressure)
    else:
        all_pressures = [(set(k[0]), v) for k, v in max_pressures.items()]
        max_pressure = 0
        for c in combinations(all_pressures, 2):
            if c[0][0].isdisjoint(c[1][0]):
                max_pressure = max(max_pressure, c[0][1] + c[1][1])

        print(max_pressure)

if __name__ == '__main__':
    part()
    part(26, True)