#!/usr/bin/python3

class Node:
    def __init__(self, num, index):
        self.next = None
        self.prev = None
        self.num = num
        self.index = index

    # def insert_right(self, new_node):
    #     new_node.next = self.next
    #     new_node.prev = self.prev
    #     self.next.prev = new_node
    #     self.prev.next = new_node

    def get_neighbors(self):
        return (self.prev, self.next)

    def __eq__(self, other):
        return self.num == other.num and self.index == other.index

    def __hash__(self):
        return hash(f'{self.num}[{self.index}]')

    def __repr__(self):
        return f'{self.num}'

def part1(key):

    with open('in') as f:
        nodes = [Node(int(l) * key, i) for (i, l) in enumerate(f.readlines())]

        zero_node = None
        for (i, n) in enumerate(nodes):
            if n.num == 0:
                zero_node = n
            if i == 0 or i == len(nodes):
                continue
            
            n.prev = nodes[i-1]
            n.prev.next = n
        nodes[len(nodes)-1].next = nodes[0]
        nodes[0].prev = nodes[len(nodes)-1]

        # num_map = {n: i for (i, n) in enumerate(nodes)}
        # print(num_map)

        orig = [n for n in nodes]
        orig_len = len(orig)

        # def validate():
        #     next_set = set()
        #     prev_set = set()
        #     next_node = nodes[0]
        #     prev_node = nodes[0]
        #     for n in range(orig_len):
        #         next_set.add(next_node)
        #         prev_set.add(prev_node)
        #         next_node = next_node.next
        #         prev_node = prev_node.prev
            
        #     unioned = next_set.union(prev_set)
        #     if len(unioned) != orig_len:
        #         print(next_set)
        #         print(prev_set)
        #         exit(1)
            

        def print_list():
            cur_node = nodes[0]
            for n in range(orig_len):
                print(f'{cur_node}', end='')
                cur_node = cur_node.next
                if n < orig_len - 1:
                    print(',', end='')
            print()

        # print_list()
        for _ in range(10):
            for cur in nodes:
        # while c < orig_len:
        #     cur = orig[c % orig_len]

                if cur.num == 0:
                    continue

                dir_attr = 'prev' if cur.num < 0 else 'next'
                # if abs(cur.num) > orig_len:
                iterations = abs(cur.num) % (orig_len - 1)
                # print(cur.num, iterations)
                # else:
                #     iterations = abs(cur.num)
                iter_node = cur
                # move cur pos
                    # [x]--[c]--[z]
                    # [x]--[z]
                    # c.prev - x
                    # c.next - z
                    # x.next = c
                    # z.prev = c
                # print(f'linking {cur.prev} and {cur.next}')
                cur_prev = cur.prev
                cur_next = cur.next
                cur_prev.next = cur_next
                cur_next.prev = cur_prev
                
                for r in range(iterations):
                    iter_node = getattr(iter_node, dir_attr)
                # print(f'{cur} moves between {iter_node} and {getattr(iter_node, dir_attr)}')

                if dir_attr == 'prev':
                    iter_node_prev = iter_node.prev
                    iter_node_prev.next = cur
                    iter_node.prev = cur
                    cur.next = iter_node
                    cur.prev = iter_node_prev
                else:
                    iter_node_next = iter_node.next
                    iter_node_next.prev = cur
                    iter_node.next = cur
                    cur.next = iter_node_next
                    cur.prev = iter_node
        total = 0
        for r in range(1,4):
            iter_node = zero_node
            for _ in range((r * 1000) % orig_len):
                iter_node = iter_node.next
            
            print(iter_node.num)
            total += iter_node.num
            
        print(total)

# trying with array
# def part1():
#     with open('in_test') as f:
#         nums = [int(l) for l in f.readlines()]

#         num_map = {n: i for (i, n) in enumerate(nums)}
#         # print(num_map)

#         orig = [n for n in nums]
#         orig_len = len(orig)
#         c = 0
#         while c < 19:
#             next = orig[c % orig_len]
#             c += 1

#             if next == 0:
#                 print('skip 0')
#                 continue

#             next_pos = None
#             cur_pos = num_map[next]

#             # figure out next position based on number, dir, cur pos and length of numbers
#             if next < 0:
#                 if abs(next) > orig_len:
#                     next_pos = cur_pos - abs(next) % orig_len
#                 else:
#                     next_pos = cur_pos - abs(next)
#             if next > 0:
#                 if next > orig_len:
#                     next_pos = next % orig_len + cur_pos
#                 else:
#                     next_pos = next + cur_pos
            
#             # orig_next_pos = next_pos
#             if next_pos > orig_len - 1:
#                 next_pos = next_pos - (orig_len - 1)
#             elif next_pos < 0:
#                 next_pos = (orig_len - 1) + next_pos 
#             elif next_pos == 0:
#                 next_pos = orig_len - 1
#             elif next_pos == (orig_len - 1):
#                 next_pos = 0
#             # if orig_next_pos != next_pos:
#                 # print(f'fixed pos: {orig_next_pos} -> {next_pos}')
            
#             num_map[next] = next_pos

#             if next_pos > cur_pos:
#                 # print(nums[:cur_pos], nums[cur_pos + 1:next_pos + 1], [next], nums[next_pos + 1:])
#                 decrement_nums = nums[:cur_pos] + nums[cur_pos + 1:next_pos + 1]
#                 nums = decrement_nums + [next] + nums[next_pos + 1:]
#                 for n in decrement_nums:
#                     num_map[n] -= 1
#             if next_pos < cur_pos:
#                 # print(nums[:next_pos], [next], nums[next_pos:cur_pos], nums[cur_pos + 1:])
#                 increment_nums = nums[next_pos:cur_pos] + nums[cur_pos + 1:]
#                 nums = nums[:next_pos] + [next] + increment_nums
#                 for n in increment_nums:
#                     num_map[n] += 1

#             print(f'[{c}]-- {next}: [{cur_pos} -> {next_pos}] - {nums}')

if __name__ == '__main__':
    key = 1
    part1(key)
    key = 811589153
    part1(key)