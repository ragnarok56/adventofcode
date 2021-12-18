
with open('in') as f:
    # clearly more readable than a regex
    [x_range, y_range] = [r.split('=')[1].split('..') for r in f.readline().split(' ')[2:4]]
    x_range = [int(x.replace(',', '')) for x in x_range]
    y_range = [int(x.replace(',', '')) for x in y_range]

def do_physics(probe):
    (x_pos, y_pos) = probe['pos']
    (x, y) = probe['vel']['x'], probe['vel']['y']
    probe['pos'] = (x_pos + x, y_pos + y)
    probe['vel']['x'] = x + -1 if x > 0 else 1 if x < 0 else 0
    probe['vel']['y'] = y - 1
    return probe

def gen_probe(x, y):
    return {
        'pos': (0, 0),
        'vel': {
            'x': x,
            'y': y
        }
    }

shots = set()

# valid x shots between 1 and max x of target zone
x_velocities = range(1, x_range[1] + 1)
# valid y shots between max x of target zone and min y of target zone
y_velocities = range(x_range[1], y_range[0] - 1, -1)

highest_trajectory = None

for x_vel in x_velocities:
    for y_vel in y_velocities:
        probe = gen_probe(x_vel, y_vel)
        trajectory = []
        while True:
            probe = do_physics(probe)
            (x, y) = probe['pos']
            trajectory.append((x, y))

            if x >= x_range[0] and x <= x_range[1] and y >= y_range[0] and y <= y_range[1]:
                shots.add((x_vel, y_vel))
                shot_highest_trajectory = max([p[1] for p in trajectory])
                if highest_trajectory is None or shot_highest_trajectory > highest_trajectory:
                    highest_trajectory = shot_highest_trajectory
                break

            # outside zone
            if x > x_range[1] or y < y_range[0]:
                break

print(f'shots landed in zone: {len(shots)}')
print(f'highest trajectory: {highest_trajectory}')
