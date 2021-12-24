with open('in_test') as f:
    algorithm = f.readline()
    f.readline()

    input = [[0 if c == '.' else 1 for c in l.strip()] for l in f.readlines()]

def get_pix(p):
    return 1 if algorithm[p] == '#' else 0

def view(output):
    num_lit = 0
    for i in range(len(output)):
        for j in range(len(output[i])):
            num_lit += output[i][j]
            print('#' if output[i][j] else '.',end='')
        print()
    return num_lit

def execute(input, inf=0):
    max_i = len(input)
    max_j = len(input[0])
    output = [ [inf]*(max_j + 4) for i in range(max_i + 4) ]

    def get(i, j):
        # if outside scope of 'input', retrieve value from infinite
        if i < 2 or j < 2 or i > max_i + 1 or j > max_j + 1:
            return inf
        else:
            return input[i - 2][j - 2]

    for i in range(max_i + 4):
        for j in range(max_j + 4):
            if i < 1 or j < 1 or i > max_i + 2 or j > max_j + 2:
                output[i][j] = get_pix(inf)
            else:
                shift = 8
                sp = 0
                for x in range(-1,2):
                    for y in range(-1,2):
                        sp |= get(i + x, j + y) << shift
                        shift -= 1
                output[i][j] = get_pix(sp)
    return output

output = execute(input)

for _ in range(49):
    output = execute(output, output[0][0])

print(view(output))