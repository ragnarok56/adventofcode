with open('in') as f:
    instructions = []
    for l in f.readlines():
        l = l.strip()
        state = True if l.split(' ')[0] == 'on' else False
        x = [int(x) for x in l.split(' ')[1].split(',')[0].split('=')[1].split('..')]
        y = [int(y) for y in l.split(' ')[1].split(',')[1].split('=')[1].split('..')]
        z = [int(z) for z in l.split(' ')[1].split(',')[2].split('=')[1].split('..')]
        instructions.append((state, x, y, z))

def gen_range(pos):
    if pos[0] < -50 or pos[1] > 50:
        return []
    return range(pos[0], pos[1] + 1)

def part1():
    cube_map = {}
    for i in instructions:
        for x in gen_range(i[1]):
            for y in gen_range(i[2]):
                for z in gen_range(i[3]):
                    if i[0]:
                        cube_map[(x,y,z)] = 1
                    else:
                        if (x, y, z) in cube_map:
                            del cube_map[(x, y, z)]
    print(len([k for k in cube_map.keys()]))

part1()

class Coord:
    def __init__(self, x, y, z):
        self.x = x
        self.y = y
        self.z = z

    def __repr__(self):
        return f'({self.x}, {self.y}, {self.z})'

class Cuboid:    
    def __init__(self, c):
        self.state = c[0]
        x, y, z = c[1], c[2], c[3]
        self.bot_left = Coord(x[0], y[0], z[0])
        self.top_right = Coord(x[1], y[1], z[1])

    def __repr__(self):
        return f'[{self.state}, {self.volume()}] {self.bot_left}, {self.top_right}'

    def volume(self):
        bl = self.bot_left
        tr = self.top_right
        return (tr.x + 1 - bl.x) * (tr.y + 1 - bl.y) * (tr.z + 1 - bl.z)

    def intersects(self, c2):
        if not(self.bot_left.x <= c2.top_right.x and self.top_right.x >= c2.bot_left.x):
            return False
        if not(self.bot_left.y <= c2.top_right.y and self.top_right.y >= c2.bot_left.y):
            return False
        if not(self.bot_left.z <= c2.top_right.z and self.top_right.z >= c2.bot_left.z):
            return False
        return True

    def intersection(self, c2):
        return Cuboid((
            not self.state,
            (max(self.bot_left.x, c2.bot_left.x), min(self.top_right.x, c2.top_right.x)),
            (max(self.bot_left.y, c2.bot_left.y), min(self.top_right.y, c2.top_right.y)),
            (max(self.bot_left.z, c2.bot_left.z), min(self.top_right.z, c2.top_right.z)),
        ))


def part2():
    total_lit = 0
    boxes = []

    for num, i in enumerate(instructions):
        cuboid = Cuboid(i)
        intersections = []
        for c in boxes:
            if c.intersects(cuboid):
                new_c = c.intersection(cuboid)
                intersections.append(new_c)

        for inter in intersections:
            boxes.append(inter)
        
        if cuboid.state:
            boxes.append(cuboid)

    total_lit = 0

    for c in boxes:
        if c.state:
            total_lit += c.volume()
        else:
            total_lit -= c.volume()
    
    print(total_lit)

part2()