
coords = set()
instructions = []

read_mode = 'coords'

with open('in') as f:
    for l in [l.strip() for l in f.readlines()]:
        if len(l) == 0:
            read_mode = 'instructions'
            continue
        if read_mode == 'coords':
            coord = l.split(',')
            coord_tuple = (int(coord[0]), int(coord[1]))
            coords.add(coord_tuple)
        else:
            instructions.append(l.split('fold along ')[1])

def print_matrix(coords):
    maxX = max(x[1] for x in coords)
    maxY = max(x[0] for x in coords)

    rangeX = range(maxX + 1)
    rangeY = range(maxY + 1)

    matrix = [['.' for y in rangeY] for x in rangeX]
    for c in coords:
        matrix[c[1]][c[0]] = '#'

    for i in rangeX:
        for j in rangeY:
            print(matrix[i][j], end='')
        print()

def fold(coords, inst):
    new_coords = set()
    (direction, index) = inst.split('=')
    index = int(index)
    for c in coords:
        if direction == 'y' and c[1] > index:
            new_coords.add((c[0], index - (c[1] - index)))
        elif direction == 'x' and c[0] > index:
            new_coords.add((index - (c[0] - index), c[1]))
        else:
            new_coords.add(c)

    return new_coords

for inst in instructions:
    coords = fold(coords, inst)
    print(len(coords))

print_matrix(coords)