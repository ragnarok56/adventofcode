import sys
import networkx as nx

cave_size_multiplier = int(sys.argv[1]) if len(sys.argv) > 1 else 1

G = nx.DiGraph()

with open('in') as f:
    prev_col = None
    prev_row = None
    last_col = None

    all_lines = f.readlines()
    max_y = len(all_lines[0]) - 1
    max_x = len(all_lines)

    def calc_risk(r, multi):
        modded_risk = r + multi
        return modded_risk % 10 if modded_risk < 10 else (modded_risk % 10) + 1

    for row in range(max_x * cave_size_multiplier):
        for col in range(max_y * cave_size_multiplier):
            row_risk_multiplier = int(row / max_x)
            col_risk_multiplier = int(col / max_y)
            risk_multiplier = row_risk_multiplier + col_risk_multiplier
            node = f'{row},{col}'
            raw_risk = int(f'{all_lines[row % max_x][col % max_y]}')
            risk = calc_risk(raw_risk, risk_multiplier)

            G.add_node(node, risk=risk)
            if prev_row is not None:
                G.add_edge(node, f'{prev_row},{col}', risk=G.nodes[f'{prev_row},{col}']['risk'])
                G.add_edge(f'{prev_row},{col}', node, risk=risk)
            if prev_col is not None:
                G.add_edge(node, f'{row},{prev_col}', risk=G.nodes[f'{row},{prev_col}']['risk'])
                G.add_edge(f'{row},{prev_col}', node, risk=risk)
            prev_col = col
        last_col = prev_col
        prev_col = None
        prev_row = row

path = nx.astar_path(G, '0,0', f'{prev_row},{last_col}', weight='risk')

total_risk = sum(G.nodes[n]['risk'] for n in path[1:])

print(total_risk)
