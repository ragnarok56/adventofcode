import numbers


class ALU:
    def __init__(self):
        self.w = 0
        self.x = 0
        self.y = 0
        self.z = 0

    def inp(self, var, value):
        setattr(self, var, value)

    def add(self, var1, var2):
        val = int(getattr(self, str(var2), var2))
        res = getattr(self, var1) + val
        setattr(self, var1, res)

    def mul(self, var1, var2):
        val = int(getattr(self, str(var2), var2))
        res = getattr(self, var1) * val
        setattr(self, var1, res)

    def div(self, var1, var2):
        if var2 == 1:
            return
        val = int(getattr(self, str(var2), var2))
        res = int(getattr(self, var1) / val)
        setattr(self, var1, res)

    def eql(self, var1, var2):
        val = int(getattr(self, str(var2), var2))
        res = 1 if getattr(self, var1) == val else 0
        setattr(self, var1, res)

    def mod(self, var1, var2):
        val = int(getattr(self, str(var2), var2))
        res = getattr(self, var1) % val
        setattr(self, var1, res)

    def __repr__(self):
        return f'w:{self.w}, x:{self.x}, y:{self.y}, z:{self.z}'

    def reset(self):
        self.w = 0
        self.x = 0
        self.y = 0
        self.z = 0

    def process_instruction(self, instr, val):
        c = instr.strip().split(' ')
        func = getattr(self, c[0])
        if len(c) < 3:
            arg2 = val
        else:
            arg2 = c[2]
        func(c[1], arg2)

    def build_program(self, instructions):
        self.steps = []
        for i, pos in enumerate(range(0, len(instructions), 18)):
            prog_type = 'increment' if instructions[pos + 4].strip().endswith('1') else 'decrement'
            self.steps.append((prog_type, int(instructions[pos + 5].strip().split(' ')[-1:][0]), int(instructions[pos + 15].strip().split(' ')[-1:][0])))

        return self.steps

    def increment(self, input, *args):
        """
        increase z val
        """
        x_val, y_val = args[1], args[2]
        x = (self.z % 26 + x_val) != input
        y = (25 * x) + 1
        z = self.z * y
        y = int(input) + y_val * x
        self.z = z + y

    def decrement(self, input, *args):
        """
        decrease z val
        """
        x_val, y_val = args[1], args[2]
        x = ((self.z % 26) + x_val) != input
        y = (25 * x) + 1
        z = int(self.z / 26) * y
        y = int(input) + y_val * x
        self.z = z + y




with open('in') as f:
    instructions = f.readlines()


def impossibly_slow_brute_force():
    alu = ALU()
    for r in reversed(range(99999999999999 + 1)):
        r_str = str(r)
        for i in instructions:
            c = i.strip().split(' ')
            func = getattr(alu, c[0])
            if len(c) < 3:
                arg2 = r_str[0]
                r_str = r_str[1:len(r_str)]
            else:
                arg2 = c[2]
            func(c[1], arg2)
        if alu.z == 0:
            print(r)
            exit(0)

        alu.reset()


# increment Z
# inp w
# mul x 0
# add x z
# mod x 26
# div z 1
# add x 10
# eql x w
# eql x 0
# mul y 0
# add y 25
# mul y x
# add y 1
# mul z y
# mul y 0
# add y w
# add y 2
# mul y x
# add z y

#input = 0 - 9
#z = some number
#x = some number % 26 = 0 -> 25
#z = z or z % 26
#x = 0 -> 25 + some number
#x = 1 if input == x
#x = ! x (flip it)
#y = 25
#y = 25 or 0
#y = 26 or 1
#z = z or (26 * some number)
#y = 0
#y = input
#y = input + (num)
#y = y or 0
#z = 

# shrink Z
# inp w
# mul x 0
# add x z
# mod x 26
# div z 26
# add x -14
# eql x w
# eql x 0
# mul y 0
# add y 25
# mul y x
# add y 1
# mul z y
# mul y 0
# add y w
# add y 7
# mul y x
# add z y

#input = 0 - 9
#z = some number
#x = 0 -> 25
#z = z % 26
#x = (0 -> 25) - some number
#x = 1 if input != x
#y = 26 or 1
#z = (26 * z) or z
#y = input
#y = input + (num)
#y = y or 0
#z = 

def proc_instr(instr, val):
    c = instr.strip().split(' ')
    func = getattr(alu, c[0])
    if len(c) < 3:
        arg2 = val
    else:
        arg2 = c[2]
    func(c[1], arg2)


def process_instruction_set(vals=None, idx=0):
    if vals is None:
        vals = [0]
    for v in vals:
        for i in range(1, 10):
            alu = ALU()
            alu.z = v
            for j in range(idx,idx+18):
                # print(f'{instructions[j]}')
                alu.process_instruction(instructions[j], i)
            yield alu.z

def test1():
    vals = list(process_instruction_set())
    print(vals)
    vals = list(process_instruction_set(vals,18))
    print(vals)
    vals = list(process_instruction_set(vals,36))
    print(vals)

def work_backward():
    vals = range(1, 12)
    vals = list(process_instruction_set(vals,234))
    print(len(vals))
    for i, v in enumerate(vals):
        if v == 0:
            print(f'{i + 1}: {v}')

    vals = range(1, 500)
    vals = list(process_instruction_set(vals,216))
    print(len(vals))
    for i, v in enumerate(vals):
        if v in [19,29,39,49,59,69,79,89,99]:
            print(f'{i + 1}: {v}')

def program_types():

    alu = ALU()
    steps = alu.build_program(instructions)

    for s in steps:
        max_z = 0
        for i in range(1,10):
            func = getattr(alu, s[0])
            func(i, *s)
            if alu.z > max_z:
                max_z = alu.z
            alu.reset()
            alu.z = max_z
        print(max_z)


def stack_solution():
    def solve(inp):
        stack=[]
        alu = ALU()
        steps = alu.build_program(instructions)
        for i, s in enumerate(steps):
            print(f'{i}: {s}, stack: {stack}')
            print(inp)
            chk,add = s[1], s[2]
            if s[0] == 'increment':
                stack.append((i, add))
            else:
                j, add = stack.pop()
                print(f'decrement op: {j}, {inp[j]}, {add}, {chk}')
                inp[i] = inp[j] + add + chk
                if inp[i] > 9:
                    inp[j] -= inp[i] - 9
                    inp[i] = 9
                if inp[i] < 1:
                    inp[j] += 1 - inp[i]
                    inp[i] = 1
        return "".join(map(str, inp))
    
    print(solve([9] * 14))
    print(solve([1] * 14))

stack_solution()
