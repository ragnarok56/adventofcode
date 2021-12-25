
locs = []
with open('in') as f:
    for l in f.readlines():
        l = l.strip()
        locs.append([c for c in l])

max_i = len(locs)
max_j = len(locs[0])

def check_east(i, j, l):
    dest_j = 0 if j + 1 >= max_j else j + 1
    return (i, dest_j) if l[i][dest_j] == '.' else None

def check_south(i, j, l):
    dest_i = 0 if i + 1 >= max_i else i + 1
    return (dest_i, j) if l[dest_i][j] == '.' else None

steps = 1
counter = 0

while True:
    east_movers = []
    south_movers = []

    # check east herd
    for i in range(max_i):
        for j in range(max_j):
            if locs[i][j] == '>':
                dest = check_east(i, j, locs)
                if dest:
                    east_movers.append(((i, j), dest))
    
    for m in east_movers:
        p = m[0]
        d = m[1]
        locs[p[0]][p[1]] = '.'
        locs[d[0]][d[1]] = '>'

    # check south herd
    for i in range(max_i):
        for j in range(max_j):
            if locs[i][j] == 'v':
                dest = check_south(i, j, locs)
                if dest:
                    south_movers.append(((i, j), dest))

    for m in south_movers:
        p = m[0]
        d = m[1]
        locs[p[0]][p[1]] = '.'
        locs[d[0]][d[1]] = 'v'

    counter += 1

    if len(east_movers) == 0 and len(south_movers) == 0:
        break

# for l in locs:
#     for c in l:
#         print(c, end='')
#     print()

print(counter)
